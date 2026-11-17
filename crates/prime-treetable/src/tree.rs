//! Pure TreeTable model: nodes, per-level sorting, root page, flattening into rows, checkbox selection with
//! propagation (after PrimeNG `treetable.ts`, no DOM).

use prime_table::model::{SortMeta, sort_rows};
use std::cmp::Ordering;

/// Tree node.
#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    /// Node key (analogue of `dataKey`): selection and expansion are stored by it.
    pub key: String,
    /// Node data.
    pub data: T,
    /// Child nodes.
    pub children: Vec<TreeNode<T>>,
    /// Explicit leaf flag (for lazy loading: `Some(false)` — an expandable node without children).
    pub leaf: Option<bool>,
    /// Initial expansion, if `expanded_keys` is not passed.
    pub expanded: bool,
}

impl<T> TreeNode<T> {
    /// Node without children.
    pub fn new(key: impl Into<String>, data: T) -> Self {
        Self { key: key.into(), data, children: Vec::new(), leaf: None, expanded: false }
    }

    /// Child nodes.
    pub fn children(mut self, children: Vec<TreeNode<T>>) -> Self {
        self.children = children;
        self
    }

    /// Explicit leaf flag.
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = Some(leaf);
        self
    }

    /// Expanded initially.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// The node can be expanded: it has children or is explicitly not a leaf (the expand button is visible).
    pub fn is_expandable(&self) -> bool {
        self.leaf == Some(false) || !self.children.is_empty()
    }
}

/// Table row: a node with its nesting level.
#[derive(Clone, Debug)]
pub struct RowNode<T> {
    /// Node key.
    pub key: String,
    /// Node data.
    pub data: T,
    /// Nesting level (root is 0).
    pub level: usize,
    /// Expand button is visible.
    pub expandable: bool,
    /// Node is expanded.
    pub expanded: bool,
    /// Parent key.
    pub parent: Option<String>,
    /// Position among siblings (from 1).
    pub position: usize,
    /// Number of siblings.
    pub set_size: usize,
}

/// Initial set of expanded nodes by the `expanded` flags.
pub fn initial_expanded<T>(roots: &[TreeNode<T>]) -> Vec<String> {
    let mut out = Vec::new();
    fn walk<T>(nodes: &[TreeNode<T>], out: &mut Vec<String>) {
        for n in nodes {
            if n.expanded {
                out.push(n.key.clone());
            }
            walk(&n.children, out);
        }
    }
    walk(roots, &mut out);
    out
}

/// Sorting siblings at every level (`sortNodes`); `compare` compares node data by field.
pub fn sort_nodes<T: Clone>(nodes: &[TreeNode<T>], meta: &[SortMeta], compare: &dyn Fn(&str, &T, &T) -> Ordering) -> Vec<TreeNode<T>> {
    if meta.is_empty() {
        return nodes.to_vec();
    }
    let mut sorted = sort_rows(nodes, meta, &|field, a: &TreeNode<T>, b: &TreeNode<T>| compare(field, &a.data, &b.data));
    for node in &mut sorted {
        node.children = sort_nodes(&node.children, meta, compare);
    }
    sorted
}

/// Page of root nodes (`serializePageNodes`): the paginator counts roots, not rows.
pub fn page_roots<T: Clone>(roots: &[TreeNode<T>], paginator: bool, first: usize, rows: usize) -> Vec<TreeNode<T>> {
    if paginator { roots.iter().skip(first).take(rows).cloned().collect() } else { roots.to_vec() }
}

/// Visible rows: depth-first traversal, descendants of expanded nodes (`serializeNodes`).
pub fn flatten<T: Clone>(roots: &[TreeNode<T>], expanded: &[String]) -> Vec<RowNode<T>> {
    let mut out = Vec::new();
    fn walk<T: Clone>(nodes: &[TreeNode<T>], parent: Option<&str>, level: usize, expanded: &[String], out: &mut Vec<RowNode<T>>) {
        for (i, node) in nodes.iter().enumerate() {
            let open = expanded.iter().any(|k| k == &node.key);
            out.push(RowNode {
                key: node.key.clone(),
                data: node.data.clone(),
                level,
                expandable: node.is_expandable(),
                expanded: open,
                parent: parent.map(str::to_owned),
                position: i + 1,
                set_size: nodes.len(),
            });
            if open {
                walk(&node.children, Some(&node.key), level + 1, expanded, out);
            }
        }
    }
    walk(roots, None, 0, expanded, &mut out);
    out
}

