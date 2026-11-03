//! TieredMenu — порт `p-tieredmenu` из PrimeNG 21.1.10.
//!
//! Меню, группы которого раскрываются во вложенных панелях рядом с пунктом. Пункты — [`prime_menu::MenuItem`]: поле `items`
//! задаёт подменю. Группу раскрывают кликом или стрелкой →, после первого раскрытия кликом работает и наведение (`auto_display`);
//! клавиатура ведёт по уровням (↓ ↑ → ← `Home` `End` `Enter` `Space` `Escape` `Tab`, поиск по началу подписи). В режиме `popup`
//! меню показывается у цели через [`TieredMenuRef`]. Вся логика переходов — в чистой модели (модуль `model`).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::MenuItem;
//! use prime_tieredmenu::TieredMenu;
//!
//! let model = vec![MenuItem::new().label("File").items(vec![MenuItem::new().label("New")]), MenuItem::new().label("Quit")];
//! let html = Owner::new().with(|| view! { <TieredMenu model=model /> }.to_html());
//! assert!(html.contains("p-tieredmenu-root-list") && html.contains("aria-haspopup=\"menu\"") && html.contains("Quit"));
//! ```

mod model;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_icons::AngleRightIcon;
use prime_menu::{MenuCommand, MenuItem, can_run_command, should_prevent_default};
use std::collections::HashMap;
use std::sync::Arc;

pub use model::{
    Click, Focus, Node, State, Tree, aria_pos_inset, aria_set_size, build, is_ancestor_or_self, is_printable, is_valid, item_at, item_id,
    submenu_side,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through частей [`TieredMenu`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TieredMenuPt {
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
pub struct TieredMenuItemContext {
    /// Пункт.
    pub item: MenuItem,
    /// Есть ли у пункта подменю.
    pub has_submenu: bool,
}

/// Управляющий объект всплывающего меню: `toggle`/`show`/`hide` от события и цели.
#[derive(Clone, Copy)]
pub struct TieredMenuRef {
    visible: RwSignal<bool>,
    target: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
}

impl Default for TieredMenuRef {
    fn default() -> Self {
        Self::new()
    }
}

fn event_target(event: &web_sys::Event) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast as _;
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()))
}

