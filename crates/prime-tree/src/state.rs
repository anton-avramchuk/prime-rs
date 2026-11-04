//! Модель Tree и чистая логика: плоский список узлов, видимые, выбор, распространение флажков, фильтр и переходы клавиатуры
//! (по `tree.ts` PrimeNG 21.1.10). Состояние (раскрытые и выбранные ключи) хранится вне узлов.

use leptos::prelude::ViewFn;
use std::collections::HashSet;

/// Узел дерева.
#[derive(Clone)]
pub struct TreeNode {
    /// Уникальный ключ (по нему хранятся раскрытие и выбор).
    pub key: String,
    /// Подпись.
    pub label: Option<String>,
    /// Значок узла.
    pub icon: Option<ViewFn>,
    /// Значок раскрытого узла с детьми (если нет `icon`).
    pub expanded_icon: Option<ViewFn>,
    /// Значок свёрнутого узла (если нет `icon`).
    pub collapsed_icon: Option<ViewFn>,
    /// Дети.
    pub children: Vec<TreeNode>,
    /// Лист ли узел; `Some(false)` — узел раскрываем без детей (ленивая загрузка).
    pub leaf: Option<bool>,
    /// Начальное раскрытие.
    pub expanded: bool,
    /// Можно ли выбрать узел (по умолчанию да).
    pub selectable: bool,
    /// Идёт загрузка детей.
    pub loading: bool,
    /// Тип узла для выбора шаблона.
    pub type_: Option<String>,
    /// Дополнительный класс `li`.
    pub style_class: Option<String>,
    /// Встроенный стиль `li`.
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
    /// Узел с ключом.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into(), ..Self::default() }
    }

    /// Подпись.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Значок.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Значки раскрытого и свёрнутого узла.
    pub fn icons(mut self, expanded: impl Into<ViewFn>, collapsed: impl Into<ViewFn>) -> Self {
        self.expanded_icon = Some(expanded.into());
        self.collapsed_icon = Some(collapsed.into());
        self
    }

    /// Дети.
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Лист (`false` — раскрываемый без детей).
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = Some(leaf);
        self
    }

    /// Начальное раскрытие.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Можно ли выбрать.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Идёт загрузка.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Тип узла.
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    /// Дополнительный класс.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Лист: без детей и не помечен `leaf = false`.
    pub fn is_leaf(&self) -> bool {
        self.leaf != Some(false) && self.children.is_empty()
    }
}

/// Режим выбора.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeSelectionMode {
    /// Один узел.
    Single,
    /// Несколько узлов.
    Multiple,
    /// Флажки с распространением по поддереву.
    Checkbox,
}

/// Режим фильтра.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeFilterMode {
    /// Совпавший узел остаётся со всеми детьми.
    #[default]
    Lenient,
    /// Остаются только совпавшие.
    Strict,
}

/// Узел в плоском списке (обход в глубину).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flat {
    /// Ключ.
    pub key: String,
    /// Ключ родителя.
    pub parent: Option<String>,
    /// Глубина (корневые — `0`).
    pub level: usize,
    /// Индекс среди соседей.
    pub index: usize,
    /// Число соседей.
    pub siblings: usize,
    /// Ключи прямых детей.
    pub children: Vec<String>,
    /// Лист.
    pub leaf: bool,
    /// Можно выбрать.
    pub selectable: bool,
    /// Подпись.
    pub label: String,
}

/// Разворачивает дерево в плоский список.
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

/// Ключи узлов с `expanded = true`.
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

/// Видимые узлы: корневые и дети раскрытых видимых узлов.
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

/// Распространение выбора вниз (`propagateDown`).
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

/// Распространение выбора вверх (`propagateUp`): предок выбран, когда выбраны все его дети.
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

/// Частично выбран: сам не выбран, но выбран хотя бы один потомок.
pub fn is_partial(flat: &[Flat], sel: &[String], key: &str) -> bool {
    fn any(flat: &[Flat], sel: &[String], key: &str) -> bool {
        find(flat, key).is_some_and(|n| n.children.iter().any(|c| sel.iter().any(|s| s == c) || any(flat, sel, c)))
    }
    !sel.iter().any(|s| s == key) && any(flat, sel, key)
}

/// Параметры выбора.
#[derive(Clone, Copy, Debug)]
pub struct SelectOpts {
    /// Режим.
    pub mode: TreeSelectionMode,
    /// Выбор с Ctrl/Meta.
    pub meta_selection: bool,
    /// Ctrl или Meta нажаты.
    pub meta_key: bool,
    /// Распространять вверх.
    pub up: bool,
    /// Распространять вниз.
    pub down: bool,
}

/// Клик по узлу. Возвращает `Some(true)` при выборе, `Some(false)` при снятии, `None`, если выбор не изменился.
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

/// Параметры фильтра.
pub struct FilterOpts<'a> {
    /// Поля поиска: `label` и/или `key`.
    pub fields: &'a [&'a str],
    /// Режим.
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

/// `findFilteredNodes`: оставляет у узла только подходящих детей и раскрывает узел, если такие есть.
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

/// Отфильтрованное дерево и ключи узлов, раскрытых фильтром. Пустой текст возвращает `None` (полное дерево).
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

/// Результат перехода.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    /// Фокус на узел.
    Focus(String),
    /// Раскрыть узел и перейти к его первому ребёнку.
    ExpandAndEnter(String),
    /// Свернуть узел.
    Collapse(String),
    /// Ничего.
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

