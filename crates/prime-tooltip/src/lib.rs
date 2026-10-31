//! Tooltip — порт директивы `pTooltip` из PrimeNG 21.1.10.
//!
//! Подсказка у любого элемента, подключаемая `use:`-директивой Leptos; разметка элемента не меняется. Подсказка показывается по
//! наведению мыши, касанию или фокусу, выбирает сторону и при нехватке места переносится на другую.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_tooltip::{TooltipOptions, TooltipPosition, tooltip};
//!
//! let html = view! {
//!     <input use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Top) placeholder="Top" />
//! }
//! .to_html();
//! // директива работает только в браузере: серверная разметка не меняется
//! assert!(!html.contains("tooltip") && html.contains("placeholder"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::Prop;

pub use state::{
    ArrowStyle, Size, TooltipEvent, TooltipPosition, activation_delay, arrow_style, has_ellipsis, leave_keeps_open, out_of_bounds, place,
    priority, root_classes,
};

/// Содержимое подсказки.
#[derive(Clone)]
pub enum TooltipContent {
    /// Текст (без разбора разметки); может быть сигналом.
    Text(Prop<String>),
    /// Разметка (аналог `[escape]="false"`): выводится как HTML.
    Html(String),
    /// Готовое представление (аналог шаблона `TemplateRef`).
    View(ViewFn),
}

impl Default for TooltipContent {
    fn default() -> Self {
        Self::Text(Prop::Static(String::new()))
    }
}

/// Параметры директивы [`tooltip`] (аналог входов `pTooltip`, `tooltipPosition` и т. д.).
#[derive(Clone)]
pub struct TooltipOptions {
    /// Содержимое.
    pub content: TooltipContent,
    /// Сторона элемента, с которой показывается подсказка.
    pub position: TooltipPosition,
    /// События показа.
    pub event: TooltipEvent,
    /// Задержка показа, мс.
    pub show_delay: Option<u32>,
    /// Задержка скрытия, мс.
    pub hide_delay: Option<u32>,
    /// Через сколько скрыть показанную подсказку, мс (после задержки показа).
    pub life: Option<u32>,
    /// Дополнительный сдвиг по вертикали, px.
    pub position_top: i32,
    /// Дополнительный сдвиг по горизонтали, px.
    pub position_left: i32,
    /// Скрывать подсказку, когда курсор уходит с элемента (иначе её можно навести мышью).
    pub auto_hide: bool,
    /// Ширина подсказки по содержимому (`width: fit-content`).
    pub fit_content: bool,
    /// Скрывать подсказку по `Escape`.
    pub hide_on_escape: bool,
    /// Показывать только если содержимое элемента не помещается в него.
    pub show_on_ellipsis: bool,
    /// Отключить подсказку.
    pub disabled: Prop<bool>,
    /// Значение CSS `position` подсказки вместо заданного темой.
    pub position_style: Option<String>,
    /// Дополнительный класс подсказки.
    pub style_class: Option<String>,
    /// Фиксированный `z-index`; `None` — автоматически по группе подсказок.
    pub z_index: Option<i32>,
    /// Атрибут `id` подсказки.
    pub id: Option<String>,
}

impl TooltipOptions {
    /// Параметры с содержимым.
    pub fn new(content: TooltipContent) -> Self {
        Self {
            content,
            position: TooltipPosition::default(),
            event: TooltipEvent::default(),
            show_delay: None,
            hide_delay: None,
            life: None,
            position_top: 0,
            position_left: 0,
            auto_hide: true,
            fit_content: true,
            hide_on_escape: true,
            show_on_ellipsis: false,
            disabled: Prop::Static(false),
            position_style: None,
            style_class: None,
            z_index: None,
            id: None,
        }
    }

