//! Menu — port of `p-menu` from PrimeNG 21.1.10.
//!
//! A list of commands and links with groups and separators: static or popping up at a target element. The popup menu
//! is controlled by [`MenuRef`] (the analog of `menu.toggle($event)` from PrimeNG): it is created in advance, passed to [`Menu`]
//! and called from handlers.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::{Menu, MenuItem};
//!
//! let html = view! {
//!     <Menu model=vec![MenuItem::new().label("New"), MenuItem::new().label("Search")] />
//! }
//! .to_html();
//! assert!(html.contains("p-menu-item-label") && html.contains("New"));
//! ```

mod state;

use leptos::prelude::*;
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub use state::{
    MenuCommand, MenuItem, Row, can_run_command, enabled_ids, flatten, has_submenu, item_classes, item_id, next_id, prev_id, root_classes,
    root_style, should_prevent_default,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::set_timeout_with_handle;
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use prime_rs_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Pass-through parts of [`Menu`].
#[derive(Clone, Debug, Default)]
pub struct MenuPt {
    /// Root.
    pub root: Pt,
    /// `start` block.
    pub start: Pt,
    /// `ul` list.
    pub list: Pt,
    /// Group header.
    pub submenu_label: Pt,
    /// Separator.
    pub separator: Pt,
    /// `end` block.
    pub end: Pt,
    /// Item `li` element.
    pub item: Pt,
    /// Item content.
    pub item_content: Pt,
    /// Item link.
    pub item_link: Pt,
    /// Item icon.
    pub item_icon: Pt,
    /// Item label.
    pub item_label: Pt,
}

/// Control object of the popup menu: `toggle`/`show`/`hide` from an event.
#[derive(Clone, Copy)]
pub struct MenuRef {
    visible: RwSignal<bool>,
    target: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
}

impl Default for MenuRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Default target: the event's `currentTarget`, or `target` if there is none.
fn event_target(event: &web_sys::Event) -> Option<web_sys::HtmlElement> {
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()))
}

impl MenuRef {
    /// Creates a hidden menu. Call inside a component (the signal owner).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false), target: StoredValue::new_local(None) }
    }

    /// The menu is currently shown (with subscription).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// The menu is currently shown (without subscription).
    pub fn visible_untracked(&self) -> bool {
        self.visible.get_untracked()
    }

    /// Shows the menu at the event's `currentTarget`.
    pub fn show(&self, event: &web_sys::Event) {
        if let Some(target) = event_target(event) {
            self.show_at(&target);
        }
    }

    /// Shows the menu at an explicit target.
    pub fn show_at(&self, target: &web_sys::HtmlElement) {
        self.target.set_value(Some(target.clone()));
        self.visible.set(true);
    }

    /// Toggles the menu; the target is the event's `currentTarget`.
    pub fn toggle(&self, event: &web_sys::Event) {
        if self.visible.get_untracked() { self.hide() } else { self.show(event) }
    }

    /// Hides the menu.
    pub fn hide(&self) {
        self.visible.try_set(false);
    }
}

/// State shared by handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    r: MenuRef,
    popup: Prop<bool>,
    model: Prop<Vec<MenuItem>>,
    menu_id: String,
    panel: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    focused: RwSignal<bool>,
    focused_id: RwSignal<Option<String>>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_visible: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_focus: Option<Callback<()>>,
    on_blur: Option<Callback<()>>,
}

impl Core {
    fn rows(&self) -> Vec<Row> {
        self.model.with(|m| flatten(m, &self.menu_id))
    }

    fn rows_untracked(&self) -> Vec<Row> {
        flatten(&self.model.get_untracked(), &self.menu_id)
    }

    fn enabled(&self) -> Vec<String> {
        enabled_ids(&self.rows_untracked())
    }

