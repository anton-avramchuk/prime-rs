//! Tree model and pure logic: flat node list, visible nodes, selection, checkbox propagation, filter and keyboard moves
//! (after PrimeNG 21.1.10 `tree.ts`). State (expanded and selected keys) is stored outside the nodes.

use leptos::prelude::ViewFn;
use std::collections::HashSet;

/// Tree node.
#[derive(Clone)]
pub struct TreeNode {
    /// Unique key (expansion and selection are stored by it).
    pub key: String,
    /// Label.
    pub label: Option<String>,
    /// Node icon.
    pub icon: Option<ViewFn>,
    /// Icon of an expanded node with children (if no `icon`).
    pub expanded_icon: Option<ViewFn>,
    /// Icon of a collapsed node (if no `icon`).
    pub collapsed_icon: Option<ViewFn>,
    /// Children.
    pub children: Vec<TreeNode>,
    /// Whether the node is a leaf; `Some(false)` — the node is expandable without children (lazy loading).
    pub leaf: Option<bool>,
    /// Initial expansion.
    pub expanded: bool,
    /// Whether the node can be selected (yes by default).
    pub selectable: bool,
    /// Children are loading.
    pub loading: bool,
    /// Node type for template selection.
    pub type_: Option<String>,
    /// Extra class of `li`.
    pub style_class: Option<String>,
    /// Inline style of `li`.
    pub style: Option<String>,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self {
            key: String::new(),
            label: None,
            icon: None,
            expanded_icon: None,
            collapsed_icon: None,
            children: Vec::new(),
            leaf: None,
            expanded: false,
            selectable: true,
            loading: false,
            type_: None,
            style_class: None,
            style: None,
        }
    }
}

impl TreeNode {
    /// Node with a key.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into(), ..Self::default() }
    }

    /// Label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Icon.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Icons of the expanded and collapsed node.
    pub fn icons(mut self, expanded: impl Into<ViewFn>, collapsed: impl Into<ViewFn>) -> Self {
        self.expanded_icon = Some(expanded.into());
        self.collapsed_icon = Some(collapsed.into());
        self
    }

    /// Children.
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Leaf (`false` — expandable without children).
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = Some(leaf);
        self
    }

    /// Initial expansion.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Whether it can be selected.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Loading.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Node type.
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    /// Extra class.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Leaf: without children and not marked `leaf = false`.
    pub fn is_leaf(&self) -> bool {
        self.leaf != Some(false) && self.children.is_empty()
    }
}

/// Selection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeSelectionMode {
    /// One node.
    Single,
    /// Several nodes.
    Multiple,
    /// Checkboxes with propagation across the subtree.
    Checkbox,
}

/// Filter mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeFilterMode {
    /// A matching node stays with all its children.
    #[default]
    Lenient,
    /// Only matching nodes stay.
    Strict,
}

/// Node in the flat list (depth-first traversal).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flat {
    /// Key.
    pub key: String,
    /// Parent key.
    pub parent: Option<String>,
    /// Depth (roots are `0`).
    pub level: usize,
    /// Index among siblings.
    pub index: usize,
    /// Number of siblings.
    pub siblings: usize,
    /// Keys of direct children.
    pub children: Vec<String>,
    /// Leaf.
    pub leaf: bool,
    /// Can be selected.
    pub selectable: bool,
    /// Label.
    pub label: String,
}

/// Flattens the tree into a flat list.
pub fn flatten(nodes: &[TreeNode]) -> Vec<Flat> {
    fn walk(nodes: &[TreeNode], parent: Option<&str>, level: usize, out: &mut Vec<Flat>) {
        for (index, n) in nodes.iter().enumerate() {
            out.push(Flat {
                key: n.key.clone(),
                parent: parent.map(str::to_owned),
                level,
                index,
                siblings: nodes.len(),
                children: n.children.iter().map(|c| c.key.clone()).collect(),
                leaf: n.is_leaf(),
                selectable: n.selectable,
                label: n.label.clone().unwrap_or_default(),
            });
            walk(&n.children, Some(&n.key), level + 1, out);
        }
    }
    let mut out = Vec::new();
    walk(nodes, None, 0, &mut out);
    out
}