    /// Текстовая подсказка.
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(TooltipContent::Text(Prop::Static(text.into())))
    }

    /// Текстовая подсказка, зависящая от сигнала.
    pub fn text_signal(text: impl Into<Signal<String>>) -> Self {
        Self::new(TooltipContent::Text(Prop::Reactive(text.into())))
    }

    /// Подсказка с разметкой.
    pub fn html(html: impl Into<String>) -> Self {
        Self::new(TooltipContent::Html(html.into()))
    }

    /// Подсказка с представлением.
    pub fn view(view: impl Into<ViewFn>) -> Self {
        Self::new(TooltipContent::View(view.into()))
    }

    /// Сторона элемента.
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// События показа.
    pub fn event(mut self, event: TooltipEvent) -> Self {
        self.event = event;
        self
    }

    /// Задержка показа, мс.
    pub fn show_delay(mut self, ms: u32) -> Self {
        self.show_delay = Some(ms);
        self
    }

    /// Задержка скрытия, мс.
    pub fn hide_delay(mut self, ms: u32) -> Self {
        self.hide_delay = Some(ms);
        self
    }

    /// Время жизни показанной подсказки, мс.
    pub fn life(mut self, ms: u32) -> Self {
        self.life = Some(ms);
        self
    }

    /// Дополнительные сдвиги `(по вертикали, по горизонтали)`, px.
    pub fn offset(mut self, top: i32, left: i32) -> Self {
        self.position_top = top;
        self.position_left = left;
        self
    }

    /// Скрывать ли подсказку при уходе курсора с элемента.
    pub fn auto_hide(mut self, auto_hide: bool) -> Self {
        self.auto_hide = auto_hide;
        self
    }

    /// Ширина подсказки по содержимому.
    pub fn fit_content(mut self, fit: bool) -> Self {
        self.fit_content = fit;
        self
    }

    /// Скрывать ли подсказку по `Escape`.
    pub fn hide_on_escape(mut self, hide: bool) -> Self {
        self.hide_on_escape = hide;
        self
    }

    /// Показывать только для обрезанного содержимого.
    pub fn show_on_ellipsis(mut self, only: bool) -> Self {
        self.show_on_ellipsis = only;
        self
    }

    /// Отключить подсказку (значение или сигнал).
    pub fn disabled(mut self, disabled: impl Into<Prop<bool>>) -> Self {
        self.disabled = disabled.into();
        self
    }

    /// Значение CSS `position`.
    pub fn position_style(mut self, position: impl Into<String>) -> Self {
        self.position_style = Some(position.into());
        self
    }

    /// Дополнительный класс.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Фиксированный `z-index`.
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = Some(z);
        self
    }

    /// Атрибут `id` подсказки.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Директива подсказки: `<input use:tooltip=TooltipOptions::text("…") />`.
