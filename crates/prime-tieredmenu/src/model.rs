//! Чистая модель TieredMenu: дерево узлов, путь активных пунктов, фокус и переходы (по `tieredmenu.ts` PrimeNG).
//!
//! Пункты — `prime_menu::MenuItem` (поле `items` задаёт подменю). Узел дерева — [`Node`] с ключом вида `"0"`, `"0_1"`,
//! `"0_1_2"`; путь активных пунктов хранит ключи раскрытых групп от корня вглубь.

use prime_menu::MenuItem;

/// Узел дерева «обработанных пунктов» (`createProcessedItems`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Ключ: индексы предков и собственный через `_`.
    pub key: String,
    /// Ключ родителя (пустая строка у корневых).
    pub parent_key: String,
    /// Глубина (корневые — `0`).
    pub level: usize,
    /// Индекс среди соседей.
    pub index: usize,
    /// Индексы от корня до пункта.
    pub path: Vec<usize>,
}

/// Строит узлы в порядке обхода в глубину.
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

/// Пункт по пути индексов.
pub fn item_at<'a>(items: &'a [MenuItem], path: &[usize]) -> Option<&'a MenuItem> {
    let (first, rest) = path.split_first()?;
    let mut item = items.get(*first)?;
    for i in rest {
        item = item.items.get(*i)?;
    }
    Some(item)
}

/// Допустимый пункт: виден, не отключён и не разделитель (`isValidItem`).
pub fn is_valid(item: &MenuItem) -> bool {
    item.visible && !item.disabled && !item.separator
}

/// Идентификатор элемента `li`: собственный `id` пункта или `<id меню>_<ключ>`.
pub fn item_id(menu_id: &str, item: &MenuItem, key: &str) -> String {
    match item.id.as_deref() {
        Some(id) if !id.is_empty() => id.to_owned(),
        _ => format!("{menu_id}_{key}"),
    }
}

/// Является ли `ancestor` предком `key` (или им самим): сравнение по границам ключа, а не по строке (`"1"` не предок `"10"`).
pub fn is_ancestor_or_self(ancestor: &str, key: &str) -> bool {
    key == ancestor || key.strip_prefix(ancestor).is_some_and(|rest| rest.starts_with('_'))
}

/// Фокус: индекс среди видимых пунктов текущего уровня и ключ родителя этого уровня.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Focus {
    /// Индекс пункта на уровне.
    pub index: Option<usize>,
    /// Ключ родителя уровня (пусто — корень).
    pub parent_key: String,
}

/// Результат клика по пункту.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Click {
    /// Ветка свёрнута, фокус остался на пункте.
    Collapsed,
    /// Группа раскрыта.
    Expanded,
    /// Лист: всё закрыто; фокус переходит на корневой пункт ветки (`Some(индекс)`).
    Leaf(Option<usize>),
}

/// Состояние дерева: путь активных групп, фокус и признак «раскрывали кликом».
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Ключи раскрытых групп от корня вглубь.
    pub active: Vec<String>,
    /// Фокус.
    pub focus: Focus,
    /// Группу раскрывали кликом или стрелкой (включает раскрытие при наведении).
    pub dirty: bool,
}

/// Дерево с пунктами: всё, что нужно переходам.
pub struct Tree<'a> {
    /// Узлы.
    pub nodes: &'a [Node],
    /// Корневые пункты модели.
    pub items: &'a [MenuItem],
}