/// Key of a node and all its descendants.
fn collect_keys<T>(node: &TreeNode<T>, out: &mut Vec<String>) {
    out.push(node.key.clone());
    for child in &node.children {
        collect_keys(child, out);
    }
}

/// Path from the root to node `key`, inclusive.
fn path_to<'a, T>(nodes: &'a [TreeNode<T>], key: &str, path: &mut Vec<&'a TreeNode<T>>) -> bool {
    for node in nodes {
        path.push(node);
        if node.key == key || path_to(&node.children, key, path) {
            return true;
        }
        path.pop();
    }
    false
}

/// Checkbox selection with propagation (`toggleNodeWithCheckbox`): down — to descendants, up — the parent is selected
/// only if all its children are selected; on unselect the parents are unselected.
pub fn checkbox_toggle<T>(roots: &[TreeNode<T>], selection: &[String], key: &str, select: bool) -> Vec<String> {
    let mut path = Vec::new();
    if !path_to(roots, key, &mut path) {
        return selection.to_vec();
    }
    let target = path[path.len() - 1];
    let mut subtree = Vec::new();
    collect_keys(target, &mut subtree);
    let mut set: Vec<String> = selection.to_vec();
    if select {
        for k in subtree {
            if !set.contains(&k) {
                set.push(k);
            }
        }
    } else {
        set.retain(|k| !subtree.contains(k));
    }
    for ancestor in path[..path.len() - 1].iter().rev() {
        if select {
            let all = ancestor.children.iter().all(|c| set.contains(&c.key));
            if all && !set.contains(&ancestor.key) {
                set.push(ancestor.key.clone());
            }
        } else {
            set.retain(|k| k != &ancestor.key);
        }
    }
    set
}

/// Node checkbox state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckState {
    /// Not selected.
    Unchecked,
    /// Some descendants selected.
    Partial,
    /// Selected.
    Checked,
}

/// Node checkbox state by selection (`partialSelected`).
pub fn check_state<T>(node: &TreeNode<T>, selection: &[String]) -> CheckState {
    if selection.contains(&node.key) {
        return CheckState::Checked;
    }
    let mut descendants = Vec::new();
    for child in &node.children {
        collect_keys(child, &mut descendants);
    }
    if descendants.iter().any(|k| selection.contains(k)) { CheckState::Partial } else { CheckState::Unchecked }
}

/// Keys of all tree nodes (select all via the header checkbox).
pub fn all_keys<T>(roots: &[TreeNode<T>]) -> Vec<String> {
    let mut out = Vec::new();
    for root in roots {
        collect_keys(root, &mut out);
    }
    out
}

