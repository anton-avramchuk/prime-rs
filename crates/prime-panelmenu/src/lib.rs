//! PanelMenu — port of PrimeNG 21.1.10 `p-panelmenu`.
//!
//! A hybrid of Accordion and Tree: a vertical list of panels, each expanding in place. Items are [`prime_menu::MenuItem`]:
//! root items become panel headers, the `items` field defines branches inside a panel, `expanded` — the initial
//! expansion. With `multiple = false` (default) one root panel is expanded; nested branches are independent.
//! Expansion can be controlled through [`PanelMenuRef`].
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::MenuItem;
//! use prime_panelmenu::PanelMenu;
//!
//! let model = vec![MenuItem::new().label("Files").expanded(true).items(vec![MenuItem::new().label("Documents")])];
//! let html = Owner::new().with(|| view! { <PanelMenu model=model /> }.to_html());
//! assert!(html.contains("p-panelmenu-panel") && html.contains("aria-expanded=\"true\"") && html.contains("Documents"));
//! ```

mod state;

use leptos::prelude::*;
use prime_icons::{ChevronDownIcon, ChevronRightIcon};
use prime_menu::{MenuCommand, MenuItem, can_run_command, should_prevent_default};
use prime_rs_core::collapsible::hidden_style;
use prime_rs_core::{OptProp, Prop, Pt};
use prime_tieredmenu::{Node, aria_pos_inset, aria_set_size, build, is_printable, item_at, item_id};
use std::collections::HashSet;
use std::sync::Arc;

