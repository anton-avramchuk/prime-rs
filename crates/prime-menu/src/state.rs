//! Item model and pure Menu logic: expanding the model into rows, identifiers, classes, choosing the next item
//! (after PrimeNG `menu.ts`).

use leptos::prelude::*;

/// Item press: the event and the item.
#[derive(Clone)]
pub struct MenuCommand {
    /// Source event.
    pub event: web_sys::MouseEvent,
    /// Pressed item.
    pub item: MenuItem,
}

/// Menu item (instead of PrimeNG `MenuItem`, without Angular router fields).
#[derive(Clone)]
pub struct MenuItem {
    /// Identifier of the `li` element; by default `<menu id>_<i>[_<j>]`.
    pub id: Option<String>,
    /// Label.
    pub label: Option<String>,
    /// Icon (for example, an icon from `prime-icons`).
    pub icon: Option<ViewFn>,
    /// Link address; without it, following the link is cancelled.
    pub url: Option<String>,
    /// Link `target`.
    pub target: Option<String>,
    /// Link `title`.
    pub title: Option<String>,
    /// Disabled: pressing does nothing.
    pub disabled: bool,
    /// Visible (default yes).
    pub visible: bool,
    /// Separator instead of an item.
    pub separator: bool,
    /// Subitems: the item becomes a group header.
    pub items: Vec<MenuItem>,
    /// Group initially expanded (PanelMenu only).
    pub expanded: bool,
    /// Additional class of `li`.
    pub style_class: Option<String>,
    /// Inline style of `li`.
    pub style: Option<String>,
    /// Additional class of the link.
    pub link_class: Option<String>,
    /// Additional class of the icon.
    pub icon_class: Option<String>,
    /// Command called on press.
    pub command: Option<Callback<MenuCommand>>,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            id: None,
            label: None,
            icon: None,
            url: None,
            target: None,
            title: None,
            disabled: false,
            visible: true,
            separator: false,
            items: Vec::new(),
            expanded: false,
            style_class: None,
            style: None,
            link_class: None,
            icon_class: None,
            command: None,
        }
    }
}

impl MenuItem {
    /// Item without properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Separator.
    pub fn separator() -> Self {
        Self { separator: true, ..Self::default() }
    }

    /// Identifier of `li`.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
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

    /// Link address.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Link `target`.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Link `title`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Disables the item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Shows or hides the item.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Subitems (group).
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }

    /// Expands the group initially (PanelMenu only).
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Command on press.
    pub fn command(mut self, command: impl Into<Callback<MenuCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Additional class of `li`.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Inline style of `li`.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Additional class of the link.
    pub fn link_class(mut self, class: impl Into<String>) -> Self {
        self.link_class = Some(class.into());
        self
    }

    /// Additional class of the icon.
    pub fn icon_class(mut self, class: impl Into<String>) -> Self {
        self.icon_class = Some(class.into());
        self
    }
}

/// List row after expanding the model.
#[derive(Clone)]
pub enum Row {
    /// Separator.
    Separator,
    /// Group header.
    Header(MenuItem),
    /// Item with a computed identifier.
    Item {
        /// Identifier of `li`.
        id: String,
        /// Item.
        item: MenuItem,
    },
}

/// The model contains groups: at least one item has subitems (like PrimeNG `hasSubMenu`).
pub fn has_submenu(model: &[MenuItem]) -> bool {
    model.iter().any(|m| !m.items.is_empty())
}

/// Item identifier: the item's `id` or `<menu_id>_<i>[_<j>]`.
pub fn item_id(item: &MenuItem, menu_id: &str, i: usize, j: Option<usize>) -> String {
    match (&item.id, j) {
        (Some(id), _) => id.clone(),
        (None, Some(j)) => format!("{menu_id}_{i}_{j}"),
        (None, None) => format!("{menu_id}_{i}"),
    }
}

/// Expands the model into a flat list of rows: invisible items and groups are dropped.
pub fn flatten(model: &[MenuItem], menu_id: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    if has_submenu(model) {
        for (i, group) in model.iter().enumerate() {
            if !group.visible {
                continue;
            }
            if group.separator {
                rows.push(Row::Separator);
                continue;
            }
            rows.push(Row::Header(group.clone()));
            for (j, item) in group.items.iter().enumerate() {
                push_item(&mut rows, item, item_id(item, menu_id, i, Some(j)));
            }
        }
    } else {
        for (i, item) in model.iter().enumerate() {
            push_item(&mut rows, item, item_id(item, menu_id, i, None));
        }
    }
    rows
}

fn push_item(rows: &mut Vec<Row>, item: &MenuItem, id: String) {
    if !item.visible {
        return;
    }
    if item.separator {
        rows.push(Row::Separator);
    } else {
        rows.push(Row::Item { id, item: item.clone() });
    }
}

/// Identifiers of available (not disabled) items in order.
pub fn enabled_ids(rows: &[Row]) -> Vec<String> {
    rows.iter()
        .filter_map(|r| match r {
            Row::Item { id, item } if !item.disabled => Some(id.clone()),
            _ => None,
        })
        .collect()
}

/// Next available item after `current` (the last stays last); without a current one — the first.
pub fn next_id(ids: &[String], current: Option<&str>) -> Option<String> {
    let pos = current.and_then(|c| ids.iter().position(|i| i == c));
    let target = pos.map_or(0, |p| p + 1);
    ids.get(target.min(ids.len().saturating_sub(1))).cloned()
}