impl<'a> Tree<'a> {
    /// Узел по ключу.
    pub fn node(&self, key: &str) -> Option<&'a Node> {
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Пункт узла.
    pub fn item(&self, node: &Node) -> &'a MenuItem {
        item_at(self.items, &node.path).expect("узел указывает на существующий пункт")
    }

    /// Группа ли узел (есть подпункты).
    pub fn is_group(&self, node: &Node) -> bool {
        !self.item(node).items.is_empty()
    }

    /// Соседи с общим родителем.
    pub fn siblings(&self, parent_key: &str) -> Vec<&'a Node> {
        self.nodes.iter().filter(|n| n.parent_key == parent_key).collect()
    }

    /// Пункты текущего уровня фокуса (`visibleItems`): дети активной группы с ключом фокуса, иначе корневые.
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

    /// Первый допустимый индекс (`findFirstItemIndex`).
    pub fn first(&self, visible: &[&Node]) -> Option<usize> {
        visible.iter().position(|n| self.valid(n))
    }

    /// Последний допустимый индекс (`findLastItemIndex`).
    pub fn last(&self, visible: &[&Node]) -> Option<usize> {
        visible.iter().rposition(|n| self.valid(n))
    }

    /// Следующий допустимый индекс; без него остаётся `index` (`findNextItemIndex`).
    pub fn next(&self, visible: &[&Node], index: usize) -> usize {
        visible.iter().enumerate().skip(index + 1).find(|(_, n)| self.valid(n)).map_or(index, |(i, _)| i)
    }

    /// Предыдущий допустимый индекс; без него остаётся `index` (`findPrevItemIndex`).
    pub fn prev(&self, visible: &[&Node], index: usize) -> usize {
        visible.iter().enumerate().take(index).rfind(|(_, n)| self.valid(n)).map_or(index, |(i, _)| i)
    }

    /// Индекс раскрытого допустимого пункта уровня (`findSelectedItemIndex`).
    pub fn selected_index(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        visible.iter().position(|n| self.valid(n) && self.selected(state, n))
    }

    /// Первый пункт для фокуса: раскрытый, а без него первый допустимый (`findFirstFocusedItemIndex`).
    pub fn first_focused(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        self.selected_index(state, visible).or_else(|| self.first(visible))
    }

    /// Последний пункт для фокуса: раскрытый, а без него последний допустимый (`findLastFocusedItemIndex`).
    pub fn last_focused(&self, state: &State, visible: &[&Node]) -> Option<usize> {
        self.selected_index(state, visible).or_else(|| self.last(visible))
    }

    fn matches(&self, node: &Node, search: &str) -> bool {
        let item = self.item(node);
        is_valid(item) && item.label.as_deref().unwrap_or_default().to_lowercase().starts_with(&search.to_lowercase())
    }

    /// Поиск по началу подписи среди пунктов уровня (`searchItems`): от текущего пункта по кругу. Возвращает индекс, найден
    /// ли пункт.
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

    /// Ставит фокус на индекс уровня (`changeFocusedItemIndex`).
    pub fn focus_index(&self, state: &mut State, index: Option<usize>) {
        if index.is_some() && state.focus.index != index {
            state.focus.index = index;
        }
    }

    /// Закрывает всё: путь, фокус и признак (`hide`).
    pub fn hide(&self, state: &mut State) {
        state.active.clear();
        state.focus = Focus::default();
        state.dirty = false;
    }

    /// Смена пути при раскрытии пункта (`onItemChange`); при наведении на узком окне путь не меняется.
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

    /// Клик по пункту (`onItemClick`).
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

    /// Наведение на пункт: меняет путь, только если уже раскрывали кликом (`onItemMouseEnter`); на касании — всегда.
    pub fn hover(&self, state: &mut State, node: &Node, touch: bool, mobile: bool) {
        if touch || state.dirty {
            self.item_change(state, node, true, mobile);
        }
    }

    /// ↓: следующий допустимый пункт (первый или раскрытый при пустом фокусе).
    pub fn arrow_down(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = match state.focus.index {
            Some(i) => Some(self.next(&visible, i)),
            None => self.first_focused(state, &visible),
        };
        self.focus_index(state, target);
    }

    /// ↑: предыдущий допустимый пункт (последний или раскрытый при пустом фокусе).
    pub fn arrow_up(&self, state: &mut State) {
        let visible = self.visible(state);
        let target = match state.focus.index {
            Some(i) => Some(self.prev(&visible, i)),
            None => self.last_focused(state, &visible),
        };
        self.focus_index(state, target);
    }

    /// →: раскрывает группу под фокусом и переходит внутрь.
    pub fn arrow_right(&self, state: &mut State) {
        let visible = self.visible(state);
        let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied()) else { return };
        if self.is_group(node) {
            self.item_change(state, node, false, false);
            state.focus = Focus { index: None, parent_key: node.key.clone() };
            self.arrow_down(state);
        }
    }

    /// ←: поднимается к родителю и сворачивает его подменю.
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

    /// `Escape`: закрывает всё, фокус — на первый допустимый корневой пункт.
    pub fn escape(&self, state: &mut State) {
        self.hide(state);
        let visible = self.visible(state);
        state.focus.index = self.first_focused(state, &visible);
    }

    /// `Tab`: лист под фокусом активируется (меняет фокус), затем всё закрывается.
    pub fn tab(&self, state: &mut State, mobile: bool) {
        let visible = self.visible(state);
        if let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied())
            && !self.is_group(node)
        {
            self.item_change(state, node, false, mobile);
        }
        self.hide(state);
    }

    /// После `Enter` вне popup на листе фокус возвращается к первому пункту (`onEnterKey`).
    pub fn after_enter(&self, state: &mut State) {
        let visible = self.visible(state);
        if let Some(node) = state.focus.index.and_then(|i| visible.get(i).copied())
            && !self.is_group(node)
        {
            state.focus.index = self.first_focused(state, &visible);
        }
    }

    /// Идентификатор пункта под фокусом (`focusedItemId`).
    pub fn focused_id(&self, state: &State, menu_id: &str) -> Option<String> {
        let index = state.focus.index?;
        let node = self.siblings(&state.focus.parent_key).get(index).copied()?;
        Some(item_id(menu_id, self.item(node), &node.key))
    }
}

