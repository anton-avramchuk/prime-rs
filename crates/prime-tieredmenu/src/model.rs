//! Pure TieredMenu model: the node tree, the path of active items, focus and transitions (after PrimeNG's `tieredmenu.ts`).
//!
//! Items are `prime_menu::MenuItem` (the `items` field defines the submenu). A tree node is a [`Node`] with a key like `"0"`, `"0_1"`,
//! `"0_1_2"`; the active item path stores the keys of expanded groups from the root down.

use prime_menu::MenuItem;

/// A node of the "processed items" tree (`createProcessedItems`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Key: ancestor indices and its own, joined by `_`.
    pub key: String,
    /// Parent key (empty string for root nodes).
    pub parent_key: String,
    /// Depth (root nodes are `0`).
    pub level: usize,
    /// Index among siblings.
    pub index: usize,
    /// Indices from the root to the item.
    pub path: Vec<usize>,
}

/// Builds nodes in depth-first order.
pub fn build(items: &[MenuItem]) -> Vec<Node> {
    fn walk(items: &[MenuItem], level: usize, parent_key: &str, parent_path: &[usize], out: &mut Vec<Node>) {
        for (index, item) in items.iter().enumerate() {
            let key = if parent_key.is_empty() { index.to_string() } else { format!("{parent_key}_{index}") };
            let mut path = parent_path.to_vec();
            path.push(index);
            out.push(Node { key: key.clone(), parent_key: parent_key.to_owned(), level, index, path: path.clone() });
            walk(&item.items, level + 1, &key, &path, out);
        }
    }
    let mut out = Vec::new();
    walk(items, 0, "", &[], &mut out);
    out
}

/// Item by index path.
pub fn item_at<'a>(items: &'a [MenuItem], path: &[usize]) -> Option<&'a MenuItem> {
    let (first, rest) = path.split_first()?;
    let mut item = items.get(*first)?;
    for i in rest {
        item = item.items.get(*i)?;
    }
    Some(item)
}

/// Valid item: visible, not disabled and not a separator (`isValidItem`).
pub fn is_valid(item: &MenuItem) -> bool {
    item.visible && !item.disabled && !item.separator
}

/// Identifier of the `li` element: the item's own `id` or `<menu id>_<key>`.
pub fn item_id(menu_id: &str, item: &MenuItem, key: &str) -> String {
    match item.id.as_deref() {
        Some(id) if !id.is_empty() => id.to_owned(),
        _ => format!("{menu_id}_{key}"),
    }
}

/// Whether `ancestor` is an ancestor of `key` (or `key` itself): compared by key boundaries, not as a string (`"1"` is not an ancestor of `"10"`).
pub fn is_ancestor_or_self(ancestor: &str, key: &str) -> bool {
    key == ancestor || key.strip_prefix(ancestor).is_some_and(|rest| rest.starts_with('_'))
}

/// Focus: the index among the visible items of the current level and the parent key of that level.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Focus {
    /// Item index within the level.
    pub index: Option<usize>,
    /// Parent key of the level (empty means root).
    pub parent_key: String,
}

/// Result of a click on an item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Click {
    /// The branch is collapsed, focus stays on the item.
    Collapsed,
    /// The group is expanded.
    Expanded,
    /// Leaf: everything is closed; focus moves to the root item of the branch (`Some(index)`).
    Leaf(Option<usize>),
}

/// Tree state: the path of active groups, focus and the "expanded by click" flag.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Keys of expanded groups from the root down.
    pub active: Vec<String>,
    /// Focus.
    pub focus: Focus,
    /// A group was expanded by click or arrow (enables expansion on hover).
    pub dirty: bool,
}

/// Tree with items: everything the transitions need.
pub struct Tree<'a> {
    /// Nodes.
    pub nodes: &'a [Node],
    /// Root items of the model.
    pub items: &'a [MenuItem],
}

