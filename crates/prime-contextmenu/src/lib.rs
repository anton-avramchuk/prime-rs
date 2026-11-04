//! ContextMenu — порт `p-contextmenu` из PrimeNG 21.1.10.
//!
//! Меню с вложенными подменю, которое открывается у курсора по правому клику (событие `contextmenu`) на цели, на всём документе
//! (`global`) или вручную через [`ContextMenuRef`]; на сенсорных устройствах — долгим касанием (`press_delay`). Пункты —
//! [`prime_menu::MenuItem`]; логика путей, клавиатуры и поиска — общая с `TieredMenu` (крейт `prime-tieredmenu`).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_contextmenu::{ContextMenu, ContextMenuRef};
//! use prime_menu::MenuItem;
//!
//! let model = vec![MenuItem::new().label("Copy"), MenuItem::new().label("Rename")];
//! let html = Owner::new().with(|| {
//!     let menu = ContextMenuRef::new();
//!     menu.show_at(10.0, 20.0);
//!     view! { <ContextMenu model=model popup_ref=menu /> }.to_html()
//! });
//! assert!(html.contains("p-contextmenu-root-list") && html.contains("Rename"));
//! ```

mod place;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_icons::AngleRightIcon;
use prime_menu::{MenuCommand, MenuItem, can_run_command, should_prevent_default};
#[cfg(target_arch = "wasm32")]
use prime_tieredmenu::submenu_side;
use prime_tieredmenu::{Click, Focus, Node, State, Tree, aria_pos_inset, aria_set_size, build, is_printable, item_id};
use std::collections::HashMap;
use std::sync::Arc;

