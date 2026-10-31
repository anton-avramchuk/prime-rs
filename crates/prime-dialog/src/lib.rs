//! Dialog — порт `p-dialog` из PrimeNG 21.1.10.
//!
//! Окно поверх страницы с заголовком, содержимым и подвалом: маска, анимации входа и выхода, перетаскивание за заголовок,
//! изменение размера, максимизация, закрытие кнопкой, `Escape` и (по настройке) кликом по маске, блокировка прокрутки страницы
//! и ловушка фокуса. Маска и окно выносятся в `body`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_dialog::{Dialog, DialogPosition};
//!
//! let visible = RwSignal::new(false);
//! let html = view! {
//!     <Dialog visible=visible header="Header" modal=true position=DialogPosition::Top>
//!         <p>"Body"</p>
//!     </Dialog>
//! }
//! .to_html();
//! // пока диалог не показан, в разметке его нет
//! assert!(!html.contains("p-dialog") && !html.contains("Body"));
//! ```

mod state;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{OptProp, Prop, Pt};
use prime_dom::{Portal, block_body_scroll, unblock_body_scroll};
use prime_icons::{TimesIcon, WindowMaximizeIcon, WindowMinimizeIcon};
use std::sync::Arc;

pub use state::{
    DialogPosition, breakpoints_css, data_p, drag_accept, drag_position, is_topmost, mask_animation_class, mask_classes, mask_inline_style,
    mask_z, resize_to, root_classes, root_inline_style,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::FocusTrap;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Запасное время выхода: анимация идёт 300 мс, но без стилей или при отключённой анимации `animationend` не придёт.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(700);

/// Pass-through частей [`Dialog`].
#[derive(Clone, Debug, Default)]
pub struct DialogPt {
    /// Окно.
    pub root: Pt,
    /// Маска.
    pub mask: Pt,
    /// Заголовок.
    pub header: Pt,
    /// Название в заголовке.
    pub title: Pt,
    /// Блок кнопок заголовка.
    pub header_actions: Pt,
    /// Кнопка максимизации.
    pub maximize_button: Pt,
    /// Кнопка закрытия.
    pub close_button: Pt,
    /// Содержимое.
    pub content: Pt,
    /// Подвал.
    pub footer: Pt,
    /// Ручка изменения размера.
    pub resize_handle: Pt,
}

/// Состояние, общее для обработчиков окна.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: usize,
    visible: RwSignal<bool>,
    /// Окно в DOM (пока играет анимация выхода, оно ещё в DOM).
    rendered: RwSignal<bool>,
    /// Идёт анимация выхода.
    leaving: RwSignal<bool>,
    maximized: RwSignal<bool>,
    mask: NodeRef<leptos::html::Div>,
    panel: NodeRef<leptos::html::Div>,
    header: NodeRef<leptos::html::Div>,
    content: NodeRef<leptos::html::Div>,
    footer: NodeRef<leptos::html::Div>,
    position: Prop<DialogPosition>,
    modal: Prop<bool>,
    dismissable_mask: Prop<bool>,
    close_on_escape: Prop<bool>,
    closable: Prop<bool>,
    draggable: Prop<bool>,
    resizable: Prop<bool>,
    keep_in_viewport: Prop<bool>,
    block_scroll: Prop<bool>,
    focus_on_show: Prop<bool>,
    focus_trap: Prop<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    min_x: Prop<f64>,
    min_y: Prop<f64>,
    breakpoints: Vec<(String, String)>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    gesture: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    modal_blocked: StoredValue<bool>,
    max_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    /// Последняя точка указателя (`pageX`, `pageY`) для перетаскивания и изменения размера.
    last: StoredValue<(f64, f64)>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_resize_init: Option<Callback<web_sys::MouseEvent>>,
    on_resize_end: Option<Callback<web_sys::MouseEvent>>,
    on_drag_end: Option<Callback<web_sys::MouseEvent>>,
    on_maximize: Option<Callback<bool>>,
    #[cfg(target_arch = "wasm32")]
    trap: StoredValue<Option<FocusTrap>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    style_el: StoredValue<Option<web_sys::Element>, LocalStorage>,
}

