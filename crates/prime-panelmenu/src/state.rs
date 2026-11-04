//! Чистая модель PanelMenu: набор раскрытых ключей, видимые пункты панели и переходы клавиатуры
//! (по `panelmenu.ts` PrimeNG 21.1.10).
//!
//! Ключ пункта — индексы предков и собственный через `_` (`"0"`, `"0_1"`, `"0_1_2"`), как у `prime_tieredmenu::Node`.
//! Корневые пункты (уровень 0) — заголовки панелей; их потомки образуют дерево внутри панели.

use prime_menu::MenuItem;
use prime_tieredmenu::{Node, is_ancestor_or_self, is_valid, item_at};
use std::collections::HashSet;

fn item<'a>(items: &'a [MenuItem], node: &Node) -> Option<&'a MenuItem> {
    item_at(items, &node.path)
}

fn valid(items: &[MenuItem], node: &Node) -> bool {
    item(items, node).is_some_and(is_valid)
}

/// Ключи пунктов с `expanded = true` — начальное состояние.
pub fn seed_expanded(nodes: &[Node], items: &[MenuItem]) -> HashSet<String> {
    nodes.iter().filter(|n| item(items, n).is_some_and(|i| i.expanded && !i.items.is_empty())).map(|n| n.key.clone()).collect()
}

/// Ключи всех групп (пунктов с подпунктами).
pub fn group_keys(nodes: &[Node], items: &[MenuItem]) -> Vec<String> {
    nodes.iter().filter(|n| item(items, n).is_some_and(|i| !i.items.is_empty())).map(|n| n.key.clone()).collect()
}

/// Переключает ключ и возвращает новое значение «раскрыт».
pub fn toggle(set: &mut HashSet<String>, key: &str) -> bool {
    if set.remove(key) {
        false
    } else {
        set.insert(key.to_owned());
        true
    }
}

/// Клик по заголовку: при `multiple = false` остальные корневые панели сворачиваются, затем ключ переключается.
pub fn toggle_root(set: &mut HashSet<String>, key: &str, multiple: bool, root_keys: &[String]) -> bool {
    if !multiple {
        for other in root_keys.iter().filter(|k| k.as_str() != key) {
            set.remove(other);
        }
    }
    toggle(set, key)
}

/// Раскрыты ли все предки ключа (`parentExpanded`): корневая панель и ветки между ней и пунктом.
pub fn ancestors_expanded(set: &HashSet<String>, key: &str) -> bool {
    let mut end = 0;
    while let Some(pos) = key[end..].find('_') {
        end += pos;
        if !set.contains(&key[..end]) {
            return false;
        }
        end += 1;
    }
    true
}

/// Видимые пункты дерева панели `root_key` в порядке обхода (`flatItems`): потомки, у которых раскрыты все ветки до корня панели.
pub fn panel_visible<'a>(nodes: &'a [Node], items: &[MenuItem], expanded: &HashSet<String>, root_key: &str) -> Vec<&'a Node> {
    let mut shown: HashSet<&str> = HashSet::new();
    let mut out = Vec::new();
    for node in nodes {
        if node.key == root_key || !is_ancestor_or_self(root_key, &node.key) {
            continue;
        }
        if !item(items, node).is_some_and(|i| i.visible) {
            continue;
        }
        let parent_open = node.parent_key == root_key || (shown.contains(node.parent_key.as_str()) && expanded.contains(&node.parent_key));
        if parent_open {
            shown.insert(node.key.as_str());
            out.push(node);
        }
    }
    out
}

/// Результат перехода по клавише внутри дерева панели.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    /// Фокус на пункт с этим ключом.
    Item(String),
    /// Дальше нет пунктов: фокус на следующий заголовок.
    HeaderNext,
    /// Выше нет пунктов: фокус на заголовок этой же панели.
    HeaderSelf,
    /// Ничего не делать.
    Stay,
}

fn first(vis: &[&Node], items: &[MenuItem]) -> Option<String> {
    vis.iter().find(|n| valid(items, n)).map(|n| n.key.clone())
}