/// Keys of nodes with `expanded = true`.
pub fn seed_expanded(nodes: &[TreeNode]) -> HashSet<String> {
    fn walk(nodes: &[TreeNode], out: &mut HashSet<String>) {
        for n in nodes {
            if n.expanded {
                out.insert(n.key.clone());
            }
            walk(&n.children, out);
        }
    }
    let mut out = HashSet::new();
    walk(nodes, &mut out);
    out
}

/// Visible nodes: roots and children of expanded visible nodes.
pub fn visible<'a>(flat: &'a [Flat], expanded: &HashSet<String>) -> Vec<&'a Flat> {
    let mut shown: HashSet<&str> = HashSet::new();
    let mut out = Vec::new();
    for n in flat {
        let ok = match &n.parent {
            None => true,
            Some(p) => shown.contains(p.as_str()) && expanded.contains(p),
        };
        if ok {
            shown.insert(&n.key);
            out.push(n);
        }
    }
    out
}

fn find<'a>(flat: &'a [Flat], key: &str) -> Option<&'a Flat> {
    flat.iter().find(|n| n.key == key)
}

fn remove(sel: &mut Vec<String>, key: &str) {
    sel.retain(|k| k != key);
}

fn add(sel: &mut Vec<String>, key: &str) {
    if !sel.iter().any(|k| k == key) {
        sel.push(key.to_owned());
    }
}

/// Downward selection propagation (`propagateDown`).
pub fn propagate_down(flat: &[Flat], sel: &mut Vec<String>, key: &str, select: bool) {
    if select {
        add(sel, key);
    } else {
        remove(sel, key);
    }
    if let Some(node) = find(flat, key) {
        for child in &node.children {
            propagate_down(flat, sel, child, select);
        }
    }
}

/// Upward selection propagation (`propagateUp`): an ancestor is selected when all its children are selected.
pub fn propagate_up(flat: &[Flat], sel: &mut Vec<String>, key: &str, select: bool) {
    let mut cur = Some(key.to_owned());
    while let Some(k) = cur {
        let Some(node) = find(flat, &k) else { break };
        if !node.children.is_empty() {
            let all = node.children.iter().all(|c| sel.contains(c));
            if select && all {
                add(sel, &k);
            } else if !select {
                remove(sel, &k);
            }
        }
        cur = node.parent.clone();
    }
}

/// Partially selected: not selected itself, but at least one descendant is selected.
pub fn is_partial(flat: &[Flat], sel: &[String], key: &str) -> bool {
    fn any(flat: &[Flat], sel: &[String], key: &str) -> bool {
        find(flat, key).is_some_and(|n| n.children.iter().any(|c| sel.iter().any(|s| s == c) || any(flat, sel, c)))
    }
    !sel.iter().any(|s| s == key) && any(flat, sel, key)
}

/// Selection parameters.
#[derive(Clone, Copy, Debug)]
pub struct SelectOpts {
    /// Mode.
    pub mode: TreeSelectionMode,
    /// Selection with Ctrl/Meta.
    pub meta_selection: bool,
    /// Ctrl or Meta pressed.
    pub meta_key: bool,
    /// Propagate up.
    pub up: bool,
    /// Propagate down.
    pub down: bool,
}

