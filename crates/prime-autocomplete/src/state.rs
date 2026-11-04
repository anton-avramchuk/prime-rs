//! Модель AutoComplete и чистая логика (по `autocomplete.ts` PrimeNG 21.1.10): плоский список вариантов с группами,
//! навигация по допустимым строкам, подпись значения, совпадение для `force_selection` и разбиение по разделителю.

/// Вариант подсказки; с `items` и `group = true` — заголовок группы.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AutoCompleteOption {
    /// Подпись.
    pub label: String,
    /// Значение (по умолчанию — подпись).
    pub value: Option<String>,
    /// Недоступен.
    pub disabled: bool,
    /// Вложенные варианты группы.
    pub items: Vec<AutoCompleteOption>,
}

impl AutoCompleteOption {
    /// Вариант с подписью.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), ..Self::default() }
    }

    /// Значение варианта.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Недоступность.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Вложенные варианты (группа).
    pub fn items(mut self, items: Vec<AutoCompleteOption>) -> Self {
        self.items = items;
        self
    }

    /// Значение варианта: заданное либо подпись.
    pub fn value_or_label(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.label)
    }
}

/// Строка плоского списка.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// Заголовок группы, а не вариант.
    pub group: bool,
    /// Подпись.
    pub label: String,
    /// Значение (у групп пусто).
    pub value: String,
    /// Недоступен.
    pub disabled: bool,
}

impl Row {
    /// Допустим для фокуса: вариант (не заголовок) и не отключён.
    pub fn valid(&self) -> bool {
        !self.group && !self.disabled
    }
}

/// Разворачивает варианты: при `group` каждый верхний вариант — заголовок, за которым идут его `items`.
pub fn flatten(options: &[AutoCompleteOption], group: bool) -> Vec<Row> {
    let row = |o: &AutoCompleteOption, group: bool| Row {
        group,
        label: o.label.clone(),
        value: if group { String::new() } else { o.value_or_label().to_owned() },
        disabled: o.disabled,
    };
    let mut out = Vec::new();
    for o in options {
        if group {
            out.push(row(o, true));
            out.extend(o.items.iter().map(|i| row(i, false)));
        } else {
            out.push(row(o, false));
        }
    }
    out
}

/// Подпись варианта со значением `value` (поиск по всем вложенным).
pub fn label_for_value<'a>(options: &'a [AutoCompleteOption], value: &str) -> Option<&'a str> {
    for o in options {
        if o.items.is_empty() && o.value_or_label() == value {
            return Some(&o.label);
        }
        if let Some(found) = label_for_value(&o.items, value) {
            return Some(found);
        }
    }
    None
}

/// Текст поля для значения: подпись известного варианта либо само значение.
pub fn input_text(options: &[AutoCompleteOption], value: &str) -> String {
    label_for_value(options, value).unwrap_or(value).to_owned()
}

/// Выбран ли вариант: одиночный режим — равенство значению, `multiple` — вхождение (при `unique`).
pub fn is_selected(row: &Row, value: &str, values: &[String], multiple: bool, unique: bool) -> bool {
    if row.group {
        return false;
    }
    if multiple { unique && values.contains(&row.value) } else { !value.is_empty() && row.value == value }
}

/// Первая допустимая строка.
pub fn first(rows: &[Row]) -> Option<usize> {
    rows.iter().position(Row::valid)
}

/// Последняя допустимая строка.
pub fn last(rows: &[Row]) -> Option<usize> {
    rows.iter().rposition(Row::valid)
}

/// Следующая допустимая строка после `index`; если её нет — `index`.
pub fn next(rows: &[Row], index: usize) -> usize {
    rows.iter().enumerate().skip(index + 1).find(|(_, r)| r.valid()).map_or(index, |(i, _)| i)
}

/// Предыдущая допустимая строка перед `index`; если её нет — `index`.
pub fn prev(rows: &[Row], index: usize) -> usize {
    rows[..index.min(rows.len())].iter().rposition(Row::valid).unwrap_or(index)
}

/// Выбранная допустимая строка, иначе первая допустимая.
pub fn first_focused(rows: &[Row], selected: impl Fn(&Row) -> bool) -> Option<usize> {
    rows.iter().position(|r| r.valid() && selected(r)).or_else(|| first(rows))
}

/// Выбранная допустимая строка, иначе последняя допустимая.
pub fn last_focused(rows: &[Row], selected: impl Fn(&Row) -> bool) -> Option<usize> {
    rows.iter().position(|r| r.valid() && selected(r)).or_else(|| last(rows))
}

