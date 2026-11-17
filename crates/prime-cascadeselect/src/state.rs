//! CascadeSelect model and pure transition logic (after PrimeNG 21.1.10 `cascadeselect.ts`).
//!
//! Options form a tree: keys `"0"`, `"0_1"`, `"0_1_2"` (indices of ancestors and its own). The path of active groups
//! ([`State::active`]) holds the expanded groups from the root down; focus is an option key, and the list of the focus level is its siblings.

/// Option: a leaf with a value or a group with nested options.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CascadeOption {
    /// Label.
    pub label: String,
    /// Leaf value (the label by default).
    pub value: Option<String>,
    /// Unavailable.
    pub disabled: bool,
    /// Nested options (group).
    pub children: Vec<CascadeOption>,
}

impl CascadeOption {
    /// Option with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), ..Self::default() }
    }

    /// Leaf value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Unavailability.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Nested options.
    pub fn children(mut self, children: Vec<CascadeOption>) -> Self {
        self.children = children;
        self
    }
}

/// Node of the tree of "processed options".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Key.
    pub key: String,
    /// Parent key (empty for root options).
    pub parent_key: String,
    /// Depth (root options are `0`).
    pub level: usize,
    /// Index among siblings.
    pub index: usize,
    /// Number of siblings.
    pub siblings: usize,
    /// Group (has nested options).
    pub group: bool,
    /// Eligible for focus.
    pub valid: bool,
    /// Label.
    pub label: String,
    /// Value (the label if not set).
    pub value: String,
}

/// Builds nodes in depth-first order.
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

/// Whether `ancestor` is an ancestor of `key` (or the key itself): compared on key boundaries.
pub fn is_ancestor_or_self(ancestor: &str, key: &str) -> bool {
    key == ancestor || key.strip_prefix(ancestor).is_some_and(|rest| rest.starts_with('_'))
}

/// Transition state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    /// Expanded groups from the root down.
    pub active: Vec<String>,
    /// Option under focus.
    pub focus: Option<String>,
    /// After a click on a group, hover expands any groups.
    pub dirty: bool,
}

/// Result of a click on an option.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Click {
    /// An expanded group collapsed.
    Closed,
    /// A group expanded.
    Group,
    /// A leaf with this value was selected.
    Select(String),
}

/// Option tree with operations.
pub struct Menu<'a> {
    /// Nodes.
    pub nodes: &'a [Node],
}