pub use place::{Viewport, place};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through частей [`ContextMenu`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContextMenuPt {
    /// Корень.
    pub root: Pt,
    /// Корневой список.
    pub root_list: Pt,
    /// Пункт `li`.
    pub item: Pt,
    /// Содержимое пункта.
    pub item_content: Pt,
    /// Ссылка пункта.
    pub item_link: Pt,
    /// Значок пункта.
    pub item_icon: Pt,
    /// Подпись пункта.
    pub item_label: Pt,
    /// Стрелка группы.
    pub submenu_icon: Pt,
    /// Подменю.
    pub submenu: Pt,
    /// Разделитель.
    pub separator: Pt,
}

/// Контекст представления `item`.
#[derive(Clone)]
pub struct ContextMenuItemContext {
    /// Пункт.
    pub item: MenuItem,
    /// Есть ли у пункта подменю.
    pub has_submenu: bool,
}

/// Управляющий объект меню: показ в точке страницы, скрытие, переключение.
#[derive(Clone, Copy)]
pub struct ContextMenuRef {
    visible: RwSignal<bool>,
    /// Точка показа (координаты страницы) и номер показа: повторный показ при открытом меню только переставляет панель.
    at: RwSignal<(f64, f64, u32)>,
}

impl Default for ContextMenuRef {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMenuRef {
    /// Создаёт скрытое меню. Вызывать внутри компонента (владелец сигналов).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false), at: RwSignal::new((0.0, 0.0, 0)) }
    }

    /// Меню показано (с подпиской).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// Показывает меню в точке (`page_x`, `page_y`) страницы.
    pub fn show_at(&self, page_x: f64, page_y: f64) {
        self.at.update(|(x, y, n)| {
            *x = page_x;
            *y = page_y;
            *n = n.wrapping_add(1);
        });
        self.visible.set(true);
    }

    /// Показывает меню у курсора события, отменяя действие по умолчанию и всплытие.
    pub fn show(&self, event: &web_sys::MouseEvent) {
        event.prevent_default();
        event.stop_propagation();
        self.show_at(f64::from(event.page_x()), f64::from(event.page_y()));
    }

    /// Показывает меню, если оно скрыто, иначе скрывает.
    pub fn toggle(&self, event: &web_sys::MouseEvent) {
        if self.visible.get_untracked() {
            self.hide();
        } else {
            self.show(event);
        }
    }

    /// Скрывает меню.
    pub fn hide(&self) {
        self.visible.try_set(false);
    }
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    model: Prop<Vec<MenuItem>>,
    nodes: Memo<Vec<Node>>,
    state: RwSignal<State>,
    /// Сторона подменю по ключу группы (`100%` или `-100%`).
    sides: RwSignal<HashMap<String, &'static str>>,
    focused: RwSignal<bool>,
    focused_key: Memo<Option<String>>,
    mobile: RwSignal<bool>,
    r: ContextMenuRef,
    auto_display: Prop<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    /// Левый верхний угол панели (относительно её предка для позиционирования); пока `None`, панель скрыта.
    place: RwSignal<Option<(f64, f64)>>,
    z: RwSignal<Option<i32>>,
    panel: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    search: StoredValue<String>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    last_show: StoredValue<u32>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    #[cfg(target_arch = "wasm32")]
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn tree<R>(&self, f: impl FnOnce(&Tree) -> R) -> R {
        self.model.with(|items| self.nodes.with_untracked(|nodes| f(&Tree { nodes, items })))
    }

    fn update<R>(&self, f: impl FnOnce(&Tree, &mut State) -> R) -> R {
        let mut state = self.state.get_untracked();
        let out = self.tree(|t| f(t, &mut state));
        self.state.set(state);
        out
    }

    fn touch(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        return web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
        #[cfg(not(target_arch = "wasm32"))]
        false
    }

    fn focus_list(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(list) = self.list.get_untracked() {
            prime_dom::focus_no_scroll(&list);
        }
    }

    /// Закрывает всё: путь, фокус и само меню.
    fn hide_menu(&self) {
        self.update(|t, st| t.hide(st));
        self.r.hide();
    }

    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    /// Клик по содержимому пункта.
    fn item_click(&self, event: web_sys::MouseEvent, key: &str) {
        let Some(item) = self.tree(|t| t.node(key).map(|n| t.item(n).clone())) else { return };
        if should_prevent_default(&item) || item.disabled {
            event.prevent_default();
        }
        if item.disabled {
            return;
        }
        if can_run_command(&item)
            && let Some(command) = item.command
        {
            command.run(MenuCommand { event, item: item.clone() });
        }
        let outcome = self.update(|t, st| t.node(key).map(|n| t.click(st, n)));
        if matches!(outcome, Some(Click::Leaf(_))) {
            self.r.hide();
        }
        self.focus_list();
    }

    /// Наведение на содержимое пункта.
    fn item_hover(&self, key: &str) {
        if !self.auto_display.get_untracked() {
            return;
        }
        let (touch, mobile) = (self.touch(), self.mobile.get_untracked());
        self.update(|t, st| {
            if let Some(node) = t.node(key) {
                t.hover(st, node, touch, mobile);
            }
        });
    }

    fn scroll_focused_into_view(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(id) = self.tree(|t| t.focused_id(&self.state.get_untracked(), &self.id))
            && let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id))
        {
            let init = web_sys::ScrollIntoViewOptions::new();
            init.set_block(web_sys::ScrollLogicalPosition::Nearest);
            init.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&init);
        }
    }

    /// `Enter`/`Space`: клик по ссылке пункта под фокусом.
    fn enter(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(id) = self.tree(|t| t.focused_id(&self.state.get_untracked(), &self.id))
            && let Some(li) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id))
        {
            let anchor =
                li.query_selector(".p-contextmenu-item-link").ok().flatten().or_else(|| li.query_selector("a,button").ok().flatten());
            if let Some(el) = anchor.or(Some(li)).and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok()) {
                el.click();
            }
        }
        self.update(|t, st| t.after_enter(st));
    }

    /// Печатаемый символ: поиск по началу подписи; строка поиска сбрасывается через 500 мс.
    fn type_char(&self, ch: &str) {
        self.search.update_value(|s| s.push_str(ch));
        let text = self.search.get_value();
        self.update(|t, st| {
            let visible = t.visible(st);
            let (found, _) = t.search(st, &visible, &text);
            t.focus_index(st, found);
        });
        self.scroll_focused_into_view();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
            let search = self.search;
            if let Ok(handle) = set_timeout_with_handle(move || drop(search.try_set_value(String::new())), Duration::from_millis(500)) {
                self.search_timer.set_value(Some(handle));
            }
        }
    }

    fn key_down(&self, e: &web_sys::KeyboardEvent) {
        let meta = e.meta_key() || e.ctrl_key();
        match e.code().as_str() {
            "ArrowDown" => {
                self.update(|t, st| t.arrow_down(st));
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowUp" => {
                self.update(|t, st| t.arrow_up(st));
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowRight" => {
                self.update(|t, st| t.arrow_right(st));
                self.search.set_value(String::new());
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowLeft" => {
                self.update(|t, st| t.arrow_left(st));
                self.search.set_value(String::new());
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "Home" => {
                self.update(|t, st| t.home(st));
                e.prevent_default();
            }
            "End" => {
                self.update(|t, st| t.end(st));
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.enter();
                e.prevent_default();
            }
            "Escape" => {
                self.update(|t, st| t.escape(st));
                self.r.hide();
                e.prevent_default();
            }
            "Tab" => {
                let mobile = self.mobile.get_untracked();
                self.update(|t, st| t.tab(st, mobile));
                self.r.hide();
            }
            "PageDown" | "PageUp" | "Backspace" | "ShiftLeft" | "ShiftRight" => {}
            _ => {
                if is_printable(&e.key(), meta) {
                    self.type_char(&e.key());
                }
            }
        }
    }

    fn list_focus(&self) {
        self.focused.set(true);
    }

    fn list_blur(&self) {
        self.focused.set(false);
        self.search.set_value(String::new());
        self.state.update(|st| {
            st.focus = Focus::default();
            st.dirty = false;
        });
    }

    /// Ставит панель у курсора: считает размеры (панель пока скрыта) и переводит точку страницы в координаты предка.
    #[cfg(target_arch = "wasm32")]
    fn position(&self) {
        let Some(panel) = self.panel.get_untracked() else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        let (page_x, page_y, _) = self.r.at.get_untracked();
        let (left, top) = client::place_panel(&panel, page_x, page_y);
        self.place.set(Some((left, top)));
    }

    /// Меню только что показано: слой, положение, фокус, подписки, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Menu) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Menu, base);
            self.z_index.set_value(Some(z));
            self.z.set(Some(z));
        }
        self.reposition();
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| this.hide_menu());
        self.cleanups.update_value(|c| c.push(Box::new(move || resize.remove())));
        self.bind_outside();
    }

    /// Новый показ (или первый): сброс пути и фокуса, положение у курсора, фокус списка, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn reposition(&self) {
        self.update(|t, st| t.hide(st));
        self.position();
        // Скрытый (`visibility: hidden`) список фокус не принимает: ждём, пока положение попадёт в стиль.
        let list = self.list;
        set_timeout(
            move || {
                if let Some(list) = list.try_get_untracked().flatten() {
                    prime_dom::focus_no_scroll(&list);
                }
            },
            Duration::ZERO,
        );
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Слушатель клика вне меню (в следующем тике: клик, открывший меню, ещё всплывает). Правая кнопка и Ctrl-клик не закрывают.
    #[cfg(target_arch = "wasm32")]
    fn bind_outside(self: &Arc<Self>) {
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                let inner = this.clone();
                let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
                    if e.button() == 2 || e.ctrl_key() {
                        return;
                    }
                    let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                    if !inner.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref())) {
                        inner.hide_menu();
                    }
                });
                this.cleanups.update_value(|c| c.push(Box::new(move || click.remove())));
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
    }

    /// Меню только что скрыто: подписки сняты, состояние сброшено, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.update(|t, st| t.hide(st));
        self.focused.set(false);
        self.place.set(None);
        self.z.set(None);
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    /// Считает сторону раскрытых подменю по месту в окне (`nestedPosition`).
    #[cfg(target_arch = "wasm32")]
    fn place_submenus(self: &Arc<Self>) {
        let this = self.clone();
        set_timeout(
            move || {
                let Some(window) = web_sys::window() else { return };
                let Some(document) = window.document() else { return };
                let client = document.document_element().map_or(0.0, |e| f64::from(e.client_width()));
                let inner = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(client);
                let (scrollbar, viewport) = ((inner - client).max(0.0), client.max(1.0));
                let scroll_x = window.scroll_x().unwrap_or(0.0);
                // Компонент мог быть размонтирован до срабатывания таймера.
                let Some(active) = this.state.try_get_untracked().map(|s| s.active) else { return };
                let mut next = HashMap::new();
                for key in active {
                    let level = this.tree(|t| t.node(&key).map(|n| n.level)).map_or(0, |l| l + 1);
                    let Some(ul) = document.get_element_by_id(&format!("{}_{key}_list", this.id)) else { continue };
                    let Some(li) = ul.parent_element() else { continue };
                    let content_width = li
                        .first_element_child()
                        .and_then(|c| c.dyn_into::<web_sys::HtmlElement>().ok())
                        .map_or(0.0, |c| f64::from(c.offset_width()));
                    let sub_width = ul.dyn_into::<web_sys::HtmlElement>().map_or(0.0, |u| f64::from(u.offset_width()));
                    let left = li.get_bounding_client_rect().left() + scroll_x;
                    next.insert(key, submenu_side(left, content_width, sub_width, viewport, scrollbar, level));
                }
                if this.sides.with_untracked(|s| *s != next) {
                    this.sides.set(next);
                }
            },
            Duration::ZERO,
        );
    }
}