impl TieredMenuRef {
    /// Создаёт скрытое меню. Вызывать внутри компонента (владелец сигналов).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false), target: StoredValue::new_local(None) }
    }

    /// Меню показано (с подпиской).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// Показывает меню у `currentTarget` события.
    pub fn show(&self, event: &web_sys::Event) {
        if let Some(target) = event_target(event) {
            self.show_at(&target);
        }
    }

    /// Показывает меню у явной цели.
    pub fn show_at(&self, target: &web_sys::HtmlElement) {
        self.target.set_value(Some(target.clone()));
        self.visible.set(true);
    }

    /// Переключает меню; цель — `currentTarget` события.
    pub fn toggle(&self, event: &web_sys::Event) {
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
    popup: Prop<bool>,
    r: TieredMenuRef,
    auto_display: Prop<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    panel: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    search: StoredValue<String>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    outside: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    #[cfg(target_arch = "wasm32")]
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

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

    fn is_popup(&self) -> bool {
        self.popup.get_untracked()
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
            // Без прокрутки страницы: иначе она сдвинула бы цель и закрыла всплывающее меню по `scroll`.
            prime_dom::focus_no_scroll(&list);
        }
    }

    fn focus_target(&self) {
        #[cfg(target_arch = "wasm32")]
        self.r.target.with_value(|t| {
            if let Some(t) = t {
                let _ = t.focus();
            }
        });
    }

    /// Закрывает всё: путь, фокус; в режиме `popup` скрывает и меню.
    fn hide_menu(&self) {
        self.update(|t, st| t.hide(st));
        if self.is_popup() {
            self.r.hide();
        }
    }

    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    fn unbind_outside(&self) {
        if let Some(remove) = self.outside.try_update_value(Option::take).flatten() {
            remove();
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
        if matches!(outcome, Some(Click::Leaf(_))) && self.is_popup() {
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
                li.query_selector(".p-tieredmenu-item-link").ok().flatten().or_else(|| li.query_selector("a,button").ok().flatten());
            if let Some(el) = anchor.or(Some(li)).and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok()) {
                el.click();
            }
        }
        if !self.is_popup() {
            self.update(|t, st| t.after_enter(st));
        }
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
                if e.alt_key() {
                    if self.is_popup() {
                        self.hide_menu();
                        self.focus_target();
                    }
                } else {
                    self.update(|t, st| t.arrow_up(st));
                    self.scroll_focused_into_view();
                }
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
                if self.is_popup() {
                    self.r.hide();
                    self.focus_target();
                } else {
                    self.focus_list();
                }
                e.prevent_default();
            }
            "Tab" => {
                let mobile = self.mobile.get_untracked();
                self.update(|t, st| t.tab(st, mobile));
                if self.is_popup() {
                    self.r.hide();
                }
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

    #[cfg(target_arch = "wasm32")]
    fn is_inside(&self, event_target: Option<web_sys::EventTarget>) -> (bool, bool) {
        let node = event_target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside_panel = self.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref()));
        let inside_target = self.r.target.with_value(|t| t.as_ref().is_some_and(|t| t.contains(node.as_ref())));
        (inside_panel, inside_target)
    }

    /// Слушатель клика вне меню (в следующем тике: клик, открывший меню, ещё всплывает).
    #[cfg(target_arch = "wasm32")]
    fn bind_outside(self: &Arc<Self>) {
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.outside.try_with_value(Option::is_some).unwrap_or(true) {
                    return;
                }
                let inner = this.clone();
                let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
                    let (panel, target) = inner.is_inside(e.target());
                    let outside_target = !inner.is_popup() || !target;
                    if !panel && outside_target {
                        inner.hide_menu();
                    }
                });
                this.outside.set_value(Some(Box::new(move || click.remove())));
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
    }

    /// Меню только что показано (`popup`): слой, выравнивание, фокус, подписки, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(target)) = (self.panel.get_untracked(), self.r.target.get_value()) else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Menu) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Menu, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        let _ = panel.style().set_property("position", "absolute");
        align_overlay(&panel, &target, AlignMode::Relative, false);
        if f64::from(target.offset_width()) > f64::from(panel.offset_width()) {
            let _ = panel.style().set_property("min-width", &format!("{}px", target.offset_width()));
        }
        self.focus_list();
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            if !this.touch() {
                this.hide_menu();
            }
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || resize.remove())));
        // Прокрутка предка цели (или сдвинувшая цель прокрутка страницы) закрывает меню.
        let this = self.clone();
        if let Some(target) = self.r.target.get_value()
            && let Some(unwatch) = prime_dom::watch_scroll(target, move || this.hide_menu())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Меню только что скрыто (`popup`): подписки сняты, состояние сброшено, цель забыта, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.update(|t, st| t.hide(st));
        self.focused.set(false);
        self.r.target.set_value(None);
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
    pt: Arc<TieredMenuPt>,
    item: Option<Callback<TieredMenuItemContext, AnyView>>,
    submenu_icon: Option<ViewFn>,
}

