//! Модель CascadeSelect и чистая логика переходов (по `cascadeselect.ts` PrimeNG 21.1.10).
//!
//! Варианты образуют дерево: ключи `"0"`, `"0_1"`, `"0_1_2"` (индексы предков и собственный). Путь активных групп
//! ([`State::active`]) хранит раскрытые группы от корня вглубь; фокус — ключ варианта, а список уровня фокуса — его соседи.

/// Вариант: лист со значением либо группа с вложенными вариантами.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CascadeOption {
    /// Подпись.
    pub label: String,
    /// Значение листа (по умолчанию — подпись).
    pub value: Option<String>,
    /// Недоступен.
    pub disabled: bool,
    /// Вложенные варианты (группа).
    pub children: Vec<CascadeOption>,
}

impl CascadeOption {
    /// Вариант с подписью.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), ..Self::default() }
    }

    /// Значение листа.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Недоступность.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Вложенные варианты.
    pub fn children(mut self, children: Vec<CascadeOption>) -> Self {
        self.children = children;
        self
    }
}

/// Узел дерева «обработанных вариантов».
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Ключ.
    pub key: String,
    /// Ключ родителя (пусто у корневых).
    pub parent_key: String,
    /// Глубина (корневые — `0`).
    pub level: usize,
    /// Индекс среди соседей.
    pub index: usize,
    /// Число соседей.
    pub siblings: usize,
    /// Группа (есть вложенные варианты).
    pub group: bool,
    /// Допустим для фокуса.
    pub valid: bool,
    /// Подпись.
    pub label: String,
    /// Значение (подпись, если не задано).
    pub value: String,
}

/// Строит узлы в порядке обхода в глубину.
pub fn build(options: &[CascadeOption]) -> Vec<Node> {
    fn walk(options: &[CascadeOption], level: usize, parent: &str, out: &mut Vec<Node>) {
        for (index, o) in options.iter().enumerate() {
            let key = if parent.is_empty() { index.to_string() } else { format!("{parent}_{index}") };
            out.push(Node {
                key: key.clone(),
                parent_key: parent.to_owned(),
                level,
                index,
                siblings: options.len(),
                group: !o.children.is_empty(),
                valid: !o.disabled,
                label: o.label.clone(),
                value: o.value.clone().unwrap_or_else(|| o.label.clone()),
            });
            walk(&o.children, level + 1, &key, out);
        }
    }
    let mut out = Vec::new();
    walk(options, 0, "", &mut out);
    out
}

/// Является ли `ancestor` предком `key` (или им самим): сравнение по границам ключа.
pub fn is_ancestor_or_self(ancestor: &str, key: &str) -> bool {
    key == ancestor || key.strip_prefix(ancestor).is_some_and(|rest| rest.starts_with('_'))
}

/// Состояние переходов.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Раскрытые группы от корня вглубь.
    pub active: Vec<String>,
    /// Вариант под фокусом.
    pub focus: Option<String>,
    /// После клика по группе наведение раскрывает любые группы.
    pub dirty: bool,
}

/// Результат клика по варианту.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Click {
    /// Раскрытая группа свернулась.
    Closed,
    /// Группа раскрылась.
    Group,
    /// Выбран лист с этим значением.
    Select(String),
}

/// Дерево вариантов с операциями.
pub struct Menu<'a> {
    /// Узлы.
    pub nodes: &'a [Node],
}

