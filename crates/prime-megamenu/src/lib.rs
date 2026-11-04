//! MegaMenu — порт `p-megamenu` из PrimeNG 21.1.10.
//!
//! Панель корневых пунктов (горизонтальная или вертикальная), у каждого из которых раскрывается широкое наложение
//! с колонками подменю. Модель — [`MegaMenuItem`]: обычный пункт и `columns` (колонка — список [`MegaMenuSubmenu`], а в нём
//! плоские [`prime_menu::MenuItem`]). На узком окне (`breakpoint`) список сворачивается в кнопку. Слоты `start` и `end`
//! выводятся по краям панели.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_megamenu::{MegaMenu, MegaMenuItem, MegaMenuSubmenu};
//! use prime_menu::MenuItem;
//!
//! let model = vec![MegaMenuItem::new().label("Furniture").columns(vec![vec![MegaMenuSubmenu::new("Living Room").items(vec![MenuItem::new().label("Armchair")])]]), MegaMenuItem::new().label("Quit")];
//! let html = Owner::new().with(|| view! { <MegaMenu model=model /> }.to_html());
//! assert!(html.contains("p-megamenu-root-list") && html.contains("role=\"menubar\"") && html.contains("Quit"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{AngleDownIcon, AngleRightIcon, BarsIcon};
use prime_menu::{MenuCommand, MenuItem, can_run_command, should_prevent_default};
use std::sync::Arc;