/// Меню с вложенными подменю.
///
/// Свойства принимают значение или сигнал. Всплывающее меню выводится внутри собственного места в разметке (не в `body`),
/// поэтому для выравнивания родитель должен быть `position: relative`, а не `overflow: hidden`.
#[component]
pub fn TieredMenu(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Всплывающий режим.
    #[prop(optional, into)]
    popup: Prop<bool>,
    /// Управляющий объект всплывающего меню (в режиме `popup`).
    #[prop(optional)]
    popup_ref: Option<TieredMenuRef>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Ширина окна, до которой включён режим `p-tieredmenu-mobile` (по умолчанию `960px`).
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
    /// Отключить фокус меню.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Атрибут `tabindex` корневого списка.
    #[prop(optional, default = Prop::Static(0), into)]
    tabindex: Prop<i32>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<TieredMenuPt>,
    /// Своё содержимое пункта вместо ссылки.
    #[prop(optional)]
    item: Option<Callback<TieredMenuItemContext, AnyView>>,
    /// Своя стрелка группы.
    #[prop(optional, into)]
    submenu_icon: Option<ViewFn>,
    /// Меню показано (`popup`).
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Меню скрыто (`popup`).
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
        let (model, id) = (model.clone(), menu_id.clone());
        Memo::new(move |_| {
            if !focused.get() {
                return None;
            }
            let st = state.get();
            let _ = &id;
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
        popup: popup.clone(),
        r,
        auto_display,
        auto_z_index,
        base_z_index,
        panel: NodeRef::new(),
        list: NodeRef::new(),
        search: StoredValue::new(String::new()),
        cleanups: StoredValue::new_local(Vec::new()),
        outside: StoredValue::new_local(None),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        on_show,
        on_hide,
        #[cfg(target_arch = "wasm32")]
        search_timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || {
            core.release();
            core.unbind_outside();
        });
    }
    #[cfg(target_arch = "wasm32")]
    {
        client::bind_media(breakpoint.get_untracked().unwrap_or_else(|| "960px".to_owned()), core.mobile);
        // Показ и скрытие `popup`.
        let c1 = core.clone();
        Effect::new(move |_| {
            let open = c1.popup.get() && c1.r.visible.get();
            // Панель попадает в DOM после этого эффекта, поэтому следим и за ссылкой на неё.
            let mounted = c1.panel.get().is_some();
            let was = c1.was_open.get_value();
            if open && mounted && !was {
                c1.was_open.set_value(true);
                c1.opened();
            } else if !open && was {
                c1.was_open.set_value(false);
                c1.closed();
            }
        });
        // Слушатель клика вне: пока открыто всплывающее меню или раскрыт путь.
        let c2 = core.clone();
        Effect::new(move |_| {
            let want = if c2.popup.get() { c2.r.visible.get() } else { c2.state.with(|s| !s.active.is_empty()) };
            if want {
                c2.bind_outside();
            } else {
                c2.unbind_outside();
            }
        });
        // Сторона подменю по месту в окне.
        let c3 = core.clone();
        Effect::new(move |_| {
            c3.state.with(|s| s.active.len());
            c3.place_submenus();
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = breakpoint;

    let ctx = Ctx { core: core.clone(), pt: pt.clone(), item, submenu_icon };
    let panel = core.panel;
    let visible = core.r.visible;
    let mobile = core.mobile;
    move || {
        let is_popup = popup.get();
        (!is_popup || visible.get()).then(|| {
            let (ctx, pt, core) = (ctx.clone(), pt.clone(), core.clone());
            let class = {
                let (pt, style_class) = (pt.clone(), style_class.clone());
                move || {
                    let mut cn = prime_core::ClassNames::new();
                    cn.add("p-tieredmenu p-component").add_if(is_popup, "p-tieredmenu-overlay").add_if(mobile.get(), "p-tieredmenu-mobile");
                    let extra = style_class.with(|c| c.clone());
                    cn.add_user(extra.as_deref());
                    pt.root.merge_class(&cn.finish_cow()).into_owned()
                }
            };
            let root_style = {
                let (pt, style) = (pt.clone(), style.clone());
                move || {
                    // Всплывающая панель сразу вне потока: иначе её вставка сдвигает страницу и закрывает меню по `scroll`.
                    let base = if is_popup { "position: absolute" } else { "" };
                    let own = style.with(|s| s.clone()).unwrap_or_default();
                    let merged = pt.root.merge_style_parts(&[base, &own]);
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
            let tab = {
                let (disabled, tabindex) = (disabled.clone(), tabindex.clone());
                move || if disabled.get() { -1 } else { tabindex.get() }
            };
            let (focus_core, blur_core, key_core) = (core.clone(), core.clone(), core.clone());
            let levels = {
                let ctx = ctx.clone();
                move || items_view(&ctx, "")
            };
            let (aria_label, aria_labelledby) = (aria_label.clone(), aria_labelledby.clone());
            let list_class = pt.root_list.merge_class("p-tieredmenu-root-list").into_owned();
            let (root_attrs, list_attrs) = (pt.root.attrs(), pt.root_list.attrs());
            view! {
                <div node_ref=panel id={menu_id.clone()} class={class} style={root_style} {..root_attrs}>
                    <ul
                        node_ref=core.list
                        id={format!("{}_list", menu_id)}
                        class={list_class}
                        role="menu"
                        tabindex={move || tab().to_string()}
                        aria-orientation="vertical"
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
    let class = ctx.pt.separator.merge_class(&join("p-tieredmenu-separator", item.style_class.as_deref())).into_owned();
    let id = item_id(&ctx.core.id, item, &node.key);
    view! { <li id={id} class={class} style={item.style.clone()} role="separator" {..ctx.pt.separator.attrs()}></li> }.into_any()
}

fn join(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(e) => format!("{base} {e}"),
        None => base.to_owned(),
    }
}

fn item_view(ctx: &Ctx, node: &Node, item: &MenuItem, set_size: usize, pos_inset: usize) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let group = !item.items.is_empty();
    let id = item_id(&core.id, item, &key);
    let state = core.state;
    let active = {
        let key = key.clone();
        move || state.with(|s| s.active.contains(&key))
    };
    let focused = {
        let (core, key) = (core.clone(), key.clone());
        move || core.focused_key.get().as_deref() == Some(key.as_str())
    };
    let class = {
        let (pt, item, active, focused) = (pt.clone(), item.clone(), active.clone(), focused.clone());
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-tieredmenu-item")
                .add_if(active(), "p-tieredmenu-item-active")
                .add_if(focused(), "p-focus")
                .add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content = match ctx.item {
        Some(template) => untrack(|| template.run(TieredMenuItemContext { item: item.clone(), has_submenu: group })),
        None => link_view(ctx, item, group, &active, &focused),
    };
    let (click_core, hover_core, click_key, hover_key) = (core.clone(), core.clone(), key.clone(), key.clone());
    let submenu = group.then(|| {
        let (ctx, key, active, item) = (ctx.clone(), key.clone(), active.clone(), item.clone());
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
                let class = ctx.pt.submenu.merge_class("p-tieredmenu-submenu").into_owned();
                let sub_id = format!("{}_{}_list", core.id, key);
                let children = items_view(&ctx, &key);
                let _ = &item;
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
    let content_class = pt.item_content.merge_class("p-tieredmenu-item-content").into_owned();
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
fn link_view(
    ctx: &Ctx,
    item: &MenuItem,
    group: bool,
    _active: &(impl Fn() -> bool + Clone),
    _focused: &(impl Fn() -> bool + Clone),
) -> AnyView {
    let pt = &ctx.pt;
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-tieredmenu-item-icon", item.icon_class.as_deref());
        view! { <span class={pt.item_icon.merge_class(&class).into_owned()} tabindex="-1" {..pt.item_icon.attrs()}>{icon.run()}</span> }
    });
    let arrow = group.then(|| match ctx.submenu_icon.clone() {
        Some(custom) => custom.run(),
        None => {
            let class = pt.submenu_icon.merge_class("p-tieredmenu-submenu-icon").into_owned();
            view! { <AngleRightIcon class={class} /> }.into_any()
        }
    });
    let link_class = join("p-tieredmenu-item-link", item.link_class.as_deref());
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
            <span class={pt.item_label.merge_class("p-tieredmenu-item-label").into_owned()} {..pt.item_label.attrs()}>{item.label.clone()}</span>
            {arrow}
        </a>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
mod client {
    //! Мобильный режим: `matchMedia("(max-width: <breakpoint>)")`.

    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

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
}