/// Click on a node. Returns `Some(true)` on select, `Some(false)` on deselect, `None` if the selection did not change.
pub fn select_click(flat: &[Flat], sel: &mut Vec<String>, key: &str, o: SelectOpts) -> Option<bool> {
    let node = find(flat, key)?;
    if !node.selectable {
        return None;
    }
    let selected = sel.iter().any(|s| s == key);
    match o.mode {
        TreeSelectionMode::Checkbox => {
            let select = !selected;
            if o.down {
                propagate_down(flat, sel, key, select);
            } else if select {
                add(sel, key);
            } else {
                remove(sel, key);
            }
            if o.up
                && let Some(parent) = &node.parent
            {
                propagate_up(flat, sel, parent, select);
            }
            Some(select)
        }
        TreeSelectionMode::Single => {
            if o.meta_selection {
                if selected && o.meta_key {
                    sel.clear();
                    return Some(false);
                }
            } else if selected {
                sel.clear();
                return Some(false);
            }
            sel.clear();
            sel.push(key.to_owned());
            Some(true)
        }
        TreeSelectionMode::Multiple => {
            if o.meta_selection {
                if selected && o.meta_key {
                    remove(sel, key);
                    return Some(false);
                }
                if !o.meta_key {
                    sel.clear();
                }
                add(sel, key);
                Some(true)
            } else if selected {
                remove(sel, key);
                Some(false)
            } else {
                add(sel, key);
                Some(true)
            }
        }
    }
}

/// Filter parameters.
pub struct FilterOpts<'a> {
    /// Search fields: `label` and/or `key`.
    pub fields: &'a [&'a str],
    /// Mode.
    pub mode: TreeFilterMode,
}

fn field_matches(node: &TreeNode, text: &str, fields: &[&str]) -> bool {
    fields.iter().any(|f| {
        let value = match *f {
            "key" => node.key.as_str(),
            _ => node.label.as_deref().unwrap_or(""),
        };
        value.to_lowercase().contains(text)
    })
}

/// `findFilteredNodes`: keeps only matching children of the node and expands the node if there are any.
fn find_filtered(node: &mut TreeNode, text: &str, o: &FilterOpts) -> bool {
    let kept: Vec<TreeNode> =
        std::mem::take(&mut node.children).into_iter().filter_map(|mut c| is_matched(&mut c, text, o).then_some(c)).collect();
    let matched = !kept.is_empty();
    node.children = kept;
    if matched {
        node.expanded = true;
    }
    matched
}

/// `isFilterMatched`.
fn is_matched(node: &mut TreeNode, text: &str, o: &FilterOpts) -> bool {
    let mut matched = field_matches(node, text, o.fields);
    if !matched || (o.mode == TreeFilterMode::Strict && !node.is_leaf()) {
        matched = find_filtered(node, text, o) || matched;
    }
    matched
}

/// Filtered tree and keys of nodes expanded by the filter. Empty text returns `None` (the full tree).
pub fn filter_tree(nodes: &[TreeNode], text: &str, o: &FilterOpts) -> Option<(Vec<TreeNode>, Vec<String>)> {
    if text.is_empty() {
        return None;
    }
    let text = text.to_lowercase();
    let mut out = Vec::new();
    for n in nodes {
        let mut copy = n.clone();
        let hit = match o.mode {
            TreeFilterMode::Strict => find_filtered(&mut copy, &text, o) || is_matched(&mut copy, &text, o),
            TreeFilterMode::Lenient => is_matched(&mut copy, &text, o) || find_filtered(&mut copy, &text, o),
        };
        if hit {
            out.push(copy);
        }
    }
    let mut keys = Vec::new();
    fn walk(nodes: &[TreeNode], keys: &mut Vec<String>) {
        for n in nodes {
            if n.expanded {
                keys.push(n.key.clone());
            }
            walk(&n.children, keys);
        }
    }
    walk(&out, &mut keys);
    Some((out, keys))
}

/// Move result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    /// Focus a node.
    Focus(String),
    /// Expand the node and move to its first child.
    ExpandAndEnter(String),
    /// Collapse the node.
    Collapse(String),
    /// Nothing.
    Stay,
}

fn pos(vis: &[&Flat], key: &str) -> Option<usize> {
    vis.iter().position(|n| n.key == key)
}

/// ↓.
pub fn arrow_down(vis: &[&Flat], key: &str) -> Move {
    match pos(vis, key) {
        Some(i) if i + 1 < vis.len() => Move::Focus(vis[i + 1].key.clone()),
        _ => Move::Stay,
    }
}