pub use state::{Click, Kind, MegaMenuItem, MegaMenuOrientation, MegaMenuSubmenu, Menu, Node, State, build, column_class};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through частей [`MegaMenu`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MegaMenuPt {
    /// Корень.
    pub root: Pt,
    /// Слот `start`.
    pub start: Pt,
    /// Кнопка меню.
    pub button: Pt,
    /// Значок кнопки.
    pub button_icon: Pt,
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
    /// Наложение.
    pub overlay: Pt,
    /// Сетка колонок.
    pub grid: Pt,
    /// Колонка.
    pub column: Pt,
    /// Подменю.
    pub submenu: Pt,
    /// Заголовок подменю.
    pub submenu_label: Pt,
    /// Разделитель.
    pub separator: Pt,
    /// Слот `end`.
    pub end: Pt,
}

/// Контекст представления `item`.
#[derive(Clone)]
pub struct MegaMenuItemContext {
    /// Пункт.
    pub item: MenuItem,
    /// Есть ли у пункта наложение с подменю.
    pub has_submenu: bool,
    /// Пункт на корневом уровне.
    pub root: bool,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    model: Prop<Vec<MegaMenuItem>>,
    nodes: Memo<Vec<Node>>,
    orientation: Prop<MegaMenuOrientation>,
    state: RwSignal<State>,
    focused: RwSignal<bool>,
    mobile: RwSignal<bool>,
    mobile_active: RwSignal<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    /// `z-index` корневого списка на время открытой мобильной панели.
    z: RwSignal<Option<i32>>,
    root_el: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    button: NodeRef<leptos::html::A>,
    search: StoredValue<String>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    #[cfg(target_arch = "wasm32")]
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

/// Пункт по ключу (`"0"`, `"0_1_0_2"`).
fn item_by_key(model: &[MegaMenuItem], key: &str) -> Option<MenuItem> {
    let parts: Vec<usize> = key.split('_').map(str::parse).collect::<Result<_, _>>().ok()?;
    match parts[..] {
        [i] => model.get(i).map(|m| m.item.clone()),
        [i, c, s, j] => model.get(i)?.columns.get(c)?.get(s)?.items.get(j).cloned(),
        _ => None,
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn vertical(&self) -> bool {
        self.orientation.get_untracked() == MegaMenuOrientation::Vertical
    }

    fn tree<R>(&self, f: impl FnOnce(&Menu) -> R) -> R {
        let vertical = self.vertical();
        self.nodes.with_untracked(|nodes| f(&Menu { nodes, vertical }))
    }

    fn update<R>(&self, f: impl FnOnce(&Menu, &mut State) -> R) -> R {
        let mut state = self.state.get_untracked();
        let out = self.tree(|t| f(t, &mut state));
        self.state.set(state);
        out
    }

    fn item(&self, key: &str) -> Option<MenuItem> {
        self.model.with(|m| item_by_key(m, key))
    }

    fn id_of(&self, key: &str) -> String {
        match self.item(key).and_then(|i| i.id).filter(|i| !i.is_empty()) {
            Some(id) => id,
            None => format!("{}_{key}", self.id),
        }
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

    /// Закрывает всё; при открытой мобильной панели закрывает её и возвращает фокус на кнопку (`hide`).
    fn hide_menu(&self) {
        if self.mobile_active.get_untracked() {
            self.mobile_active.set(false);
            self.release_z();
            #[cfg(target_arch = "wasm32")]
            {
                let button = self.button;
                set_timeout(
                    move || {
                        if let Some(button) = button.try_get_untracked().flatten() {
                            prime_dom::focus_no_scroll(&button);
                        }
                    },
                    Duration::from_millis(100),
                );
            }
        }
        self.update(|t, st| t.hide(st));
    }

    fn release_z(&self) {
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        self.z.try_set(None);
    }

    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        self.release_z();
    }

    /// Кнопка меню: открывает или закрывает мобильную панель (`toggle`).
    fn toggle(self: &Arc<Self>) {
        if self.mobile_active.get_untracked() {
            self.mobile_active.set(false);
            self.release_z();
            self.hide_menu();
        } else {
            self.mobile_active.set(true);
            #[cfg(target_arch = "wasm32")]
            {
                if self.auto_z_index.get_untracked() {
                    let base = use_prime_config().z_index.base(ZGroup::Menu) + self.base_z_index.get_untracked();
                    let z = prime_core::zindex::generate(ZGroup::Menu, base);
                    self.z_index.set_value(Some(z));
                    self.z.set(Some(z));
                }
                let this = self.clone();
                set_timeout(move || this.show(), Duration::ZERO);
            }
        }
    }

    /// Фокус на первый пункт и на список (`show`).
    fn show(&self) {
        self.update(|t, st| st.focus = t.first_focused(st));
        self.focus_list();
    }

    /// Клик по содержимому пункта.
    fn item_click(&self, event: web_sys::MouseEvent, key: &str) {
        let Some(item) = self.item(key) else { return };
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
        self.state.update(|st| st.dirty = true);
        let outcome = self.update(|t, st| t.click(st, key));
        match outcome {
            Click::Leaf => self.hide_menu(),
            Click::Closed => self.focus_list_unless_mobile(),
            Click::Opened => self.focus_list(),
        }
    }

    fn focus_list_unless_mobile(&self) {
        if !self.mobile_active.get_untracked() {
            self.focus_list();
        }
    }

    /// Наведение на содержимое пункта.
    fn item_hover(&self, key: &str) {
        let mobile = self.mobile_active.get_untracked();
        self.update(|t, st| t.hover(st, key, mobile));
    }

    fn scroll_focused_into_view(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(key) = self.state.get_untracked().focus
            && let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&self.id_of(&key)))
        {
            let init = web_sys::ScrollIntoViewOptions::new();
            init.set_block(web_sys::ScrollLogicalPosition::Nearest);
            init.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&init);
        }
    }

    /// `Enter`/`Space`: клик по ссылке пункта под фокусом.
    fn enter(&self) {
        let Some(key) = self.state.get_untracked().focus else { return };
        let was_group = self.tree(|t| t.node(&key).is_some_and(|n| n.group));
        #[cfg(target_arch = "wasm32")]
        if let Some(li) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&self.id_of(&key))) {
            let anchor = li.query_selector(".p-megamenu-item-link").ok().flatten().or_else(|| li.query_selector("a,button").ok().flatten());
            if let Some(el) = anchor.or(Some(li)).and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok()) {
                el.click();
            }
        }
        self.update(|t, st| t.after_enter(st, was_group));
    }

    /// Печатаемый символ: поиск по началу подписи; строка поиска сбрасывается через 500 мс.
    fn type_char(&self, ch: &str) {
        self.search.update_value(|s| s.push_str(ch));
        let text = self.search.get_value();
        self.update(|t, st| t.search(st, &text));
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
                self.search.set_value(String::new());
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowUp" => {
                self.update(|t, st| t.arrow_up(st, e.alt_key()));
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
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "End" => {
                self.update(|t, st| t.end(st));
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.enter();
                e.prevent_default();
            }
            "Escape" => {
                self.update(|t, st| t.escape(st));
                e.prevent_default();
            }
            "Tab" => {
                self.update(|t, st| t.tab(st));
            }
            "PageDown" | "PageUp" | "Backspace" | "ShiftLeft" | "ShiftRight" => {}
            _ => {
                if !meta && e.key().chars().count() == 1 {
                    self.type_char(&e.key());
                }
            }
        }
    }

    /// Фокус в списке: снаружи — фокус на первый пункт (`onMenuFocus`).
    fn list_focus(&self, e: web_sys::FocusEvent) {
        self.focused.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            let related = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let from_outside = related.is_none_or(|r| !self.root_el.get_untracked().is_some_and(|root| root.contains(Some(&r))));
            if from_outside {
                self.update(|t, st| t.focus_in(st));
            }
        }
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    /// Фокус ушёл из списка: если он остался внутри меню — ничего, иначе сброс (`onMenuBlur`).
    fn list_blur(self: &Arc<Self>, e: web_sys::FocusEvent) {
        #[cfg(target_arch = "wasm32")]
        {
            let related = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            if related.is_some_and(|r| self.root_el.get_untracked().is_some_and(|root| root.contains(Some(&r)))) {
                return;
            }
            let this = self.clone();
            set_timeout(
                move || {
                    let active = web_sys::window().and_then(|w| w.document()).and_then(|d| d.active_element());
                    let node = active.and_then(|a| a.dyn_into::<web_sys::Node>().ok());
                    if node.is_some_and(|n| this.root_el.try_get_untracked().flatten().is_some_and(|root| root.contains(Some(&n)))) {
                        return;
                    }
                    this.reset_focus();
                    if let Some(cb) = this.on_blur {
                        cb.run(e);
                    }
                },
                Duration::ZERO,
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.reset_focus();
            if let Some(cb) = self.on_blur {
                cb.run(e);
            }
        }
    }

    fn reset_focus(&self) {
        self.focused.try_set(false);
        self.search.try_set_value(String::new());
        self.state.try_update(|st| {
            st.focus = None;
            st.dirty = false;
        });
    }

    /// Слушатели клика вне меню и `resize`.
    #[cfg(target_arch = "wasm32")]
    fn bind_outside(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            if this.root_el.get_untracked().is_some_and(|root| root.contains(node.as_ref())) {
                return;
            }
            this.hide_menu();
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || click.remove())));
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            if !this.touch() {
                this.hide_menu();
                this.focus_list();
            }
            this.mobile_active.try_set(false);
            this.release_z();
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || resize.remove())));
    }
}