    /// Removes window subscriptions and releases the z-index.
    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_rs_core::zindex::revert(z);
        }
    }

    fn focus_target(&self) {
        self.r.target.with_value(|t| {
            if let Some(t) = t {
                let _ = t.focus();
            }
        });
    }

    fn set_focused_id(&self, id: Option<String>) {
        self.focused_id.set(id);
    }

    fn list_focus(&self) {
        if self.focused.get_untracked() {
            return;
        }
        self.focused.set(true);
        if !self.popup.get_untracked() {
            self.set_focused_id(self.enabled().into_iter().next());
        }
        if let Some(cb) = self.on_focus {
            cb.run(());
        }
    }

    fn list_blur(&self) {
        if !self.focused.get_untracked() {
            return;
        }
        self.focused.set(false);
        self.set_focused_id(None);
        if let Some(cb) = self.on_blur {
            cb.run(());
        }
    }

    fn item_click(&self, event: web_sys::MouseEvent, item: &MenuItem, id: &str) {
        if !self.focused.get_untracked() {
            self.focused.set(true);
            if let Some(cb) = self.on_focus {
                cb.run(());
            }
        }
        if should_prevent_default(item) {
            event.prevent_default();
        }
        if item.disabled {
            return;
        }
        if let Some(command) = item.command {
            command.run(MenuCommand { event, item: item.clone() });
        }
        if self.popup.get_untracked() {
            self.r.hide();
        } else if self.focused_id.get_untracked().as_deref() != Some(id) {
            self.set_focused_id(Some(id.to_string()));
        }
    }

    fn enter(&self) {
        let Some(id) = self.focused_id.get_untracked() else { return };
        let Some(li) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id)) else { return };
        if self.popup.get_untracked() {
            self.focus_target();
        }
        let anchor = li.query_selector("a").ok().flatten().unwrap_or(li);
        if let Ok(el) = anchor.dyn_into::<web_sys::HtmlElement>() {
            el.click();
        }
    }

    fn key_down(&self, event: &web_sys::KeyboardEvent) {
        let ids = self.enabled();
        let current = self.focused_id.get_untracked();
        match event.code().as_str() {
            "ArrowDown" => {
                self.set_focused_id(next_id(&ids, current.as_deref()));
                event.prevent_default();
            }
            "ArrowUp" => {
                if event.alt_key() && self.popup.get_untracked() {
                    self.focus_target();
                    self.r.hide();
                } else {
                    self.set_focused_id(prev_id(&ids, current.as_deref()));
                }
                event.prevent_default();
            }
            "Home" => {
                self.set_focused_id(ids.first().cloned());
                event.prevent_default();
            }
            "End" => {
                self.set_focused_id(ids.last().cloned());
                event.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.enter();
                event.prevent_default();
            }
            "Escape" | "Tab" if self.popup.get_untracked() => {
                self.focus_target();
                self.r.hide();
            }
            _ => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn is_inside(&self, event_target: Option<web_sys::EventTarget>) -> (bool, bool) {
        let node = event_target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside_panel = self.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref()));
        let inside_target = self.r.target.with_value(|t| t.as_ref().is_some_and(|t| t.contains(node.as_ref())));
        (inside_panel, inside_target)
    }

    /// The menu has just been shown: z-index, alignment, focus, subscriptions, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(target)) = (self.panel.get_untracked(), self.r.target.get_value()) else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Menu) + self.base_z_index.get_untracked();
            let z = prime_rs_core::zindex::generate(ZGroup::Menu, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        align_overlay(&panel, &target, AlignMode::Relative, false);
        if let Some(list) = self.list.get_untracked() {
            prime_dom::focus_no_scroll(&list);
        }
        // Subscriptions go in the next tick: the click that opened the menu is still bubbling to the window and would close it itself.
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.r.visible.is_disposed() || !this.r.visible.get_untracked() {
                    return;
                }
                this.bind_window();
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// An outside click, window resize and scrolling of a target ancestor hide the menu.
    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            if !this.r.visible.get_untracked() {
                return;
            }
            let (panel, target) = this.is_inside(e.target());
            if !panel && !target {
                this.r.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.r.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || resize.remove()));
        });

        // Scrolling of a target ancestor (or page scroll that moved the target) closes the menu.
        let this = self.clone();
        if let Some(target) = self.r.target.get_value()
            && let Some(unwatch) = prime_dom::watch_scroll(target, move || this.r.hide())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
    }

    /// The menu has just been hidden: subscriptions removed, focus and target forgotten, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.focused.set(false);
        self.set_focused_id(None);
        self.r.target.set_value(None);
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

fn join_class(base: &str, extra: &Option<String>) -> String {
    match extra.as_deref().filter(|c| !c.is_empty()) {
        Some(c) => format!("{base} {c}"),
        None => base.to_string(),
    }
}

