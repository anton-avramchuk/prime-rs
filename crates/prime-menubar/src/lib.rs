//! Menubar — порт `p-menubar` из PrimeNG 21.1.10.
//!
//! Горизонтальная панель меню с выпадающими подменю. Пункты — [`prime_menu::MenuItem`]: поле `items` задаёт подменю. Логика
//! путей, фокуса и поиска общая с `TieredMenu` (крейт `prime-tieredmenu`); корневой уровень горизонтальный: ← → ходят по
//! корню, ↓ раскрывает группу. На узком окне (`breakpoint`) список сворачивается в кнопку. Слоты `start` и `end` выводятся по
//! краям панели.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::MenuItem;
//! use prime_menubar::Menubar;
//!
//! let model = vec![MenuItem::new().label("File").items(vec![MenuItem::new().label("New")]), MenuItem::new().label("Quit")];
//! let html = Owner::new().with(|| view! { <Menubar model=model /> }.to_html());
//! assert!(html.contains("p-menubar-root-list") && html.contains("role=\"menubar\"") && html.contains("Quit"));
//! ```

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{AngleDownIcon, AngleRightIcon, BarsIcon};
use prime_menu::{MenuCommand, MenuItem, can_run_command, should_prevent_default};
use prime_tieredmenu::{Click, Focus, Node, State, Tree, aria_pos_inset, aria_set_size, build, is_printable, item_id};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through частей [`Menubar`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MenubarPt {
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
    /// Подменю.
    pub submenu: Pt,
    /// Разделитель.
    pub separator: Pt,
    /// Слот `end`.
    pub end: Pt,
}

/// Контекст представления `item`.
#[derive(Clone)]
pub struct MenubarItemContext {
    /// Пункт.
    pub item: MenuItem,
    /// Есть ли у пункта подменю.
    pub has_submenu: bool,
    /// Пункт на корневом уровне.
    pub root: bool,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    model: Prop<Vec<MenuItem>>,
    nodes: Memo<Vec<Node>>,
    state: RwSignal<State>,
    focused: RwSignal<bool>,
    focused_key: Memo<Option<String>>,
    mobile: RwSignal<bool>,
    mobile_active: RwSignal<bool>,
    auto_display: Prop<bool>,
    auto_hide: Prop<bool>,
    auto_hide_delay: u32,
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
    #[cfg(target_arch = "wasm32")]
    hide_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
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

    /// Закрывает всё: путь и фокус; при открытой мобильной панели фокус возвращается на кнопку (`hide`).
    fn hide_menu(&self) {
        #[cfg(target_arch = "wasm32")]
        if self.mobile_active.get_untracked() {
            let button = self.button;
            set_timeout(
                move || {
                    if let Some(button) = button.try_get_untracked().flatten() {
                        prime_dom::focus_no_scroll(&button);
                    }
                },
                Duration::ZERO,
            );
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
        self.update(|t, st| {
            let visible = t.visible(st);
            st.focus = Focus { index: t.first_focused(st, &visible), parent_key: String::new() };
        });
        self.focus_list();
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
        self.state.update(|st| st.dirty = true);
        let outcome = self.update(|t, st| t.node(key).map(|n| t.click(st, n)));
        if matches!(outcome, Some(Click::Leaf(_))) {
            self.mobile_active.set(false);
            self.release_z();
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

    /// Курсор покинул меню: при `auto_hide` подменю закрывается через `auto_hide_delay`.
    fn mouse_leave(self: &Arc<Self>) {
        #[cfg(target_arch = "wasm32")]
        if self.auto_hide.get_untracked() {
            self.cancel_auto_hide();
            let this = self.clone();
            if let Ok(handle) = set_timeout_with_handle(move || this.hide_menu(), Duration::from_millis(u64::from(self.auto_hide_delay))) {
                self.hide_timer.set_value(Some(handle));
            }
        }
    }

    /// Курсор вернулся в меню: отложенное закрытие отменяется.
    fn mouse_enter(&self) {
        self.cancel_auto_hide();
    }

    fn cancel_auto_hide(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.hide_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
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
            let anchor = li.query_selector(".p-menubar-item-link").ok().flatten().or_else(|| li.query_selector("a,button").ok().flatten());
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
                self.update(|t, st| t.bar_down(st));
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowUp" => {
                self.update(|t, st| t.bar_up(st));
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowRight" => {
                self.update(|t, st| t.bar_right(st));
                self.search.set_value(String::new());
                self.scroll_focused_into_view();
                e.prevent_default();
            }
            "ArrowLeft" => {
                self.update(|t, st| t.bar_left(st));
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
                self.hide_menu();
                self.update(|t, st| t.escape(st));
                self.focus_list();
                e.prevent_default();
            }
            "Tab" => {
                let mobile = self.mobile.get_untracked();
                self.update(|t, st| t.tab(st, mobile));
            }
            "PageDown" | "PageUp" | "Backspace" | "ShiftLeft" | "ShiftRight" => {}
            _ => {
                if is_printable(&e.key(), meta) {
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
            let idle = self.state.with_untracked(|s| s.focus.index.is_none() && s.active.is_empty() && !s.dirty);
            if from_outside && idle {
                self.update(|t, st| {
                    let visible = t.visible(st);
                    st.focus = Focus { index: t.first_focused(st, &visible), parent_key: String::new() };
                });
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
            st.focus = Focus::default();
            st.dirty = false;
        });
    }

    /// Слушатель клика вне меню: закрывает подменю, а вне кнопки при открытой панели — и панель.
    #[cfg(target_arch = "wasm32")]
    fn bind_outside(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let in_list = this.list.get_untracked().is_some_and(|l| l.contains(node.as_ref()));
            if in_list {
                return;
            }
            let in_button = this.button.get_untracked().is_some_and(|b| b.contains(node.as_ref()));
            if this.mobile_active.get_untracked() && !in_button {
                this.mobile_active.set(false);
                this.release_z();
            } else {
                this.hide_menu();
            }
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || click.remove())));
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            if !this.touch() {
                this.hide_menu();
                this.focus_list();
            }
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || resize.remove())));
    }
}