impl Menu<'_> {
    /// Node by key.
    pub fn node(&self, key: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Children of the node with key `parent` (root ones for an empty key).
    pub fn children(&self, parent: &str) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.parent_key == parent).collect()
    }

    /// List of the focus level: siblings of the option under focus, or root ones.
    pub fn visible(&self, st: &State) -> Vec<&Node> {
        match st.focus.as_deref().and_then(|f| self.node(f)) {
            Some(n) => self.children(&n.parent_key),
            None => self.children(""),
        }
    }

    /// Keys of the path from the root to the leaf with value `value` (empty if there is none).
    pub fn path_to_value(&self, value: &str) -> Vec<String> {
        let Some(leaf) = self.nodes.iter().find(|n| !n.group && n.value == value) else { return Vec::new() };
        self.nodes.iter().filter(|n| is_ancestor_or_self(&n.key, &leaf.key)).map(|n| n.key.clone()).collect()
    }

    /// Label of the leaf with value `value`.
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

    /// The selected (in-path) eligible option of the list, otherwise the first eligible one.
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

    /// Opening the panel: path to the selected value and initial focus.
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

    /// Closing the panel.
    pub fn hide(&self, st: &mut State) {
        st.active.clear();
        st.focus = None;
    }

    /// `onOptionChange`: focus on an option, closing sibling and nested groups, expanding the group.
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

    /// `onOptionClick`. `prevent_selection` — focus moved by a key, the leaf is not selected then.
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

    /// Hover over an option.
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

    /// Moving focus by a key: a group expands, a leaf closes the groups of its level.
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

    /// ↓ with the panel open.
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

    /// ↑ with the panel open.
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

    /// →: a group expands; an already expanded one — focus on its first option.
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

    /// ←: closes the group of its level and returns focus to the parent group.
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

    /// `Tab`: the leaf under focus stays in the path; the panel is closed by the caller.
    pub fn tab(&self, st: &mut State, mobile: bool) {
        if let Some(key) = st.focus.clone()
            && self.node(&key).is_some_and(|n| !n.group)
        {
            self.option_change(st, &key, false, mobile);
        }
    }

    /// Search by label prefix from the current option (inclusive), wrapping around.
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
        assert!(m.path_to_value("Australia").is_empty(), "a group has no value");
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
            "list of the selected level"
        );
        let mut st = State::default();
        m.show(&mut st, None, false);
        assert_eq!((st.active.is_empty(), st.focus.clone()), (true, None), "without a value and auto_option_focus there is no focus");
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
        assert_eq!(st.active, keys(&["0", "0_1"]), "a sibling group closes the previous one");
        assert_eq!(m.option_click(&mut st, "0_1_0", false, false, false), Some(Click::Select("Brisbane".into())));
        assert_eq!(m.option_click(&mut st, "0_1", false, false, false), Some(Click::Closed), "an expanded group collapses");
        assert_eq!(st.active, keys(&["0"]), "parents stay in the path");
        assert_eq!(m.option_click(&mut st, "0", false, false, false), Some(Click::Closed));
        assert!(st.active.is_empty());
        assert_eq!(m.option_click(&mut st, "3", false, true, false), None, "a leaf is not selected on a key-driven move");
    }

    #[test]
    fn hover_rules() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        m.hover(&mut st, "0_0", false, true, false);
        assert!(st.active.is_empty(), "before a click nested groups do not expand");
        m.hover(&mut st, "0", false, true, false);
        assert_eq!(st.active, keys(&["0"]), "before a click root groups expand");
        m.hover(&mut st, "3", false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"));
        let mut st = State { dirty: true, ..State::default() };
        m.hover(&mut st, "0", false, true, false);
        m.hover(&mut st, "0_0", false, true, false);
        assert_eq!(st.active, keys(&["0", "0_0"]), "after a click hover expands any groups");
        m.hover(&mut st, "0_1", false, false, false);
        assert_eq!(st.active, keys(&["0", "0_0"]), "without focus_on_hover nothing changes");
        let mut st = State { dirty: true, ..State::default() };
        m.hover(&mut st, "0", false, true, true);
        assert!(st.active.is_empty() && st.focus.as_deref() == Some("0"), "in a narrow window the path does not change");
        let mut st = State::default();
        m.hover(&mut st, "0", true, true, false);
        assert_eq!(st.active, keys(&["0"]), "with a selected value hover works right away");
    }

    #[test]
    fn arrows_walk_levels() {
        let nodes = build(&options());
        let m = Menu { nodes: &nodes };
        let mut st = State::default();
        m.arrow_down(&mut st, false, true, false);
        assert_eq!((st.focus.as_deref(), st.active.clone()), (Some("0"), keys(&["0"])), "focus on a group expands it");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!((st.focus.as_deref(), st.active.clone()), (Some("1"), keys(&["1"])), "moving to another group closes the previous one");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"), "the unavailable one is skipped");
        assert!(st.active.is_empty(), "a leaf closes the groups of its level");
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("3"), "nothing below");
        m.arrow_up(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("1"));
        m.arrow_up(&mut st, false, true, false);
        m.arrow_up(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.arrow_right(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0"), "→ on an expanded group enters it");
        assert_eq!(st.active, keys(&["0", "0_0"]));
        m.arrow_right(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_0"));
        m.arrow_down(&mut st, false, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_2"), "Newcastle is disabled");
        m.arrow_left(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0_0"), "← returns to the parent group");
        assert_eq!(st.active, keys(&["0", "0_0"]));
        m.arrow_left(&mut st, true, false);
        assert_eq!(st.focus.as_deref(), Some("0"));
        m.arrow_left(&mut st, true, false);
        assert!(st.active.is_empty(), "← at the root closes the group");
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
        assert_eq!(st.focus.as_deref(), Some("3"), "the disabled Other is not found");
        m.search(&mut st, "ca", false, false);
        assert_eq!(st.focus.as_deref(), Some("1"), "wrapping around");
        let mut st = State { focus: Some("0_0_2".into()), ..State::default() };
        m.search(&mut st, "s", false, false);
        assert_eq!(st.focus.as_deref(), Some("0_0_0"));
        m.tab(&mut st, false);
        assert_eq!((st.active.len(), st.focus.as_deref()), (0, Some("0_0_0")), "Tab on a leaf only moves focus, the path is not extended");
    }
}