/// Everything needed to render the list rows.
#[derive(Clone)]
struct Parts {
    pt: Arc<MenuPt>,
    item: Option<Callback<MenuItem, AnyView>>,
    submenu_header: Option<Callback<MenuItem, AnyView>>,
}

fn render_row(core: &Arc<Core>, parts: &Parts, row: Row) -> AnyView {
    let pt = &parts.pt;
    match row {
        Row::Separator => {
            view! { <li class={pt.separator.merge_class("p-menu-separator")} role="separator" {..pt.separator.attrs()}></li> }.into_any()
        }
        Row::Header(group) => {
            let content = match parts.submenu_header {
                Some(cb) => untrack(|| cb.run(group.clone())),
                None => view! { <span>{group.label.clone()}</span> }.into_any(),
            };
            view! { <li class={pt.submenu_label.merge_class("p-menu-submenu-label")} role="none" {..pt.submenu_label.attrs()}>{content}</li> }.into_any()
        }
        Row::Item { id, item } => render_item(core, parts, id, item),
    }
}

fn render_item(core: &Arc<Core>, parts: &Parts, id: String, item: MenuItem) -> AnyView {
    let pt = &parts.pt;
    let focused = {
        let (core, id) = (core.clone(), id.clone());
        move || core.focused_id.get().as_deref() == Some(id.as_str())
    };
    let class = {
        let (item, focused, pt) = (item.clone(), focused.clone(), pt.clone());
        move || pt.item.merge_class(&item_classes(&item, focused()))
    };
    let content = match parts.item {
        Some(cb) => untrack(|| cb.run(item.clone())),
        None => {
            let icon = item.icon.clone().map(|icon| {
                let class = join_class("p-menu-item-icon", &item.icon_class);
                view! { <span class={pt.item_icon.merge_class(&class)} {..pt.item_icon.attrs()}>{icon.run()}</span> }
            });
            let link_class = join_class("p-menu-item-link", &item.link_class);
            view! {
                <a
                    class={pt.item_link.merge_class(&link_class)}
                    tabindex="-1"
                    href={item.url.clone()}
                    target={item.target.clone()}
                    title={item.title.clone()}
                    {..pt.item_link.attrs()}
                >
                    {icon}
                    <span class={pt.item_label.merge_class("p-menu-item-label")} {..pt.item_label.attrs()}>{item.label.clone()}</span>
                </a>
            }
            .into_any()
        }
    };
    let on_click = {
        let (core, item, id) = (core.clone(), item.clone(), id.clone());
        move |e: web_sys::MouseEvent| core.item_click(e, &item, &id)
    };
    let disabled = if item.disabled { "true" } else { "false" };
    let focused_attr = {
        let focused = focused.clone();
        move || if focused() { "true" } else { "false" }
    };
    view! {
        <li
            class=class
            id={id}
            role="menuitem"
            aria-label={item.label.clone()}
            aria-disabled=disabled
            data-p-disabled=disabled
            data-p-focused=focused_attr
            style={item.style.clone()}
            {..pt.item.attrs()}
        >
            <div class={pt.item_content.merge_class("p-menu-item-content")} on:click={on_click} {..pt.item_content.attrs()}>{content}</div>
        </li>
    }
    .into_any()
}

