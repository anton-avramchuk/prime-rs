//! Модель MegaMenu и чистая логика переходов (по `megamenu.ts` PrimeNG 21.1.10).
//!
//! Дерево трёхуровневое: корневой пункт (ключ `"0"`) → подменю в колонке (`"0_1_0"`: корень, колонка, индекс) → пункт
//! подменю (`"0_1_0_2"`). Раскрыт может быть один корневой пункт ([`State::active`]); навигация внутри наложения идёт по
//! плоскому списку пунктов всех его подменю, а корневая — по корневым пунктам.

use prime_menu::MenuItem;

/// Ориентация корневого списка.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MegaMenuOrientation {
    /// Корневые пункты в строку, наложение под пунктом.
    #[default]
    Horizontal,
    /// Корневые пункты в столбец, наложение справа от пункта.
    Vertical,
}

/// Подменю в колонке: заголовок и плоский список пунктов.
#[derive(Clone)]
pub struct MegaMenuSubmenu {
    /// Заголовок.
    pub label: Option<String>,
    /// Виден (по умолчанию да).
    pub visible: bool,
    /// Дополнительный класс заголовка.
    pub style_class: Option<String>,
    /// Встроенный стиль заголовка.
    pub style: Option<String>,
    /// Пункты.
    pub items: Vec<MenuItem>,
}

impl Default for MegaMenuSubmenu {
    fn default() -> Self {
        Self { label: None, visible: true, style_class: None, style: None, items: Vec::new() }
    }
}

impl MegaMenuSubmenu {
    /// Подменю с заголовком.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: Some(label.into()), ..Self::default() }
    }

    /// Пункты подменю.
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }

    /// Показывает или скрывает подменю.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Дополнительный класс заголовка.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }
}

/// Корневой пункт: обычный [`MenuItem`] (без `items`) и колонки подменю.
#[derive(Clone, Default)]
pub struct MegaMenuItem {
    /// Свойства пункта (`items` игнорируется).
    pub item: MenuItem,
    /// Колонки: каждая — список подменю.
    pub columns: Vec<Vec<MegaMenuSubmenu>>,
}

impl From<MenuItem> for MegaMenuItem {
    fn from(item: MenuItem) -> Self {
        Self { item, columns: Vec::new() }
    }
}

impl MegaMenuItem {
    /// Пункт без свойств.
    pub fn new() -> Self {
        Self::default()
    }

    /// Подпись.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.item = self.item.label(label);
        self
    }

    /// Значок.
    pub fn icon(mut self, icon: impl Into<leptos::prelude::ViewFn>) -> Self {
        self.item = self.item.icon(icon);
        self
    }

    /// Отключает пункт.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.item = self.item.disabled(disabled);
        self
    }

    /// Показывает или скрывает пункт.
    pub fn visible(mut self, visible: bool) -> Self {
        self.item = self.item.visible(visible);
        self
    }

    /// Команда при нажатии.
    pub fn command(mut self, command: impl Into<leptos::prelude::Callback<prime_menu::MenuCommand>>) -> Self {
        self.item = self.item.command(command);
        self
    }

    /// Колонки подменю.
    pub fn columns(mut self, columns: Vec<Vec<MegaMenuSubmenu>>) -> Self {
        self.columns = columns;
        self
    }
}

/// Вид узла дерева.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Корневой пункт.
    Root,
    /// Подменю в колонке.
    Submenu,
    /// Пункт подменю.
    Item,
}

/// Узел дерева «обработанных пунктов».
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Ключ.
    pub key: String,
    /// Ключ родителя (пусто у корневых).
    pub parent_key: String,
    /// Ключ корневого пункта.
    pub root_key: String,
    /// Вид.
    pub kind: Kind,
    /// Индекс среди соседей.
    pub index: usize,
    /// Номер колонки (у корневых — их индекс).
    pub column: usize,
    /// Корневой пункт с подменю.
    pub group: bool,
    /// Виден (сам пункт и его подменю).
    pub shown: bool,
    /// Допустим для фокуса: виден, не отключён, не разделитель, не заголовок.
    pub valid: bool,
    /// Подпись (для поиска).
    pub label: String,
    /// Номер колонки у пункта — число колонок корня (для класса `col-N`).
    pub columns: usize,
}