impl Core {
    /// Атрибут окна, на который ссылаются правила `breakpoints`.
    #[cfg(target_arch = "wasm32")]
    fn attr_name(&self) -> String {
        format!("data-dlg-{}", self.id)
    }

    /// Снимает слушатели `Escape` и жестов и ловушку фокуса.
    fn release_input(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        self.stop_gesture();
        #[cfg(target_arch = "wasm32")]
        {
            self.trap.try_update_value(|t| t.take());
        }
    }

    /// Снимает слушатели `mousemove`/`mouseup` жеста и возвращает выделение текста.
    fn stop_gesture(&self) {
        let handles = self.gesture.try_update_value(std::mem::take).unwrap_or_default();
        if handles.is_empty() {
            return;
        }
        for cleanup in handles {
            cleanup();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) {
            let _ = body.remove_attribute("data-p-unselectable-text");
            let _ = body.style().remove_property("user-select");
        }
    }

    /// Освобождает слой, блокировки прокрутки и элемент `style` с правилами `breakpoints`.
    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if self.modal_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            unblock_body_scroll();
        }
        if self.max_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            unblock_body_scroll();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(el) = self.style_el.try_update_value(Option::take).flatten() {
            el.remove();
        }
    }

    /// Полное освобождение при размонтировании компонента.
    fn teardown(&self) {
        self.release_input();
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Переключает максимизацию; немодальный диалог без `block_scroll` блокирует прокрутку страницы.
    fn maximize(&self) {
        let now = !self.maximized.get_untracked();
        self.maximized.set(now);
        if !self.modal.get_untracked() && !self.block_scroll.get_untracked() {
            if now {
                block_body_scroll();
                self.max_blocked.set_value(true);
            } else if self.max_blocked.try_update_value(std::mem::take).unwrap_or(false) {
                unblock_body_scroll();
            }
        }
        if let Some(cb) = self.on_maximize {
            cb.run(now);
        }
    }

    /// Окно только что попало в DOM: слой, блокировка прокрутки, ловушка фокуса, Escape, `breakpoints`, фокус, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let Some(panel) = self.panel.get_untracked() else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Modal) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Modal, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
            if let Some(mask) = self.mask.get_untracked() {
                let mask: web_sys::HtmlElement = mask.unchecked_into();
                let _ = mask.style().set_property("z-index", &mask_z(z).to_string());
            }
        }
        if self.modal.get_untracked() {
            block_body_scroll();
            self.modal_blocked.set_value(true);
        }
        if !self.breakpoints.is_empty() {
            let attr = self.attr_name();
            let _ = panel.set_attribute(&attr, "");
            if let Some(doc) = web_sys::window().and_then(|w| w.document())
                && let (Some(head), Ok(style)) = (doc.head(), doc.create_element("style"))
            {
                style.set_text_content(Some(&breakpoints_css(&attr, &self.breakpoints)));
                let _ = head.append_child(&style);
                self.style_el.set_value(Some(style));
            }
        }
        if self.focus_trap.get_untracked() {
            self.trap.set_value(FocusTrap::attach(&panel));
        }
        if self.close_on_escape.get_untracked() && self.closable.get_untracked() {
            let this = self.clone();
            let handle = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
                if e.key() != "Escape" || !this.visible.get_untracked() {
                    return;
                }
                let own = this.z_index.get_value().unwrap_or(0);
                if is_topmost(own, prime_core::zindex::current()) {
                    this.visible.set(false);
                }
            });
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.remove())));
        }
        if self.focus_on_show.get_untracked() {
            let this = self.clone();
            set_timeout(move || this.focus(), Duration::from_millis(5));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Фокус на первый фокусируемый элемент содержимого, затем подвала и заголовка.
    #[cfg(target_arch = "wasm32")]
    fn focus(&self) {
        if self.rendered.is_disposed() {
            return;
        }
        for target in [self.content, self.footer, self.header] {
            if let Some(el) = target.get_untracked()
                && let Some(focusable) = prime_dom::first_focusable(&el)
            {
                let _ = focusable.focus();
                return;
            }
        }
    }

    /// Закрытие началось: слушатели сняты, играет анимация выхода, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.release_input();
        self.leaving.set(true);
        let this = self.clone();
        let handle = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK);
        if let Ok(handle) = handle {
            self.timer.set_value(Some(handle));
        }
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    /// Анимация выхода закончилась: окно убирается из DOM, слой и блокировки освобождаются, максимизация сбрасывается.
    #[cfg(target_arch = "wasm32")]
    fn finish_leave(&self) {
        if self.rendered.is_disposed() || !self.leaving.get_untracked() {
            return;
        }
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        self.release_layer();
        self.maximized.set(false);
        self.leaving.set(false);
        self.rendered.set(false);
    }

    /// Начало жеста: запоминает точку, запрещает выделение текста и подписывается на `mousemove`/`mouseup` окна.
    #[cfg(target_arch = "wasm32")]
    fn start_gesture(
        self: &Arc<Self>,
        e: &web_sys::MouseEvent,
        on_move: impl Fn(&Arc<Core>, &web_sys::MouseEvent) + 'static,
        on_up: impl Fn(&Arc<Core>, &web_sys::MouseEvent) + 'static,
    ) {
        self.stop_gesture();
        self.last.set_value((f64::from(e.page_x()), f64::from(e.page_y())));
        if let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) {
            let _ = body.set_attribute("data-p-unselectable-text", "true");
            let _ = body.style().set_property("user-select", "none");
        }
        let this = self.clone();
        let moving = window_event_listener(leptos::ev::mousemove, move |e: web_sys::MouseEvent| on_move(&this, &e));
        let this = self.clone();
        let up = window_event_listener(leptos::ev::mouseup, move |e: web_sys::MouseEvent| on_up(&this, &e));
        self.gesture.update_value(|g| {
            g.push(Box::new(move || moving.remove()));
            g.push(Box::new(move || up.remove()));
        });
    }

    /// `mousedown` на заголовке: перетаскивание, если нажали не на кнопки.
    #[cfg(target_arch = "wasm32")]
    fn init_drag(self: &Arc<Self>, e: &web_sys::MouseEvent) {
        if !self.draggable.get_untracked() {
            return;
        }
        let on_actions = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|el| el.closest("[data-pc-section=headeractions]").ok().flatten().is_some());
        if on_actions {
            return;
        }
        if let Some(panel) = self.panel.get_untracked() {
            let panel: web_sys::HtmlElement = panel.unchecked_into();
            let _ = panel.style().set_property("margin", "0");
        }
        self.start_gesture(
            e,
            |this, e| this.on_drag(e),
            |this, e| {
                this.stop_gesture();
                if let Some(cb) = this.on_drag_end {
                    cb.run(e.clone());
                }
            },
        );
    }

    #[cfg(target_arch = "wasm32")]
    fn on_drag(&self, e: &web_sys::MouseEvent) {
        let Some(panel) = self.panel.get_untracked() else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        let margin = |name: &str| {
            web_sys::window()
                .and_then(|w| w.get_computed_style(&panel).ok().flatten())
                .and_then(|s| s.get_property_value(name).ok())
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        let (page_x, page_y) = (f64::from(e.page_x()), f64::from(e.page_y()));
        let (last_x, last_y) = self.last.get_value();
        let rect = client_origin(&panel);
        let (left, top) = drag_position(rect.0, rect.1, margin("margin-left"), margin("margin-top"), page_x - last_x, page_y - last_y);
        let viewport = prime_dom::dom::viewport();
        let (accept_x, accept_y) = drag_accept(
            left,
            top,
            f64::from(panel.offset_width()),
            f64::from(panel.offset_height()),
            viewport.width,
            viewport.height,
            self.min_x.get_untracked(),
            self.min_y.get_untracked(),
            self.keep_in_viewport.get_untracked(),
        );
        let style = panel.style();
        let _ = style.set_property("position", "fixed");
        if accept_x {
            self.last.update_value(|l| l.0 = page_x);
            let _ = style.set_property("left", &format!("{left}px"));
        }
        if accept_y {
            self.last.update_value(|l| l.1 = page_y);
            let _ = style.set_property("top", &format!("{top}px"));
        }
    }

    /// `mousedown` на ручке: изменение размера.
    #[cfg(target_arch = "wasm32")]
    fn init_resize(self: &Arc<Self>, e: &web_sys::MouseEvent) {
        if !self.resizable.get_untracked() {
            return;
        }
        self.start_gesture(
            e,
            |this, e| this.on_resize(e),
            |this, e| {
                this.stop_gesture();
                if let Some(cb) = this.on_resize_end {
                    cb.run(e.clone());
                }
            },
        );
        if let Some(cb) = self.on_resize_init {
            cb.run(e.clone());
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn on_resize(&self, e: &web_sys::MouseEvent) {
        let (Some(panel), Some(content)) = (self.panel.get_untracked(), self.content.get_untracked()) else { return };
        let (panel, content): (web_sys::HtmlElement, web_sys::HtmlElement) = (panel.unchecked_into(), content.unchecked_into());
        let (page_x, page_y) = (f64::from(e.page_x()), f64::from(e.page_y()));
        let (last_x, last_y) = self.last.get_value();
        let style = panel.style();
        let px = |name: &str| style.get_property_value(name).ok().and_then(|v| v.trim_end_matches("px").parse::<f64>().ok());
        let dragged = |name: &str| px(name).is_some_and(|v| v != 0.0);
        let (width, height) = (f64::from(panel.offset_width()), f64::from(panel.offset_height()));
        let viewport = prime_dom::dom::viewport();
        let rect = client_origin(&panel);
        let (new_width, new_height) = resize_to(
            width,
            height,
            page_x - last_x,
            page_y - last_y,
            !(dragged("top") && dragged("left")),
            px("min-width"),
            px("min-height"),
            rect.0,
            rect.1,
            viewport.width,
            viewport.height,
        );
        if let Some(w) = new_width {
            let _ = style.set_property("width", &format!("{w}px"));
        }
        if let Some(h) = new_height {
            let content_height = f64::from(content.offset_height());
            let _ = content.style().set_property("height", &format!("{}px", content_height + h - height));
            if !style.get_property_value("height").unwrap_or_default().is_empty() {
                let _ = style.set_property("height", &format!("{h}px"));
            }
        }
        self.last.set_value((page_x, page_y));
    }
}

/// Левый и верхний край элемента в координатах окна (`getBoundingClientRect`).
#[cfg(target_arch = "wasm32")]
fn client_origin(el: &web_sys::HtmlElement) -> (f64, f64) {
    let rect = el.get_bounding_client_rect();
    (rect.left(), rect.top())
}

/// Окно поверх страницы с заголовком, содержимым и подвалом, которое можно перетаскивать, масштабировать и разворачивать.
///
/// Каждое свойство принимает значение или сигнал; `visible` — двусторонний сигнал. Маска и окно выводятся в `body`
/// и до гидратации не отдаются сервером.
#[component]
pub fn Dialog(
    /// Двусторонний сигнал видимости; без него состояние хранится внутри (по умолчанию скрыт).
    #[prop(optional)]
    visible: Option<RwSignal<bool>>,
    /// Название в заголовке.
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Положение окна на экране.
    #[prop(optional, into)]
    position: Prop<DialogPosition>,
    /// Модальная маска: блокирует страницу и прокрутку.
    #[prop(optional, into)]
    modal: Prop<bool>,
    /// Закрывать диалог нажатием на маску.
    #[prop(optional, into)]
    dismissable_mask: Prop<bool>,
    /// Закрывать диалог клавишей `Escape`.
    #[prop(optional, default = Prop::Static(true), into)]
    close_on_escape: Prop<bool>,
    /// Показывать кнопку закрытия (и разрешать закрытие по `Escape` и маске).
    #[prop(optional, default = Prop::Static(true), into)]
    closable: Prop<bool>,
    /// Разрешить перетаскивание за заголовок.
    #[prop(optional, default = Prop::Static(true), into)]
    draggable: Prop<bool>,
    /// Разрешить изменение размера.
    #[prop(optional, default = Prop::Static(true), into)]
    resizable: Prop<bool>,
    /// Показывать кнопку максимизации.
    #[prop(optional, into)]
    maximizable: Prop<bool>,
    /// Не выпускать окно за пределы экрана при перетаскивании.
    #[prop(optional, default = Prop::Static(true), into)]
    keep_in_viewport: Prop<bool>,
    /// Минимальная координата `left` при перетаскивании.
    #[prop(optional, into)]
    min_x: Prop<f64>,
    /// Минимальная координата `top` при перетаскивании.
    #[prop(optional, into)]
    min_y: Prop<f64>,
    /// Показывать заголовок.
    #[prop(optional, default = Prop::Static(true), into)]
    show_header: Prop<bool>,
    /// Блокировать прокрутку страницы при максимизации немодального диалога.
    #[prop(optional, into)]
    block_scroll: Prop<bool>,
    /// Переводить фокус на первый фокусируемый элемент при показе.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_on_show: Prop<bool>,
    /// Не выпускать фокус за пределы окна.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_trap: Prop<bool>,
    /// Автоматически назначать `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Базовый `z-index` поверх значения конфигурации.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Ширина окна по ширине экрана: пары «максимальная ширина экрана — ширина окна», например `("575px", "90vw")`.
    #[prop(optional, into)]
    breakpoints: Vec<(String, String)>,
    /// Атрибут `role` окна.
    #[prop(optional, default = Prop::Static("dialog".to_owned()), into)]
    role: Prop<String>,
    /// Атрибут `aria-label` кнопки закрытия.
    #[prop(optional, into)]
    close_aria_label: OptProp<String>,
    /// Встроенный стиль содержимого.
    #[prop(optional, into)]
    content_style: OptProp<String>,
    /// Дополнительный класс содержимого.
    #[prop(optional, into)]
    content_style_class: OptProp<String>,
    /// Встроенный стиль маски.
    #[prop(optional, into)]
    mask_style: OptProp<String>,
    /// Дополнительный класс маски.
    #[prop(optional, into)]
    mask_style_class: OptProp<String>,
    /// Встроенный стиль окна.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс окна.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Значок кнопки закрытия вместо `TimesIcon`.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// Значок кнопки максимизации вместо `WindowMaximizeIcon`.
    #[prop(optional, into)]
    maximize_icon: Option<ViewFn>,
    /// Значок кнопки возврата размера вместо `WindowMinimizeIcon`.
    #[prop(optional, into)]
    minimize_icon: Option<ViewFn>,
    /// Своё содержимое заголовка вместо названия.
    #[prop(optional, into)]
    header_content: Option<ViewFn>,
    /// Подвал окна.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Заменяет всё содержимое окна (заголовок, содержимое, подвал).
    #[prop(optional, into)]
    headless: Option<ViewFn>,
    /// Вызывается после показа.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Вызывается, когда диалог начинает закрываться.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Вызывается в начале изменения размера.
    #[prop(optional, into)]
    on_resize_init: Option<Callback<web_sys::MouseEvent>>,
    /// Вызывается в конце изменения размера.
    #[prop(optional, into)]
    on_resize_end: Option<Callback<web_sys::MouseEvent>>,
    /// Вызывается в конце перетаскивания.
    #[prop(optional, into)]
    on_drag_end: Option<Callback<web_sys::MouseEvent>>,
    /// Вызывается при максимизации (`true`) и возврате размера (`false`).
    #[prop(optional, into)]
    on_maximize: Option<Callback<bool>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<DialogPt>,
    /// Содержимое окна.
    children: ChildrenFn,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let visible = visible.unwrap_or_default();
    let core = Arc::new(Core {
        id: prime_core::ids::next_unique(),
        visible,
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        maximized: RwSignal::new(false),
        mask: NodeRef::new(),
        panel: NodeRef::new(),
        header: NodeRef::new(),
        content: NodeRef::new(),
        footer: NodeRef::new(),
        position,
        modal,
        dismissable_mask,
        close_on_escape,
        closable,
        draggable,
        resizable,
        keep_in_viewport,
        block_scroll,
        focus_on_show,
        focus_trap,
        auto_z_index,
        base_z_index,
        min_x,
        min_y,
        breakpoints,
        cleanups: StoredValue::new_local(Vec::new()),
        gesture: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        modal_blocked: StoredValue::new(false),
        max_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        last: StoredValue::new((0.0, 0.0)),
        on_show,
        on_hide,
        on_resize_init,
        on_resize_end,
        on_drag_end,
        on_maximize,
        #[cfg(target_arch = "wasm32")]
        trap: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        style_el: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }
    // Показ, закрытие и побочные эффекты (слой, ловушка фокуса, Escape, жесты) — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.visible.get();
            // Окно попадает в DOM после этого эффекта, поэтому следим и за ссылкой на него.
            let mounted = core.panel.get().is_some();
            let was_open = core.was_open.get_value();
            if shown {
                if let Some(handle) = core.timer.try_update_value(Option::take).flatten() {
                    handle.clear();
                }
                // Повторный показ во время выхода: окно ещё в DOM, возвращаем его в состояние «открыто».
                if core.leaving.get_untracked() {
                    core.leaving.set(false);
                }
                if !core.rendered.get_untracked() {
                    core.rendered.set(true);
                }
                if mounted && !was_open {
                    core.was_open.set_value(true);
                    core.opened();
                }
            } else if was_open {
                core.was_open.set_value(false);
                core.begin_leave();
            } else if core.rendered.get_untracked() && !core.leaving.get_untracked() {
                // Закрыли, не дождавшись появления окна в DOM.
                core.rendered.set(false);
            }
        });
    }

    let parts = Arc::new(Parts {
        header,
        role,
        close_aria_label,
        show_header,
        maximizable,
        content_style,
        content_style_class,
        mask_style,
        mask_style_class,
        style,
        style_class,
        close_icon,
        maximize_icon,
        minimize_icon,
        header_content,
        footer,
        headless,
        children,
    });
    // Содержимое строится заново внутри `Portal` при каждом показе: готовые представления не `Send + Sync`.
    move || {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        core.rendered.get().then(move || view! { <Portal>{build_dialog(&core, &pt, &parts)}</Portal> })
    }
}