impl<'a> Tree<'a> {
    /// Node by key.
    pub fn node(&self, key: &str) -> Option<&'a Node> {
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Item of the node.
    pub fn item(&self, node: &Node) -> &'a MenuItem {
        item_at(self.items, &node.path).expect("the node points to an existing item")
    }

    /// Whether the node is a group (has sub-items).
    pub fn is_group(&self, node: &Node) -> bool {
        !self.item(node).items.is_empty()
    }

    /// Siblings sharing a parent.
    pub fn siblings(&self, parent_key: &str) -> Vec<&'a Node> {
        self.nodes.iter().filter(|n| n.parent_key == parent_key).collect()
    }

    /// Items of the current focus level (`visibleItems`): children of the active group with the focus key, otherwise the root ones.
    pub fn visible(&self, state: &State) -> Vec<&'a Node> {
        let parent = state.active.iter().find(|k| **k == state.focus.parent_key);
        self.siblings(parent.map_or("", String::as_str))
    }

    fn valid(&self, node: &Node) -> bool {
        is_valid(self.item(node))
    }

    fn selected(&self, state: &State, node: &Node) -> bool {
        state.active.contains(&node.key)
    }

    /// First valid index (`findFirstItemIndex`).
    pub fn first(&self, visible: &[&Node]) -> Option<usize> {
        visible.iter().position(|n| self.valid(n))
    }

    /// Last valid index (`findLastItemIndex`).
    pub fn last(&self, visible: &[&Node]) -> Option<usize> {
        visible.iter().rposition(|n| self.valid(n))
    }

    /// Next valid index; without one, `index` is kept (`findNextItemIndex`).
    pub fn next(&self, visible: &[&Node], index: usize) -> usize {
        visible.iter().enumerate().skip(index + 1).find(|(_, n)| self.valid(n)).map_or(index, |(i, _)| i)
    }

    /// Previous valid index; without one, `index` is kept (`findPrevItemIndex`).
    pub fn prev(&self, visible: &[&Node], index: usize) -> usize {
        visible.iter().enumerate().take(index).rfind(|(_, n)| self.valid(n)).map_or(index, |(i, _)| i)
    }

    /// Index of the expanded valid item of the level (`findSelectedItemIndex`).
    pub fn selected_index(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        visible.iter().position(|n| self.valid(n) && self.selected(state, n))
    }

    /// First item for focus: the expanded one, otherwise the first valid one (`findFirstFocusedItemIndex`).
    pub fn first_focused(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        self.selected_index(state, visible).or_else(|| self.first(visible))
    }

    /// Last item for focus: the expanded one, otherwise the last valid one (`findLastFocusedItemIndex`).
    pub fn last_focused(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        self.selected_index(state, visible).or_else(|| self.last(visible))
    }

    fn matches(&self, node: &Node, search: &str) -> bool {
        let item = self.item(node);
        is_valid(item) && item.label.as_deref().unwrap_or_default().to_lowercase().starts_with(&search.to_lowercase())
    }

    /// Search by label prefix among the level's items (`searchItems`): from the current item, wrapping around. Returns the index and whether
    /// an item was found.
    pub fn search(&self, state: &State, visible: &[&Node], search: &str) -> (Option<usize>, bool) {
        let mut found = match state.focus.index {
            Some(current) => {
                let after = visible.iter().enumerate().skip(current).find(|(_, n)| self.matches(n, search)).map(|(i, _)| i);
                after.or_else(|| visible.iter().enumerate().take(current).find(|(_, n)| self.matches(n, search)).map(|(i, _)| i))
            }
            None => visible.iter().position(|n| self.matches(n, search)),
        };
        let matched = found.is_some();
        if found.is_none() && state.focus.index.is_none() {
            found = self.first_focused(state, visible);
        }
        (found, matched)
    }

    /// Sets focus to a level index (`changeFocusedItemIndex`).
    pub fn focus_index(&self, state: &mut State, index: Option<usize>) {
        if index.is_some() && state.focus.index != index {
            state.focus.index = index;
        }
    }

    /// Closes everything: the path, focus and flag (`hide`).
    pub fn hide(&self, state: &mut State) {
        state.active.clear();
        state.focus = Focus::default();
        state.dirty = false;
    }

    /// Path change when an item is expanded (`onItemChange`); on hover in a narrow window the path does not change.
    pub fn item_change(&self, state: &mut State, node: &Node, hover: bool, mobile: bool) {
        let group = self.is_group(node);
        let mut path: Vec<String> = state
            .active
            .iter()
            .filter(|k| self.node(k).is_some_and(|p| p.parent_key != node.parent_key && p.parent_key != node.key))
            .cloned()
            .collect();
        if group {
            path.push(node.key.clone());
        }
        state.focus = Focus { index: Some(node.index), parent_key: node.parent_key.clone() };
        if group {
            state.dirty = true;
        }
        if hover && mobile {
            return;
        }
        state.active = path;
    }

    /// Click on an item (`onItemClick`).
    pub fn click(&self, state: &mut State, node: &Node) -> Click {
        if state.active.contains(&node.key) {
            state.active.retain(|k| *k != node.key && is_ancestor_or_self(k, &node.key));
            state.focus = Focus { index: Some(node.index), parent_key: node.parent_key.clone() };
            state.dirty = true;
            Click::Collapsed
        } else if self.is_group(node) {
            self.item_change(state, node, false, false);
            Click::Expanded
        } else {
            let root = if node.level == 0 {
                Some(node.index)
            } else {
                state.active.iter().filter_map(|k| self.node(k)).find(|p| p.parent_key.is_empty()).map(|p| p.index)
            };
            self.hide(state);
            self.focus_index(state, root);
            Click::Leaf(root)
        }
    }

    /// Hover over an item: changes the path only if it was already expanded by click (`onItemMouseEnter`); on touch, always.
    pub fn hover(&self, state: &mut State, node: &Node, touch: bool, mobile: bool) {
        if touch || state.dirty {
            self.item_change(state, node, true, mobile);
        }
    }

    /// ↓: next valid item (the first or expanded one when focus is empty).
    pub fn arrow_down(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = match state.focus.index {
            Some(i) => Some(self.next(&visible, i)),
            None => self.first_focused(state, &visible),
        };
        self.focus_index(state, target);
    }

    /// ↑: previous valid item (the last or expanded one when focus is empty).
    pub fn arrow_up(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = match state.focus.index {
            Some(i) => Some(self.prev(&visible, i)),
            None => self.last_focused(state, &visible),
        };
        self.focus_index(state, target);
    }

    /// →: expands the focused group and moves inside.
    pub fn arrow_right(&self, state: &mut State) {
        let visible = self.visible(state);
        let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied()) else { return };
        if self.is_group(node) {
            self.item_change(state, node, false, false);
            state.focus = Focus { index: None, parent_key: node.key.clone() };
            self.arrow_down(state);
        }
    }

    /// ←: moves up to the parent and collapses its submenu.
    pub fn arrow_left(&self, state: &mut State) {
        let visible = self.visible(state);
        let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied()) else { return };
        if node.level > 0 {
            let parent_parent = state
                .active
                .iter()
                .find(|k| **k == node.parent_key)
                .and_then(|k| self.node(k))
                .map_or_else(String::new, |p| p.parent_key.clone());
            state.focus = Focus { index: None, parent_key: parent_parent };
            self.arrow_down(state);
        }
        let parent_key = state.focus.parent_key.clone();
        state.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != parent_key));
    }

    /// The focused node and the "it is in a submenu" flag (its parent is in the expanded path).
    fn focused_node(&self, state: &State) -> Option<(&'a Node, bool)> {
        let visible = self.visible(state);
        let node = state.focus.index.and_then(|i| visible.get(i).copied())?;
        let in_submenu = !node.parent_key.is_empty() && state.active.contains(&node.parent_key);
        Some((node, in_submenu))
    }

    /// Expands the focused group and sets focus "inside" with an empty index (the shared part of the menubar arrows).
    fn enter_group(&self, state: &mut State, node: &Node) {
        self.item_change(state, node, false, false);
        state.focus = Focus { index: None, parent_key: node.key.clone() };
    }

    /// Menubar, ↓: on a root group expands it and enters; on a root leaf nothing; in a submenu, the next item.
    pub fn bar_down(&self, state: &mut State) {
        match self.focused_node(state) {
            Some((node, _)) if node.level == 0 => {
                if self.is_group(node) {
                    self.enter_group(state, node);
                    self.arrow_down(state);
                }
            }
            _ => self.arrow_down(state),
        }
    }

    /// Menubar, →: in a submenu on a group expands it and enters, on a leaf nothing; on the root, the next item.
    pub fn bar_right(&self, state: &mut State) {
        match self.focused_node(state) {
            Some((node, true)) => {
                if self.is_group(node) {
                    self.enter_group(state, node);
                    self.arrow_down(state);
                }
            }
            _ => self.arrow_down(state),
        }
    }

    /// Menubar, ←: in a submenu moves up to the parent and collapses it; on the root, the previous item.
    pub fn bar_left(&self, state: &mut State) {
        match self.focused_node(state) {
            Some((node, true)) => {
                if let Some(parent) = self.node(&node.parent_key) {
                    self.item_change(state, parent, false, false);
                    let level = state.focus.parent_key.clone();
                    state.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != level));
                }
            }
            _ => self.arrow_up(state),
        }
    }

    /// Menubar, ↑: on a root group expands it and focuses the last item; on the first submenu item moves up
    /// to the parent and collapses it; otherwise the previous item.
    pub fn bar_up(&self, state: &mut State) {
        match self.focused_node(state) {
            Some((node, _)) if node.level == 0 => {
                if self.is_group(node) {
                    self.enter_group(state, node);
                    let visible = self.visible(state);
                    let last = self.last(&visible);
                    self.focus_index(state, last);
                }
            }
            Some((node, _)) if state.focus.index == Some(0) => {
                let grand = self.node(&node.parent_key).map_or_else(String::new, |p| p.parent_key.clone());
                state.focus = Focus { index: None, parent_key: grand };
                self.arrow_up(state);
                let level = state.focus.parent_key.clone();
                state.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != level));
            }
            _ => self.arrow_up(state),
        }
    }

    /// `Home`.
    pub fn home(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = self.first(&visible);
        self.focus_index(state, target);
    }

    /// `End`.
    pub fn end(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = self.last(&visible);
        self.focus_index(state, target);
    }

    /// `Escape`: closes everything, focus goes to the first valid root item.
    pub fn escape(&self, state: &mut State) {
        self.hide(state);
        let visible = self.visible(state);
        state.focus.index = self.first_focused(state, &visible);
    }

    /// `Tab`: the focused leaf is activated (changes focus), then everything closes.
    pub fn tab(&self, state: &mut State, mobile: bool) {
        let visible = self.visible(state);
        if let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied())
            && !self.is_group(node)
        {
            self.item_change(state, node, false, mobile);
        }
        self.hide(state);
    }

    /// After `Enter` on a leaf outside popup, focus returns to the first item (`onEnterKey`).
    pub fn after_enter(&self, state: &mut State) {
        let visible = self.visible(state);
        if let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied())
            && !self.is_group(node)
        {
            state.focus.index = self.first_focused(state, &visible);
        }
    }

    /// Identifier of the focused item (`focusedItemId`).
    pub fn focused_id(&self, state: &State, menu_id: &str) -> Option<String> {
        let index = state.focus.index?;
        let node = self.siblings(&state.focus.parent_key).get(index).copied()?;
        Some(item_id(menu_id, self.item(node), &node.key))
    }
}