/// Строит узлы в порядке обхода в глубину.
pub fn build(model: &[MegaMenuItem]) -> Vec<Node> {
    let mut out = Vec::new();
    for (i, root) in model.iter().enumerate() {
        let key = i.to_string();
        let group = root.columns.iter().any(|c| !c.is_empty());
        out.push(Node {
            key: key.clone(),
            parent_key: String::new(),
            root_key: key.clone(),
            kind: Kind::Root,
            index: i,
            column: i,
            group,
            shown: root.item.visible,
            valid: root.item.visible && !root.item.disabled && !root.item.separator,
            label: root.item.label.clone().unwrap_or_default(),
            columns: root.columns.len(),
        });
        for (c, column) in root.columns.iter().enumerate() {
            for (s, sub) in column.iter().enumerate() {
                let sub_key = format!("{key}_{c}_{s}");
                out.push(Node {
                    key: sub_key.clone(),
                    parent_key: key.clone(),
                    root_key: key.clone(),
                    kind: Kind::Submenu,
                    index: s,
                    column: c,
                    group: false,
                    shown: sub.visible,
                    valid: false,
                    label: sub.label.clone().unwrap_or_default(),
                    columns: root.columns.len(),
                });
                for (j, item) in sub.items.iter().enumerate() {
                    out.push(Node {
                        key: format!("{sub_key}_{j}"),
                        parent_key: sub_key.clone(),
                        root_key: key.clone(),
                        kind: Kind::Item,
                        index: j,
                        column: c,
                        group: false,
                        shown: sub.visible && item.visible,
                        valid: sub.visible && item.visible && !item.disabled && !item.separator,
                        label: item.label.clone().unwrap_or_default(),
                        columns: root.columns.len(),
                    });
                }
            }
        }
    }
    out
}

/// Класс колонки по числу колонок пункта (`p-megamenu-col-N`).
pub fn column_class(columns: usize, mobile: bool) -> &'static str {
    if mobile {
        return "p-megamenu-col-12";
    }
    match columns {
        2 => "p-megamenu-col-6",
        3 => "p-megamenu-col-4",
        4 => "p-megamenu-col-3",
        6 => "p-megamenu-col-2",
        _ => "p-megamenu-col-12",
    }
}

/// Состояние переходов.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Ключ раскрытого корневого пункта.
    pub active: Option<String>,
    /// Ключ пункта под фокусом.
    pub focus: Option<String>,
    /// После клика или нажатия мыши по меню наведение раскрывает группы.
    pub dirty: bool,
}

/// Результат клика.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Click {
    /// Раскрытый пункт свернулся.
    Closed,
    /// Группа раскрылась.
    Opened,
    /// Лист: меню нужно закрыть.
    Leaf,
}

/// Дерево с ориентацией: все переходы.
pub struct Menu<'a> {
    /// Узлы.
    pub nodes: &'a [Node],
    /// Вертикальная ориентация.
    pub vertical: bool,
}