/// Свойства, нужные только при выводе окна.
struct Parts {
    header: OptProp<String>,
    role: Prop<String>,
    close_aria_label: OptProp<String>,
    show_header: Prop<bool>,
    maximizable: Prop<bool>,
    content_style: OptProp<String>,
    content_style_class: OptProp<String>,
    mask_style: OptProp<String>,
    mask_style_class: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    close_icon: Option<ViewFn>,
    maximize_icon: Option<ViewFn>,
    minimize_icon: Option<ViewFn>,
    header_content: Option<ViewFn>,
    footer: Option<ViewFn>,
    headless: Option<ViewFn>,
    children: ChildrenFn,
}

/// Добавляет пользовательский класс к базовому.
fn with_user_class(mut base: String, user: &OptProp<String>) -> String {
    user.with(|c| {
        if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
            base.push(' ');
            base.push_str(c);
        }
    });
    base
}

/// Кнопка заголовка: вторичная, текстовая, круглая.
fn header_button(
    class: &'static str,
    icon: ViewFn,
    label: Signal<Option<String>>,
    on_click: Callback<web_sys::MouseEvent>,
    pt: Pt,
) -> AnyView {
    view! {
        <Button
            button_type="button"
            text=true
            rounded=true
            severity=ButtonSeverity::Secondary
            icon=icon
            aria_label=label
            style_class=class
            on_click=on_click
            pt=ButtonPt { root: pt, ..Default::default() }
        />
    }
    .into_any()
}