pub use state::{
    Left, Move, Right, ancestors_expanded, arrow_down, arrow_left, arrow_right, arrow_up, end, group_keys, header_edge, header_step, home,
    panel_visible, search, seed_expanded, toggle, toggle_root,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through parts of [`PanelMenu`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PanelMenuPt {
    /// Root.
    pub root: Pt,
    /// Panel (`div.p-panelmenu-panel`).
    pub panel: Pt,
    /// Panel header.
    pub header: Pt,
    /// Header content.
    pub header_content: Pt,
    /// Header link.
    pub header_link: Pt,
    /// Header icon.
    pub header_icon: Pt,
    /// Header label.
    pub header_label: Pt,
    /// Content area (expansion container).
    pub content_container: Pt,
    /// Content wrapper.
    pub content_wrapper: Pt,
    /// Panel content.
    pub content: Pt,
    /// Root list of the panel.
    pub root_list: Pt,
    /// Nested list.
    pub submenu: Pt,
    /// Item `li`.
    pub item: Pt,
    /// Item content.
    pub item_content: Pt,
    /// Item link.
    pub item_link: Pt,
    /// Item icon.
    pub item_icon: Pt,
    /// Item label.
    pub item_label: Pt,
    /// Group chevron.
    pub submenu_icon: Pt,
    /// Separator.
    pub separator: Pt,
}

/// Context of the `item` view.
#[derive(Clone)]
pub struct PanelMenuItemContext {
    /// Item.
    pub item: MenuItem,
    /// Whether the item has subitems.
    pub has_submenu: bool,
    /// The item is a panel header.
    pub root: bool,
}

/// Expansion control: the analog of `collapseAll` and of the PrimeNG `controlled` demo. Created in advance and passed
/// to [`PanelMenu`] via the `panel_ref` prop.
#[derive(Clone, Copy)]
pub struct PanelMenuRef {
    expanded: RwSignal<HashSet<String>>,
    groups: StoredValue<Option<Memo<Vec<String>>>>,
}

impl Default for PanelMenuRef {
    fn default() -> Self {
        Self::new()
    }
}

impl PanelMenuRef {
    /// Creates the control object. Call inside a component (the signals' owner).
    pub fn new() -> Self {
        Self { expanded: RwSignal::new(HashSet::new()), groups: StoredValue::new(None) }
    }

    fn groups(&self) -> Vec<String> {
        self.groups.get_value().map(|m| m.get_untracked()).unwrap_or_default()
    }

    /// Expands all groups.
    pub fn expand_all(&self) {
        self.expanded.set(self.groups().into_iter().collect());
    }

    /// Collapses all panels and branches.
    pub fn collapse_all(&self) {
        self.expanded.set(HashSet::new());
    }

    /// Whether all groups are expanded (subscribing).
    pub fn all_expanded(&self) -> bool {
        let groups = self.groups.get_value().map(|m| m.get()).unwrap_or_default();
        self.expanded.with(|e| !groups.is_empty() && groups.iter().all(|g| e.contains(g)))
    }

    /// Expands all groups, and if all are already expanded — collapses.
    pub fn toggle_all(&self) {
        if self.expanded.with_untracked(|e| {
            let groups = self.groups();
            !groups.is_empty() && groups.iter().all(|g| e.contains(g))
        }) {
            self.collapse_all();
        } else {
            self.expand_all();
        }
    }

    /// Whether the item with this key is expanded (`"0"`, `"0_1"`, …; subscribing).
    pub fn is_expanded(&self, key: &str) -> bool {
        self.expanded.with(|e| e.contains(key))
    }
}

/// Focus state of a single panel tree.
#[derive(Clone, Copy)]
struct PanelFocus {
    focused: RwSignal<bool>,
    key: RwSignal<Option<String>>,
    search: StoredValue<String>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl PanelFocus {
    fn new() -> Self {
        Self {
            focused: RwSignal::new(false),
            key: RwSignal::new(None),
            search: StoredValue::new(String::new()),
            #[cfg(target_arch = "wasm32")]
            timer: StoredValue::new_local(None),
        }
    }

    fn reset(&self) {
        self.focused.try_set(false);
        self.key.try_set(None);
        self.search.try_set_value(String::new());
    }
}

/// State shared by the handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    model: Prop<Vec<MenuItem>>,
    nodes: Memo<Vec<Node>>,
    expanded: RwSignal<HashSet<String>>,
    multiple: Prop<bool>,
    root_el: NodeRef<leptos::html::Div>,
}

/// Root panel: the header and its identifiers.
struct Header {
    key: String,
    disabled: bool,
    header_id: String,
    list_id: String,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn tree<R>(&self, f: impl FnOnce(&[Node], &[MenuItem]) -> R) -> R {
        self.model.with(|items| self.nodes.with_untracked(|nodes| f(nodes, items)))
    }

    fn panel_id(&self, index: usize) -> String {
        format!("{}_{index}", self.id)
    }

    fn header_id(&self, item: &MenuItem, index: usize) -> String {
        match item.id.as_deref().filter(|i| !i.is_empty()) {
            Some(id) => format!("{id}_header"),
            None => format!("{}_header", self.panel_id(index)),
        }
    }

    fn content_id(&self, item: &MenuItem, index: usize) -> String {
        match item.id.as_deref().filter(|i| !i.is_empty()) {
            Some(id) => format!("{id}_content"),
            None => format!("{}_content", self.panel_id(index)),
        }
    }

    fn list_id(&self, item: &MenuItem, index: usize) -> String {
        match item.id.as_deref().filter(|i| !i.is_empty()) {
            Some(id) => format!("{id}_list"),
            None => format!("{}_list", self.panel_id(index)),
        }
    }

    /// Visible headers in display order.
    fn headers(&self) -> Vec<Header> {
        self.tree(|nodes, items| {
            nodes
                .iter()
                .filter(|n| n.level == 0)
                .filter_map(|n| {
                    let item = items.get(n.index)?;
                    item.visible.then(|| Header {
                        key: n.key.clone(),
                        disabled: item.disabled,
                        header_id: self.header_id(item, n.index),
                        list_id: self.list_id(item, n.index),
                    })
                })
                .collect()
        })
    }

    fn root_keys(&self) -> Vec<String> {
        self.tree(|nodes, _| nodes.iter().filter(|n| n.level == 0).map(|n| n.key.clone()).collect())
    }

    fn item_of(&self, key: &str) -> Option<MenuItem> {
        self.tree(|nodes, items| nodes.iter().find(|n| n.key == key).and_then(|n| item_at(items, &n.path)).cloned())
    }

    fn visible_keys(&self, root_key: &str) -> Vec<String> {
        let expanded = self.expanded.get_untracked();
        self.tree(|nodes, items| panel_visible(nodes, items, &expanded, root_key).into_iter().map(|n| n.key.clone()).collect())
    }

    /// Moves within the panel tree.
    fn with_visible<R>(&self, root_key: &str, f: impl FnOnce(&[&Node], &[MenuItem], &HashSet<String>) -> R) -> R {
        let expanded = self.expanded.get_untracked();
        self.tree(|nodes, items| f(&panel_visible(nodes, items, &expanded, root_key), items, &expanded))
    }

    /// Focus the element by `id` (browser only).
    fn focus_id(&self, id: &str) {
        #[cfg(target_arch = "wasm32")]
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = el.focus();
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = id;
    }

    /// Focus the header of the adjacent panel (`updateFocusedHeader`).
    fn focus_header_from(&self, root_key: &str, forward: bool) {
        let headers = self.headers();
        let disabled: Vec<bool> = headers.iter().map(|h| h.disabled).collect();
        if let Some(cur) = headers.iter().position(|h| h.key == root_key)
            && let Some(next) = header_step(&disabled, cur, forward)
        {
            self.focus_id(&headers[next].header_id);
        }
    }

    fn focus_own_header(&self, root_key: &str) {
        if let Some(h) = self.headers().iter().find(|h| h.key == root_key) {
            self.focus_id(&h.header_id);
        }
    }

    fn apply_move(&self, panel: PanelFocus, root_key: &str, mv: Move) {
        match mv {
            Move::Item(key) => {
                panel.key.set(Some(key));
                self.scroll_focused_into_view(panel);
            }
            Move::HeaderNext => self.focus_header_from(root_key, true),
            Move::HeaderSelf => self.focus_own_header(root_key),
            Move::Stay => {}
        }
    }

    fn scroll_focused_into_view(&self, panel: PanelFocus) {
        #[cfg(target_arch = "wasm32")]
        if let Some(id) = self.focused_li_id(panel)
            && let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id))
        {
            let init = web_sys::ScrollIntoViewOptions::new();
            init.set_block(web_sys::ScrollLogicalPosition::Nearest);
            init.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&init);
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = panel;
    }

    /// `id` of the item's `li` element under focus.
    fn focused_li_id(&self, panel: PanelFocus) -> Option<String> {
        let key = panel.key.get_untracked()?;
        self.tree(|nodes, items| {
            let node = nodes.iter().find(|n| n.key == key)?;
            Some(item_id(&self.id, item_at(items, &node.path)?, &key))
        })
    }

    /// Click on the panel header.
    fn header_click(&self, event: web_sys::MouseEvent, key: &str) {
        let Some(item) = self.item_of(key) else { return };
        if item.disabled {
            event.prevent_default();
            return;
        }
        if can_run_command(&item)
            && let Some(command) = item.command
        {
            command.run(MenuCommand { event: event.clone(), item: item.clone() });
        }
        let roots = self.root_keys();
        let multiple = self.multiple.get_untracked();
        self.expanded.update(|set| {
            toggle_root(set, key, multiple, &roots);
        });
        #[cfg(target_arch = "wasm32")]
        if let Some(el) = event.current_target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
            let _ = el.focus();
        }
    }

    /// Keys on the header.
    fn header_key_down(&self, e: &web_sys::KeyboardEvent, root_key: &str) {
        let headers = self.headers();
        let disabled: Vec<bool> = headers.iter().map(|h| h.disabled).collect();
        let Some(cur) = headers.iter().position(|h| h.key == root_key) else { return };
        let has_visible = |key: &str| self.expanded.with_untracked(|s| s.contains(key)) && !self.visible_keys(key).is_empty();
        match e.code().as_str() {
            "ArrowDown" => {
                if has_visible(root_key) {
                    self.focus_id(&headers[cur].list_id);
                } else if let Some(next) = header_step(&disabled, cur, true) {
                    self.focus_id(&headers[next].header_id);
                }
                e.prevent_default();
            }
            "ArrowUp" => {
                if let Some(prev) = header_step(&disabled, cur, false) {
                    let target = &headers[prev];
                    if has_visible(&target.key) { self.focus_id(&target.list_id) } else { self.focus_id(&target.header_id) }
                }
                e.prevent_default();
            }
            "Home" => {
                if let Some(i) = header_edge(&disabled, false) {
                    self.focus_id(&headers[i].header_id);
                }
                e.prevent_default();
            }
            "End" => {
                if let Some(i) = header_edge(&disabled, true) {
                    self.focus_id(&headers[i].header_id);
                }
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                #[cfg(target_arch = "wasm32")]
                if let Some(el) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    let link = el.query_selector(".p-panelmenu-header-link").ok().flatten().or(Some(el));
                    if let Some(el) = link.and_then(|l| l.dyn_into::<web_sys::HtmlElement>().ok()) {
                        el.click();
                    }
                }
                e.prevent_default();
            }
            _ => {}
        }
    }

    /// Click on the content of a tree item.
    fn item_click(&self, panel: PanelFocus, event: web_sys::MouseEvent, key: &str) {
        let Some(item) = self.item_of(key) else { return };
        if item.disabled {
            return;
        }
        if should_prevent_default(&item) {
            event.prevent_default();
        }
        if can_run_command(&item)
            && let Some(command) = item.command
        {
            command.run(MenuCommand { event, item: item.clone() });
        }
        if !item.items.is_empty() {
            self.expanded.update(|set| {
                toggle(set, key);
            });
        }
        panel.key.set(Some(key.to_owned()));
    }

    /// Focus entered the panel tree (`onFocus`). If it did not come from the tree itself, an item is selected: the clicked or the first one
    /// when focus came from the component (header), and the last one when from outside. In PrimeNG the selection is made once per
    /// entry into the component, so when returning from a header the previous item stays; here it is updated on every entry.
    fn list_focus(&self, panel: PanelFocus, root_key: &str, e: &web_sys::FocusEvent) {
        panel.focused.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            let Some(related) = e.related_target() else { return };
            let within = |node: Option<web_sys::Node>, related: &web_sys::EventTarget| {
                node.is_some_and(|n| related.dyn_ref::<web_sys::Node>().is_some_and(|r| n.contains(Some(r))))
            };
            let list = e.current_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            if within(list, &related) {
                return;
            }
            let inside = within(self.root_el.get_untracked().map(|r| r.unchecked_into()), &related);
            let keys = self.visible_keys(root_key);
            let valid = |k: &String| self.item_of(k).is_some_and(|i| i.visible && !i.disabled && !i.separator);
            let by_target = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .and_then(|t| t.closest("li").ok().flatten())
                .map(|li| li.id())
                .and_then(|id| keys.iter().find(|k| self.item_of(k).is_some_and(|i| item_id(&self.id, &i, k) == id)).cloned());
            let found = if inside {
                by_target.filter(valid).or_else(|| keys.iter().find(|k| valid(k)).cloned())
            } else {
                keys.iter().rev().find(|k| valid(k)).cloned()
            };
            panel.key.set(found);
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (e, root_key);
    }

    /// Focus left the tree (`onBlur`): if it left the whole component — reset.
    fn list_blur(&self, panel: PanelFocus, e: &web_sys::FocusEvent) {
        #[cfg(target_arch = "wasm32")]
        {
            let related = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let inside = related.is_some_and(|r| self.root_el.get_untracked().is_some_and(|root| root.contains(Some(&r))));
            if panel.focused.get_untracked() && !inside {
                panel.reset();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = e;
            panel.reset();
        }
    }

    /// `Enter`/`Space` on an item: a click on its link (or content).
    fn enter(&self, panel: PanelFocus) {
        #[cfg(target_arch = "wasm32")]
        if let Some(id) = self.focused_li_id(panel)
            && let Some(li) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id))
            && let Some(content) = li.query_selector(":scope > .p-panelmenu-item-content").ok().flatten()
        {
            let target = content.query_selector("a,button").ok().flatten().unwrap_or(content);
            if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                el.click();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = panel;
    }

    /// Printable character: search by label prefix; the search string is reset after 500 ms.
    fn type_char(&self, panel: PanelFocus, root_key: &str, ch: &str) {
        panel.search.update_value(|s| s.push_str(ch));
        let text = panel.search.get_value();
        let focus = panel.key.get_untracked();
        if let Some(found) = self.with_visible(root_key, |vis, items, _| search(vis, items, focus.as_deref(), &text)) {
            panel.key.set(Some(found));
            self.scroll_focused_into_view(panel);
        }
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(handle) = panel.timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
            let search = panel.search;
            if let Ok(handle) = set_timeout_with_handle(move || drop(search.try_set_value(String::new())), Duration::from_millis(500)) {
                panel.timer.set_value(Some(handle));
            }
        }
    }

    /// Keys in the panel tree (`onKeyDown`).
    fn list_key_down(&self, panel: PanelFocus, root_key: &str, e: &web_sys::KeyboardEvent) {
        let meta = e.meta_key() || e.ctrl_key();
        let focus = panel.key.get_untracked();
        match e.code().as_str() {
            "ArrowDown" => {
                let mv = self.with_visible(root_key, |vis, items, _| arrow_down(vis, items, focus.as_deref()));
                self.apply_move(panel, root_key, mv);
                e.prevent_default();
            }
            "ArrowUp" => {
                let mv = self.with_visible(root_key, |vis, items, _| arrow_up(vis, items, focus.as_deref()));
                self.apply_move(panel, root_key, mv);
                e.prevent_default();
            }
            "ArrowLeft" => {
                if focus.is_some() {
                    let left = self.tree(|nodes, _| arrow_left(nodes, &self.expanded.get_untracked(), root_key, focus.as_deref()));
                    match left {
                        Left::Collapse(key) => {
                            self.expanded.update(|s| {
                                s.remove(&key);
                            });
                        }
                        Left::Parent(key) => panel.key.set(Some(key)),
                        Left::Stay => {}
                    }
                    e.prevent_default();
                }
            }
            "ArrowRight" => {
                if focus.is_some() {
                    let right = self.tree(|nodes, items| arrow_right(nodes, items, &self.expanded.get_untracked(), focus.as_deref()));
                    match right {
                        Right::Expand(key) => self.expanded.update(|s| {
                            s.insert(key);
                        }),
                        Right::Down => {
                            let mv = self.with_visible(root_key, |vis, items, _| arrow_down(vis, items, focus.as_deref()));
                            self.apply_move(panel, root_key, mv);
                        }
                        Right::Stay => {}
                    }
                    e.prevent_default();
                }
            }
            "Home" => {
                let mv = self.with_visible(root_key, |vis, items, _| home(vis, items));
                self.apply_move(panel, root_key, mv);
                e.prevent_default();
            }
            "End" => {
                let mv = self.with_visible(root_key, |vis, items, _| end(vis, items));
                self.apply_move(panel, root_key, mv);
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.enter(panel);
                e.prevent_default();
            }
            "Escape" | "Tab" | "PageDown" | "PageUp" | "Backspace" | "ShiftLeft" | "ShiftRight" => {}
            _ => {
                if is_printable(&e.key(), meta) {
                    self.type_char(panel, root_key, &e.key());
                }
            }
        }
    }
}