impl Menu<'_> {
    /// Узел по ключу.
    pub fn node(&self, key: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Дети узла с ключом `parent` (корневые при пустом ключе).
    pub fn children(&self, parent: &str) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.parent_key == parent).collect()
    }

    /// Список уровня фокуса: соседи варианта под фокусом либо корневые.
    pub fn visible(&self, st: &State) -> Vec<&Node> {
        match st.focus.as_deref().and_then(|f| self.node(f)) {
            Some(n) => self.children(&n.parent_key),
            None => self.children(""),
        }
    }

    /// Ключи пути от корня до листа со значением `value` (пусто, если такого нет).
    pub fn path_to_value(&self, value: &str) -> Vec<String> {
        let Some(leaf) = self.nodes.iter().find(|n| !n.group && n.value == value) else { return Vec::new() };
        self.nodes.iter().filter(|n| is_ancestor_or_self(&n.key, &leaf.key)).map(|n| n.key.clone()).collect()
    }

    /// Подпись листа со значением `value`.
    pub fn label_of(&self, value: &str) -> Option<&str> {
        self.nodes.iter().find(|n| !n.group && n.value == value).map(|n| n.label.as_str())
    }

    fn is_active(st: &State, key: &str) -> bool {
        st.active.iter().any(|k| k == key)
    }

    fn first_valid(list: &[&Node]) -> Option<String> {
        list.iter().find(|n| n.valid).map(|n| n.key.clone())
    }

    fn last_valid(list: &[&Node]) -> Option<String> {
        list.iter().rev().find(|n| n.valid).map(|n| n.key.clone())
    }

    /// Выбранный (находящийся в пути) допустимый вариант списка, иначе первый допустимый.
    fn first_focused(list: &[&Node], st: &State) -> Option<String> {
        list.iter().find(|n| n.valid && Self::is_active(st, &n.key)).map(|n| n.key.clone()).or_else(|| Self::first_valid(list))
    }

    fn last_focused(list: &[&Node], st: &State) -> Option<String> {
        list.iter().find(|n| n.valid && Self::is_active(st, &n.key)).map(|n| n.key.clone()).or_else(|| Self::last_valid(list))
    }

    fn step(list: &[&Node], cur: &str, forward: bool) -> Option<String> {
        let pos = list.iter().position(|n| n.key == cur)?;
        let found = if forward { list[pos + 1..].iter().find(|n| n.valid) } else { list[..pos].iter().rev().find(|n| n.valid) };
        Some(found.map_or_else(|| cur.to_owned(), |n| n.key.clone()))
    }

    /// Открытие панели: путь к выбранному значению и начальный фокус.
    pub fn show(&self, st: &mut State, value: Option<&str>, auto_option_focus: bool) {
        let path = value.map(|v| self.path_to_value(v)).unwrap_or_default();
        if let Some(last) = path.last() {
            st.active = path.clone();
            st.focus = Some(last.clone());
            return;
        }
        if value.is_some() {
            st.active.clear();
        }
        let roots = self.children("");
        st.focus = if auto_option_focus {
            Self::first_focused(&roots, st)
        } else {
            roots.iter().find(|n| n.valid && Self::is_active(st, &n.key)).map(|n| n.key.clone())
        };
    }

    /// Закрытие панели.
    pub fn hide(&self, st: &mut State) {
        st.active.clear();
        st.focus = None;
    }

    /// `onOptionChange`: фокус на вариант, закрытие соседних и вложенных групп, раскрытие группы.
    pub fn option_change(&self, st: &mut State, key: &str, hover: bool, mobile: bool) {
        let Some(node) = self.node(key) else { return };
        let mut active: Vec<String> = st
            .active
            .iter()
            .filter(|k| self.node(k).is_some_and(|p| p.parent_key != node.parent_key && p.parent_key != node.key))
            .cloned()
            .collect();
        st.focus = Some(key.to_owned());
        if hover && mobile {
            return;
        }
        if node.group {
            active.push(key.to_owned());
        }
        st.active = active;
    }

    /// `onOptionClick`. `prevent_selection` — переход фокуса клавишей, лист при этом не выбирается.
    pub fn option_click(&self, st: &mut State, key: &str, hover: bool, prevent_selection: bool, mobile: bool) -> Option<Click> {
        let node = self.node(key)?;
        if Self::is_active(st, key) {
            st.active.retain(|k| k != key && is_ancestor_or_self(k, key));
            st.focus = Some(key.to_owned());
            return Some(Click::Closed);
        }
        if node.group {
            self.option_change(st, key, hover, mobile);
            st.dirty = true;
            return Some(Click::Group);
        }
        st.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != node.parent_key));
        st.active.push(key.to_owned());
        st.focus = Some(key.to_owned());
        (!prevent_selection).then(|| Click::Select(node.value.clone()))
    }

    /// Наведение на вариант.
    pub fn hover(&self, st: &mut State, key: &str, has_value: bool, focus_on_hover: bool, mobile: bool) {
        if !focus_on_hover {
            return;
        }
        let Some(node) = self.node(key) else { return };
        if st.dirty || has_value {
            self.option_change(st, key, true, mobile);
        } else if node.level == 0 {
            if node.group {
                self.option_click(st, key, true, false, mobile);
            } else {
                self.option_change(st, key, true, mobile);
            }
        }
    }

    /// Перенос фокуса клавишей: группа раскрывается, лист закрывает группы своего уровня.
    fn focus_move(&self, st: &mut State, key: &str, focus_on_hover: bool, mobile: bool) {
        let Some(node) = self.node(key) else { return };
        if !focus_on_hover {
            st.focus = Some(key.to_owned());
            return;
        }
        if node.group && !Self::is_active(st, key) {
            self.option_change(st, key, false, mobile);
            st.dirty = true;
        } else {
            st.focus = Some(key.to_owned());
            if !node.group {
                st.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != node.parent_key));
            }
        }
    }

    /// ↓ при открытой панели.
    pub fn arrow_down(&self, st: &mut State, clicked: bool, focus_on_hover: bool, mobile: bool) {
        let list = self.visible(st);
        let next = match st.focus.as_deref() {
            Some(f) => Self::step(&list, f, true),
            None if clicked => Self::first_valid(&list),
            None => Self::first_focused(&list, st),
        };
        if let Some(key) = next {
            self.focus_move(st, &key, focus_on_hover, mobile);
        }
    }

    /// ↑ при открытой панели.
    pub fn arrow_up(&self, st: &mut State, clicked: bool, focus_on_hover: bool, mobile: bool) {
        let list = self.visible(st);
        let prev = match st.focus.as_deref() {
            Some(f) => Self::step(&list, f, false),
            None if clicked => Self::last_valid(&list),
            None => Self::last_focused(&list, st),
        };
        if let Some(key) = prev {
            self.focus_move(st, &key, focus_on_hover, mobile);
        }
    }

    /// →: группа раскрывается; уже раскрытая — фокус на её первый вариант.
    pub fn arrow_right(&self, st: &mut State, focus_on_hover: bool, mobile: bool) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else { return };
        if !node.group {
            return;
        }
        let key = node.key.clone();
        if Self::is_active(st, &key) {
            let list = self.children(&key);
            if let Some(first) = Self::first_focused(&list, st) {
                self.focus_move(st, &first, focus_on_hover, mobile);
            }
        } else {
            self.option_change(st, &key, false, mobile);
        }
    }

    /// ←: закрывает группу своего уровня и возвращает фокус на группу-родителя.
    pub fn arrow_left(&self, st: &mut State, focus_on_hover: bool, mobile: bool) {
        let Some(node) = st.focus.as_deref().and_then(|f| self.node(f)) else { return };
        let parent_key = node.parent_key.clone();
        let root = node.level == 0;
        st.active.retain(|k| self.node(k).is_some_and(|p| p.parent_key != parent_key));
        if !root && let Some(parent) = self.node(&parent_key) {
            let siblings = self.children(&parent.parent_key);
            if let Some(key) = Self::first_focused(&siblings, st) {
                st.focus = Some(key.clone());
                let _ = (focus_on_hover, mobile);
            }
        }
    }

    /// `Home`.
    pub fn home(&self, st: &mut State, focus_on_hover: bool, mobile: bool) {
        if let Some(key) = Self::first_valid(&self.visible(st)) {
            self.focus_move(st, &key, focus_on_hover, mobile);
        }
    }

    /// `End`.
    pub fn end(&self, st: &mut State, focus_on_hover: bool, mobile: bool) {
        if let Some(key) = Self::last_valid(&self.visible(st)) {
            self.focus_move(st, &key, focus_on_hover, mobile);
        }
    }

    /// `Tab`: лист под фокусом остаётся в пути; панель закрывается вызывающим.
    pub fn tab(&self, st: &mut State, mobile: bool) {
        if let Some(key) = st.focus.clone()
            && self.node(&key).is_some_and(|n| !n.group)
        {
            self.option_change(st, &key, false, mobile);
        }
    }

    /// Поиск по началу подписи от текущего варианта (включительно) по кругу.
    pub fn search(&self, st: &mut State, text: &str, focus_on_hover: bool, mobile: bool) {
        let needle = text.to_lowercase();
        let list = self.visible(st);
        let matches = |n: &&&Node| n.valid && n.label.to_lowercase().starts_with(&needle);
        let cur = st.focus.as_deref().and_then(|f| list.iter().position(|n| n.key == f));
        let found = match cur {
            Some(cur) => list[cur..].iter().find(matches).or_else(|| list[..cur].iter().find(matches)),
            None => list.iter().find(matches),
        }
        .map(|n| n.key.clone());
        let target = found.or_else(|| if cur.is_none() { Self::first_focused(&list, st) } else { None });
        if let Some(key) = target {
            self.focus_move(st, &key, focus_on_hover, mobile);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(l: &str) -> CascadeOption {
        CascadeOption::new(l)
    }

    /// 0 Australia{ 0_0 NSW{Sydney, Newcastle}, 0_1 Queensland{Brisbane} }, 1 Canada{ Ontario{Toronto} }, 2 Other(off), 3 Plain
    fn options() -> Vec<CascadeOption> {
        vec![
            CascadeOption::new("Australia").children(vec![
                CascadeOption::new("New South Wales").children(vec![
                    leaf("Sydney"),
                    leaf("Newcastle").disabled(true),
                    leaf("Wollongong").value("w"),
                ]),
                CascadeOption::new("Queensland").children(vec![leaf("Brisbane")]),
            ]),
            CascadeOption::new("Canada").children(vec![CascadeOption::new("Ontario").children(vec![leaf("Toronto")])]),
            CascadeOption::new("Other").disabled(true),
            leaf("Plain"),
        ]
    }

    fn keys(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn nodes_and_paths() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let all: Vec<&str> = nodes.iter().map(|n| n.key.as_str()).collect();
        assert_eq!(&all[..6], ["0", "0_0", "0_0_0", "0_0_1", "0_0_2", "0_1"]);
        assert_eq!(m.path_to_value("w"), keys(&["0", "0_0", "0_0_2"]));
        assert_eq!(m.path_to_value("Sydney"), keys(&["0", "0_0", "0_0_0"]));
        assert!(m.path_to_value("Australia").is_empty(), "группа не имеет значения");
        assert!(m.path_to_value("zzz").is_empty());
        assert_eq!(m.label_of("w"), Some("Wollongong"));
        assert_eq!(m.label_of("Wollongong"), None);
        let n = nodes.iter().find(|n| n.key == "0_0_2").unwrap();
        assert_eq!((n.value.as_str(), n.level, n.siblings, n.valid, n.group), ("w", 2, 3, true, false));
        assert!(is_ancestor_or_self("1", "1_0") && !is_ancestor_or_self("1", "10") && is_ancestor_or_self("0_0", "0_0"));
    }

    #[test]
    fn show_restores_the_path() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        m.show(&mut st, Some("w"), false);
        assert_eq!((st.active.clone(), st.focus.as_deref()), (keys(&["0", "0_0", "0_0_2"]), Some("0_0_2")));
        assert_eq!(
            m.visible(&st).iter().map(|n| n.key.as_str()).collect::<Vec<_>>(),
            ["0_0_0", "0_0_1", "0_0_2"],
            "список уровня выбранного"
        );
        let mut st = State::default();
        m.show(&mut st, None, false);
        assert_eq!((st.active.is_empty(), st.focus.clone()), (true, None), "без значения и auto_option_focus фокуса нет");
        m.show(&mut st, None, true);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.hide(&mut st);
        assert_eq!(st, State::default());
    }

    #[test]
    fn click_group_leaf_and_collapse() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        assert_eq!(m.option_click(&mut st, "0", false, false, false), Some(Click::Group));
        assert_eq!((st.active.clone(), st.dirty), (keys(&["0"]), true));
        assert_eq!(m.option_click(&mut st, "0_0", false, false, false), Some(Click::Group));
        assert_eq!(st.active, keys(&["0", "0_0"]));
        assert_eq!(m.option_click(&mut st, "0_1", false, false, false), Some(Click::Group));
        assert_eq!(st.active, keys(&["0", "0_1"]), "соседняя группа закрывает прежнюю");
        assert_eq!(m.option_click(&mut st, "0_1_0", false, false, false), Some(Click::Select("Brisbane".into())));
        assert_eq!(m.option_click(&mut st, "0_1", false, false, false), Some(Click::Closed), "раскрытая группа сворачивается");
        assert_eq!(st.active, keys(&["0"]), "родители остаются в пути");
        assert_eq!(m.option_click(&mut st, "0", false, false, false), Some(Click::Closed));
        assert!(st.active.is_empty());
        assert_eq!(m.option_click(&mut st, "3", false, true, false), None, "лист при переходе клавишей не выбирается");
    }

    #[test]
    fn hover_rules() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        m.hover(&mut st, "0_0", false, true, false);
        assert!(st.active.is_empty(), "до клика вложенные группы не раскрываются");
        m.hover(&mut st, "0", false, true, false);
        assert_eq!(st.active, keys(&["0"]), "до клика корневые группы раскрываются");
        m.hover(&mut st, "3", false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"));
        let mut st = State { dirty: true, ..State::default() };
        m.hover(&mut st, "0", false, true, false);
        m.hover(&mut st, "0_0", false, true, false);
        assert_eq!(st.active, keys(&["0", "0_0"]), "после клика наведение раскрывает любые группы");
        m.hover(&mut st, "0_1", false, false, false);
        assert_eq!(st.active, keys(&["0", "0_0"]), "без focus_on_hover ничего не меняется");
        let mut st = State { dirty: true, ..State::default() };
        m.hover(&mut st, "0", false, true, true);
        assert!(st.active.is_empty() && st.focus.as_deref() == Some("0"), "на узком окне путь не меняется");
        let mut st = State::default();
        m.hover(&mut st, "0", true, true, false);
        assert_eq!(st.active, keys(&["0"]), "при выбранном значении наведение работает сразу");
    }

    #[test]
    fn arrows_walk_levels() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        m.arrow_down(&mut st, false, true, false);
        assert_eq!((st.focus.as_deref(), st.active.clone()), (Some("0"), keys(&["0"])), "фокус на группу раскрывает её");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!((st.focus.as_deref(), st.active.clone()), (Some("1"), keys(&["1"])), "переход на другую группу закрывает прежнюю");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"), "недоступный пропущен");
        assert!(st.active.is_empty(), "лист закрывает группы своего уровня");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"), "ниже нет");
        m.arrow_up(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("1"));
        m.arrow_up(&mut st, false, true, false);
        m.arrow_up(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.arrow_right(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0"), "→ на раскрытой группе входит в неё");
        assert_eq!(st.active, keys(&["0", "0_0"]));
        m.arrow_right(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_0"));
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_2"), "Newcastle отключён");
        m.arrow_left(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0"), "← возвращает на группу-родителя");
        assert_eq!(st.active, keys(&["0", "0_0"]));
        m.arrow_left(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.arrow_left(&mut st, true, false);
        assert!(st.active.is_empty(), "← на корне закрывает группу");
    }

    #[test]
    fn arrow_right_opens_a_closed_group_and_leaf_ignores_it() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State { focus: Some("1".into()), ..State::default() };
        m.arrow_right(&mut st, true, false);
        assert_eq!(st.active, keys(&["1"]));
        assert_eq!(st.focus.as_deref(), Some("1"));
        let mut st = State { focus: Some("3".into()), ..State::default() };
        m.arrow_right(&mut st, true, false);
        assert!(st.active.is_empty() && st.focus.as_deref() == Some("3"));
    }

    #[test]
    fn home_end_search_and_tab() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State { focus: Some("1".into()), ..State::default() };
        m.end(&mut st, false, false);
        assert_eq!(st.focus.as_deref(), Some("3"));
        m.home(&mut st, false, false);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.search(&mut st, "p", false, false);
        assert_eq!(st.focus.as_deref(), Some("3"));
        m.search(&mut st, "o", false, false);
        assert_eq!(st.focus.as_deref(), Some("3"), "отключённый Other не находится");
        m.search(&mut st, "ca", false, false);
        assert_eq!(st.focus.as_deref(), Some("1"), "по кругу");
        let mut st = State { focus: Some("0_0_2".into()), ..State::default() };
        m.search(&mut st, "s", false, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_0"));
        m.tab(&mut st, false);
        assert_eq!((st.active.len(), st.focus.as_deref()), (0, Some("0_0_0")), "Tab на листе только переносит фокус, путь не пополняется");
    }
}