/// `aria-posinset`: номер среди вариантов без заголовков групп (с единицы).
pub fn pos_inset(rows: &[Row], index: usize) -> usize {
    index - rows[..index].iter().filter(|r| r.group).count() + 1
}

/// Число вариантов без заголовков групп (`aria-setsize`).
pub fn set_size(rows: &[Row]) -> usize {
    rows.iter().filter(|r| !r.group).count()
}

/// Строка, подпись которой совпадает с текстом без учёта регистра (`isOptionMatched`).
pub fn matched(rows: &[Row], text: &str) -> Option<usize> {
    let t = text.to_lowercase();
    rows.iter().position(|r| r.valid() && r.label.to_lowercase() == t)
}

/// Разбивает текст по разделителю: непустые усечённые части.
pub fn split_separator(text: &str, separator: &str) -> Vec<String> {
    text.split(separator).map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opt(l: &str) -> AutoCompleteOption {
        AutoCompleteOption::new(l)
    }

    fn rows() -> Vec<Row> {
        flatten(&[opt("Apple"), opt("Avocado").disabled(true), opt("Banana").value("b"), opt("Cherry")], false)
    }

    fn grouped() -> Vec<AutoCompleteOption> {
        vec![
            opt("Fruits").items(vec![opt("Apple"), opt("Pear").value("p")]),
            opt("Veg").items(vec![opt("Leek").disabled(true), opt("Kale")]),
        ]
    }

    #[test]
    fn flatten_plain_and_grouped() {
        let r = rows();
        assert_eq!(r.len(), 4);
        assert_eq!((r[2].label.as_str(), r[2].value.as_str(), r[1].disabled), ("Banana", "b", true));
        let g = flatten(&grouped(), true);
        let labels: Vec<(&str, bool)> = g.iter().map(|r| (r.label.as_str(), r.group)).collect();
        assert_eq!(labels, [("Fruits", true), ("Apple", false), ("Pear", false), ("Veg", true), ("Leek", false), ("Kale", false)]);
        assert_eq!(g[2].value, "p");
        assert!(!g[0].valid() && g[1].valid() && !g[4].valid());
        assert_eq!(set_size(&g), 4);
        assert_eq!((pos_inset(&g, 1), pos_inset(&g, 2), pos_inset(&g, 5)), (1, 2, 4), "заголовки не считаются");
    }

    #[test]
    fn labels_and_input_text() {
        let g = grouped();
        assert_eq!(label_for_value(&g, "p"), Some("Pear"));
        assert_eq!(label_for_value(&g, "Apple"), Some("Apple"));
        assert_eq!(label_for_value(&g, "Fruits"), None, "группа не имеет значения");
        assert_eq!(input_text(&g, "p"), "Pear");
        assert_eq!(input_text(&g, "typed"), "typed");
    }

    #[test]
    fn selection_rules() {
        let r = rows();
        let vals = vec!["b".to_owned()];
        assert!(is_selected(&r[2], "b", &[], false, true) && !is_selected(&r[0], "b", &[], false, true));
        assert!(!is_selected(&r[2], "", &[], false, true));
        assert!(
            is_selected(&r[2], "", &vals, true, true) && !is_selected(&r[2], "", &vals, true, false),
            "без unique выбранным не считается"
        );
        let g = flatten(&grouped(), true);
        assert!(!is_selected(&g[0], "Fruits", &[], false, true));
    }

    #[test]
    fn navigation_skips_disabled_and_groups() {
        let r = rows();
        assert_eq!((first(&r), last(&r)), (Some(0), Some(3)));
        assert_eq!(next(&r, 0), 2, "отключённый пропущен");
        assert_eq!((next(&r, 3), prev(&r, 0)), (3, 0), "на краях остаётся");
        assert_eq!(prev(&r, 3), 2);
        let g = flatten(&grouped(), true);
        assert_eq!(first(&g), Some(1));
        assert_eq!(next(&g, 2), 5, "заголовок и отключённый пропущены");
        assert_eq!(prev(&g, 5), 2);
        assert_eq!(first(&[]), None);
        assert_eq!(first_focused(&r, |row| row.value == "b"), Some(2), "выбранный важнее первого");
        assert_eq!(first_focused(&r, |_| false), Some(0));
        assert_eq!(last_focused(&r, |_| false), Some(3));
    }

    #[test]
    fn matching_and_separator() {
        let r = rows();
        assert_eq!(matched(&r, "APPLE"), Some(0));
        assert_eq!(matched(&r, "avocado"), None, "отключённый не совпадает");
        assert_eq!(matched(&r, "app"), None);
        assert_eq!(split_separator(" a, b ,, c ", ","), ["a", "b", "c"]);
        assert!(split_separator("  ", ",").is_empty());
    }
}