/// Everything needed to render the levels.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<PanelMenuPt>,
    item: Option<Callback<PanelMenuItemContext, AnyView>>,
    submenu_icon: Option<ViewFn>,
    header_icon: Option<ViewFn>,
}

/// Animates the area on state change (browser only); the first call does nothing.
#[cfg(target_arch = "wasm32")]
fn bind_collapse(region: NodeRef<leptos::html::Div>, collapsed: impl Fn() -> bool + 'static) {
    Effect::new(move |prev: Option<bool>| {
        let now = collapsed();
        if let (Some(p), Some(el)) = (prev, region.get())
            && p != now
        {
            let el: web_sys::HtmlElement = (*el).clone();
            prime_rs_core::collapsible::animate(&el, !now, || {});
        }
        now
    });
}

fn join(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(e) => format!("{base} {e}"),
        None => base.to_owned(),
    }
}

/// A vertical accordion menu with nested branches.
///
/// Props accept a value or a signal.
#[component]
pub fn PanelMenu(
    /// Items.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Whether several root panels can be expanded at once.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// `id` attribute (the basis of part identifiers).
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Expansion control.
    #[prop(optional)]
    panel_ref: Option<PanelMenuRef>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<PanelMenuPt>,
    /// Custom item (and header) content instead of the link.
    #[prop(optional)]
    item: Option<Callback<PanelMenuItemContext, AnyView>>,
    /// Custom group chevron.
    #[prop(optional, into)]
    submenu_icon: Option<ViewFn>,
    /// Custom chevron of the group header.
    #[prop(optional, into)]
    header_icon: Option<ViewFn>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_rs_core::ids::next_unique()));
    let nodes = Memo::new({
        let model = model.clone();
        move |_| model.with(|items| build(items))
    });
    let expanded = panel_ref.map_or_else(|| RwSignal::new(HashSet::new()), |r| r.expanded);
    let seed = model.with(|items| nodes.with_untracked(|ns| seed_expanded(ns, items)));
    expanded.set(seed);
    if let Some(r) = panel_ref {
        let model = model.clone();
        r.groups.set_value(Some(Memo::new(move |_| model.with(|items| nodes.with(|ns| group_keys(ns, items))))));
    }
    let core = Arc::new(Core { id: menu_id.clone(), model: model.clone(), nodes, expanded, multiple, root_el: NodeRef::new() });
    let ctx = Ctx { core: core.clone(), pt: pt.clone(), item, submenu_icon, header_icon };

    let class = {
        let (pt, style_class) = (pt.clone(), style_class.clone());
        move || {
            let mut cn = prime_rs_core::ClassNames::new();
            cn.add("p-panelmenu p-component");
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
    let panels = {
        let ctx = ctx.clone();
        move || panels_view(&ctx)
    };
    let root_attrs = pt.root.attrs();
    view! {
        <div node_ref=core.root_el id={menu_id} class={class} style={root_style} {..root_attrs}>
            {panels}
        </div>
    }
}

/// Root item panels.
fn panels_view(ctx: &Ctx) -> AnyView {
    let rows: Vec<(Node, MenuItem)> = ctx.core.tree(|nodes, items| {
        nodes.iter().filter(|n| n.level == 0).filter_map(|n| items.get(n.index).map(|i| (n.clone(), i.clone()))).collect()
    });
    rows.iter().filter(|(_, item)| item.visible).map(|(node, item)| panel_view(ctx, node, item)).collect_view().into_any()
}

fn panel_view(ctx: &Ctx, node: &Node, item: &MenuItem) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let group = !item.items.is_empty();
    let header_id = core.header_id(item, node.index);
    let content_id = core.content_id(item, node.index);
    let list_id = core.list_id(item, node.index);
    let expanded = core.expanded;
    let active = {
        let key = key.clone();
        move || expanded.with(|s| s.contains(&key))
    };
    let panel = PanelFocus::new();

    let header_class = {
        let (pt, item, active) = (pt.clone(), item.clone(), active.clone());
        move || {
            let mut cn = prime_rs_core::ClassNames::new();
            cn.add("p-panelmenu-header").add_if(active() && group, "p-panelmenu-header-active").add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.header.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let header_content = match ctx.item {
        Some(template) => untrack(|| template.run(PanelMenuItemContext { item: item.clone(), has_submenu: group, root: true })),
        None => header_link_view(ctx, item, group, active.clone()),
    };
    let container_class = {
        let (pt, active) = (pt.clone(), active.clone());
        move || {
            let mut cn = prime_rs_core::ClassNames::new();
            cn.add("p-panelmenu-content-container").add_if(active(), "p-panelmenu-expanded");
            pt.content_container.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let region: NodeRef<leptos::html::Div> = NodeRef::new();
    #[cfg(target_arch = "wasm32")]
    bind_collapse(region, {
        let active = active.clone();
        move || !active()
    });
    let initial_style = pt.content_container.merge_style(hidden_style(!expanded.with_untracked(|s| s.contains(&key))).unwrap_or(""));

    let tree = tree_list_view(ctx, panel, &key, list_id, active.clone());
    let (click_core, key_core, click_key, key_key) = (core.clone(), core.clone(), key.clone(), key.clone());
    let disabled = item.disabled;
    let label = item.label.clone();
    let header_content_class = pt.header_content.merge_class("p-panelmenu-header-content").into_owned();
    let panel_class = join(&pt.panel.merge_class("p-panelmenu-panel"), None);
    let (a1, a2, a3) = (active.clone(), active.clone(), active.clone());
    let (panel_attrs, header_attrs, hc_attrs, cc_attrs, cw_attrs, c_attrs) = (
        pt.panel.attrs(),
        pt.header.attrs(),
        pt.header_content.attrs(),
        pt.content_container.attrs(),
        pt.content_wrapper.attrs(),
        pt.content.attrs(),
    );
    let wrapper_class = pt.content_wrapper.merge_class("p-panelmenu-content-wrapper").into_owned();
    let content_class = pt.content.merge_class("p-panelmenu-content").into_owned();
    view! {
        <div class={panel_class} style={item.style.clone()} {..panel_attrs}>
            <div
                id={header_id.clone()}
                class={header_class}
                style={item.style.clone()}
                role="button"
                tabindex="0"
                aria-expanded={move || a1().to_string()}
                aria-label={label}
                aria-controls={content_id.clone()}
                aria-disabled={disabled.to_string()}
                data-p-highlight={move || a2().to_string()}
                data-p-disabled={disabled.to_string()}
                on:click={move |e| click_core.header_click(e, &click_key)}
                on:keydown={move |e| key_core.header_key_down(&e, &key_key)}
                {..header_attrs}
            >
                <div class={header_content_class} {..hc_attrs}>{header_content}</div>
            </div>
            <div
                node_ref=region
                id={content_id}
                class={container_class}
                style={(!initial_style.is_empty()).then_some(initial_style)}
                role="region"
                aria-labelledby={header_id}
                data-p-expanded={move || a3().to_string()}
                {..cc_attrs}
            >
                <div class={wrapper_class} {..cw_attrs}>
                    <div class={content_class} {..c_attrs}>{tree}</div>
                </div>
            </div>
        </div>
    }
    .into_any()
}

/// Header link: chevron, icon, label.
fn header_link_view(ctx: &Ctx, item: &MenuItem, group: bool, active: impl Fn() -> bool + Clone + Send + Sync + 'static) -> AnyView {
    let pt = &ctx.pt;
    let arrow = group.then(|| match ctx.header_icon.clone() {
        Some(custom) => custom.run(),
        None => chevron(pt.submenu_icon.merge_class("p-panelmenu-submenu-icon").into_owned(), active),
    });
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-panelmenu-header-icon", item.icon_class.as_deref());
        view! { <span class={pt.header_icon.merge_class(&class).into_owned()} {..pt.header_icon.attrs()}>{icon.run()}</span> }
    });
    let link_class = join("p-panelmenu-header-link", item.link_class.as_deref());
    view! {
        <a
            class={pt.header_link.merge_class(&link_class).into_owned()}
            href={item.url.clone()}
            target={item.target.clone()}
            title={item.title.clone()}
            tabindex="-1"
            {..pt.header_link.attrs()}
        >
            {arrow}
            {icon}
            <span class={pt.header_label.merge_class("p-panelmenu-header-label").into_owned()} {..pt.header_label.attrs()}>{item.label.clone()}</span>
        </a>
    }
    .into_any()
}

/// Chevron: down for an expanded group, right for a collapsed one.
fn chevron(class: String, active: impl Fn() -> bool + Send + Sync + 'static) -> AnyView {
    let down_class = class.clone();
    view! {
        {move || if active() { view! { <ChevronDownIcon class={down_class.clone()} /> }.into_any() } else { view! { <ChevronRightIcon class={class.clone()} /> }.into_any() }}
    }
    .into_any()
}

/// Root list of the panel.
fn tree_list_view(
    ctx: &Ctx,
    panel: PanelFocus,
    root_key: &str,
    list_id: String,
    active: impl Fn() -> bool + Clone + Send + Sync + 'static,
) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let class = pt.root_list.merge_class("p-panelmenu-root-list p-panelmenu-submenu").into_owned();
    let (focus_core, blur_core, key_core) = (core.clone(), core.clone(), core.clone());
    let (focus_key, key_key) = (root_key.to_owned(), root_key.to_owned());
    let activedescendant = {
        let core = core.clone();
        move || {
            if !panel.focused.get() {
                return None;
            }
            panel.key.track();
            core.focused_li_id(panel)
        }
    };
    let levels = {
        let (ctx, root_key) = (ctx.clone(), root_key.to_owned());
        move || items_view(&ctx, panel, &root_key, &root_key)
    };
    let attrs = pt.root_list.attrs();
    view! {
        <ul
            id={list_id}
            class={class}
            role="tree"
            tabindex="-1"
            aria-hidden={move || (!active()).to_string()}
            aria-activedescendant={activedescendant}
            on:focusin={move |e| focus_core.list_focus(panel, &focus_key, &e)}
            on:focusout={move |e| blur_core.list_blur(panel, &e)}
            on:keydown={move |e| key_core.list_key_down(panel, &key_key, &e)}
            {..attrs}
        >
            {levels}
        </ul>
    }
    .into_any()
}

/// Items of one level inside panel `root_key`.
fn items_view(ctx: &Ctx, panel: PanelFocus, root_key: &str, parent_key: &str) -> AnyView {
    let core = &ctx.core;
    let rows: Vec<(Node, MenuItem)> = core.tree(|nodes, items| {
        nodes
            .iter()
            .filter(|n| n.parent_key == parent_key)
            .filter_map(|n| item_at(items, &n.path).map(|i| (n.clone(), i.clone())))
            .collect()
    });
    let refs: Vec<&MenuItem> = rows.iter().map(|(_, i)| i).collect();
    rows.iter()
        .enumerate()
        .filter(|(_, (_, item))| item.visible)
        .map(|(index, (node, item))| {
            if item.separator {
                separator_view(ctx, node, item)
            } else {
                item_view(ctx, panel, root_key, node, item, aria_set_size(&refs), aria_pos_inset(&refs, index))
            }
        })
        .collect_view()
        .into_any()
}

fn separator_view(ctx: &Ctx, node: &Node, item: &MenuItem) -> AnyView {
    let class = ctx.pt.separator.merge_class(&join("p-menuitem-separator", item.style_class.as_deref())).into_owned();
    let id = item_id(&ctx.core.id, item, &node.key);
    view! { <li id={id} class={class} style={item.style.clone()} role="separator" {..ctx.pt.separator.attrs()}></li> }.into_any()
}

fn item_view(ctx: &Ctx, panel: PanelFocus, root_key: &str, node: &Node, item: &MenuItem, set_size: usize, pos_inset: usize) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let group = !item.items.is_empty();
    let id = item_id(&core.id, item, &key);
    let expanded = core.expanded;
    let active = {
        let key = key.clone();
        move || expanded.with(|s| s.contains(&key))
    };
    let focused = move |key: &str| panel.focused.get() && panel.key.with(|k| k.as_deref() == Some(key));
    let class = {
        let (pt, item, key) = (pt.clone(), item.clone(), key.clone());
        move || {
            let mut cn = prime_rs_core::ClassNames::new();
            cn.add("p-panelmenu-item").add_if(focused(&key) && !item.disabled, "p-focus").add_if(item.disabled, "p-disabled");
            cn.add_user(item.style_class.as_deref());
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let tab_key = key.clone();
    let tabindex = move || if expanded.with(|s| ancestors_expanded(s, &tab_key)) { "0" } else { "-1" };
    let content = match ctx.item {
        Some(template) => untrack(|| template.run(PanelMenuItemContext { item: item.clone(), has_submenu: group, root: false })),
        None => link_view(ctx, item, group, active.clone(), tabindex),
    };
    let click_core = core.clone();
    let click_key = key.clone();
    let submenu = group.then(|| {
        let region: NodeRef<leptos::html::Div> = NodeRef::new();
        #[cfg(target_arch = "wasm32")]
        bind_collapse(region, {
            let active = active.clone();
            move || !active()
        });
        let initial_style = pt.content_container.merge_style(hidden_style(!expanded.with_untracked(|s| s.contains(&key))).unwrap_or(""));
        let container_class = {
            let (pt, active) = (pt.clone(), active.clone());
            move || {
                let mut cn = prime_rs_core::ClassNames::new();
                cn.add("p-panelmenu-content-container").add_if(active(), "p-panelmenu-expanded");
                pt.content_container.merge_class(&cn.finish_cow()).into_owned()
            }
        };
        let hidden = active.clone();
        let sub_id = format!("{id}_list");
        let children = items_view(ctx, panel, root_key, &key);
        let (wrapper_class, wrapper_attrs) = (pt.content_wrapper.merge_class("p-panelmenu-content-wrapper").into_owned(), pt.content_wrapper.attrs());
        let (class, attrs) = (pt.submenu.merge_class("p-panelmenu-submenu").into_owned(), pt.submenu.attrs());
        view! {
            <div node_ref=region class={container_class} style={(!initial_style.is_empty()).then_some(initial_style)} {..pt.content_container.attrs()}>
                <div class={wrapper_class} {..wrapper_attrs}>
                    <ul id={sub_id} class={class} role="tree" tabindex="-1" aria-hidden={move || (!hidden()).to_string()} {..attrs}>
                        {children}
                    </ul>
                </div>
            </div>
        }
    });
    let ae = {
        let active = active.clone();
        move || group.then(|| active().to_string())
    };
    let fo = {
        let key = key.clone();
        move || focused(&key).to_string()
    };
    let label = item.label.clone();
    let disabled = item.disabled;
    let level = node.level;
    let content_class = pt.item_content.merge_class("p-panelmenu-item-content").into_owned();
    let mut item_attrs = pt.item.attrs();
    item_attrs.push(leptos::attr::custom::custom_attribute("aria-level", level.to_string()).into_any_attr());
    let content_attrs = pt.item_content.attrs();
    view! {
        <li
            id={id}
            class={class}
            style={item.style.clone()}
            role="treeitem"
            aria-label={label}
            aria-expanded={ae}
            aria-setsize={set_size.to_string()}
            aria-posinset={pos_inset.to_string()}
            data-p-disabled={disabled.to_string()}
            data-p-focused={fo}
            {..item_attrs}
        >
            <div class={content_class} on:click={move |e| click_core.item_click(panel, e, &click_key)} {..content_attrs}>
                {content}
            </div>
            {submenu}
        </li>
    }
    .into_any()
}

/// Default item content: a link with the group chevron, icon and label.
fn link_view(
    ctx: &Ctx,
    item: &MenuItem,
    group: bool,
    active: impl Fn() -> bool + Clone + Send + Sync + 'static,
    tabindex: impl Fn() -> &'static str + Send + Sync + 'static,
) -> AnyView {
    let pt = &ctx.pt;
    let arrow = group.then(|| match ctx.submenu_icon.clone() {
        Some(custom) => custom.run(),
        None => chevron(pt.submenu_icon.merge_class("p-panelmenu-submenu-icon").into_owned(), active),
    });
    let icon = item.icon.clone().map(|icon| {
        let class = join("p-panelmenu-item-icon", item.icon_class.as_deref());
        view! { <span class={pt.item_icon.merge_class(&class).into_owned()} {..pt.item_icon.attrs()}>{icon.run()}</span> }
    });
    let link_class = join("p-panelmenu-item-link", item.link_class.as_deref());
    view! {
        <a
            class={pt.item_link.merge_class(&link_class).into_owned()}
            href={item.url.clone()}
            target={item.target.clone()}
            title={item.title.clone()}
            tabindex={tabindex}
            {..pt.item_link.attrs()}
        >
            {arrow}
            {icon}
            <span class={pt.item_label.merge_class("p-panelmenu-item-label").into_owned()} {..pt.item_label.attrs()}>{item.label.clone()}</span>
        </a>
    }
    .into_any()
}