/// A list of commands and links, static or popup.
///
/// Each property accepts a value or a signal. The popup menu is rendered inside its own place in the markup
/// (not in `body`), so for alignment the parent must be `position: relative`, not `overflow: hidden`.
#[component]
pub fn Menu(
    /// Menu items; items with subitems become groups.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Popup mode: the menu is shown via [`MenuRef`].
    #[prop(optional, into)]
    popup: Prop<bool>,
    /// Control object of the popup menu.
    #[prop(optional)]
    menu_ref: Option<MenuRef>,
    /// Automatically assign `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the config value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// `aria-label` attribute of the list.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the list.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// `id` attribute of the root; a unique one is created by default.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `tabindex` of the list.
    #[prop(optional, default = Prop::Static(0), into)]
    tabindex: Prop<i32>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called after the popup menu is shown and aligned.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Called after the popup menu is hidden.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Called when the list receives focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    /// Called when the list loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<()>>,
    /// View of the item content instead of a link with icon and label.
    #[prop(optional, into)]
    item: Option<Callback<MenuItem, AnyView>>,
    /// View of the group header instead of a label.
    #[prop(optional, into)]
    submenu_header: Option<Callback<MenuItem, AnyView>>,
    /// Content before the list.
    #[prop(optional, into)]
    start: Option<ViewFn>,
    /// Content after the list.
    #[prop(optional, into)]
    end: Option<ViewFn>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<MenuPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let r = menu_ref.unwrap_or_default();
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_rs_core::ids::next_unique()));
    let core = Arc::new(Core {
        r,
        popup: popup.clone(),
        model,
        menu_id: menu_id.clone(),
        panel: NodeRef::new(),
        list: NodeRef::new(),
        focused: RwSignal::new(false),
        focused_id: RwSignal::new(None),
        auto_z_index,
        base_z_index,
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_visible: StoredValue::new(false),
        on_show,
        on_hide,
        on_focus,
        on_blur,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // Side effects of showing and hiding (position, listeners, focus) — only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let visible = core.r.visible.get() && core.popup.get();
            // The panel gets into the DOM after this effect, so we also watch the reference to it.
            let mounted = core.panel.get().is_some();
            let was = core.was_visible.get_value();
            if visible && mounted && !was {
                core.was_visible.set_value(true);
                core.opened();
            } else if !visible && was {
                core.was_visible.set_value(false);
                core.closed();
            }
        });
    }

    let parts = Parts { pt: pt.clone(), item, submenu_header };
    let panel = core.panel;
    let list = core.list;
    let visible = r.visible;
    let root_id = menu_id.clone();
    move || {
        let is_popup = popup.get();
        (!is_popup || visible.get()).then(|| {
            let (core, parts, pt) = (core.clone(), parts.clone(), pt.clone());
            let (style, style_class, aria_label, aria_labelledby) =
                (style.clone(), style_class.clone(), aria_label.clone(), aria_labelledby.clone());
            let class = {
                let (pt, style_class) = (pt.clone(), style_class.clone());
                move || {
                    let mut cn = String::from(root_classes(is_popup));
                    if let Some(c) = style_class.get().filter(|c| !c.is_empty()) {
                        cn.push(' ');
                        cn.push_str(&c);
                    }
                    pt.root.merge_class(&cn)
                }
            };
            let root_style_attr = {
                let pt = pt.clone();
                move || {
                    let user = style.get().unwrap_or_default();
                    pt.root.merge_style_parts(&[root_style(is_popup), &user])
                }
            };
            let activedescendant = {
                let core = core.clone();
                move || if core.focused.get() { core.focused_id.get() } else { None }
            };
            let rows = {
                let (core, parts) = (core.clone(), parts.clone());
                move || core.rows().into_iter().map(|row| render_row(&core, &parts, row)).collect_view()
            };
            let (on_focus_c, on_blur_c, on_key_c) = (core.clone(), core.clone(), core.clone());
            let start =
                start.clone().map(|s| view! { <div class={pt.start.merge_class("p-menu-start")} {..pt.start.attrs()}>{s.run()}</div> });
            let end = end.clone().map(|s| view! { <div class={pt.end.merge_class("p-menu-end")} {..pt.end.attrs()}>{s.run()}</div> });
            view! {
                <div
                    node_ref=panel
                    class=class
                    style=root_style_attr
                    id={root_id.clone()}
                    data-p={is_popup.then_some("overlay")}
                    {..pt.root.attrs()}
                >
                    {start}
                    <ul
                        node_ref=list
                        class={pt.list.merge_class("p-menu-list")}
                        role="menu"
                        id={format!("{}_list", root_id)}
                        tabindex={let tabindex = tabindex.clone(); move || tabindex.get().to_string()}
                        aria-activedescendant=activedescendant
                        aria-label={move || aria_label.get()}
                        aria-labelledby={move || aria_labelledby.get()}
                        on:focus=move |_| on_focus_c.list_focus()
                        on:blur=move |_| on_blur_c.list_blur()
                        on:keydown=move |e| on_key_c.key_down(&e)
                        {..pt.list.attrs()}
                    >
                        {rows}
                    </ul>
                    {end}
                </div>
            }
        })
    }
}