/// Маска и окно.
fn build_dialog(core: &Arc<Core>, pt: &Arc<DialogPt>, parts: &Arc<Parts>) -> AnyView {
    let (mask_ref, panel_ref, header_ref, content_ref, footer_ref) = (core.mask, core.panel, core.header, core.content, core.footer);
    let (leaving, maximized, visible) = (core.leaving, core.maximized, core.visible);
    let label_id = format!("pn_id_{}_header", core.id);

    let content = if let Some(headless) = parts.headless.clone() {
        headless.run().into_any()
    } else {
        let handle = core.resizable.get_untracked().then(|| {
            let core = core.clone();
            view! {
                <div
                    class={pt.resize_handle.merge_class("p-resizable-handle")}
                    style="z-index: 90"
                    on:mousedown=move |e: web_sys::MouseEvent| resize_down(&core, &e)
                    {..pt.resize_handle.attrs()}
                ></div>
            }
        });

        let header = parts.show_header.get_untracked().then(|| {
            let title = match parts.header_content.clone() {
                Some(custom) => custom.run().into_any(),
                None => {
                    let text = parts.header.get();
                    view! { <span id=label_id.clone() class={pt.title.merge_class("p-dialog-title")} {..pt.title.attrs()}>{text}</span> }.into_any()
                }
            };
            let maximize_button = parts.maximizable.get_untracked().then(|| {
                let (custom_max, custom_min) = (parts.maximize_icon.clone(), parts.minimize_icon.clone());
                let icon = ViewFn::from(move || {
                    let (custom_max, custom_min) = (custom_max.clone(), custom_min.clone());
                    move || {
                        if maximized.get() {
                            custom_min.clone().map_or_else(|| view! { <WindowMinimizeIcon /> }.into_any(), |c| c.run().into_any())
                        } else {
                            custom_max.clone().map_or_else(|| view! { <WindowMaximizeIcon /> }.into_any(), |c| c.run().into_any())
                        }
                    }
                });
                let label = Signal::derive(move || {
                    let locale = prime_core::use_prime_config().locale;
                    Some(locale.text(if maximized.get() { "aria.minimizeLabel" } else { "aria.maximizeLabel" }).to_owned())
                });
                let core = core.clone();
                header_button(
                    "p-dialog-maximize-button",
                    icon,
                    label,
                    Callback::new(move |_| core.maximize()),
                    pt.maximize_button.clone(),
                )
            });
            let close_button = core.closable.get_untracked().then(|| {
                let icon = parts.close_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <TimesIcon /> }));
                let label = parts
                    .close_aria_label
                    .get()
                    .unwrap_or_else(|| prime_core::use_prime_config().locale.text("aria.close").to_owned());
                header_button(
                    "p-dialog-close-button",
                    icon,
                    Signal::stored(Some(label)),
                    Callback::new(move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        visible.set(false);
                    }),
                    pt.close_button.clone(),
                )
            });
            let core = core.clone();
            view! {
                <div
                    node_ref=header_ref
                    class={pt.header.merge_class("p-dialog-header")}
                    on:mousedown=move |e: web_sys::MouseEvent| drag_down(&core, &e)
                    {..pt.header.attrs()}
                >
                    {title}
                    <div class={pt.header_actions.merge_class("p-dialog-header-actions")} data-pc-section="headeractions" {..pt.header_actions.attrs()}>
                        {maximize_button}
                        {close_button}
                    </div>
                </div>
            }
        });

        let content_class = with_user_class(String::from("p-dialog-content"), &parts.content_style_class);
        let content_style = parts.content_style.get();
        let footer = parts.footer.clone().map(|footer| {
            view! {
                <div node_ref=footer_ref class={pt.footer.merge_class("p-dialog-footer")} {..pt.footer.attrs()}>
                    {footer.run()}
                </div>
            }
        });
        view! {
            {handle}
            {header}
            <div node_ref=content_ref class={pt.content.merge_class(&content_class)} style={content_style} {..pt.content.attrs()}>
                {(parts.children)()}
            </div>
            {footer}
        }
        .into_any()
    };

    let root_class = {
        let (pt, style_class) = (pt.clone(), parts.style_class.clone());
        move || pt.root.merge_class(&with_user_class(root_classes(maximized.get(), leaving.get()), &style_class))
    };
    let root_style = {
        let user = parts.style.get().unwrap_or_default();
        pt.root.merge_style_parts(&[root_inline_style(), &user])
    };
    let mask_class = {
        let (pt, position, modal, user) = (pt.clone(), core.position.clone(), core.modal.clone(), parts.mask_style_class.clone());
        move || {
            let (modal, position) = (modal.get(), position.get());
            let mut base = mask_classes(position, modal);
            let animation = mask_animation_class(modal, leaving.get());
            if !animation.is_empty() {
                base.push(' ');
                base.push_str(animation);
            }
            pt.mask.merge_class(&with_user_class(base, &user))
        }
    };
    let mask_style = {
        let (pt, position, modal, user) = (pt.clone(), core.position.clone(), core.modal.clone(), parts.mask_style.clone());
        move || pt.mask.merge_style_parts(&[&mask_inline_style(position.get(), modal.get()), &user.get().unwrap_or_default()])
    };

    let (data_core, mask_core, end_core, role_parts) = (core.clone(), core.clone(), core.clone(), parts.clone());
    view! {
        <div
            node_ref=mask_ref
            class={mask_class}
            style={mask_style}
            data-p={
                let core = core.clone();
                move || data_p(core.maximized.get(), core.modal.get())
            }
            on:mousedown=move |e: web_sys::MouseEvent| mask_down(&mask_core, &e)
            {..pt.mask.attrs()}
        >
            <div
                node_ref=panel_ref
                class={root_class}
                style={root_style}
                role={move || role_parts.role.get()}
                aria-modal="true"
                aria-labelledby={label_id}
                data-p={move || data_p(data_core.maximized.get(), data_core.modal.get())}
                on:animationend=move |e: web_sys::AnimationEvent| animation_end(&end_core, &e)
                {..pt.root.attrs()}
            >
                {content}
            </div>
        </div>
    }
    .into_any()
}

/// Нажатие на самой маске (не на окне) закрывает диалог при `dismissable_mask` и `closable`.
fn mask_down(core: &Arc<Core>, event: &web_sys::MouseEvent) {
    let on_mask = event.target().is_some_and(|t| event.current_target().is_some_and(|c| t == c));
    if on_mask && core.dismissable_mask.get_untracked() && core.closable.get_untracked() {
        core.visible.set(false);
    }
}

/// `animationend` окна (не вложенных элементов) завершает выход.
fn animation_end(core: &Arc<Core>, event: &web_sys::AnimationEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        let own = event.target().is_some_and(|t| event.current_target().is_some_and(|c| t == c));
        if own && core.leaving.get_untracked() {
            core.finish_leave();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, event);
}

fn drag_down(core: &Arc<Core>, event: &web_sys::MouseEvent) {
    #[cfg(target_arch = "wasm32")]
    core.init_drag(event);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, event);
}

fn resize_down(core: &Arc<Core>, event: &web_sys::MouseEvent) {
    #[cfg(target_arch = "wasm32")]
    core.init_resize(event);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, event);
}