impl Menu<'_> {
    /// Узел по ключу.
    pub fn node(&self, key: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Видимые корневые пункты.
    pub fn roots(&self) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.kind == Kind::Root && n.shown).collect()
    }

    /// Видимые пункты подменю корня (плоский список).
    pub fn items(&self, root: &str) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.kind == Kind::Item && n.shown && n.root_key == root).collect()
    }

    /// Список, по которому ходит фокус.
    pub fn list(&self, st: &State) -> Vec<&Node> {
        match st.focus.as_deref().and_then(|f| self.node(f)) {
            Some(n) if n.kind == Kind::Item => self.items(&n.root_key),
            _ => self.roots(),
        }
    }

    fn first_valid(list: &[&Node]) -> Option<String> {
        list.iter().find(|n| n.valid).map(|n| n.key.clone())
    }

    fn last_valid(list: &[&Node]) -> Option<String> {
        list.iter().rev().find(|n| n.valid).map(|n| n.key.clone())
    }

    /// Первый пункт для фокуса: выбранный корень, иначе первый допустимый.
    pub fn first_focused(&self, st: &State) -> Option<String> {
        let roots = self.roots();
        st.active
            .as_ref()
            .and_then(|a| roots.iter().find(|n| &n.key == a && n.valid).map(|n| n.key.clone()))
            .or_else(|| Self::first_valid(&roots))
    }

    fn step(list: &[&Node], cur: Option<&str>, forward: bool) -> Option<String> {
        let Some(pos) = cur.and_then(|c| list.iter().position(|n| n.key == c)) else {
            return if forward { Self::first_valid(list) } else { Self::last_valid(list) };
        };
        let found = if forward { list[pos + 1..].iter().find(|n| n.valid) } else { list[..pos].iter().rev().find(|n| n.valid) };
        Some(found.map_or_else(|| list[pos].key.clone(), |n| n.key.clone()))
    }

    /// Фокус вошёл снаружи и меню в покое: первый пункт.
    pub fn focus_in(&self, st: &mut State) {
        if st.focus.is_none() && st.active.is_none() && !st.dirty {
            st.focus = self.first_focused(st);
        }
    }

    /// Закрывает всё.
    pub fn hide(&self, st: &mut State) {
        *st = State::default();
    }

    fn move_root(&self, st: &mut State, target: &str) {
        st.focus = Some(target.to_owned());
        if st.active.is_some() {
            st.active = self.node(target).filter(|n| n.group).map(|n| n.key.clone());
        }
    }

    fn back_to_root(&self, st: &mut State) {
        if let Some(root) = st.focus.as_deref().and_then(|f| self.node(f)).map(|n| n.root_key.clone()) {
            st.focus = Some(root);
            st.active = None;
        }
    }

    fn open_and_enter(&self, st: &mut State, root: &str) {
        st.active = Some(root.to_owned());
        st.dirty = true;
        if let Some(first) = Self::first_valid(&self.items(root)) {
            st.focus = Some(first);
        }
    }

    fn column_first(&self, root: &str, column: usize) -> Option<String> {
        self.items(root).into_iter().find(|n| n.column == column && n.valid).map(|n| n.key.clone())
    }

    /// Клик по пункту (после проверки отключённости и вызова команды).
    pub fn click(&self, st: &mut State, key: &str) -> Click {
        let Some(node) = self.node(key) else { return Click::Leaf };
        if st.active.as_deref() == Some(key) {
            st.active = None;
            st.focus = Some(key.to_owned());
            st.dirty = node.kind != Kind::Root;
            Click::Closed
        } else if node.group {
            st.active = Some(key.to_owned());
            st.focus = Some(key.to_owned());
            st.dirty = true;
            Click::Opened
        } else {
            Click::Leaf
        }
    }

    /// Наведение на пункт.
    pub fn hover(&self, st: &mut State, key: &str, mobile: bool) {
        if mobile || !st.dirty {
            return;
        }
        let Some(node) = self.node(key) else { return };
        if node.group {
            st.active = Some(key.to_owned());
        } else if node.kind == Kind::Root {
            st.active = None;
        }
        st.focus = Some(key.to_owned());
    }

    /// ↓.
    pub fn arrow_down(&self, st: &mut State) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else {
            st.focus = self.first_focused(st);
            return;
        };
        if node.kind == Kind::Item {
            st.focus = Self::step(&self.items(&node.root_key), Some(&node.key), true);
        } else if !self.vertical && node.group {
            let key = node.key.clone();
            self.open_and_enter(st, &key);
        } else if let Some(next) = Self::step(&self.roots(), Some(&node.key), true) {
            self.move_root(st, &next);
        }
    }

    /// ↑ (`alt` в горизонтальной ориентации возвращает из наложения на корневой пункт).
    pub fn arrow_up(&self, st: &mut State, alt: bool) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else {
            st.focus = Self::last_valid(&self.roots());
            return;
        };
        if alt && !self.vertical {
            if node.kind == Kind::Item {
                self.back_to_root(st);
            }
        } else if node.kind == Kind::Item {
            st.focus = Self::step(&self.items(&node.root_key), Some(&node.key), false);
        } else if let Some(prev) = Self::step(&self.roots(), Some(&node.key), false) {
            self.move_root(st, &prev);
        }
    }

    /// →.
    pub fn arrow_right(&self, st: &mut State) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else { return };
        if node.kind == Kind::Item {
            if let Some(next) = self.column_first(&node.root_key, node.column + 1) {
                st.focus = Some(next);
            }
        } else if self.vertical && node.group {
            let key = node.key.clone();
            self.open_and_enter(st, &key);
        } else if let Some(next) = Self::step(&self.roots(), Some(&node.key), true) {
            self.move_root(st, &next);
        }
    }

    /// ←.
    pub fn arrow_left(&self, st: &mut State) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else { return };
        if node.kind == Kind::Item {
            if self.vertical && node.column == 0 {
                self.back_to_root(st);
            } else if node.column > 0
                && let Some(prev) = self.column_first(&node.root_key, node.column - 1)
            {
                st.focus = Some(prev);
            }
        } else if !self.vertical
            && let Some(prev) = Self::step(&self.roots(), Some(&node.key), false)
        {
            self.move_root(st, &prev);
        }
    }

    /// `Home`.
    pub fn home(&self, st: &mut State) {
        if let Some(first) = Self::first_valid(&self.list(st)) {
            st.focus = Some(first);
        }
    }

    /// `End`.
    pub fn end(&self, st: &mut State) {
        if let Some(last) = Self::last_valid(&self.list(st)) {
            st.focus = Some(last);
        }
    }

    /// `Escape`: закрывает наложение, фокус на его корневом пункте.
    pub fn escape(&self, st: &mut State) {
        if let Some(active) = st.active.take() {
            st.focus = Some(active);
        }
    }

    /// `Tab`: закрывает всё.
    pub fn tab(&self, st: &mut State) {
        self.hide(st);
    }

    /// После `Enter`/`Space` на листе фокус возвращается на первый корневой пункт.
    pub fn after_enter(&self, st: &mut State, was_group: bool) {
        if !was_group {
            st.focus = self.first_focused(st);
        }
    }

    /// Поиск по началу подписи: от текущего пункта (включая его) по кругу.
    pub fn search(&self, st: &mut State, text: &str) {
        let needle = text.to_lowercase();
        let list = self.list(st);
        let matches = |n: &&&Node| n.valid && n.label.to_lowercase().starts_with(&needle);
        let cur = st.focus.as_deref().and_then(|f| list.iter().position(|n| n.key == f));
        let found = match cur {
            Some(cur) => list[cur..].iter().find(matches).or_else(|| list[..cur].iter().find(matches)),
            None => list.iter().find(matches),
        };
        match found {
            Some(n) => st.focus = Some(n.key.clone()),
            None if cur.is_none() => st.focus = self.first_focused(st),
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sub(name: &str, items: &[&str]) -> MegaMenuSubmenu {
        MegaMenuSubmenu::new(name).items(items.iter().map(|l| MenuItem::new().label(*l)).collect())
    }

    /// 0 Furniture{ col0:[Living{a,b}], col1:[Kitchen{c,d(off)}, Bath{e}], col2:[Bed{f}] }, 1 Plain, 2 Sep(off), 3 Tech{col0:[Phones{g,h}]}
    fn model() -> Vec<MegaMenuItem> {
        let mut kitchen = sub("Kitchen", &["Chair", "Dead"]);
        kitchen.items[1] = kitchen.items[1].clone().disabled(true);
        vec![
            MegaMenuItem::new().label("Furniture").columns(vec![
                vec![sub("Living", &["Armchair", "Bed"])],
                vec![kitchen, sub("Bath", &["Soap"])],
                vec![sub("Bedroom", &["Cupboard"])],
            ]),
            MegaMenuItem::new().label("Plain"),
            MegaMenuItem::new().label("Sep").disabled(true),
            MegaMenuItem::new().label("Tech").columns(vec![vec![sub("Phones", &["Android", "iPhone"])]]),
        ]
    }

    fn st(active: Option<&str>, focus: Option<&str>) -> State {
        State { active: active.map(Into::into), focus: focus.map(Into::into), dirty: true }
    }

    #[test]
    fn keys_and_flags() {
        let nodes = build(&model());
        let keys: Vec<&str> = nodes.iter().map(|n| n.key.as_str()).collect();
        assert_eq!(&keys[..8], ["0", "0_0_0", "0_0_0_0", "0_0_0_1", "0_1_0", "0_1_0_0", "0_1_0_1", "0_1_1"]);
        let get = |k: &str| nodes.iter().find(|n| n.key == k).unwrap();
        assert!(get("0").group && !get("1").group && !get("2").valid && get("1").valid);
        assert_eq!((get("0_1_1_0").column, get("0_2_0_0").column, get("0").columns), (1, 2, 3));
        assert!(!get("0_1_0_1").valid && !get("0_1_0").valid, "отключённый и заголовок недопустимы");
        assert_eq!(get("0_1_0_0").root_key, "0");
    }

    #[test]
    fn column_classes() {
        assert_eq!(
            [2, 3, 4, 6, 1, 5].map(|n| column_class(n, false)),
            ["p-megamenu-col-6", "p-megamenu-col-4", "p-megamenu-col-3", "p-megamenu-col-2", "p-megamenu-col-12", "p-megamenu-col-12"]
        );
        assert_eq!(column_class(3, true), "p-megamenu-col-12");
    }

    #[test]
    fn click_opens_closes_and_leaves() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: false };
        let mut s = State::default();
        assert_eq!(m.click(&mut s, "0"), Click::Opened);
        assert_eq!((s.active.as_deref(), s.focus.as_deref(), s.dirty), (Some("0"), Some("0"), true));
        assert_eq!(m.click(&mut s, "0"), Click::Closed);
        assert_eq!((s.active.as_deref(), s.dirty), (None, false), "повторный клик по корню закрывает");
        assert_eq!(m.click(&mut s, "1"), Click::Leaf);
        assert_eq!(m.click(&mut s, "0_0_0_0"), Click::Leaf);
    }

    #[test]
    fn hover_needs_dirty_and_follows_groups() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: false };
        let mut s = State::default();
        m.hover(&mut s, "0", false);
        assert_eq!(s, State::default(), "без клика наведение ничего не делает");
        let mut s = State { dirty: true, ..State::default() };
        m.hover(&mut s, "0", true);
        assert_eq!(s.active, None, "на узком окне путь не меняется");
        m.hover(&mut s, "0", false);
        assert_eq!(s.active.as_deref(), Some("0"));
        m.hover(&mut s, "0_0_0_1", false);
        assert_eq!(
            (s.active.as_deref(), s.focus.as_deref()),
            (Some("0"), Some("0_0_0_1")),
            "наведение на пункт наложения держит его открытым"
        );
        m.hover(&mut s, "1", false);
        assert_eq!((s.active, s.focus.as_deref()), (None, Some("1")), "наведение на обычный корень закрывает наложение");
    }

    #[test]
    fn horizontal_root_and_overlay_navigation() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: false };
        let mut s = State::default();
        m.focus_in(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0"));
        m.arrow_down(&mut s);
        assert_eq!((s.active.as_deref(), s.focus.as_deref()), (Some("0"), Some("0_0_0_0")), "↓ раскрывает и входит в первую колонку");
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_0_0_1"));
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_1_0_0"), "плоский список идёт в следующую колонку");
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_1_1_0"), "отключённый пропущен");
        m.arrow_right(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_2_0_0"), "→ — первый пункт следующей колонки");
        m.arrow_right(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_2_0_0"), "дальше колонок нет");
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_1_0_0"));
        m.arrow_left(&mut s);
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_0_0_0"), "← у первой колонки остаётся");
        m.arrow_up(&mut s, false);
        assert_eq!(s.focus.as_deref(), Some("0_0_0_0"), "↑ на первом пункте не выходит");
        m.end(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_2_0_0"));
        m.home(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_0_0_0"));
        m.arrow_up(&mut s, true);
        assert_eq!((s.active, s.focus.as_deref()), (None, Some("0")), "Alt+↑ возвращает на корень");
    }

    #[test]
    fn horizontal_roots_follow_the_overlay() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: false };
        let mut s = st(Some("0"), Some("0"));
        m.arrow_right(&mut s);
        assert_eq!((s.active.clone(), s.focus.as_deref()), (None, Some("1")), "→ на обычный корень закрывает наложение");
        m.arrow_right(&mut s);
        assert_eq!(s.focus.as_deref(), Some("3"), "отключённый корень пропущен");
        m.arrow_left(&mut s);
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0"));
        let mut s = st(Some("3"), Some("3"));
        m.arrow_left(&mut s);
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0"));
        let mut s = st(Some("0"), Some("0"));
        m.arrow_down(&mut s);
        m.escape(&mut s);
        assert_eq!((s.active, s.focus.as_deref()), (None, Some("0")), "Escape возвращает на корень");
        let mut s = st(None, Some("1"));
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("3"), "↓ на обычном корне идёт к следующему (как в PrimeNG)");
    }

    #[test]
    fn vertical_navigation() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: true };
        let mut s = State::default();
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0"));
        m.arrow_down(&mut s);
        assert_eq!(s.focus.as_deref(), Some("1"), "в вертикальной ориентации ↓ идёт по корню");
        m.arrow_up(&mut s, false);
        assert_eq!(s.focus.as_deref(), Some("0"));
        m.arrow_right(&mut s);
        assert_eq!((s.active.as_deref(), s.focus.as_deref()), (Some("0"), Some("0_0_0_0")), "→ раскрывает и входит");
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0"), "← из первой колонки — на корень");
        assert_eq!(s.active, None);
        let mut s = st(Some("0"), Some("0_1_0_0"));
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("0_0_0_0"), "← на колонку левее");
        m.arrow_left(&mut s);
        assert_eq!((s.active, s.focus.as_deref()), (None, Some("0")));
        let mut s = st(None, Some("1"));
        m.arrow_left(&mut s);
        assert_eq!(s.focus.as_deref(), Some("1"));
    }

    #[test]
    fn search_tab_and_after_enter() {
        let nodes = build(&model());
        let m = Menu { nodes: &nodes, vertical: false };
        let mut s = st(None, Some("0"));
        m.search(&mut s, "t");
        assert_eq!(s.focus.as_deref(), Some("3"), "поиск по корню");
        m.search(&mut s, "z");
        assert_eq!(s.focus.as_deref(), Some("3"), "без совпадений фокус остаётся");
        let mut s = st(Some("0"), Some("0_0_0_0"));
        m.search(&mut s, "cu");
        assert_eq!(s.focus.as_deref(), Some("0_2_0_0"), "поиск по пунктам наложения");
        m.search(&mut s, "ar");
        assert_eq!(s.focus.as_deref(), Some("0_0_0_0"), "по кругу");
        m.tab(&mut s);
        assert_eq!(s, State::default());
        let mut s = st(None, None);
        m.after_enter(&mut s, false);
        assert_eq!(s.focus.as_deref(), Some("0"));
        let mut s = st(None, Some("1"));
        m.after_enter(&mut s, true);
        assert_eq!(s.focus.as_deref(), Some("1"));
    }
}