/// `aria-setsize`: число видимых пунктов без разделителей.
pub fn aria_set_size(siblings: &[&MenuItem]) -> usize {
    siblings.iter().filter(|i| i.visible && !i.separator).count()
}

/// `aria-posinset`: номер пункта среди видимых без разделителей (с единицы).
pub fn aria_pos_inset(siblings: &[&MenuItem], index: usize) -> usize {
    let skipped = siblings.iter().take(index).filter(|i| !i.visible || (i.visible && i.separator)).count();
    index - skipped + 1
}

/// Сторона подменю (`nestedPosition`): `"100%"` — справа от пункта, `"-100%"` — слева.
pub fn submenu_side(parent_left: f64, item_width: f64, sub_width: f64, viewport_width: f64, scrollbar: f64, level: usize) -> &'static str {
    let left = parent_left.trunc();
    if left + item_width + sub_width > viewport_width - scrollbar {
        if left < sub_width {
            // слишком узкое окно
            if level % 2 == 1 { "100%" } else { "-100%" }
        } else {
            "-100%"
        }
    } else {
        "100%"
    }
}

/// Печатаемый символ для поиска: один символ без Ctrl/Meta.
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
        assert!(!is_ancestor_or_self("1", "10"), "по границе ключа, а не по строке");
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
        assert_eq!(tree.next(&root, 2), 4, "разделитель пропущен");
        assert_eq!(tree.next(&root, 4), 4, "на границе остаётся");
        assert_eq!(tree.prev(&root, 4), 2);
        assert_eq!(tree.prev(&root, 0), 0);
        let file = tree.siblings("0");
        assert_eq!(keys(&file), ["0_0", "0_1", "0_2", "0_3"]);
        assert_eq!(tree.last(&file), Some(1), "Print отключён, разделитель пропущен");
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
        assert_eq!(tree.visible(&st).len(), 5, "родитель не раскрыт — корневой уровень");
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
        // повторный клик по File сворачивает ветку до него
        assert_eq!(tree.click(&mut st, file), Click::Collapsed);
        assert_eq!(st.active, Vec::<String>::new(), "самого пункта в пути нет, предков нет");
        // раскрыть заново и кликнуть по New дважды: свернётся New, File останется
        tree.click(&mut st, file);
        tree.click(&mut st, new);
        assert_eq!(tree.click(&mut st, new), Click::Collapsed);
        assert_eq!(st.active, ["0"]);
        // лист глубоко внутри закрывает всё и ведёт фокус на корневой пункт ветки
        tree.click(&mut st, new);
        let share = tree.node("0_0_0").unwrap();
        assert_eq!(tree.click(&mut st, share), Click::Leaf(Some(0)));
        assert_eq!((st.active.clone(), st.dirty), (Vec::<String>::new(), false));
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() });
        // корневой лист
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
        assert_eq!(st.active, ["1"], "File закрыт, Edit открыт");
    }

    #[test]
    fn hover_needs_dirty_unless_touch_and_ignores_mobile() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        let edit = tree.node("1").unwrap();
        tree.hover(&mut st, edit, false, false);
        assert!(st.active.is_empty(), "до клика наведение ничего не раскрывает");
        tree.click(&mut st, tree.node("0").unwrap());
        tree.hover(&mut st, edit, false, false);
        assert_eq!(st.active, ["1"]);
        let mut touch = State::default();
        tree.hover(&mut touch, edit, true, false);
        assert_eq!(touch.active, ["1"], "на касании раскрывает сразу");
        let mut mobile = State { dirty: true, ..State::default() };
        tree.hover(&mut mobile, edit, false, true);
        assert!(mobile.active.is_empty(), "на узком окне путь не меняется");
        assert_eq!(mobile.focus.index, Some(1), "но фокус переходит на пункт");
    }

    #[test]
    fn arrows_walk_levels() {
        let items = menu();
        let nodes = build(&items);
        let tree = Tree { nodes: &nodes, items: &items };
        let mut st = State::default();
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(0), "первый допустимый");
        tree.arrow_down(&mut st);
        tree.arrow_down(&mut st);
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(4), "Users → Quit, разделитель пропущен");
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(4));
        tree.arrow_up(&mut st);
        assert_eq!(st.focus.index, Some(2));
        // → на листе ничего не делает
        tree.arrow_right(&mut st);
        assert!(st.active.is_empty());
        // → на группе
        tree.home(&mut st);
        tree.arrow_right(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() }, "первый пункт подменю");
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(1));
        tree.arrow_down(&mut st);
        assert_eq!(st.focus.index, Some(1), "Print отключён: остаёмся на Open");
        tree.arrow_up(&mut st);
        assert_eq!(st.focus.index, Some(0), "New");
        tree.arrow_right(&mut st);
        assert_eq!(st.active, ["0", "0_0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0_0".into() });
        // ← поднимается на New и сворачивает его подменю
        tree.arrow_left(&mut st);
        assert_eq!(st.active, ["0"]);
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: "0".into() }, "фокус на New (раскрытый пункт уровня)");
        // ← ещё раз поднимается на File и сворачивает его
        tree.arrow_left(&mut st);
        assert_eq!(st.active, Vec::<String>::new());
        assert_eq!(st.focus, Focus { index: Some(0), parent_key: String::new() });
        // ← на корне ничего не меняет в фокусе
        tree.arrow_left(&mut st);
        assert_eq!(st.focus.index, Some(0));
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
        // Tab на листе закрывает всё
        tree.end(&mut st);
        tree.tab(&mut st, false);
        assert_eq!(st.focus, Focus::default());
        // Enter на листе вне popup возвращает фокус к первому пункту
        tree.end(&mut st);
        tree.after_enter(&mut st);
        assert_eq!(st.focus.index, Some(0));
        // Enter на группе фокус не трогает
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
        assert_eq!(tree.search(&st, &root, "Q"), (Some(4), true), "без учёта регистра");
        assert_eq!(tree.search(&st, &root, "z"), (Some(0), false), "не найдено и фокуса нет: первый пункт");
        let at_edit = State { focus: Focus { index: Some(1), parent_key: String::new() }, ..State::default() };
        assert_eq!(tree.search(&at_edit, &root, "u"), (Some(2), true), "Users после Edit");
        let at_quit = State { focus: Focus { index: Some(4), parent_key: String::new() }, ..State::default() };
        assert_eq!(tree.search(&at_quit, &root, "f"), (Some(0), true), "по кругу к началу");
        assert_eq!(tree.search(&at_quit, &root, "z"), (None, false), "есть фокус, но не найдено: ничего не меняется");
        // отключённые пункты не находятся
        let file = State { active: vec!["0".into()], focus: Focus { index: None, parent_key: "0".into() }, dirty: true };
        let sub = tree.visible(&file);
        assert_eq!(tree.search(&file, &sub, "p"), (Some(0), false), "Print отключён: найденным не считается, фокус на раскрытом/первом");
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
        // (левый край пункта, ширина пункта, ширина подменю, окно, полоса, уровень) → сторона
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