/// →: свёрнутый узел раскрывается и фокус уходит к первому ребёнку; раскрытый — без действия (как в PrimeNG).
pub fn arrow_right(flat: &[Flat], expanded: &HashSet<String>, key: &str) -> Move {
    match find(flat, key) {
        Some(n) if !n.leaf && !expanded.contains(key) => Move::ExpandAndEnter(key.to_owned()),
        _ => Move::Stay,
    }
}

/// ←: раскрытый сворачивается, иначе фокус на родителя.
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

/// Узел с `tabindex="0"`: сфокусированный, иначе первый выбранный видимый, иначе первый видимый.
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
        assert_eq!(keys(&visible(&flat, &set(&["a1"]))), ["a", "b", "c"], "дети свёрнутого предка скрыты");
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
        assert_eq!(sel, ["a"], "единичный выбор заменяет");
        assert_eq!(select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Single)), Some(false));
        assert!(sel.is_empty(), "повторный клик снимает");
        let mut sel = vec![];
        select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Multiple));
        select_click(&flat, &mut sel, "b", opts(TreeSelectionMode::Multiple));
        assert_eq!(sel, ["a", "b"]);
        assert_eq!(select_click(&flat, &mut sel, "a", opts(TreeSelectionMode::Multiple)), Some(false));
        assert_eq!(sel, ["b"]);
        let meta = |k| SelectOpts { meta_selection: true, meta_key: k, ..opts(TreeSelectionMode::Multiple) };
        select_click(&flat, &mut sel, "c", meta(false));
        assert_eq!(sel, ["c"], "без Ctrl выбор заменяется");
        select_click(&flat, &mut sel, "b", meta(true));
        assert_eq!(sel, ["c", "b"], "с Ctrl дополняется");
        assert_eq!(select_click(&flat, &mut sel, "c", meta(true)), Some(false));
        assert_eq!(sel, ["b"]);
        let single_meta = |k| SelectOpts { meta_selection: true, meta_key: k, ..opts(TreeSelectionMode::Single) };
        assert_eq!(select_click(&flat, &mut sel, "b", single_meta(false)), Some(true), "без Ctrl выбранный остаётся выбранным");
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
        assert_eq!(sel, ["a1", "a11", "a12"], "вниз: сам узел и потомки");
        assert!(is_partial(&flat, &sel, "a"), "предок частично выбран");
        assert_eq!(select_click(&flat, &mut sel, "a2", cb), Some(true));
        assert!(sel.contains(&"a".to_owned()) && !is_partial(&flat, &sel, "a"), "выбраны все дети — выбран и предок");
        assert_eq!(select_click(&flat, &mut sel, "a11", cb), Some(false));
        assert!(!sel.contains(&"a".to_owned()) && !sel.contains(&"a1".to_owned()), "снятие ребёнка снимает предков");
        assert!(is_partial(&flat, &sel, "a1") && is_partial(&flat, &sel, "a"));
        select_click(&flat, &mut sel, "a12", cb);
        assert!(!is_partial(&flat, &sel, "a1") && !sel.contains(&"a1".to_owned()));
        let mut sel = vec![];
        let no_prop = SelectOpts { up: false, down: false, ..cb };
        select_click(&flat, &mut sel, "a1", no_prop);
        assert_eq!(sel, ["a1"], "без распространения только сам узел");
        assert!(is_partial(&flat, &sel, "a"), "частичность выводится из выбора потомков");
        let mut sel = vec![];
        select_click(&flat, &mut sel, "a", SelectOpts { up: false, ..cb });
        assert_eq!(sel.len(), 5, "вниз без вверх: всё поддерево");
        select_click(&flat, &mut sel, "a11", SelectOpts { down: false, ..cb });
        assert!(!sel.contains(&"a".to_owned()), "вверх без вниз: предки пересчитаны");
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
            "предки остаются, лишнее отброшено"
        );
        assert_eq!(out[0].children[0].children.len(), 1);
        assert_eq!(exp, ["a", "a1"], "предки совпавшего раскрыты");
        let (out, _) = filter_tree(&tree(), "apple", &by).unwrap();
        assert_eq!(out[0].children[0].children.len(), 2, "lenient: совпавший узел сохраняет детей");
        let strict = FilterOpts { fields: &["label"], mode: TreeFilterMode::Strict };
        let (out, _) = filter_tree(&tree(), "apple", &strict).unwrap();
        assert_eq!(
            (out[0].children[0].key.as_str(), out[0].children[0].children.len()),
            ("a1", 0),
            "strict: совпавший узел остаётся без несовпавших детей"
        );
        let (out, _) = filter_tree(&tree(), "red", &strict).unwrap();
        assert_eq!(out[0].children[0].children[0].key, "a11");
        let (out, _) = filter_tree(&tree(), "BETA", &by).unwrap();
        assert_eq!(out.len(), 1, "регистр не важен");
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
        assert_eq!(arrow_up(&vis, "b"), Move::Focus("a2".into()), "вверх с узла после раскрытой ветки — на её последний");
        assert_eq!(arrow_right(&flat, &set(&["a"]), "a1"), Move::ExpandAndEnter("a1".into()));
        assert_eq!(arrow_right(&flat, &ex, "a1"), Move::Stay);
        assert_eq!(arrow_right(&flat, &ex, "b"), Move::Stay, "у листа → ничего");
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
        assert_eq!(tab_stop(&vis, &["b".to_owned()], Some("a11")).as_deref(), Some("b"), "невидимый фокус не годится");
        assert_eq!(tab_stop(&[], &[], None), None);
    }
}