/// ↑.
pub fn arrow_up(vis: &[&Flat], key: &str) -> Move {
    match pos(vis, key) {
        Some(i) if i > 0 => Move::Focus(vis[i - 1].key.clone()),
        _ => Move::Stay,
    }
}

/// →: a collapsed node expands and focus moves to its first child; an expanded one — no action (as in PrimeNG).
pub fn arrow_right(flat: &[Flat], expanded: &HashSet<String>, key: &str) -> Move {
    match find(flat, key) {
        Some(n) if !n.leaf && !expanded.contains(key) => Move::ExpandAndEnter(key.to_owned()),
        _ => Move::Stay,
    }
}

/// ←: an expanded node collapses, otherwise focus goes to the parent.
pub fn arrow_left(flat: &[Flat], expanded: &HashSet<String>, key: &str) -> Move {
    let Some(n) = find(flat, key) else { return Move::Stay };
    if expanded.contains(key) && !n.children.is_empty() {
        return Move::Collapse(key.to_owned());
    }
    n.parent.clone().map_or(Move::Stay, Move::Focus)
}

/// `Home`.
pub fn home(vis: &[&Flat]) -> Move {
    vis.first().map_or(Move::Stay, |n| Move::Focus(n.key.clone()))
}

/// `End`.
pub fn end(vis: &[&Flat]) -> Move {
    vis.last().map_or(Move::Stay, |n| Move::Focus(n.key.clone()))
}