/// Previous available item before `current` (the first stays first); without a current one — the first.
pub fn prev_id(ids: &[String], current: Option<&str>) -> Option<String> {
    let pos = current.and_then(|c| ids.iter().position(|i| i == c));
    let target = pos.map_or(0, |p| p.saturating_sub(1));
    ids.get(target).cloned()
}

/// Root classes.
pub fn root_classes(popup: bool) -> &'static str {
    if popup { "p-menu p-component p-menu-overlay" } else { "p-menu p-component" }
}

/// Root inline style: `absolute` for popup, `relative` for static.
pub fn root_style(popup: bool) -> &'static str {
    if popup { "position: absolute;" } else { "position: relative;" }
}

/// Classes of the item `li`.
pub fn item_classes(item: &MenuItem, focused: bool) -> String {
    let mut cn = String::from("p-menu-item");
    if focused {
        cn.push_str(" p-focus");
    }
    if item.disabled {
        cn.push_str(" p-disabled");
    }
    if let Some(c) = item.style_class.as_deref().filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(c);
    }
    cn
}

/// Whether to cancel following the link: the item is disabled or has no `url`.
pub fn should_prevent_default(item: &MenuItem) -> bool {
    item.disabled || item.url.is_none()
}

/// Whether the item command can be called.
pub fn can_run_command(item: &MenuItem) -> bool {
    !item.disabled && item.command.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(label: &str) -> MenuItem {
        MenuItem::new().label(label)
    }

    fn ids(rows: &[Row]) -> Vec<String> {
        rows.iter()
            .map(|r| match r {
                Row::Separator => "--".to_string(),
                Row::Header(h) => format!("#{}", h.label.clone().unwrap_or_default()),
                Row::Item { id, .. } => id.clone(),
            })
            .collect()
    }

    #[test]
    fn item_id_table() {
        let plain = leaf("a");
        let named = leaf("a").id("custom");
        assert_eq!(item_id(&plain, "m", 1, None), "m_1");
        assert_eq!(item_id(&plain, "m", 1, Some(2)), "m_1_2");
        assert_eq!(item_id(&named, "m", 1, Some(2)), "custom");
    }

    #[test]
    fn flatten_flat_model_skips_hidden_and_marks_separators() {
        let model = vec![leaf("a"), MenuItem::separator(), leaf("b").visible(false), leaf("c")];
        assert_eq!(ids(&flatten(&model, "m")), ["m_0", "--", "m_3"]);
    }

    #[test]
    fn flatten_groups_emit_headers_and_nested_ids() {
        let model = vec![
            leaf("Docs").items(vec![leaf("New"), MenuItem::separator(), leaf("Hidden").visible(false)]),
            MenuItem::separator(),
            leaf("Profile").items(vec![leaf("Settings")]),
            leaf("Gone").visible(false).items(vec![leaf("x")]),
        ];
        assert_eq!(ids(&flatten(&model, "m")), ["#Docs", "m_0_0", "--", "--", "#Profile", "m_2_0"]);
    }

    #[test]
    fn has_submenu_looks_at_any_item() {
        assert!(!has_submenu(&[]));
        assert!(!has_submenu(&[leaf("a")]));
        assert!(has_submenu(&[leaf("a").items(vec![leaf("b")])]));
        assert!(has_submenu(&[MenuItem::separator(), leaf("b").items(vec![leaf("c")])]));
    }

    #[test]
    fn enabled_ids_skip_disabled_headers_and_separators() {
        let model = vec![leaf("a"), leaf("b").disabled(true), MenuItem::separator(), leaf("c")];
        assert_eq!(enabled_ids(&flatten(&model, "m")), ["m_0", "m_3"]);
    }

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn next_and_prev_table() {
        let list = s(&["a", "b", "c"]);
        // (current, next, prev)
        let cases = [(None, "a", "a"), (Some("a"), "b", "a"), (Some("b"), "c", "a"), (Some("c"), "c", "b"), (Some("zzz"), "a", "a")];
        for (cur, next, prev) in cases {
            assert_eq!(next_id(&list, cur).as_deref(), Some(next), "next {cur:?}");
            assert_eq!(prev_id(&list, cur).as_deref(), Some(prev), "prev {cur:?}");
        }
        assert_eq!(next_id(&[], None), None);
        assert_eq!(prev_id(&[], Some("a")), None);
    }

    #[test]
    fn classes_table() {
        assert_eq!(root_classes(false), "p-menu p-component");
        assert_eq!(root_classes(true), "p-menu p-component p-menu-overlay");
        assert_eq!(root_style(true), "position: absolute;");
        assert_eq!(root_style(false), "position: relative;");
        let plain = leaf("a");
        assert_eq!(item_classes(&plain, false), "p-menu-item");
        assert_eq!(item_classes(&plain, true), "p-menu-item p-focus");
        let both = leaf("a").disabled(true).style_class("x");
        assert_eq!(item_classes(&both, true), "p-menu-item p-focus p-disabled x");
    }

    #[test]
    fn prevent_default_and_command_rules() {
        let with_url = leaf("a").url("/x");
        assert!(!should_prevent_default(&with_url));
        assert!(should_prevent_default(&leaf("a")));
        assert!(should_prevent_default(&with_url.clone().disabled(true)));
        assert!(!can_run_command(&leaf("a")));
        let cmd = leaf("a").command(Callback::new(|_: MenuCommand| {}));
        assert!(can_run_command(&cmd));
        assert!(!can_run_command(&cmd.disabled(true)));
    }
}