fn last(vis: &[&Node], items: &[MenuItem]) -> Option<String> {
    vis.iter().rev().find(|n| valid(items, n)).map(|n| n.key.clone())
}

fn position(vis: &[&Node], key: &str) -> Option<usize> {
    vis.iter().position(|n| n.key == key)
}

/// ↓: следующий допустимый пункт; на последнем — следующий заголовок.
pub fn arrow_down(vis: &[&Node], items: &[MenuItem], focus: Option<&str>) -> Move {
    let Some(cur) = focus.and_then(|f| position(vis, f)) else {
        return first(vis, items).map_or(Move::HeaderNext, Move::Item);
    };
    match vis[cur + 1..].iter().find(|n| valid(items, n)) {
        Some(next) => Move::Item(next.key.clone()),
        None => Move::HeaderNext,
    }
}

/// ↑: предыдущий допустимый пункт; на первом — заголовок этой панели.
pub fn arrow_up(vis: &[&Node], items: &[MenuItem], focus: Option<&str>) -> Move {
    let Some(cur) = focus.and_then(|f| position(vis, f)) else {
        return last(vis, items).map_or(Move::HeaderSelf, Move::Item);
    };
    match vis[..cur].iter().rev().find(|n| valid(items, n)) {
        Some(prev) => Move::Item(prev.key.clone()),
        None => Move::HeaderSelf,
    }
}

/// `Home`.
pub fn home(vis: &[&Node], items: &[MenuItem]) -> Move {
    first(vis, items).map_or(Move::Stay, Move::Item)
}

/// `End`.
pub fn end(vis: &[&Node], items: &[MenuItem]) -> Move {
    last(vis, items).map_or(Move::Stay, Move::Item)
}

/// Результат ←.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Left {
    /// Свернуть ветку с этим ключом.
    Collapse(String),
    /// Фокус на родителя.
    Parent(String),
    /// Ничего.
    Stay,
}

/// ←: раскрытая группа сворачивается, иначе фокус на родителя (если родитель — не заголовок панели).
pub fn arrow_left(nodes: &[Node], expanded: &HashSet<String>, panel_root: &str, focus: Option<&str>) -> Left {
    let Some(node) = focus.and_then(|f| nodes.iter().find(|n| n.key == f)) else { return Left::Stay };
    if expanded.contains(&node.key) {
        return Left::Collapse(node.key.clone());
    }
    if !node.parent_key.is_empty() && node.parent_key != panel_root { Left::Parent(node.parent_key.clone()) } else { Left::Stay }
}

/// Результат →.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Right {
    /// Раскрыть ветку с этим ключом.
    Expand(String),
    /// Ветка уже раскрыта: ведёт себя как ↓.
    Down,
    /// Ничего.
    Stay,
}

/// →: у группы — раскрытие, у раскрытой — переход вниз.
pub fn arrow_right(nodes: &[Node], items: &[MenuItem], expanded: &HashSet<String>, focus: Option<&str>) -> Right {
    let Some(node) = focus.and_then(|f| nodes.iter().find(|n| n.key == f)) else { return Right::Stay };
    if !item(items, node).is_some_and(|i| !i.items.is_empty()) {
        return Right::Stay;
    }
    if expanded.contains(&node.key) { Right::Down } else { Right::Expand(node.key.clone()) }
}

/// Поиск по началу подписи: от текущего пункта (включая его) до конца, затем с начала. Без совпадения и без фокуса — первый допустимый.
pub fn search(vis: &[&Node], items: &[MenuItem], focus: Option<&str>, text: &str) -> Option<String> {
    let needle = text.to_lowercase();
    let matches = |n: &&&Node| {
        valid(items, n) && item(items, n).and_then(|i| i.label.as_deref()).is_some_and(|l| l.to_lowercase().starts_with(&needle))
    };
    let cur = focus.and_then(|f| position(vis, f));
    let found = match cur {
        Some(cur) => vis[cur..].iter().find(matches).or_else(|| vis[..cur].iter().find(matches)),
        None => vis.iter().find(matches),
    };
    match found {
        Some(n) => Some(n.key.clone()),
        None if cur.is_none() => first(vis, items),
        None => None,
    }
}