/// `aria-setsize`: the number of visible items without separators.
pub fn aria_set_size(siblings: &[&MenuItem]) -> usize {
    siblings.iter().filter(|i| i.visible && !i.separator).count()
}

/// `aria-posinset`: the item number among visible items without separators (one-based).
pub fn aria_pos_inset(siblings: &[&MenuItem], index: usize) -> usize {
    let skipped = siblings.iter().take(index).filter(|i| !i.visible || (i.visible && i.separator)).count();
    index - skipped + 1
}

/// Submenu side (`nestedPosition`): `"100%"` is to the right of the item, `"-100%"` to the left.
pub fn submenu_side(parent_left: f64, item_width: f64, sub_width: f64, viewport_width: f64, scrollbar: f64, level: usize) -> &'static str {
    let left = parent_left.trunc();
    if left + item_width + sub_width > viewport_width - scrollbar {
        if left < sub_width {
            // window too narrow
            if level % 2 == 1 { "100%" } else { "-100%" }
        } else {
            "-100%"
        }
    } else {
        "100%"
    }
}

/// Printable character for search: a single character without Ctrl/Meta.
pub fn is_printable(key: &str, ctrl_or_meta: bool) -> bool {
    !ctrl_or_meta && key.chars().count() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(label: &str) -> MenuItem {
        MenuItem::new().label(label)
    }

    /// File{ New{Share,Folder}, Open, ─, Print(off) }, Edit{Undo,Redo}, Users, ─, Quit
    fn menu() -> Vec<MenuItem> {
        vec![
            leaf("File").items(vec![
                leaf("New").items(vec![leaf("Share"), leaf("Folder")]),
                leaf("Open"),
                MenuItem::separator(),
                leaf("Print").disabled(true),
            ]),
            leaf("Edit").items(vec![leaf("Undo"), leaf("Redo")]),
            leaf("Users"),
            MenuItem::separator(),
            leaf("Quit"),
        ]
    }

    fn keys(nodes: &[&Node]) -> Vec<String> {
        nodes.iter().map(|n| n.key.clone()).collect()
    }

    #[test]
    fn build_assigns_keys_levels_and_parents() {
        let items = menu();
        let nodes = build(&items);
        let table: Vec<(&str, &str, usize, usize)> =
            nodes.iter().map(|n| (n.key.as_str(), n.parent_key.as_str(), n.level, n.index)).collect();
        assert_eq!(
            table,
            [
                ("0", "", 0, 0),
                ("0_0", "0", 1, 0),
                ("0_0_0", "0_0", 2, 0),
                ("0_0_1", "0_0", 2, 1),
                ("0_1", "0", 1, 1),
                ("0_2", "0", 1, 2),
                ("0_3", "0", 1, 3),
                ("1", "", 0, 1),
                ("1_0", "1", 1, 0),
                ("1_1", "1", 1, 1),
                ("2", "", 0, 2),
                ("3", "", 0, 3),
                ("4", "", 0, 4),
            ]
        );
        let tree = Tree { nodes: &nodes, items: &items };
        assert_eq!(tree.item(tree.node("0_0_1").unwrap()).label.as_deref(), Some("Folder"));
        assert!(tree.is_group(tree.node("0_0").unwrap()) && !tree.is_group(tree.node("2").unwrap()));
        assert_eq!(keys(&tree.siblings("0_0")), ["0_0_0", "0_0_1"]);
        assert!(item_at(&items, &[9]).is_none());
        assert_eq!(item_id("m", &items[0], "0"), "m_0");
        assert_eq!(item_id("m", &leaf("x").id("own"), "0"), "own");
    }

    #[test]
    fn valid_items_and_ancestor_check() {
        assert!(is_valid(&leaf("a")));
        assert!(!is_valid(&leaf("a").disabled(true)));
        assert!(!is_valid(&leaf("a").visible(false)));
        assert!(!is_valid(&MenuItem::separator()));
        assert!(is_ancestor_or_self("0", "0_1_2"));
        assert!(is_ancestor_or_self("0_1", "0_1"));
        assert!(!is_ancestor_or_self("1", "10"), "by key boundary, not as a string");
        assert!(!is_ancestor_or_self("0_1", "0_10"));
        assert!(!is_ancestor_or_self("0_1", "0"));
    }

    #[test]
    fn next_prev_first_last_skip_invalid() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let root = tree.siblings("");
        assert_eq!(tree.first(&root), Some(0));
        assert_eq!(tree.last(&root), Some(4));
        assert_eq!(tree.next(&root, 2), 4, "separator skipped");
        assert_eq!(tree.next(&root, 4), 4, "stays at the boundary");
        assert_eq!(tree.prev(&root, 4), 2);
        assert_eq!(tree.prev(&root, 0), 0);
        let file = tree.siblings("0");
        assert_eq!(keys(&file), ["0_0", "0_1", "0_2", "0_3"]);
        assert_eq!(tree.last(&file), Some(1), "Print is disabled, separator skipped");
        assert_eq!(tree.next(&file, 1), 1);
    }

    #[test]
    fn visible_follows_the_focus_parent_through_the_active_path() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        assert_eq!(tree.visible(&st).len(), 5);
        st.focus.parent_key = "0".into();
        assert_eq!(tree.visible(&st).len(), 5, "parent not expanded: root level");
        st.active = vec!["0".into()];
        assert_eq!(keys(&tree.visible(&st)), ["0_0", "0_1", "0_2", "0_3"]);
    }

    #[test]
    fn click_expands_collapses_and_closes_on_leaf() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        let file = tree.node("0").unwrap();
        assert_eq!(tree.click(&mut st, file), Click::Expanded);
        assert_eq!(st.active, ["0"]);
        assert!(st.dirty);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() });
        let new = tree.node("0_0").unwrap();
        assert_eq!(tree.click(&mut st, new), Click::Expanded);
        assert_eq!(st.active, ["0", "0_0"]);
        // a repeated click on File collapses the branch down to it
        assert_eq!(tree.click(&mut st, file), Click::Collapsed);
        assert_eq!(st.active, Vec::<String>::new(), "the item itself is not in the path, and there are no ancestors");
        // expand again and click New twice: New collapses, File stays
        tree.click(&mut st, file);
        tree.click(&mut st, new);
        assert_eq!(tree.click(&mut st, new), Click::Collapsed);
        assert_eq!(st.active, ["0"]);
        // a deeply nested leaf closes everything and moves focus to the root item of the branch
        tree.click(&mut st, new);
        let share = tree.node("0_0_0").unwrap();
        assert_eq!(tree.click(&mut st, share), Click::Leaf(Some(0)));
        assert_eq!((st.active.clone(), st.dirty), (Vec::<String>::new(), false));
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() });
        // root leaf
        let quit = tree.node("4").unwrap();
        assert_eq!(tree.click(&mut st, quit), Click::Leaf(Some(4)));
    }

    #[test]
    fn click_on_a_sibling_group_replaces_the_branch() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        tree.click(&mut st, tree.node("0").unwrap());
        tree.click(&mut st, tree.node("1").unwrap());
        assert_eq!(st.active, ["1"], "File closed, Edit open");
    }

    #[test]
    fn hover_needs_dirty_unless_touch_and_ignores_mobile() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        let edit = tree.node("1").unwrap();
        tree.hover(&mut st, edit, false, false);
        assert!(st.active.is_empty(), "hover expands nothing before a click");
        tree.click(&mut st, tree.node("0").unwrap());
        tree.hover(&mut st, edit, false, false);
        assert_eq!(st.active, ["1"]);
        let mut touch = State::default();
        tree.hover(&mut touch, edit, true, false);
        assert_eq!(touch.active, ["1"], "on touch it expands immediately");
        let mut mobile = State { dirty: true, ..State::default() };
        tree.hover(&mut mobile, edit, false, true);
        assert!(mobile.active.is_empty(), "in a narrow window the path does not change");
        assert_eq!(mobile.focus.index, Some(1), "but focus moves to the item");
    }

    #[test]
    fn arrows_walk_levels() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(0), "first valid");
        tree.arrow_down(&mut st);
        tree.arrow_down(&mut st);
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(4), "Users → Quit, separator skipped");
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(4));
        tree.arrow_up(&mut st);
        assert_eq!(st.focus.index, Some(2));
        // → on a leaf does nothing
        tree.arrow_right(&mut st);
        assert!(st.active.is_empty());
        // → on a group
        tree.home(&mut st);
        tree.arrow_right(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() }, "first submenu item");
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(1));
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(1), "Print is disabled: stay on Open");
        tree.arrow_up(&mut st);
        assert_eq!(st.focus.index, Some(0), "New");
        tree.arrow_right(&mut st);
        assert_eq!(st.active, ["0", "0_0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0_0".into() });
        // ← moves up to New and collapses its submenu
        tree.arrow_left(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() }, "focus on New (the expanded item of the level)");
        // ← once more moves up to File and collapses it
        tree.arrow_left(&mut st);
        assert_eq!(st.active, Vec::<String>::new());
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() });
        // ← on the root changes nothing in focus
        tree.arrow_left(&mut st);
        assert_eq!(st.focus.index, Some(0));
    }

    #[test]
    fn bar_keys_walk_the_root_horizontally_and_submenus_vertically() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        // → on empty focus: the first valid item
        tree.bar_right(&mut st);
        assert_eq!(st.focus.index, Some(0));
        tree.bar_right(&mut st);
        assert_eq!(st.focus.index, Some(1), "Edit");
        tree.bar_right(&mut st);
        tree.bar_right(&mut st);
        assert_eq!(st.focus.index, Some(4), "Users → Quit, separator skipped");
        tree.bar_right(&mut st);
        assert_eq!(st.focus.index, Some(4), "nowhere further");
        tree.bar_left(&mut st);
        assert_eq!(st.focus.index, Some(2), "← on the root: the previous item");
        // ↓ on a root leaf does nothing
        tree.bar_down(&mut st);
        assert!(st.active.is_empty() && st.focus.index == Some(2));
        // ↓ on a group expands it and enters
        tree.home(&mut st);
        tree.bar_down(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() });
        // ↓ / ↑ inside a submenu
        tree.bar_down(&mut st);
        assert_eq!(st.focus.index, Some(1));
        tree.bar_up(&mut st);
        assert_eq!(st.focus.index, Some(0), "New");
        // → on a group expands the nested one and enters
        tree.bar_right(&mut st);
        assert_eq!(st.active, ["0", "0_0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0_0".into() });
        // → on a submenu leaf does nothing
        tree.bar_right(&mut st);
        assert_eq!(st.active, ["0", "0_0"]);
        // ↑ on the first item returns to the parent and collapses its submenu
        tree.bar_up(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() }, "focus on New");
        // ← moves up to the parent and collapses the branch
        tree.bar_left(&mut st);
        assert_eq!(st.active, Vec::<String>::new());
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() }, "focus on File");
    }

    #[test]
    fn bar_up_on_a_root_group_opens_it_at_the_last_item() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        tree.bar_right(&mut st);
        tree.bar_up(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(
            st.focus,
            Focus { index: Some(1), parent_key: "0".into() },
            "Print is disabled, separator skipped: the last valid item is Open"
        );
    }

    #[test]
    fn home_end_escape_tab_and_enter() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        tree.end(&mut st);
        assert_eq!(st.focus.index, Some(4));
        tree.home(&mut st);
        assert_eq!(st.focus.index, Some(0));
        tree.arrow_right(&mut st);
        tree.escape(&mut st);
        assert_eq!(
            (st.active.clone(), st.focus.clone(), st.dirty),
            (Vec::new(), Focus { index: Some(0), parent_key: String::new() }, false)
        );
        // Tab on a leaf closes everything
        tree.end(&mut st);
        tree.tab(&mut st, false);
        assert_eq!(st.focus, Focus::default());
        // Enter on a leaf outside popup returns focus to the first item
        tree.end(&mut st);
        tree.after_enter(&mut st);
        assert_eq!(st.focus.index, Some(0));
        // Enter on a group does not touch focus
        tree.home(&mut st);
        tree.after_enter(&mut st);
        assert_eq!(st.focus.index, Some(0));
    }

    #[test]
    fn typing_searches_by_prefix_from_the_current_item_and_wraps() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let st = State::default();
        let root = tree.siblings("");
        assert_eq!(tree.search(&st, &root, "e"), (Some(1), true), "Edit");
        assert_eq!(tree.search(&st, &root, "Q"), (Some(4), true), "case-insensitive");
        assert_eq!(tree.search(&st, &root, "z"), (Some(0), false), "not found and no focus: the first item");
        let at_edit = State { focus: Focus { index: Some(1), parent_key: String::new() }, ..State::default() };
        assert_eq!(tree.search(&at_edit, &root, "u"), (Some(2), true), "Users after Edit");
        let at_quit = State { focus: Focus { index: Some(4), parent_key: String::new() }, ..State::default() };
        assert_eq!(tree.search(&at_quit, &root, "f"), (Some(0), true), "wraps around to the start");
        assert_eq!(tree.search(&at_quit, &root, "z"), (None, false), "there is focus but nothing found: nothing changes");
        // disabled items are not found
        let file = State { active: vec!["0".into()], focus: Focus { index: None, parent_key: "0".into() }, dirty: true };
        let sub = tree.visible(&file);
        assert_eq!(tree.search(&file, &sub, "p"), (Some(0), false), "Print is disabled: not counted as found, focus on the expanded/first item");
        assert!(is_printable("a", false) && !is_printable("a", true) && !is_printable("Enter", false) && !is_printable("", false));
    }

    #[test]
    fn focused_id_uses_the_level_of_the_focus() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        assert_eq!(tree.focused_id(&State::default(), "m"), None);
        let root = State { focus: Focus { index: Some(1), parent_key: String::new() }, ..State::default() };
        assert_eq!(tree.focused_id(&root, "m").as_deref(), Some("m_1"));
        let nested = State { focus: Focus { index: Some(1), parent_key: "0_0".into() }, ..State::default() };
        assert_eq!(tree.focused_id(&nested, "m").as_deref(), Some("m_0_0_1"));
    }

    #[test]
    fn aria_counts_skip_separators_and_hidden_items() {
        let items = [leaf("a"), MenuItem::separator(), leaf("b").visible(false), leaf("c")];
        let refs: Vec<&MenuItem> = items.iter().collect();
        assert_eq!(aria_set_size(&refs), 2);
        assert_eq!(aria_pos_inset(&refs, 0), 1);
        assert_eq!(aria_pos_inset(&refs, 3), 2);
    }

    #[test]
    fn submenu_side_table() {
        // (item left edge, item width, submenu width, window, scrollbar, level) → side
        let table = [
            (100.0, 200.0, 200.0, 1000.0, 0.0, 1, "100%"),
            (700.0, 200.0, 200.0, 1000.0, 0.0, 1, "-100%"),
            (100.0, 200.0, 800.0, 400.0, 0.0, 1, "100%"),
            (100.0, 200.0, 800.0, 400.0, 0.0, 2, "-100%"),
            (790.0, 200.0, 200.0, 1000.0, 15.0, 1, "-100%"),
            (600.5, 200.0, 200.0, 1000.0, 0.0, 1, "100%"),
        ];
        for (left, item, sub, vp, sb, level, want) in table {
            assert_eq!(submenu_side(left, item, sub, vp, sb, level), want, "{left} {item} {sub} {vp} {sb} {level}");
        }
    }
}
