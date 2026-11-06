//! Чистая модель TreeTable: узлы, сортировка по уровням, страница корней, развёртка в строки, выбор флажками с
//! распространением (по `treetable.ts` PrimeNG, без DOM).

use prime_table::model::{SortMeta, sort_rows};
use std::cmp::Ordering;

/// Узел дерева.
#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    /// Ключ узла (аналог `dataKey`): по нему хранятся выбор и раскрытие.
    pub key: String,
    /// Данные узла.
    pub data: T,
    /// Дочерние узлы.
    pub children: Vec<TreeNode<T>>,
    /// Явный признак листа (для ленивой загрузки: `Some(false)` — узел раскрываемый без детей).
    pub leaf: Option<bool>,
    /// Начальное раскрытие, если `expanded_keys` не передан.
    pub expanded: bool,
}

impl<T> TreeNode<T> {
    /// Узел без детей.
    pub fn new(key: impl Into<String>, data: T) -> Self {
        Self { key: key.into(), data, children: Vec::new(), leaf: None, expanded: false }
    }

    /// Дочерние узлы.
    pub fn children(mut self, children: Vec<TreeNode<T>>) -> Self {
        self.children = children;
        self
    }

    /// Явный признак листа.
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = Some(leaf);
        self
    }

    /// Раскрыт изначально.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Узел можно раскрыть: есть дети либо он явно не лист (кнопка раскрытия видна).
    pub fn is_expandable(&self) -> bool {
        self.leaf == Some(false) || !self.children.is_empty()
    }
}

/// Строка таблицы: узел с уровнем вложенности.
#[derive(Clone, Debug)]
pub struct RowNode<T> {
    /// Ключ узла.
    pub key: String,
    /// Данные узла.
    pub data: T,
    /// Уровень вложенности (корень — 0).
    pub level: usize,
    /// Кнопка раскрытия видна.
    pub expandable: bool,
    /// Узел раскрыт.
    pub expanded: bool,
    /// Ключ родителя.
    pub parent: Option<String>,
    /// Номер среди соседей (с 1).
    pub position: usize,
    /// Число соседей.
    pub set_size: usize,
}

/// Начальный набор раскрытых узлов по флагам `expanded`.
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

/// Сортировка соседей на каждом уровне (`sortNodes`); `compare` сравнивает данные узлов по полю.
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

/// Страница корневых узлов (`serializePageNodes`): пагинатор считает корни, а не строки.
pub fn page_roots<T: Clone>(roots: &[TreeNode<T>], paginator: bool, first: usize, rows: usize) -> Vec<TreeNode<T>> {
    if paginator { roots.iter().skip(first).take(rows).cloned().collect() } else { roots.to_vec() }
}

/// Видимые строки: обход в глубину, потомки раскрытых узлов (`serializeNodes`).
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

/// Ключ узла и всех его потомков.
fn collect_keys<T>(node: &TreeNode<T>, out: &mut Vec<String>) {
    out.push(node.key.clone());
    for child in &node.children {
        collect_keys(child, out);
    }
}

/// Путь от корня до узла `key` включительно.
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

/// Выбор флажком с распространением (`toggleNodeWithCheckbox`): вниз — на потомков, вверх — родитель выбран,
/// только если выбраны все его дети; при снятии родители снимаются.
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

/// Состояние флажка узла.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckState {
    /// Не выбран.
    Unchecked,
    /// Выбран часть потомков.
    Partial,
    /// Выбран.
    Checked,
}

/// Состояние флажка узла по выбору (`partialSelected`).
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

/// Ключи всех узлов дерева (выбор всех флажком в заголовке).
pub fn all_keys<T>(roots: &[TreeNode<T>]) -> Vec<String> {
    let mut out = Vec::new();
    for root in roots {
        collect_keys(root, &mut out);
    }
    out
}

/// Узел по ключу.
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
        assert_eq!(row_keys(&flatten(&tree(), &keys("b"))), "a f h", "потомки скрытого родителя не показываются");
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
        assert!(n("a", 1).leaf(false).is_expandable(), "ленивый узел без детей");
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
        assert_eq!(sorted.iter().map(|n| n.key.as_str()).collect::<Vec<_>>(), ["a", "h", "f"], "корни по значению 1, 4, 5");
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
        assert_eq!(page_roots(&tree(), false, 5, 1).len(), 3, "без пагинатора — все корни");
        assert!(page_roots(&tree(), true, 9, 2).is_empty());
    }

    #[test]
    fn checkbox_propagates_down_and_up() {
        let t = tree();
        let sel = checkbox_toggle(&t, &[], "b", true);
        assert_eq!(sel, keys("b d e"), "узел и потомки; родитель a не выбран: c ещё нет");
        let sel = checkbox_toggle(&t, &sel, "c", true);
        assert_eq!(sel, keys("b d e c a"), "когда выбраны все дети, родитель выбирается");
        let sel = checkbox_toggle(&t, &sel, "d", false);
        assert_eq!(sel, keys("e c"), "при снятии родители b и a снимаются");
        let sel = checkbox_toggle(&t, &sel, "a", false);
        assert!(sel.is_empty(), "снятие корня снимает всё поддерево: {sel:?}");
        assert_eq!(checkbox_toggle(&t, &keys("x"), "zzz", true), keys("x"), "неизвестный ключ");
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