/// Шаг по заголовкам: следующий (или предыдущий) не отключённый; в конце — переход по кругу на первый (последний).
pub fn header_step(disabled: &[bool], current: usize, forward: bool) -> Option<usize> {
    let ok = |i: &usize| !disabled[*i];
    if forward {
        (current + 1..disabled.len()).find(ok).or_else(|| (0..disabled.len()).find(ok))
    } else {
        (0..current).rev().find(ok).or_else(|| (0..disabled.len()).rev().find(ok))
    }
}

/// Первый и последний не отключённые заголовки.
pub fn header_edge(disabled: &[bool], last: bool) -> Option<usize> {
    let ok = |i: &usize| !disabled[*i];
    if last { (0..disabled.len()).rev().find(ok) } else { (0..disabled.len()).find(ok) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime_tieredmenu::build;

    fn model() -> Vec<MenuItem> {
        vec![
            MenuItem::new().label("Files").items(vec![
                MenuItem::new().label("Docs").items(vec![
                    MenuItem::new().label("Invoices"),
                    MenuItem::new().label("Clients").disabled(true),
                    MenuItem::new().label("Zebra"),
                ]),
                MenuItem::separator(),
                MenuItem::new().label("Images"),
                MenuItem::new().label("Hidden").visible(false),
            ]),
            MenuItem::new().label("Cloud").items(vec![MenuItem::new().label("Upload")]),
            MenuItem::new().label("Leaf"),
        ]
    }

    fn set(keys: &[&str]) -> HashSet<String> {
        keys.iter().map(|k| (*k).to_owned()).collect()
    }

    fn keys(vis: &[&Node]) -> Vec<String> {
        vis.iter().map(|n| n.key.clone()).collect()
    }

    #[test]
    fn seed_and_groups() {
        let mut m = model();
        m[0].expanded = true;
        m[2].expanded = true; // лист не раскрывается
        let nodes = build(&m);
        assert_eq!(seed_expanded(&nodes, &m), set(&["0"]));
        assert_eq!(group_keys(&nodes, &m), vec!["0", "0_0", "1"]);
    }

    #[test]
    fn toggle_rules() {
        let roots = vec!["0".to_owned(), "1".to_owned(), "2".to_owned()];
        let mut s = set(&["0", "0_0"]);
        assert!(toggle_root(&mut s, "1", false, &roots));
        assert_eq!(s, set(&["1", "0_0"]), "одиночный режим снимает остальные корни, вложенные не трогает");
        assert!(toggle_root(&mut s, "0", true, &roots));
        assert_eq!(s, set(&["0", "1", "0_0"]));
        assert!(!toggle_root(&mut s, "1", true, &roots));
        assert!(!s.contains("1"));
    }

    #[test]
    fn ancestors() {
        let s = set(&["0", "0_1"]);
        assert!(ancestors_expanded(&s, "0"));
        assert!(ancestors_expanded(&s, "0_2"));
        assert!(ancestors_expanded(&s, "0_1_3"));
        assert!(!ancestors_expanded(&s, "1_0"));
        assert!(!ancestors_expanded(&s, "0_2_0"));
    }

    #[test]
    fn visible_follows_expansion() {
        let m = model();
        let nodes = build(&m);
        assert_eq!(
            keys(&panel_visible(&nodes, &m, &set(&["0"]), "0")),
            vec!["0_0", "0_1", "0_2"],
            "скрытый пункт пропущен, вложенное свёрнуто"
        );
        assert_eq!(keys(&panel_visible(&nodes, &m, &set(&["0", "0_0"]), "0")), vec!["0_0", "0_0_0", "0_0_1", "0_0_2", "0_1", "0_2"]);
        assert_eq!(
            keys(&panel_visible(&nodes, &m, &set(&["0_0"]), "0")),
            vec!["0_0", "0_0_0", "0_0_1", "0_0_2", "0_1", "0_2"],
            "раскрытие самой панели не учитывается"
        );
        assert_eq!(keys(&panel_visible(&nodes, &m, &set(&["1"]), "1")), vec!["1_0"]);
        assert!(panel_visible(&nodes, &m, &set(&[]), "2").is_empty());
    }

    #[test]
    fn vertical_moves() {
        let m = model();
        let nodes = build(&m);
        let vis = panel_visible(&nodes, &m, &set(&["0", "0_0"]), "0");
        assert_eq!(arrow_down(&vis, &m, Some("0_0")), Move::Item("0_0_0".into()));
        assert_eq!(arrow_down(&vis, &m, Some("0_0_0")), Move::Item("0_0_2".into()), "отключённый пропущен");
        assert_eq!(arrow_down(&vis, &m, Some("0_0_2")), Move::Item("0_2".into()), "разделитель пропущен");
        assert_eq!(arrow_down(&vis, &m, Some("0_2")), Move::HeaderNext);
        assert_eq!(arrow_down(&vis, &m, None), Move::Item("0_0".into()));
        assert_eq!(arrow_up(&vis, &m, Some("0_2")), Move::Item("0_0_2".into()));
        assert_eq!(arrow_up(&vis, &m, Some("0_0")), Move::HeaderSelf);
        assert_eq!(arrow_up(&vis, &m, None), Move::Item("0_2".into()));
        assert_eq!(home(&vis, &m), Move::Item("0_0".into()));
        assert_eq!(end(&vis, &m), Move::Item("0_2".into()));
        assert_eq!(home(&[], &m), Move::Stay);
        assert_eq!(arrow_down(&[], &m, None), Move::HeaderNext);
    }

    #[test]
    fn horizontal_moves() {
        let m = model();
        let nodes = build(&m);
        assert_eq!(arrow_left(&nodes, &set(&["0", "0_0"]), "0", Some("0_0")), Left::Collapse("0_0".into()));
        assert_eq!(arrow_left(&nodes, &set(&["0"]), "0", Some("0_0_1")), Left::Parent("0_0".into()));
        assert_eq!(arrow_left(&nodes, &set(&["0"]), "0", Some("0_2")), Left::Stay, "родитель верхнего уровня — заголовок");
        assert_eq!(arrow_left(&nodes, &set(&[]), "0", None), Left::Stay);
        assert_eq!(arrow_right(&nodes, &m, &set(&["0"]), Some("0_0")), Right::Expand("0_0".into()));
        assert_eq!(arrow_right(&nodes, &m, &set(&["0", "0_0"]), Some("0_0")), Right::Down);
        assert_eq!(arrow_right(&nodes, &m, &set(&["0"]), Some("0_2")), Right::Stay, "у листа → ничего");
    }

    #[test]
    fn search_wraps_and_skips_disabled() {
        let m = model();
        let nodes = build(&m);
        let vis = panel_visible(&nodes, &m, &set(&["0", "0_0"]), "0");
        assert_eq!(search(&vis, &m, Some("0_0"), "i"), Some("0_0_0".into()));
        assert_eq!(search(&vis, &m, Some("0_2"), "d"), Some("0_0".into()), "переход по кругу, регистр не важен");
        assert_eq!(search(&vis, &m, Some("0_0"), "cl"), None, "отключённый не находится");
        assert_eq!(search(&vis, &m, None, "q"), Some("0_0".into()), "без фокуса и совпадения — первый");
        assert_eq!(search(&vis, &m, Some("0_0"), "q"), None);
    }

    #[test]
    fn header_steps() {
        let d = [false, true, false, true];
        assert_eq!(header_step(&d, 0, true), Some(2));
        assert_eq!(header_step(&d, 2, true), Some(0), "по кругу, отключённый в конце пропущен");
        assert_eq!(header_step(&d, 0, false), Some(2));
        assert_eq!(header_step(&d, 2, false), Some(0));
        assert_eq!(header_edge(&d, false), Some(0));
        assert_eq!(header_edge(&d, true), Some(2));
        assert_eq!(header_edge(&[true], false), None);
        assert_eq!(header_step(&[true, true], 0, true), None);
    }
}
