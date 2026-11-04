//! Модель пункта и чистая логика Menu: разворачивание модели в строки, идентификаторы, классы, выбор следующего пункта
//! (по `menu.ts` PrimeNG).

use leptos::prelude::*;

/// Нажатие на пункт: событие и пункт.
#[derive(Clone)]
pub struct MenuCommand {
    /// Исходное событие.
    pub event: web_sys::MouseEvent,
    /// Нажатый пункт.
    pub item: MenuItem,
}

/// Пункт Menu (вместо `MenuItem` PrimeNG, без полей маршрутизатора Angular).
#[derive(Clone)]
pub struct MenuItem {
    /// Идентификатор элемента `li`; по умолчанию `<id меню>_<i>[_<j>]`.
    pub id: Option<String>,
    /// Подпись.
    pub label: Option<String>,
    /// Значок (например, иконка из `prime-icons`).
    pub icon: Option<ViewFn>,
    /// Адрес ссылки; без него переход по ссылке отменяется.
    pub url: Option<String>,
    /// `target` ссылки.
    pub target: Option<String>,
    /// `title` ссылки.
    pub title: Option<String>,
    /// Отключён: нажатие ничего не вызывает.
    pub disabled: bool,
    /// Виден (по умолчанию да).
    pub visible: bool,
    /// Разделитель вместо пункта.
    pub separator: bool,
    /// Подпункты: пункт становится заголовком группы.
    pub items: Vec<MenuItem>,
    /// Группа изначально раскрыта (только PanelMenu).
    pub expanded: bool,
    /// Дополнительный класс `li`.
    pub style_class: Option<String>,
    /// Встроенный стиль `li`.
    pub style: Option<String>,
    /// Дополнительный класс ссылки.
    pub link_class: Option<String>,
    /// Дополнительный класс значка.
    pub icon_class: Option<String>,
    /// Команда, вызываемая при нажатии.
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
    /// Пункт без свойств.
    pub fn new() -> Self {
        Self::default()
    }

    /// Разделитель.
    pub fn separator() -> Self {
        Self { separator: true, ..Self::default() }
    }

    /// Идентификатор `li`.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
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

    /// Адрес ссылки.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// `target` ссылки.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// `title` ссылки.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Отключает пункт.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Показывает или скрывает пункт.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Подпункты (группа).
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }

    /// Раскрывает группу изначально (только PanelMenu).
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Команда при нажатии.
    pub fn command(mut self, command: impl Into<Callback<MenuCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Дополнительный класс `li`.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Встроенный стиль `li`.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Дополнительный класс ссылки.
    pub fn link_class(mut self, class: impl Into<String>) -> Self {
        self.link_class = Some(class.into());
        self
    }

    /// Дополнительный класс значка.
    pub fn icon_class(mut self, class: impl Into<String>) -> Self {
        self.icon_class = Some(class.into());
        self
    }
}

/// Строка списка после разворачивания модели.
#[derive(Clone)]
pub enum Row {
    /// Разделитель.
    Separator,
    /// Заголовок группы.
    Header(MenuItem),
    /// Пункт с вычисленным идентификатором.
    Item {
        /// Идентификатор `li`.
        id: String,
        /// Пункт.
        item: MenuItem,
    },
}

/// Модель содержит группы: хотя бы у одного пункта есть подпункты (как `hasSubMenu` PrimeNG).
pub fn has_submenu(model: &[MenuItem]) -> bool {
    model.iter().any(|m| !m.items.is_empty())
}

/// Идентификатор пункта: `id` пункта либо `<menu_id>_<i>[_<j>]`.
pub fn item_id(item: &MenuItem, menu_id: &str, i: usize, j: Option<usize>) -> String {
    match (&item.id, j) {
        (Some(id), _) => id.clone(),
        (None, Some(j)) => format!("{menu_id}_{i}_{j}"),
        (None, None) => format!("{menu_id}_{i}"),
    }
}

/// Разворачивает модель в плоский список строк: невидимые пункты и группы отбрасываются.
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

/// Идентификаторы доступных (не отключённых) пунктов по порядку.
pub fn enabled_ids(rows: &[Row]) -> Vec<String> {
    rows.iter()
        .filter_map(|r| match r {
            Row::Item { id, item } if !item.disabled => Some(id.clone()),
            _ => None,
        })
        .collect()
}

/// Следующий доступный пункт после `current` (последний остаётся последним); без текущего — первый.
pub fn next_id(ids: &[String], current: Option<&str>) -> Option<String> {
    let pos = current.and_then(|c| ids.iter().position(|i| i == c));
    let target = pos.map_or(0, |p| p + 1);
    ids.get(target.min(ids.len().saturating_sub(1))).cloned()
}

/// Предыдущий доступный пункт перед `current` (первый остаётся первым); без текущего — первый.
pub fn prev_id(ids: &[String], current: Option<&str>) -> Option<String> {
    let pos = current.and_then(|c| ids.iter().position(|i| i == c));
    let target = pos.map_or(0, |p| p.saturating_sub(1));
    ids.get(target).cloned()
}

/// Классы корня.
pub fn root_classes(popup: bool) -> &'static str {
    if popup { "p-menu p-component p-menu-overlay" } else { "p-menu p-component" }
}

/// Инлайн-стиль корня: `absolute` у всплывающего, `relative` у статичного.
pub fn root_style(popup: bool) -> &'static str {
    if popup { "position: absolute;" } else { "position: relative;" }
}

/// Классы `li` пункта.
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

/// Нужно ли отменять переход по ссылке: пункт отключён либо без `url`.
pub fn should_prevent_default(item: &MenuItem) -> bool {
    item.disabled || item.url.is_none()
}

/// Можно ли вызвать команду пункта.
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