/// Всё, что нужно для вывода уровней.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<ContextMenuPt>,
    item: Option<Callback<ContextMenuItemContext, AnyView>>,
    submenu_icon: Option<ViewFn>,
}

/// Контекстное меню с вложенными подменю.
///
/// Свойства принимают значение или сигнал. Панель выводится внутри собственного места в разметке (не в `body`) и позиционируется
/// абсолютно, поэтому предок не должен быть `overflow: hidden`.
#[component]
pub fn ContextMenu(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Управляющий объект меню.
    #[prop(optional)]
    popup_ref: Option<ContextMenuRef>,
    /// CSS-селектор цели: правый клик по ней открывает меню.
    #[prop(optional, into)]
    target: OptProp<String>,
    /// Открывать меню по правому клику на всём документе.
    #[prop(optional, into)]
    global: Prop<bool>,
    /// Событие-триггер (по умолчанию `contextmenu`).
    #[prop(optional, into)]
    trigger_event: OptProp<String>,
    /// Задержка долгого касания в миллисекундах (по умолчанию 500).
    #[prop(optional, default = 500, into)]
    press_delay: u32,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Ширина окна, до которой включён режим `p-contextmenu-mobile` (по умолчанию `960px`).
    #[prop(optional, into)]
    breakpoint: OptProp<String>,
    /// Автоматически назначать `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Базовый `z-index` поверх значения конфигурации.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Раскрывать группы при наведении (после первого раскрытия кликом).
    #[prop(optional, default = Prop::Static(true), into)]
    auto_display: Prop<bool>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Атрибут `aria-label` корневого списка.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` корневого списка.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ContextMenuPt>,
    /// Своё содержимое пункта вместо ссылки.
    #[prop(optional)]
    item: Option<Callback<ContextMenuItemContext, AnyView>>,
    /// Своя стрелка группы.
    #[prop(optional, into)]
    submenu_icon: Option<ViewFn>,
    /// Меню показано.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Меню скрыто.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let nodes = Memo::new({
        let model = model.clone();
        move |_| model.with(|items| build(items))
    });
    let state = RwSignal::new(State::default());
    let focused = RwSignal::new(false);
    let r = popup_ref.unwrap_or_default();
    let focused_key = {
        let model = model.clone();
        Memo::new(move |_| {
            if !focused.get() {
                return None;
            }
            let st = state.get();
            model.with(|items| {
                nodes.with(|ns| Tree { nodes: ns, items }.siblings(&st.focus.parent_key).get(st.focus.index?).map(|n| n.key.clone()))
            })
        })
    };
    let core = Arc::new(Core {
        id: menu_id.clone(),
        model: model.clone(),
        nodes,
        state,
        sides: RwSignal::new(HashMap::new()),
        focused,
        focused_key,
        mobile: RwSignal::new(false),
        r,
        auto_display,
        auto_z_index,
        base_z_index,
        place: RwSignal::new(None),
        z: RwSignal::new(None),
        panel: NodeRef::new(),
        list: NodeRef::new(),
        search: StoredValue::new(String::new()),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        last_show: StoredValue::new(0),
        on_show,
        on_hide,
        #[cfg(target_arch = "wasm32")]
        search_timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    #[cfg(target_arch = "wasm32")]
    {
        client::bind_media(breakpoint.get_untracked().unwrap_or_else(|| "960px".to_owned()), core.mobile);
        // Показ, повторный показ и скрытие.
        let c1 = core.clone();
        Effect::new(move |_| {
            let open = c1.r.visible.get();
            let (_, _, shows) = c1.r.at.get();
            // Панель попадает в DOM после этого эффекта, поэтому следим и за ссылкой на неё.
            let mounted = c1.panel.get().is_some();
            let was = c1.was_open.get_value();
            if open && mounted && !was {
                c1.was_open.set_value(true);
                c1.last_show.set_value(shows);
                c1.opened();
            } else if open && mounted && was && c1.last_show.get_value() != shows {
                c1.last_show.set_value(shows);
                c1.reposition();
            } else if !open && was {
                c1.was_open.set_value(false);
                c1.closed();
            }
        });
        // Сторона подменю по месту в окне.
        let c3 = core.clone();
        Effect::new(move |_| {
            c3.state.with(|s| s.active.len());
            c3.place_submenus();
        });
        // Триггер: цель, документ, долгое касание.
        let c4 = core.clone();
        let (target, global, trigger_event) = (target.clone(), global.clone(), trigger_event.clone());
        Effect::new(move |prev: Option<()>| {
            if prev.is_some() {
                return;
            }
            let event = trigger_event.get_untracked().unwrap_or_else(|| "contextmenu".to_owned());
            let remove = client::bind_trigger(&c4.r, target.get_untracked(), global.get_untracked(), &event, press_delay);
            c4.cleanups_forever(remove);
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (breakpoint, target, global, trigger_event, press_delay);

    let ctx = Ctx { core: core.clone(), pt: pt.clone(), item, submenu_icon };
    let panel = core.panel;
    let visible = core.r.visible;
    let mobile = core.mobile;
    move || {
        visible.get().then(|| {
            let (ctx, pt, core) = (ctx.clone(), pt.clone(), core.clone());
            let class = {
                let (pt, style_class) = (pt.clone(), style_class.clone());
                move || {
                    let mut cn = prime_core::ClassNames::new();
                    cn.add("p-contextmenu p-component").add_if(mobile.get(), "p-contextmenu-mobile");
                    let extra = style_class.with(|c| c.clone());
                    cn.add_user(extra.as_deref());
                    pt.root.merge_class(&cn.finish_cow()).into_owned()
                }
            };
            let root_style = {
                let (pt, style, core) = (pt.clone(), style.clone(), core.clone());
                move || {
                    // Пока положение не посчитано, панель невидима: иначе она мигнула бы не на месте.
                    let base = match core.place.get() {
                        Some((left, top)) => format!("position: absolute; left: {left}px; top: {top}px"),
                        None => "position: absolute; left: 0; top: 0; visibility: hidden".to_owned(),
                    };
                    let z = core.z.get().map(|z| format!("z-index: {z}")).unwrap_or_default();
                    let own = style.with(|s| s.clone()).unwrap_or_default();
                    let merged = pt.root.merge_style_parts(&[&base, &z, &own]);
                    (!merged.is_empty()).then_some(merged)
                }
            };
            let activedescendant = {
                let core = core.clone();
                move || {
                    core.focused_key.get()?;
                    core.tree(|t| t.focused_id(&core.state.get(), &core.id))
                }
            };
            let (focus_core, blur_core, key_core) = (core.clone(), core.clone(), core.clone());
            let levels = {
                let ctx = ctx.clone();
                move || items_view(&ctx, "")
            };
            let (aria_label, aria_labelledby) = (aria_label.clone(), aria_labelledby.clone());
            let list_class = pt.root_list.merge_class("p-contextmenu-root-list").into_owned();
            let (root_attrs, list_attrs) = (pt.root.attrs(), pt.root_list.attrs());
            view! {
                <div node_ref=panel id={menu_id.clone()} class={class} style={root_style} {..root_attrs}>
                    <ul
                        node_ref=core.list
                        id={format!("{}_list", menu_id)}
                        class={list_class}
                        role="menu"
                        tabindex="0"
                        aria-label={move || aria_label.get()}
                        aria-labelledby={move || aria_labelledby.get()}
                        aria-activedescendant={activedescendant}
                        on:focus={move |_| focus_core.list_focus()}
                        on:blur={move |_| blur_core.list_blur()}
                        on:keydown={move |e| key_core.key_down(&e)}
                        {..list_attrs}
                    >
                        {levels}
                    </ul>
                </div>
            }
        })
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    /// Подписка живёт до размонтирования компонента (в отличие от подписок показа, которые снимаются при скрытии).
    #[cfg(target_arch = "wasm32")]
    fn cleanups_forever(&self, remove: Box<dyn FnOnce()>) {
        let held = StoredValue::new_local(Some(remove));
        on_cleanup(move || {
            if let Some(Some(remove)) = held.try_update_value(Option::take) {
                remove();
            }
        });
    }
}

/// Пункты одного уровня.
fn items_view(ctx: &Ctx, parent_key: &str) -> AnyView {
    let core = &ctx.core;
    let rows: Vec<(Node, MenuItem)> = core.tree(|t| t.siblings(parent_key).into_iter().map(|n| (n.clone(), t.item(n).clone())).collect());
    let refs: Vec<&MenuItem> = rows.iter().map(|(_, i)| i).collect();
    rows.iter()
        .enumerate()
        .filter(|(_, (_, item))| item.visible)
        .map(|(index, (node, item))| {
            if item.separator {
                separator_view(ctx, node, item)
            } else {
                item_view(ctx, node, item, aria_set_size(&refs), aria_pos_inset(&refs, index))
            }
        })
        .collect_view()
        .into_any()
}

fn separator_view(ctx: &Ctx, node: &Node, item: &MenuItem) -> AnyView {
    let class = ctx.pt.separator.merge_class(&join("p-contextmenu-separator", item.style_class.as_deref())).into_owned();
    let id = item_id(&ctx.core.id, item, &node.key);
    view! { <li id={id} class={class} style={item.style.clone()} role="separator" {..ctx.pt.separator.attrs()}></li> }.into_any()
}

fn join(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(e) => format!("{base} {e}"),
        None => base.to_owned(),
    }
}

#[allow(clippy::clone_on_copy)] // замыкание над `Memo` — `Copy`, `clone()` оставлен для единообразия с остальными замыканиями
fn item_view(ctx: &Ctx, node: &Node, item: &MenuItem, set_size: usize, pos_inset: usize) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let group = !item.items.is_empty();
    let id = item_id(&core.id, item, &key);
    let state = core.state;
    // Memo: подменю перестраивается только при смене флага этого пункта, а не при любом изменении состояния.
    let active = {
        let key = key.clone();
        let memo = Memo::new(move |_| state.with(|s| s.active.contains(&key)));
        move || memo.get()
    };
    let focused = {
        let (core, key) = (core.clone(), key.clone());
        move || core.focused_key.get().as_deref() == Some(key.as_str())
    };
    let class = {
        let (pt, item, active, focused) = (pt.clone(), item.clone(), active.clone(), focused.clone());
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-contextmenu-item")
                .add_if(active(), "p-contextmenu-item-active")
                .add_if(focused(), "p-focus")
                .add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content = match ctx.item {
        Some(template) => untrack(|| template.run(ContextMenuItemContext { item: item.clone(), has_submenu: group })),
        None => link_view(ctx, item, group),
    };
    let (click_core, hover_core, click_key, hover_key) = (core.clone(), core.clone(), key.clone(), key.clone());
    let submenu = group.then(|| {
        let (ctx, key, active) = (ctx.clone(), key.clone(), active.clone());
        let labelledby = id.clone();
        move || {
            active().then(|| {
                let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
                let side = {
                    let (core, key) = (core.clone(), key.clone());
                    move || {
                        let left = core.sides.with(|s| s.get(&key).copied()).unwrap_or("100%");
                        pt.submenu.merge_style(&format!("display: flex; top: 0; left: {left}"))
                    }
                };
                let class = ctx.pt.submenu.merge_class("p-contextmenu-submenu").into_owned();
                let sub_id = format!("{}_{}_list", core.id, key);
                let children = items_view(&ctx, &key);
                view! { <ul id={sub_id} class={class} style={side} role="menu" aria-labelledby={labelledby.clone()} {..ctx.pt.submenu.attrs()}>{children}</ul> }
            })
        }
    });
    let (hp, ae) = (group.then_some("menu"), {
        let active = active.clone();
        move || group.then(|| active().to_string())
    });
    let (hl, fo) = (active.clone(), focused.clone());
    let label = item.label.clone();
    let disabled = item.disabled;
    let content_class = pt.item_content.merge_class("p-contextmenu-item-content").into_owned();
    let (item_attrs, content_attrs) = (pt.item.attrs(), pt.item_content.attrs());
    view! {
        <li
            id={id.clone()}
            class={class}
            style={item.style.clone()}
            role="menuitem"
            data-p-highlight={move || hl().to_string()}
            data-p-focused={move || fo().to_string()}
            data-p-disabled={disabled.to_string()}
            aria-label={label}
            aria-disabled={disabled.then_some("true")}
            aria-haspopup={hp}
            aria-expanded={ae}
            aria-setsize={set_size.to_string()}
            aria-posinset={pos_inset.to_string()}
            {..item_attrs}
        >
            <div
                class={content_class}
                on:click={move |e| click_core.item_click(e, &click_key)}
                on:mouseenter={move |_| hover_core.item_hover(&hover_key)}
                {..content_attrs}
            >
                {content}
            </div>
            {submenu}
        </li>
    }
    .into_any()
}

/// Стандартное содержимое пункта: ссылка со значком, подписью и стрелкой группы.
fn link_view(ctx: &Ctx, item: &MenuItem, group: bool) -> AnyView {
    let pt = &ctx.pt;
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-contextmenu-item-icon", item.icon_class.as_deref());
        view! { <span class={pt.item_icon.merge_class(&class).into_owned()} tabindex="-1" {..pt.item_icon.attrs()}>{icon.run()}</span> }
    });
    let arrow = group.then(|| match ctx.submenu_icon.clone() {
        Some(custom) => custom.run(),
        None => {
            let class = pt.submenu_icon.merge_class("p-contextmenu-submenu-icon").into_owned();
            view! { <AngleRightIcon class={class} /> }.into_any()
        }
    });
    let link_class = join("p-contextmenu-item-link", item.link_class.as_deref());
    view! {
        <a
            class={pt.item_link.merge_class(&link_class).into_owned()}
            href={item.url.clone()}
            target={item.target.clone()}
            title={item.title.clone()}
            tabindex="-1"
            {..pt.item_link.attrs()}
        >
            {icon}
            <span class={pt.item_label.merge_class("p-contextmenu-item-label").into_owned()} {..pt.item_label.attrs()}>{item.label.clone()}</span>
            {arrow}
        </a>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
mod client {
    //! Браузерная часть: `matchMedia`, триггеры показа, перевод точки страницы в координаты панели.

    use super::{ContextMenuRef, Viewport, place};
    use leptos::leptos_dom::helpers::set_timeout_with_handle;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use std::time::Duration;

    /// Ведёт `mobile` по медиазапросу и снимает слушатель при размонтировании.
    pub fn bind_media(breakpoint: String, mobile: RwSignal<bool>) {
        let Some(window) = web_sys::window() else { return };
        let Some(query) = window.match_media(&format!("(max-width: {breakpoint})")).ok().flatten() else { return };
        mobile.set(query.matches());
        let closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(move |e: web_sys::MediaQueryListEvent| {
            mobile.try_set(e.matches());
        });
        let _ = query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        let held = StoredValue::new_local((query, closure));
        on_cleanup(move || {
            held.try_with_value(|(query, closure)| {
                let _ = query.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            });
        });
    }

    /// Вешает слушатель и возвращает функцию снятия.
    fn listen(target: &web_sys::EventTarget, name: &str, handler: impl FnMut(web_sys::Event) + 'static) -> Box<dyn FnOnce()> {
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(handler);
        let _ = target.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref());
        let (target, name) = (target.clone(), name.to_owned());
        Box::new(move || {
            let _ = target.remove_event_listener_with_callback(&name, closure.as_ref().unchecked_ref());
        })
    }

    fn is_touch_device() -> bool {
        web_sys::window().and_then(|w| w.navigator().user_agent().ok()).is_some_and(|ua| {
            let ua = ua.to_lowercase();
            ua.contains("iphone") || ua.contains("ipad") || ua.contains("android")
        })
    }

    /// Подключает показ меню к цели или документу; возвращает функцию снятия всех слушателей.
    pub fn bind_trigger(r: &ContextMenuRef, target: Option<String>, global: bool, event: &str, press_delay: u32) -> Box<dyn FnOnce()> {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else { return Box::new(|| {}) };
        let host: Option<web_sys::EventTarget> = if global {
            Some(document.clone().into())
        } else {
            target.and_then(|sel| document.query_selector(&sel).ok().flatten()).map(Into::into)
        };
        let Some(host) = host else { return Box::new(|| {}) };
        let r = *r;
        if !is_touch_device() {
            return listen(&host, event, move |e| {
                if let Ok(mouse) = e.dyn_into::<web_sys::MouseEvent>() {
                    r.show(&mouse);
                }
            });
        }
        let timer = StoredValue::new_local(None);
        let start = listen(&host, "touchstart", move |e| {
            let Ok(touch) = e.dyn_into::<web_sys::TouchEvent>() else { return };
            let Some(point) = touch.touches().get(0) else { return };
            let (x, y) = (f64::from(point.page_x()), f64::from(point.page_y()));
            if let Ok(handle) = set_timeout_with_handle(move || r.show_at(x, y), Duration::from_millis(u64::from(press_delay)))
                && let Some(old) = timer.try_update_value(|t| t.replace(handle)).flatten()
            {
                old.clear();
            }
        });
        let end = listen(&host, "touchend", move |_| {
            if let Some(handle) = timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
        });
        Box::new(move || {
            start();
            end();
            if let Some(handle) = timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
        })
    }

    /// Считает положение панели у курсора и переводит его в координаты предка для позиционирования.
    pub fn place_panel(panel: &web_sys::HtmlElement, page_x: f64, page_y: f64) -> (f64, f64) {
        let Some(window) = web_sys::window() else { return (page_x, page_y) };
        let Some(document) = window.document() else { return (page_x, page_y) };
        let client = document.document_element().map_or(0.0, |e| f64::from(e.client_width()));
        let inner = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(client);
        let height = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let (scroll_x, scroll_y) = (window.scroll_x().unwrap_or(0.0), window.scroll_y().unwrap_or(0.0));
        let scrollbar = (inner - client).max(0.0);
        let view = Viewport { width: inner - scrollbar, height, scroll_x, scroll_y };
        let (left, top) = place(page_x, page_y, f64::from(panel.offset_width()), f64::from(panel.offset_height()), view);
        // Предок для позиционирования: абсолютная панель считается от него, а не от страницы.
        let (mut ox, mut oy) = (0.0, 0.0);
        if let Some(parent) = panel.offset_parent().and_then(|p| p.dyn_into::<web_sys::HtmlElement>().ok()) {
            let static_body = parent.tag_name().eq_ignore_ascii_case("body")
                && window
                    .get_computed_style(&parent)
                    .ok()
                    .flatten()
                    .and_then(|s| s.get_property_value("position").ok())
                    .is_none_or(|p| p == "static");
            if !static_body {
                let rect = parent.get_bounding_client_rect();
                ox = rect.left() + scroll_x + f64::from(parent.client_left()) - f64::from(parent.scroll_left());
                oy = rect.top() + scroll_y + f64::from(parent.client_top()) - f64::from(parent.scroll_top());
            }
        }
        (left - ox, top - oy)
    }
}