/// Node by key.
pub fn find<'a, T>(roots: &'a [TreeNode<T>], key: &str) -> Option<&'a TreeNode<T>> {
    let mut path = Vec::new();
    path_to(roots, key, &mut path).then(|| path[path.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    fn n(key: &str, value: i32) -> TreeNode<i32> {
        TreeNode::new(key, value)
    }

    /// a(1) { b(3) { d(7), e(2) }, c(9) }   f(5) { g(1) }   h(4)
    fn tree() -> Vec<TreeNode<i32>> {
        vec![
            n("a", 1).children(vec![n("b", 3).children(vec![n("d", 7), n("e", 2)]), n("c", 9)]),
            n("f", 5).children(vec![n("g", 1)]),
            n("h", 4),
        ]
    }

    fn row_keys(rows: &[RowNode<i32>]) -> String {
        rows.iter().map(|r| r.key.as_str()).collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn flatten_follows_expansion() {
        assert_eq!(row_keys(&flatten(&tree(), &[])), "a f h");
        assert_eq!(row_keys(&flatten(&tree(), &keys("a"))), "a b c f h");
        assert_eq!(row_keys(&flatten(&tree(), &keys("a b"))), "a b d e c f h");
        assert_eq!(row_keys(&flatten(&tree(), &keys("b"))), "a f h", "descendants of a hidden parent are not shown");
        let rows = flatten(&tree(), &keys("a b"));
        let d = rows.iter().find(|r| r.key == "d").unwrap();
        assert_eq!((d.level, d.parent.as_deref(), d.position, d.set_size, d.expandable), (2, Some("b"), 1, 2, false));
        let a = &rows[0];
        assert_eq!((a.level, a.parent.clone(), a.expandable, a.expanded, a.set_size), (0, None, true, true, 3));
    }

    #[test]
    fn expandable_by_children_or_leaf_flag() {
        assert!(n("a", 1).children(vec![n("b", 2)]).is_expandable());
        assert!(!n("a", 1).is_expandable());
        assert!(n("a", 1).leaf(false).is_expandable(), "lazy node without children");
        assert!(!n("a", 1).leaf(true).is_expandable());
    }

    #[test]
    fn initial_expansion_flags() {
        let tree = vec![n("a", 1).expanded(true).children(vec![n("b", 2).expanded(true)]), n("c", 3)];
        assert_eq!(initial_expanded(&tree), keys("a b"));
    }

    #[test]
    fn sort_is_per_level() {
        let cmp = |_: &str, a: &i32, b: &i32| a.cmp(b);
        let sorted = sort_nodes(&tree(), &[SortMeta::new("v", 1)], &cmp);
        assert_eq!(sorted.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["a", "h", "f"], "roots by value 1, 4, 5");
        let a = &sorted[0];
        assert_eq!(a.children.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["b", "c"], "3 < 9");
        assert_eq!(a.children[0].children.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["e", "d"], "2 < 7");
        let desc = sort_nodes(&tree(), &[SortMeta::new("v", -1)], &cmp);
        assert_eq!(desc.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["f", "h", "a"]);
        assert_eq!(desc[2].children[0].key, "c");
        assert_eq!(sort_nodes(&tree(), &[], &cmp)[0].key, "a");
    }

    #[test]
    fn paginator_counts_roots() {
        let page = page_roots(&tree(), true, 1, 2);
        assert_eq!(page.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["f", "h"]);
        assert_eq!(page_roots(&tree(), true, 0, 1).len(), 1);
        assert_eq!(page_roots(&tree(), false, 5, 1).len(), 3, "without a paginator — all roots");
        assert!(page_roots(&tree(), true, 9, 2).is_empty());
    }

    #[test]
    fn checkbox_propagates_down_and_up() {
        let t = tree();
        let sel = checkbox_toggle(&t, &[], "b", true);
        assert_eq!(sel, keys("b d e"), "the node and descendants; parent a is not selected: c is not there yet");
        let sel = checkbox_toggle(&t, &sel, "c", true);
        assert_eq!(sel, keys("b d e c a"), "when all children are selected, the parent is selected");
        let sel = checkbox_toggle(&t, &sel, "d", false);
        assert_eq!(sel, keys("e c"), "on unselect parents b and a are unselected");
        let sel = checkbox_toggle(&t, &sel, "a", false);
        assert!(sel.is_empty(), "unselecting the root unselects the whole subtree: {sel:?}");
        assert_eq!(checkbox_toggle(&t, &keys("x"), "zzz", true), keys("x"), "unknown key");
    }

    #[test]
    fn check_states() {
        let t = tree();
        let a = &t[0];
        assert_eq!(check_state(a, &[]), CheckState::Unchecked);
        assert_eq!(check_state(a, &keys("d")), CheckState::Partial);
        assert_eq!(check_state(a, &keys("a")), CheckState::Checked);
        assert_eq!(check_state(&t[2], &keys("a")), CheckState::Unchecked);
    }

    #[test]
    fn helpers() {
        assert_eq!(all_keys(&tree()), keys("a b d e c f g h"));
        assert_eq!(find(&tree(), "e").map(|n| n.data), Some(2));
        assert!(find(&tree(), "zzz").is_none());
    }
}