///
/// Вызывается Leptos в браузере при построении и гидратации элемента (и при повторном построении атрибутов — тогда параметры
/// обновляются, а слушатели не дублируются); на сервере не вызывается. При размонтировании элемента слушатели и показанная
/// подсказка убираются.
pub fn tooltip(el: web_sys::Element, options: TooltipOptions) {
    #[cfg(target_arch = "wasm32")]
    imp::attach(el, options);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (el, options);
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::*;
    use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
    use prime_core::{ZGroup, use_prime_config};
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::Duration;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    use web_sys::{Event, EventTarget, HtmlElement, MouseEvent};

    /// Свойство элемента с номером его состояния (без DOM-атрибута).
    const KEY: &str = "__primeTooltip";

    type Undo = Box<dyn FnOnce()>;

    thread_local! {
        static TIPS: RefCell<HashMap<usize, Rc<Tip>>> = RefCell::new(HashMap::new());
        static NEXT: Cell<usize> = const { Cell::new(1) };
    }

    /// Подписка на событие; результат снимает её.
    fn listen(target: &EventTarget, name: &'static str, mut f: impl FnMut(Event) + 'static) -> Undo {
        let cb = Closure::<dyn FnMut(Event)>::new(move |e: Event| f(e));
        let _ = target.add_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
        let target = target.clone();
        Box::new(move || {
            let _ = target.remove_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
        })
    }

    /// Подписка в фазе захвата (для `scroll`, который не всплывает).
    fn listen_capture(target: &EventTarget, name: &'static str, mut f: impl FnMut(Event) + 'static) -> Undo {
        let cb = Closure::<dyn FnMut(Event)>::new(move |e: Event| f(e));
        let _ = target.add_event_listener_with_callback_and_bool(name, cb.as_ref().unchecked_ref(), true);
        let target = target.clone();
        Box::new(move || {
            let _ = target.remove_event_listener_with_callback_and_bool(name, cb.as_ref().unchecked_ref(), true);
        })
    }

    fn run(undos: Vec<Undo>) {
        for undo in undos {
            undo();
        }
    }

    /// Показанная подсказка в DOM.
    struct Container {
        root: HtmlElement,
        text: HtmlElement,
        arrow: HtmlElement,
        /// Смонтированное представление (`TooltipContent::View`).
        mount: Option<Box<dyn Any>>,
        /// Снятие слушателя `mouseleave` подсказки.
        leave: Option<Undo>,
    }

    struct Tip {
        id: usize,
        host: HtmlElement,
        options: RefCell<TooltipOptions>,
        container: RefCell<Option<Container>>,
        show_timer: Cell<Option<TimeoutHandle>>,
        hide_timer: Cell<Option<TimeoutHandle>>,
        active: Cell<bool>,
        interaction: Cell<bool>,
        host_listeners: RefCell<Vec<Undo>>,
        shown_listeners: RefCell<Vec<Undo>>,
        escape: RefCell<Option<Undo>>,
        touch_doc: RefCell<Option<Undo>>,
        z: Cell<Option<i32>>,
    }

    /// Подключает директиву к элементу или обновляет уже подключённую.
    pub(super) fn attach(el: web_sys::Element, options: TooltipOptions) {
        let existing = js_sys::Reflect::get(&el, &KEY.into())
            .ok()
            .and_then(|v| v.as_f64())
            .and_then(|id| TIPS.with_borrow(|m| m.get(&(id as usize)).cloned()));
        if let Some(tip) = existing {
            tip.update(options);
            return;
        }
        let Ok(host) = el.clone().dyn_into::<HtmlElement>() else { return };
        let id = NEXT.with(|n| {
            let id = n.get();
            n.set(id + 1);
            id
        });
        let _ = js_sys::Reflect::set(&el, &KEY.into(), &(id as f64).into());
        let tip = Rc::new(Tip {
            id,
            host,
            options: RefCell::new(options),
            container: RefCell::new(None),
            show_timer: Cell::new(None),
            hide_timer: Cell::new(None),
            active: Cell::new(false),
            interaction: Cell::new(false),
            host_listeners: RefCell::new(Vec::new()),
            shown_listeners: RefCell::new(Vec::new()),
            escape: RefCell::new(None),
            touch_doc: RefCell::new(None),
            z: Cell::new(None),
        });
        TIPS.with_borrow_mut(|m| m.insert(id, tip.clone()));
        tip.bind_host();
        {
            // Смена текста или `disabled` во время показа обновляет подсказку.
            let tip = tip.clone();
            Effect::new(move |_| {
                tip.track();
                tip.refresh();
            });
        }
        on_cleanup(move || {
            if let Some(tip) = TIPS.with_borrow_mut(|m| m.remove(&id)) {
                tip.destroy();
            }
        });
    }

    impl Tip {
        fn opts(&self) -> TooltipOptions {
            self.options.borrow().clone()
        }

        /// Читает реактивные части параметров, чтобы эффект следил за ними.
        fn track(&self) {
            let options = self.options.borrow();
            if let TooltipContent::Text(text) = &options.content {
                text.get();
            }
            options.disabled.get();
        }

        fn label(&self) -> String {
            match &self.options.borrow().content {
                TooltipContent::Text(text) => text.get_untracked(),
                TooltipContent::Html(html) => html.clone(),
                TooltipContent::View(_) => "view".to_owned(),
            }
        }

        fn update(self: &Rc<Self>, options: TooltipOptions) {
            *self.options.borrow_mut() = options;
            self.bind_host();
            self.refresh();
        }

        /// Слушатели элемента по выбранным событиям.
        fn bind_host(self: &Rc<Self>) {
            run(self.host_listeners.take());
            let event = self.options.borrow().event;
            let mut undos: Vec<Undo> = Vec::new();
            let host: EventTarget = self.host.clone().into();
            if event.hover() {
                let tip = self.clone();
                undos.push(listen(&host, "mouseenter", move |_| {
                    if tip.container.borrow().is_none() && tip.show_timer.take().is_none() {
                        tip.activate();
                    }
                }));
                let tip = self.clone();
                undos.push(listen(&host, "mouseleave", move |e| {
                    let related = e.dyn_ref::<MouseEvent>().and_then(MouseEvent::related_target);
                    let on_tooltip = related.and_then(|t| t.dyn_into::<web_sys::Element>().ok()).is_some_and(|el| {
                        let list = el.class_list();
                        list.contains("p-tooltip") || list.contains("p-tooltip-text") || list.contains("p-tooltip-arrow")
                    });
                    if !leave_keeps_open(tip.options.borrow().auto_hide, on_tooltip) {
                        tip.deactivate();
                    }
                }));
                let tip = self.clone();
                undos.push(listen(&host, "click", move |_| tip.deactivate()));
                let tip = self.clone();
                undos.push(listen(&host, "touchstart", move |_| {
                    if tip.container.borrow().is_none() && tip.show_timer.take().is_none() {
                        tip.activate();
                        if !tip.options.borrow().auto_hide {
                            tip.bind_touch_doc();
                        }
                    }
                }));
                let tip = self.clone();
                undos.push(listen(&host, "touchend", move |_| {
                    if tip.options.borrow().auto_hide {
                        tip.deactivate();
                    }
                }));
            }
            if event.focus() {
                let target: EventTarget = self.focus_target().into();
                let tip = self.clone();
                undos.push(listen(&target, "focus", move |_| tip.activate()));
                let tip = self.clone();
                undos.push(listen(&target, "blur", move |_| tip.deactivate()));
            }
            *self.host_listeners.borrow_mut() = undos;
        }

        /// Элемент, на котором слушается фокус: компонент внутри, поле в обёртке `p-inputwrapper` или сам элемент.
        fn focus_target(&self) -> HtmlElement {
            let inner = |sel: &str| self.host.query_selector(sel).ok().flatten().and_then(|e| e.dyn_into::<HtmlElement>().ok());
            inner(".p-component")
                .or_else(|| self.host.class_list().contains("p-inputwrapper").then(|| inner("input")).flatten())
                .unwrap_or_else(|| self.host.clone())
        }

        fn bind_touch_doc(self: &Rc<Self>) {
            if self.touch_doc.borrow().is_some() {
                return;
            }
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let tip = self.clone();
            let undo = listen(&doc.into(), "touchstart", move |e| {
                let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let in_tip = tip.container.borrow().as_ref().is_some_and(|c| c.root.contains(node.as_ref()));
                if tip.container.borrow().is_some() && !in_tip && !tip.host.contains(node.as_ref()) {
                    tip.deactivate();
                    if let Some(undo) = tip.touch_doc.take() {
                        undo();
                    }
                }
            });
            *self.touch_doc.borrow_mut() = Some(undo);
        }

        fn clear_show(&self) {
            if let Some(handle) = self.show_timer.take() {
                handle.clear();
            }
        }

        fn clear_hide(&self) {
            if let Some(handle) = self.hide_timer.take() {
                handle.clear();
            }
        }

        fn has_ellipsis(&self) -> bool {
            let h = &self.host;
            has_ellipsis(
                f64::from(h.offset_width()),
                f64::from(h.scroll_width()),
                f64::from(h.offset_height()),
                f64::from(h.scroll_height()),
            )
        }

        fn activate(self: &Rc<Self>) {
            if self.interaction.get() {
                return;
            }
            let o = self.opts();
            if o.show_on_ellipsis && !self.has_ellipsis() {
                return;
            }
            self.active.set(true);
            self.clear_hide();
            let (show_after, hide_after) = activation_delay(o.show_delay, o.life);
            match show_after {
                Some(ms) => {
                    let tip = self.clone();
                    if let Ok(handle) = set_timeout_with_handle(
                        move || {
                            tip.show_timer.take();
                            tip.show();
                        },
                        Duration::from_millis(u64::from(ms)),
                    ) {
                        self.show_timer.set(Some(handle));
                    }
                }
                None => self.show(),
            }
            if let Some(ms) = hide_after {
                let tip = self.clone();
                if let Ok(handle) = set_timeout_with_handle(move || tip.hide(), Duration::from_millis(u64::from(ms))) {
                    self.hide_timer.set(Some(handle));
                }
            }
            if o.hide_on_escape {
                self.unbind_escape();
                let tip = self.clone();
                let win: EventTarget = web_sys::window().expect("window").into();
                *self.escape.borrow_mut() = Some(listen(&win, "keydown", move |e| {
                    if e.dyn_ref::<web_sys::KeyboardEvent>().is_some_and(|k| k.key() == "Escape") {
                        tip.deactivate();
                    }
                }));
            }
            self.interaction.set(true);
        }

        fn unbind_escape(&self) {
            if let Some(undo) = self.escape.take() {
                undo();
            }
        }

        fn deactivate(self: &Rc<Self>) {
            self.interaction.set(false);
            self.active.set(false);
            self.clear_show();
            match self.options.borrow().hide_delay.filter(|d| *d > 0) {
                Some(ms) => {
                    self.clear_hide();
                    let tip = self.clone();
                    if let Ok(handle) = set_timeout_with_handle(move || tip.hide(), Duration::from_millis(u64::from(ms))) {
                        self.hide_timer.set(Some(handle));
                    }
                }
                None => self.hide(),
            }
            self.unbind_escape();
        }

        /// Создаёт подсказку в `body`, выравнивает, показывает и подписывается на события окна.
        fn show(self: &Rc<Self>) {
            let o = self.opts();
            if self.label().is_empty() || o.disabled.get_untracked() {
                return;
            }
            self.create(&o);
            if let Some(c) = self.container.borrow().as_ref() {
                let _ = c.root.style().set_property("display", "inline-block");
            }
            self.align();
            if let Some(c) = self.container.borrow().as_ref() {
                // Появление: прозрачность 0 → 1 переходом 250 мс (аналог `fadeIn` PrimeNG).
                let style = c.root.style();
                let _ = style.set_property("opacity", "0");
                let _ = style.set_property("transition", "opacity 250ms");
                let _ = c.root.offset_width();
                let _ = style.set_property("opacity", "1");
                match o.z_index {
                    Some(z) => {
                        let _ = style.set_property("z-index", &z.to_string());
                    }
                    None => {
                        let z = prime_core::zindex::generate(ZGroup::Tooltip, use_prime_config().z_index.base(ZGroup::Tooltip));
                        self.z.set(Some(z));
                        let _ = style.set_property("z-index", &z.to_string());
                    }
                }
            }
            self.bind_shown();
        }

        fn create(self: &Rc<Self>, o: &TooltipOptions) {
            self.remove();
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Some(body) = doc.body() else { return };
            let make = |class: &str, section: &str| -> Option<HtmlElement> {
                let el = doc.create_element("div").ok()?.dyn_into::<HtmlElement>().ok()?;
                el.set_class_name(class);
                let _ = el.set_attribute("data-pc-section", section);
                Some(el)
            };
            let (Some(root), Some(arrow), Some(text)) = (
                make(&root_classes(o.position, o.style_class.as_deref()), "root"),
                make("p-tooltip-arrow", "arrow"),
                make("p-tooltip-text", "text"),
            ) else {
                return;
            };
            let _ = root.set_attribute("role", "tooltip");
            if let Some(id) = &o.id {
                let _ = root.set_attribute("id", id);
            }
            let _ = root.append_child(&arrow);
            let _ = root.append_child(&text);
            let _ = body.append_child(&root);
            let style = root.style();
            let _ = style.set_property("display", "none");
            if let Some(position) = &o.position_style {
                let _ = style.set_property("position", position);
            }
            if o.fit_content {
                let _ = style.set_property("width", "fit-content");
            }
            let mut leave = None;
            if o.auto_hide {
                let _ = style.set_property("pointer-events", "none");
            } else {
                let _ = style.set_property("pointer-events", "unset");
                let tip = self.clone();
                leave = Some(listen(&root.clone().into(), "mouseleave", move |_| tip.deactivate()));
            }
            *self.container.borrow_mut() = Some(Container { root, text, arrow, mount: None, leave });
            self.update_text();
        }

        /// Записывает содержимое в `p-tooltip-text`.
        fn update_text(&self) {
            let content = self.options.borrow().content.clone();
            let mut slot = self.container.borrow_mut();
            let Some(c) = slot.as_mut() else { return };
            c.mount = None;
            c.text.set_text_content(None);
            match content {
                TooltipContent::Text(text) => c.text.set_text_content(Some(&text.get_untracked())),
                TooltipContent::Html(html) => c.text.set_inner_html(&html),
                TooltipContent::View(view) => {
                    let handle = leptos::mount::mount_to(c.text.clone(), move || view.run());
                    c.mount = Some(Box::new(handle));
                }
            }
        }

        /// Выбирает сторону: основную, а при выходе за окно — запасные по приоритету.
        fn align(&self) {
            let o = self.opts();
            for (index, position) in priority(o.position).into_iter().enumerate() {
                if index == 0 || self.out_of_bounds() {
                    self.align_to(position, &o);
                } else {
                    break;
                }
            }
        }

        fn out_of_bounds(&self) -> bool {
            let slot = self.container.borrow();
            let Some(c) = slot.as_ref() else { return false };
            let rect = c.root.get_bounding_client_rect();
            let viewport = prime_dom::dom::viewport();
            out_of_bounds(
                rect.left(),
                rect.top(),
                f64::from(c.root.offset_width()),
                f64::from(c.root.offset_height()),
                viewport.width,
                viewport.height,
            )
        }

        fn align_to(&self, position: TooltipPosition, o: &TooltipOptions) {
            let slot = self.container.borrow();
            let Some(c) = slot.as_ref() else { return };
            let style = c.root.style();
            let _ = style.set_property("left", "-999px");
            let _ = style.set_property("top", "-999px");
            c.root.set_class_name(&root_classes(position, o.style_class.as_deref()));
            let tip = Size { width: f64::from(c.root.offset_width()), height: f64::from(c.root.offset_height()) };
            let host = Size { width: f64::from(self.host.offset_width()), height: f64::from(self.host.offset_height()) };
            let rect = self.host.get_bounding_client_rect();
            let scroll = prime_dom::dom::scroll();
            let (dx, dy) = place(position, host, tip);
            let left = rect.left() + scroll.left + dx + f64::from(o.position_left);
            let top = rect.top() + scroll.top + dy + f64::from(o.position_top);
            let _ = style.set_property("left", &format!("{left}px"));
            let _ = style.set_property("top", &format!("{top}px"));
            let arrow = arrow_style(position, tip.width);
            let a = c.arrow.style();
            for (name, value) in [("top", arrow.top), ("right", arrow.right), ("bottom", arrow.bottom), ("left", arrow.left)] {
                match value {
                    Some(v) => {
                        let _ = a.set_property(name, &v);
                    }
                    None => {
                        let _ = a.remove_property(name);
                    }
                }
            }
        }

        /// События окна на время показа: изменение размера и прокрутка предка элемента скрывают подсказку.
        fn bind_shown(self: &Rc<Self>) {
            run(self.shown_listeners.take());
            let win: EventTarget = web_sys::window().expect("window").into();
            let mut undos: Vec<Undo> = Vec::new();
            let tip = self.clone();
            undos.push(listen(&win, "resize", move |_| tip.hide()));
            let tip = self.clone();
            undos.push(listen_capture(&win, "scroll", move |e| {
                let scrolled = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let is_document = scrolled.as_ref().is_some_and(|n| n.node_type() == web_sys::Node::DOCUMENT_NODE);
                let ancestor = scrolled.as_ref().is_some_and(|s| s.contains(Some(&tip.host)));
                // прокрутка самой страницы подсказку не трогает: она позиционирована в координатах страницы и едет вместе с ней
                if ancestor && !is_document && tip.container.borrow().is_some() {
                    tip.hide();
                }
            }));
            *self.shown_listeners.borrow_mut() = undos;
        }

        fn hide(&self) {
            if let Some(z) = self.z.take() {
                prime_core::zindex::revert(z);
            }
            self.remove();
        }

        /// Убирает подсказку из DOM, снимает события окна и таймеры.
        fn remove(&self) {
            let container = self.container.borrow_mut().take();
            if let Some(c) = container {
                c.root.remove();
                if let Some(leave) = c.leave {
                    leave();
                }
                drop(c.mount);
            }
            run(self.shown_listeners.take());
            self.clear_show();
            self.clear_hide();
        }

        /// Изменение текста или `disabled` во время показа.
        fn refresh(self: &Rc<Self>) {
            if !self.active.get() {
                return;
            }
            if self.options.borrow().disabled.get_untracked() {
                self.deactivate();
                return;
            }
            if self.label().is_empty() {
                self.hide();
            } else if self.container.borrow().is_some() {
                self.update_text();
                self.align();
            } else {
                self.show();
            }
        }

        fn destroy(&self) {
            run(self.host_listeners.take());
            self.unbind_escape();
            if let Some(undo) = self.touch_doc.take() {
                undo();
            }
            self.hide();
            let _ = js_sys::Reflect::delete_property(&self.host, &KEY.into());
            let _ = self.id;
        }
    }
}