/// Всё, что нужно для вывода уровней.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<MenubarPt>,
    item: Option<Callback<MenubarItemContext, AnyView>>,
    submenu_icon: Option<ViewFn>,
}

/// Горизонтальное меню с подменю.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn Menubar(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Ширина окна, до которой включён мобильный режим (по умолчанию `960px`).
    #[prop(optional, into)]
    breakpoint: OptProp<String>,
    /// Автоматически назначать `z-index` открытой мобильной панели.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Базовый `z-index` поверх значения конфигурации.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Раскрывать группы при наведении (после первого раскрытия кликом).
    #[prop(optional, default = Prop::Static(true), into)]
    auto_display: Prop<bool>,
    /// Закрывать подменю после ухода курсора с меню.
    #[prop(optional, into)]
    auto_hide: Prop<bool>,
    /// Задержка автоскрытия в миллисекундах (по умолчанию 100).
    #[prop(optional, default = 100)]
    auto_hide_delay: u32,
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
    pt: Option<MenubarPt>,
    /// Слот в начале панели.
    #[prop(optional, into)]
    start: Option<ViewFn>,
    /// Слот в конце панели.
    #[prop(optional, into)]
    end: Option<ViewFn>,
    /// Своё содержимое пункта вместо ссылки.
    #[prop(optional)]
    item: Option<Callback<MenubarItemContext, AnyView>>,
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
    let focused = RwSignal::new(false);
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
        focused,
        focused_key,
        mobile: RwSignal::new(false),
        mobile_active: RwSignal::new(false),
        auto_display,
        auto_hide,
        auto_hide_delay,
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
        #[cfg(target_arch = "wasm32")]
        hide_timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || {
            core.cancel_auto_hide();
            core.release();
        });
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
            let want = c2.state.with(|s| !s.active.is_empty()) || c2.mobile_active.get();
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
    let mobile = core.mobile;
    let mobile_active = core.mobile_active;
    let class = {
        let (pt, style_class) = (pt.clone(), style_class.clone());
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-menubar p-component").add_if(mobile.get(), "p-menubar-mobile").add_if(mobile_active.get(), "p-menubar-mobile-active");
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
            core.focused_key.get()?;
            core.tree(|t| t.focused_id(&core.state.get(), &core.id))
        }
    };
    let list_style = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let z = core.z.get().map(|z| format!("z-index: {z}")).unwrap_or_default();
            let merged = pt.root_list.merge_style_parts(&[&z]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let levels = {
        let ctx = ctx.clone();
        move || items_view(&ctx, "")
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
                        class={pt.button.merge_class("p-menubar-button").into_owned()}
                        on:click={move |e: web_sys::MouseEvent| {
                            click_core.toggle();
                            e.prevent_default();
                        }}
                        on:keydown={move |e: web_sys::KeyboardEvent| {
                            let code = e.code();
                            if code == "Enter" || code == "Space" {
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
        start.map(|s| view! { <div class={pt.start.merge_class("p-menubar-start").into_owned()} {..pt.start.attrs()}>{s.run()}</div> });
    let end = end.map(|s| view! { <div class={pt.end.merge_class("p-menubar-end").into_owned()} {..pt.end.attrs()}>{s.run()}</div> });
    let list_class = pt.root_list.merge_class("p-menubar-root-list").into_owned();
    let (root_attrs, list_attrs) = (pt.root.attrs(), pt.root_list.attrs());
    let (focus_core, blur_core, key_core, down_core, leave_core, enter_core) =
        (core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
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
                tabindex="0"
                aria-label={move || aria_label.get()}
                aria-labelledby={move || aria_labelledby.get()}
                aria-activedescendant={activedescendant}
                on:focus={move |e| focus_core.list_focus(e)}
                on:blur={move |e| blur_core.list_blur(e)}
                on:keydown={move |e| key_core.key_down(&e)}
                on:mousedown={move |_| down_core.state.update(|s| s.dirty = true)}
                on:mouseleave={move |_| leave_core.mouse_leave()}
                on:mouseenter={move |_| enter_core.mouse_enter()}
                {..list_attrs}
            >
                {levels}
            </ul>
            {end}
        </div>
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
    let class = ctx.pt.separator.merge_class(&join("p-menubar-separator", item.style_class.as_deref())).into_owned();
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
    let root = node.level == 0;
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
            cn.add("p-menubar-item")
                .add_if(active(), "p-menubar-item-active")
                .add_if(focused(), "p-focus")
                .add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content = match ctx.item {
        Some(template) => untrack(|| template.run(MenubarItemContext { item: item.clone(), has_submenu: group, root })),
        None => link_view(ctx, item, group, root),
    };
    let (click_core, hover_core, click_key, hover_key) = (core.clone(), core.clone(), key.clone(), key.clone());
    let submenu = group.then(|| {
        let (ctx, key, active) = (ctx.clone(), key.clone(), active.clone());
        let labelledby = id.clone();
        move || {
            active().then(|| {
                let class = ctx.pt.submenu.merge_class("p-menubar-submenu").into_owned();
                let style = ctx.pt.submenu.merge_style("display: flex");
                let sub_id = format!("{}_{}_list", ctx.core.id, key);
                let children = items_view(&ctx, &key);
                view! { <ul id={sub_id} class={class} style={style} role="menu" aria-labelledby={labelledby.clone()} {..ctx.pt.submenu.attrs()}>{children}</ul> }
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
    let content_class = pt.item_content.merge_class("p-menubar-item-content").into_owned();
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

/// Стандартное содержимое пункта: ссылка со значком, подписью и стрелкой группы (вниз у корня, вправо у вложенных).
fn link_view(ctx: &Ctx, item: &MenuItem, group: bool, root: bool) -> AnyView {
    let pt = &ctx.pt;
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-menubar-item-icon", item.icon_class.as_deref());
        view! { <span class={pt.item_icon.merge_class(&class).into_owned()} tabindex="-1" {..pt.item_icon.attrs()}>{icon.run()}</span> }
    });
    let arrow = group.then(|| match ctx.submenu_icon.clone() {
        Some(custom) => custom.run(),
        None => {
            let class = pt.submenu_icon.merge_class("p-menubar-submenu-icon").into_owned();
            if root { view! { <AngleDownIcon class={class} /> }.into_any() } else { view! { <AngleRightIcon class={class} /> }.into_any() }
        }
    });
    let link_class = join("p-menubar-item-link", item.link_class.as_deref());
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
            <span class={pt.item_label.merge_class("p-menubar-item-label").into_owned()} {..pt.item_label.attrs()}>{item.label.clone()}</span>
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