/// Node with `tabindex="0"`: the focused one, else the first selected visible one, else the first visible one.
pub fn tab_stop(vis: &[&Flat], sel: &[String], focus: Option<&str>) -> Option<String> {
    focus
        .filter(|f| pos(vis, f).is_some())
        .map(str::to_owned)
        .or_else(|| vis.iter().find(|n| sel.iter().any(|s| s == &n.key)).map(|n| n.key.clone()))
        .or_else(|| vis.first().map(|n| n.key.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(key: &str, label: &str) -> TreeNode {
        TreeNode::new(key).label(label)
    }

    /// a{ a1{a11,a12}, a2 }, b, c{ c1 }
    fn tree() -> Vec<TreeNode> {
        vec![
            n("a", "Alpha").children(vec![n("a1", "Apple").children(vec![n("a11", "Red"), n("a12", "Green")]), n("a2", "Avocado")]),
            n("b", "Beta"),
            n("c", "Gamma").children(vec![n("c1", "Cherry")]),
        ]
    }

    fn set(keys: &[&str]) -> HashSet<String> {
        keys.iter().map(|k| (*k).to_owned()).collect()
    }

    fn opts(mode: TreeSelectionMode) -> SelectOpts {
        SelectOpts { mode, meta_selection: false, meta_key: false, up: true, down: true }
    }

    fn keys(vis: &[&Flat]) -> Vec<String> {
        vis.iter().map(|n| n.key.clone()).collect()
    }

    #[test]
    fn flatten_and_visible() {
        let flat = flatten(&tree());
        let all: Vec<&str> = flat.iter().map(|f| f.key.as_str()).collect();
        assert_eq!(all, ["a", "a1", "a11", "a12", "a2", "b", "c", "c1"]);
        let a1 = flat.iter().find(|f| f.key == "a1").unwrap();
        assert_eq!((a1.level, a1.index, a1.siblings, a1.parent.as_deref(), a1.leaf), (1, 0, 2, Some("a"), false));
        assert_eq!(keys(&visible(&flat, &set(&[]))), ["a", "b", "c"]);
        assert_eq!(keys(&visible(&flat, &set(&["a"]))), ["a", "a1", "a2", "b", "c"]);
        assert_eq!(keys(&visible(&flat, &set(&["a", "a1"]))), ["a", "a1", "a11", "a12", "a2", "b", "c"]);
        assert_eq!(keys(&visible(&flat, &set(&["a1"]))), ["a", "b", "c"], "children of a collapsed ancestor are hidden");
    }

    #[test]
    fn seed_and_leaf_flags() {
        let mut t = tree();
        t[0].children[0].expanded = true;
        assert_eq!(seed_expanded(&t), set(&["a1"]));
        assert!(n("x", "x").is_leaf() && !n("x", "x").leaf(false).is_leaf() && !t[0].is_leaf());
    }

    #[test]
    fn single_and_multiple_selection() {
        let flat = flatten(&tree());
        let mut sel = vec![];
        assert_eq!(select_click(&flat, &mut sel, "b", opts(TreeSelectionMode::Single)), Some(true));
        assert_eq!(select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Single)), Some(true));
        assert_eq!(sel, ["a"], "single selection replaces");
        assert_eq!(select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Single)), Some(false));
        assert!(sel.is_empty(), "repeated click deselects");
        let mut sel = vec![];
        select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Multiple));
        select_click(&flat, &mut sel, "b", opts(TreeSelectionMode::Multiple));
        assert_eq!(sel, ["a", "b"]);
        assert_eq!(select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Multiple)), Some(false));
        assert_eq!(sel, ["b"]);
        let meta = |k| SelectOpts { meta_selection: true, meta_key: k, ..opts(TreeSelectionMode::Multiple) };
        select_click(&flat, &mut sel, "c", meta(false));
        assert_eq!(sel, ["c"], "without Ctrl the selection is replaced");
        select_click(&flat, &mut sel, "b", meta(true));
        assert_eq!(sel, ["c", "b"], "with Ctrl it is extended");
        assert_eq!(select_click(&flat, &mut sel, "c", meta(true)), Some(false));
        assert_eq!(sel, ["b"]);
        let single_meta = |k| SelectOpts { meta_selection: true, meta_key: k, ..opts(TreeSelectionMode::Single) };
        assert_eq!(select_click(&flat, &mut sel, "b", single_meta(false)), Some(true), "without Ctrl a selected node stays selected");
        assert_eq!(select_click(&flat, &mut sel, "b", single_meta(true)), Some(false));
    }

    #[test]
    fn not_selectable_nodes_are_ignored() {
        let mut t = tree();
        t[1].selectable = false;
        let flat = flatten(&t);
        let mut sel = vec![];
        assert_eq!(select_click(&flat, &mut sel, "b", opts(TreeSelectionMode::Single)), None);
        assert!(sel.is_empty());
    }

    #[test]
    fn checkbox_propagation() {
        let flat = flatten(&tree());
        let cb = opts(TreeSelectionMode::Checkbox);
        let mut sel = vec![];
        assert_eq!(select_click(&flat, &mut sel, "a1", cb), Some(true));
        assert_eq!(sel, ["a1", "a11", "a12"], "down: the node itself and descendants");
        assert!(is_partial(&flat, &sel, "a"), "ancestor is partially selected");
        assert_eq!(select_click(&flat, &mut sel, "a2", cb), Some(true));
        assert!(sel.contains(&"a".to_owned()) && !is_partial(&flat, &sel, "a"), "all children selected — ancestor selected too");
        assert_eq!(select_click(&flat, &mut sel, "a11", cb), Some(false));
        assert!(!sel.contains(&"a".to_owned()) && !sel.contains(&"a1".to_owned()), "deselecting a child deselects ancestors");
        assert!(is_partial(&flat, &sel, "a1") && is_partial(&flat, &sel, "a"));
        select_click(&flat, &mut sel, "a12", cb);
        assert!(!is_partial(&flat, &sel, "a1") && !sel.contains(&"a1".to_owned()));
        let mut sel = vec![];
        let no_prop = SelectOpts { up: false, down: false, ..cb };
        select_click(&flat, &mut sel, "a1", no_prop);
        assert_eq!(sel, ["a1"], "without propagation only the node itself");
        assert!(is_partial(&flat, &sel, "a"), "partial state is derived from descendant selection");
        let mut sel = vec![];
        select_click(&flat, &mut sel, "a", SelectOpts { up: false, ..cb });
        assert_eq!(sel.len(), 5, "down without up: the whole subtree");
        select_click(&flat, &mut sel, "a11", SelectOpts { down: false, ..cb });
        assert!(!sel.contains(&"a".to_owned()), "up without down: ancestors recomputed");
    }

    #[test]
    fn filter_lenient_and_strict() {
        let by = FilterOpts { fields: &["label"], mode: TreeFilterMode::Lenient };
        assert!(filter_tree(&tree(), "", &by).is_none());
        let (out, exp) = filter_tree(&tree(), "gre", &by).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(
            (out[0].key.as_str(), out[0].children.len(), out[0].children[0].key.as_str()),
            ("a", 1, "a1"),
            "ancestors stay, the rest is dropped"
        );
        assert_eq!(out[0].children[0].children.len(), 1);
        assert_eq!(exp, ["a", "a1"], "ancestors of the match are expanded");
        let (out, _) = filter_tree(&tree(), "apple", &by).unwrap();
        assert_eq!(out[0].children[0].children.len(), 2, "lenient: a matching node keeps its children");
        let strict = FilterOpts { fields: &["label"], mode: TreeFilterMode::Strict };
        let (out, _) = filter_tree(&tree(), "apple", &strict).unwrap();
        assert_eq!(
            (out[0].children[0].key.as_str(), out[0].children[0].children.len()),
            ("a1", 0),
            "strict: a matching node stays without non-matching children"
        );
        let (out, _) = filter_tree(&tree(), "red", &strict).unwrap();
        assert_eq!(out[0].children[0].children[0].key, "a11");
        let (out, _) = filter_tree(&tree(), "BETA", &by).unwrap();
        assert_eq!(out.len(), 1, "case does not matter");
        let by_key = FilterOpts { fields: &["label", "key"], mode: TreeFilterMode::Lenient };
        let (out, _) = filter_tree(&tree(), "c1", &by_key).unwrap();
        assert_eq!(out[0].key, "c");
        assert!(filter_tree(&tree(), "zzz", &by).unwrap().0.is_empty());
    }

    #[test]
    fn keyboard_moves() {
        let flat = flatten(&tree());
        let ex = set(&["a", "a1"]);
        let vis = visible(&flat, &ex);
        assert_eq!(arrow_down(&vis, "a"), Move::Focus("a1".into()));
        assert_eq!(arrow_down(&vis, "c"), Move::Stay);
        assert_eq!(arrow_up(&vis, "a1"), Move::Focus("a".into()));
        assert_eq!(arrow_up(&vis, "a"), Move::Stay);
        assert_eq!(arrow_up(&vis, "b"), Move::Focus("a2".into()), "up from a node after an expanded branch — to its last one");
        assert_eq!(arrow_right(&flat, &set(&["a"]), "a1"), Move::ExpandAndEnter("a1".into()));
        assert_eq!(arrow_right(&flat, &ex, "a1"), Move::Stay);
        assert_eq!(arrow_right(&flat, &ex, "b"), Move::Stay, "a leaf → nothing");
        assert_eq!(arrow_left(&flat, &ex, "a1"), Move::Collapse("a1".into()));
        assert_eq!(arrow_left(&flat, &ex, "a11"), Move::Focus("a1".into()));
        assert_eq!(arrow_left(&flat, &set(&[]), "a"), Move::Stay);
        assert_eq!(home(&vis), Move::Focus("a".into()));
        assert_eq!(end(&vis), Move::Focus("c".into()));
        assert_eq!(home(&[]), Move::Stay);
    }

    #[test]
    fn tab_stop_prefers_focus_then_selection() {
        let flat = flatten(&tree());
        let vis = visible(&flat, &set(&["a"]));
        assert_eq!(tab_stop(&vis, &[], None).as_deref(), Some("a"));
        assert_eq!(tab_stop(&vis, &["b".to_owned()], None).as_deref(), Some("b"));
        assert_eq!(tab_stop(&vis, &["b".to_owned()], Some("a2")).as_deref(), Some("a2"));
        assert_eq!(tab_stop(&vis, &["b".to_owned()], Some("a11")).as_deref(), Some("b"), "an invisible focus is not valid");
        assert_eq!(tab_stop(&[], &[], None), None);
    }
}