/// Всё, что нужно для вывода уровней.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<MegaMenuPt>,
    item: Option<Callback<MegaMenuItemContext, AnyView>>,
    submenu_icon: Option<ViewFn>,
}

/// Меню с широким наложением из колонок подменю.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn MegaMenu(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<MegaMenuItem>>,
    /// Ориентация корневого списка.
    #[prop(optional, into)]
    orientation: Prop<MegaMenuOrientation>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Ширина окна, до которой включён мобильный режим (по умолчанию `960px`).
    #[prop(optional, into)]
    breakpoint: OptProp<String>,
    /// Высота корневого списка, после которой он прокручивается.
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Автоматически назначать `z-index` открытой мобильной панели.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Базовый `z-index` поверх значения конфигурации.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Отключает фокус корневого списка.
    #[prop(optional, into)]
    disabled: Prop<bool>,
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
    pt: Option<MegaMenuPt>,
    /// Слот в начале панели.
    #[prop(optional, into)]
    start: Option<ViewFn>,
    /// Слот в конце панели.
    #[prop(optional, into)]
    end: Option<ViewFn>,
    /// Своё содержимое пункта вместо ссылки.
    #[prop(optional)]
    item: Option<Callback<MegaMenuItemContext, AnyView>>,
    /// Своя стрелка группы.
    #[prop(optional, into)]
    submenu_icon: Option<ViewFn>,
    /// Свой значок кнопки меню.
    #[prop(optional, into)]
    menu_icon: Option<ViewFn>,
    /// Фокус вошёл в список.
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Фокус ушёл из меню.
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let nodes = Memo::new({
        let model = model.clone();
        move |_| model.with(|items| build(items))
    });
    let state = RwSignal::new(State::default());
    let core = Arc::new(Core {
        id: menu_id.clone(),
        model: model.clone(),
        nodes,
        orientation,
        state,
        focused: RwSignal::new(false),
        mobile: RwSignal::new(false),
        mobile_active: RwSignal::new(false),
        auto_z_index,
        base_z_index,
        z: RwSignal::new(None),
        root_el: NodeRef::new(),
        list: NodeRef::new(),
        button: NodeRef::new(),
        search: StoredValue::new(String::new()),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        on_focus,
        on_blur,
        #[cfg(target_arch = "wasm32")]
        search_timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    #[cfg(target_arch = "wasm32")]
    {
        let c1 = core.clone();
        client::bind_media(breakpoint.get_untracked().unwrap_or_else(|| "960px".to_owned()), core.mobile, move || {
            c1.mobile_active.try_set(false);
            c1.release_z();
        });
        // Слушатели клика вне и `resize`: пока раскрыт путь или открыта мобильная панель.
        let c2 = core.clone();
        Effect::new(move |_| {
            let want = c2.state.with(|s| s.active.is_some()) || c2.mobile_active.get();
            let bound = c2.cleanups.with_value(|c| !c.is_empty());
            if want && !bound {
                c2.bind_outside();
            } else if !want && bound {
                for cleanup in c2.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
                    cleanup();
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = breakpoint;

    let ctx = Ctx { core: core.clone(), pt: pt.clone(), item, submenu_icon };
    let (mobile, mobile_active) = (core.mobile, core.mobile_active);
    let class = {
        let (pt, style_class, core) = (pt.clone(), style_class.clone(), core.clone());
        move || {
            let vertical = core.orientation.get() == MegaMenuOrientation::Vertical;
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-megamenu p-component")
                .add_if(mobile.get(), "p-megamenu-mobile")
                .add_if(mobile_active.get(), "p-megamenu-mobile-active")
                .add_if(!vertical, "p-megamenu-horizontal")
                .add_if(vertical, "p-megamenu-vertical");
            let extra = style_class.with(|c| c.clone());
            cn.add_user(extra.as_deref());
            pt.root.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let root_style = {
        let (pt, style) = (pt.clone(), style.clone());
        move || {
            let own = style.with(|s| s.clone()).unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&own]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let activedescendant = {
        let core = core.clone();
        move || {
            if !core.focused.get() {
                return None;
            }
            core.state.with(|s| s.focus.clone()).map(|k| core.id_of(&k))
        }
    };
    let list_style = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let scroll = scroll_height.with(|h| h.clone()).map(|h| format!("max-height: {h}; overflow: auto")).unwrap_or_default();
            let z = core.z.get().map(|z| format!("z-index: {z}")).unwrap_or_default();
            let merged = pt.root_list.merge_style_parts(&[&scroll, &z]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let levels = {
        let ctx = ctx.clone();
        move || roots_view(&ctx)
    };
    let button = {
        let (core, pt, model, menu_id) = (core.clone(), pt.clone(), model.clone(), menu_id.clone());
        move || {
            let has_items = model.with(|m| !m.is_empty());
            has_items.then(|| {
                let (click_core, key_core) = (core.clone(), core.clone());
                let icon = match menu_icon.clone() {
                    Some(custom) => custom.run(),
                    None => view! { <BarsIcon class={pt.button_icon.merge_class("").into_owned()} /> }.into_any(),
                };
                let label = prime_core::use_prime_config().locale.text("aria.navigation").to_owned();
                view! {
                    <a
                        node_ref=core.button
                        tabindex="0"
                        role="button"
                        aria-haspopup="true"
                        aria-expanded={move || mobile_active.get().to_string()}
                        aria-controls={menu_id.clone()}
                        aria-label={label}
                        class={pt.button.merge_class("p-megamenu-button").into_owned()}
                        on:click={move |e: web_sys::MouseEvent| {
                            click_core.toggle();
                            e.prevent_default();
                        }}
                        on:keydown={move |e: web_sys::KeyboardEvent| {
                            let code = e.code();
                            if code == "Enter" || code == "NumpadEnter" || code == "Space" {
                                key_core.toggle();
                                e.prevent_default();
                            }
                        }}
                        {..pt.button.attrs()}
                    >
                        {icon}
                    </a>
                }
            })
        }
    };
    let start =
        start.map(|s| view! { <div class={pt.start.merge_class("p-megamenu-start").into_owned()} {..pt.start.attrs()}>{s.run()}</div> });
    let end = end.map(|s| view! { <div class={pt.end.merge_class("p-megamenu-end").into_owned()} {..pt.end.attrs()}>{s.run()}</div> });
    let list_class = pt.root_list.merge_class("p-megamenu-root-list").into_owned();
    let (root_attrs, list_attrs) = (pt.root.attrs(), pt.root_list.attrs());
    let (focus_core, blur_core, key_core, down_core, orient_core) = (core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
    let tabindex = move || if disabled.get() { "-1" } else { "0" };
    view! {
        <div node_ref=core.root_el id={menu_id.clone()} class={class} style={root_style} {..root_attrs}>
            {start}
            {button}
            <ul
                node_ref=core.list
                id={format!("{}_list", menu_id)}
                class={list_class}
                style={list_style}
                role="menubar"
                tabindex={tabindex}
                aria-orientation={move || if orient_core.orientation.get() == MegaMenuOrientation::Vertical { "vertical" } else { "horizontal" }}
                aria-label={move || aria_label.get()}
                aria-labelledby={move || aria_labelledby.get()}
                aria-activedescendant={activedescendant}
                on:focus={move |e| focus_core.list_focus(e)}
                on:blur={move |e| blur_core.list_blur(e)}
                on:keydown={move |e| key_core.key_down(&e)}
                on:mousedown={move |_| down_core.state.update(|s| s.dirty = true)}
                {..list_attrs}
            >
                {levels}
            </ul>
            {end}
        </div>
    }
}

fn join(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(e) => format!("{base} {e}"),
        None => base.to_owned(),
    }
}

/// Количество видимых пунктов без разделителей и позиция среди них (`aria-setsize`/`aria-posinset`).
fn aria_pos(items: &[&MenuItem], index: usize) -> (usize, usize) {
    let size = items.iter().filter(|i| i.visible && !i.separator).count();
    let skipped = items.iter().take(index).filter(|i| !i.visible || i.separator).count();
    (size, index - skipped + 1)
}

/// Корневые пункты.
fn roots_view(ctx: &Ctx) -> AnyView {
    let rows: Vec<MegaMenuItem> = ctx.core.model.with(|m| m.clone());
    let refs: Vec<&MenuItem> = rows.iter().map(|m| &m.item).collect();
    rows.iter()
        .enumerate()
        .filter(|(_, m)| m.item.visible)
        .map(|(index, m)| {
            if m.item.separator {
                separator_view(ctx, &index.to_string(), &m.item)
            } else {
                let (size, pos) = aria_pos(&refs, index);
                root_view(ctx, index, m, size, pos)
            }
        })
        .collect_view()
        .into_any()
}

fn separator_view(ctx: &Ctx, key: &str, item: &MenuItem) -> AnyView {
    let class = ctx.pt.separator.merge_class(&join("p-megamenu-separator", item.style_class.as_deref())).into_owned();
    let id = ctx.core.id_of(key);
    view! { <li id={id} class={class} style={item.style.clone()} role="separator" {..ctx.pt.separator.attrs()}></li> }.into_any()
}

/// Корневой пункт с наложением.
fn root_view(ctx: &Ctx, index: usize, mega: &MegaMenuItem, set_size: usize, pos_inset: usize) -> AnyView {
    let key = index.to_string();
    let group = mega.columns.iter().any(|c| !c.is_empty());
    let active = {
        let (state, key) = (ctx.core.state, key.clone());
        move || state.with(|s| s.active.as_deref() == Some(key.as_str()))
    };
    let overlay = group.then(|| {
        let (ctx, active) = (ctx.clone(), active.clone());
        move || active().then(|| overlay_view(&ctx, index))
    });
    item_view(ctx, &key, &mega.item, 1, set_size, pos_inset, group, true, active, overlay)
}

/// Наложение корневого пункта: сетка колонок с подменю.
fn overlay_view(ctx: &Ctx, root: usize) -> AnyView {
    let columns: Vec<Vec<MegaMenuSubmenu>> = ctx.core.model.with(|m| m.get(root).map(|r| r.columns.clone()).unwrap_or_default());
    let count = columns.len();
    let (pt, mobile) = (ctx.pt.clone(), ctx.core.mobile);
    let cols = columns
        .iter()
        .enumerate()
        .map(|(c, column)| {
            let subs =
                column.iter().enumerate().filter(|(_, sub)| sub.visible).map(|(s, sub)| submenu_view(ctx, root, c, s, sub)).collect_view();
            let pt = pt.clone();
            let class = move || pt.column.merge_class(column_class(count, mobile.get())).into_owned();
            view! { <div class={class} {..ctx.pt.column.attrs()}>{subs}</div> }
        })
        .collect_view();
    view! {
        <div class={pt.overlay.merge_class("p-megamenu-overlay").into_owned()} {..pt.overlay.attrs()}>
            <div class={pt.grid.merge_class("p-megamenu-grid").into_owned()} {..pt.grid.attrs()}>{cols}</div>
        </div>
    }
    .into_any()
}

fn submenu_view(ctx: &Ctx, root: usize, c: usize, s: usize, sub: &MegaMenuSubmenu) -> AnyView {
    let sub_key = format!("{root}_{c}_{s}");
    let pt = &ctx.pt;
    let label = sub.label.clone().map(|label| {
        let class = pt.submenu_label.merge_class(&join("p-megamenu-submenu-label", sub.style_class.as_deref())).into_owned();
        view! { <li class={class} style={sub.style.clone()} role="presentation" {..pt.submenu_label.attrs()}>{label}</li> }
    });
    let refs: Vec<&MenuItem> = sub.items.iter().collect();
    let items = sub
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.visible)
        .map(|(j, item)| {
            let key = format!("{sub_key}_{j}");
            if item.separator {
                separator_view(ctx, &key, item)
            } else {
                let (size, pos) = aria_pos(&refs, j);
                item_view(ctx, &key, item, 2, size, pos, false, false, || false, None::<fn() -> Option<AnyView>>)
            }
        })
        .collect_view();
    let list_id = format!("{}_list", ctx.core.id_of(&sub_key));
    let class = pt.submenu.merge_class("p-megamenu-submenu").into_owned();
    view! {
        <ul id={list_id} class={class} role="menu" tabindex="-1" {..pt.submenu.attrs()}>
            {label}
            {items}
        </ul>
    }
    .into_any()
}

/// Пункт `li`: корневой (с наложением) или из подменю.
#[allow(clippy::too_many_arguments)]
fn item_view<O, V>(
    ctx: &Ctx,
    key: &str,
    item: &MenuItem,
    level: usize,
    set_size: usize,
    pos_inset: usize,
    group: bool,
    root: bool,
    active: impl Fn() -> bool + Clone + Send + Sync + 'static,
    overlay: Option<O>,
) -> AnyView
where
    O: Fn() -> Option<V> + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let id = core.id_of(key);
    let state = core.state;
    let focused = {
        let (core, key) = (core.clone(), key.to_owned());
        move || core.focused.get() && state.with(|s| s.focus.as_deref() == Some(key.as_str()))
    };
    let class = {
        let (pt, item, active, focused) = (pt.clone(), item.clone(), active.clone(), focused.clone());
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-megamenu-item")
                .add_if(active(), "p-megamenu-item-active")
                .add_if(focused(), "p-focus")
                .add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content = match ctx.item {
        Some(template) => untrack(|| template.run(MegaMenuItemContext { item: item.clone(), has_submenu: group, root })),
        None => link_view(ctx, item, group),
    };
    let (click_core, hover_core, click_key, hover_key) = (core.clone(), core.clone(), key.to_owned(), key.to_owned());
    let (hp, ae) = (group.then_some("menu"), {
        let active = active.clone();
        move || group.then(|| active().to_string())
    });
    let (hl, fo) = (active.clone(), focused.clone());
    let label = item.label.clone();
    let disabled = item.disabled;
    let content_class = pt.item_content.merge_class("p-megamenu-item-content").into_owned();
    let (item_attrs, content_attrs) = (pt.item.attrs(), pt.item_content.attrs());
    let mut item_attrs = item_attrs;
    item_attrs.push(leptos::attr::custom::custom_attribute("aria-level", level.to_string()).into_any_attr());
    view! {
        <li
            id={id}
            class={class}
            style={item.style.clone()}
            role="menuitem"
            data-p-active={move || hl().to_string()}
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
            {overlay}
        </li>
    }
    .into_any()
}

/// Стандартное содержимое пункта: ссылка со значком, подписью и стрелкой группы.
fn link_view(ctx: &Ctx, item: &MenuItem, group: bool) -> AnyView {
    let pt = &ctx.pt;
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-megamenu-item-icon", item.icon_class.as_deref());
        view! { <span class={pt.item_icon.merge_class(&class).into_owned()} tabindex="-1" {..pt.item_icon.attrs()}>{icon.run()}</span> }
    });
    let arrow = group.then(|| match ctx.submenu_icon.clone() {
        Some(custom) => custom.run(),
        None => {
            let class = pt.submenu_icon.merge_class("p-megamenu-submenu-icon").into_owned();
            let (core, class2) = (ctx.core.clone(), class.clone());
            view! {
                {move || {
                    if core.orientation.get() == MegaMenuOrientation::Horizontal || core.mobile_active.get() {
                        view! { <AngleDownIcon class={class2.clone()} /> }.into_any()
                    } else {
                        view! { <AngleRightIcon class={class.clone()} /> }.into_any()
                    }
                }}
            }
            .into_any()
        }
    });
    let link_class = join("p-megamenu-item-link", item.link_class.as_deref());
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
            <span class={pt.item_label.merge_class("p-megamenu-item-label").into_owned()} {..pt.item_label.attrs()}>{item.label.clone()}</span>
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

    /// Ведёт `mobile` по медиазапросу, вызывает `on_change` при смене и снимает слушатель при размонтировании.
    pub fn bind_media(breakpoint: String, mobile: RwSignal<bool>, on_change: impl Fn() + 'static) {
        let Some(window) = web_sys::window() else { return };
        let Some(query) = window.match_media(&format!("(max-width: {breakpoint})")).ok().flatten() else { return };
        mobile.set(query.matches());
        let closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(move |e: web_sys::MediaQueryListEvent| {
            mobile.try_set(e.matches());
            on_change();
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
