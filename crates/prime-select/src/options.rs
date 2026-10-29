//! Варианты, группы и чистая логика списка Select: плоский список, фильтр, индексы фокуса, поиск набором
//! (`visibleOptions`, `findNextOptionIndex` и родственные функции PrimeNG).

/// Вариант выбора.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectOption {
    /// Метка варианта.
    pub label: String,
    /// Значение варианта.
    pub value: String,
    /// Вариант недоступен.
    pub disabled: bool,
}

impl SelectOption {
    /// Доступный вариант с меткой и значением.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), disabled: false }
    }

    /// Недоступный вариант.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Группа вариантов.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectGroup {
    /// Заголовок группы.
    pub label: String,
    /// Варианты группы.
    pub items: Vec<SelectOption>,
}

impl SelectGroup {
    /// Группа с заголовком и вариантами.
    pub fn new(label: impl Into<String>, items: Vec<SelectOption>) -> Self {
        Self { label: label.into(), items }
    }
}

/// Элемент плоского списка: заголовок группы или вариант.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// Заголовок группы (`index` — номер группы в списке групп).
    Group {
        /// Номер группы.
        index: usize,
        /// Заголовок.
        label: String,
    },
    /// Вариант.
    Option(SelectOption),
}

impl Entry {
    /// Допустим ли элемент для фокуса и выбора: вариант, не отключённый.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Option(o) if !o.disabled)
    }

    /// Вариант, если это он.
    pub fn option(&self) -> Option<&SelectOption> {
        match self {
            Self::Option(o) => Some(o),
            Self::Group { .. } => None,
        }
    }
}

/// Подходит ли метка под строку поиска: «содержит» без учёта регистра, пробелы по краям строки отбрасываются.
pub fn matches_filter(label: &str, filter: &str) -> bool {
    let needle = filter.trim().to_lowercase();
    needle.is_empty() || label.to_lowercase().contains(&needle)
}

/// Плоский список видимых элементов: варианты или группы с вариантами, отфильтрованные по строке поиска.
/// Группы без подходящих вариантов опускаются.
pub fn visible_entries(options: &[SelectOption], groups: &[SelectGroup], filter: &str) -> Vec<Entry> {
    if groups.is_empty() {
        return options.iter().filter(|o| matches_filter(&o.label, filter)).cloned().map(Entry::Option).collect();
    }
    let mut out = Vec::new();
    for (index, group) in groups.iter().enumerate() {
        let items: Vec<&SelectOption> = group.items.iter().filter(|o| matches_filter(&o.label, filter)).collect();
        if !items.is_empty() {
            out.push(Entry::Group { index, label: group.label.clone() });
            out.extend(items.into_iter().cloned().map(Entry::Option));
        }
    }
    out
}

/// Вариант с заданным значением среди всех вариантов (без фильтра).
pub fn find_option(options: &[SelectOption], groups: &[SelectGroup], value: &str) -> Option<SelectOption> {
    if groups.is_empty() {
        options.iter().find(|o| o.value == value).cloned()
    } else {
        groups.iter().flat_map(|g| g.items.iter()).find(|o| o.value == value).cloned()
    }
}

/// Индекс первого допустимого элемента.
pub fn first_valid(entries: &[Entry]) -> Option<usize> {
    entries.iter().position(Entry::is_valid)
}

/// Индекс последнего допустимого элемента.
pub fn last_valid(entries: &[Entry]) -> Option<usize> {
    entries.iter().rposition(Entry::is_valid)
}

/// Индекс следующего допустимого элемента после `index`; если его нет — прежний (`findNextOptionIndex`).
pub fn next_valid(entries: &[Entry], index: usize) -> usize {
    entries.iter().enumerate().skip(index + 1).find(|(_, e)| e.is_valid()).map_or(index, |(i, _)| i)
}

/// Индекс предыдущего допустимого элемента перед `index`; если его нет — прежний (`findPrevOptionIndex`).
pub fn prev_valid(entries: &[Entry], index: usize) -> usize {
    entries.iter().enumerate().take(index).rev().find(|(_, e)| e.is_valid()).map_or(index, |(i, _)| i)
}

/// Индекс выбранного допустимого варианта.
pub fn selected_index(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    let value = value?;
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| o.value == value))
}

/// Индекс для первой фокусировки: выбранный или первый допустимый (`findFirstFocusedOptionIndex`).
pub fn first_focused(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    selected_index(entries, value).or_else(|| first_valid(entries))
}

/// Индекс для фокусировки с конца: выбранный или последний допустимый (`findLastFocusedOptionIndex`).
pub fn last_focused(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    selected_index(entries, value).or_else(|| last_valid(entries))
}

/// Первый допустимый вариант, метка которого начинается с набранной строки без учёта регистра (`searchOptions`).
pub fn search_match(entries: &[Entry], typed: &str) -> Option<usize> {
    let typed = typed.to_lowercase();
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| o.label.to_lowercase().starts_with(&typed)))
}

/// Число вариантов без заголовков групп (`aria-setsize`).
pub fn set_size(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.option().is_some()).count()
}

/// Номер варианта среди вариантов без заголовков групп (`aria-posinset`, с единицы).
pub fn pos_in_set(entries: &[Entry], index: usize) -> usize {
    entries.iter().take(index + 1).filter(|e| e.option().is_some()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<SelectOption> {
        vec![
            SelectOption::new("New York", "NY"),
            SelectOption::new("Rome", "RM").disabled(true),
            SelectOption::new("London", "LDN"),
            SelectOption::new("Istanbul", "IST"),
            SelectOption::new("Paris", "PRS"),
        ]
    }

    fn groups() -> Vec<SelectGroup> {
        vec![
            SelectGroup::new("Germany", vec![SelectOption::new("Berlin", "B"), SelectOption::new("Frankfurt", "F")]),
            SelectGroup::new("USA", vec![SelectOption::new("Chicago", "C"), SelectOption::new("Los Angeles", "LA").disabled(true)]),
            SelectGroup::new("Japan", vec![SelectOption::new("Kyoto", "K")]),
        ]
    }

    fn labels(entries: &[Entry]) -> Vec<String> {
        entries
            .iter()
            .map(|e| match e {
                Entry::Group { label, .. } => format!("[{label}]"),
                Entry::Option(o) => o.label.clone(),
            })
            .collect()
    }

    #[test]
    fn flat_list_without_filter() {
        let entries = visible_entries(&opts(), &[], "");
        assert_eq!(labels(&entries), ["New York", "Rome", "London", "Istanbul", "Paris"]);
        assert_eq!(set_size(&entries), 5);
    }

    #[test]
    fn filter_is_case_insensitive_trimmed_substring() {
        let all = opts();
        assert_eq!(labels(&visible_entries(&all, &[], "or")), ["New York"]);
        assert_eq!(labels(&visible_entries(&all, &[], "  RO ")), ["Rome"]);
        assert_eq!(labels(&visible_entries(&all, &[], "n")), ["New York", "London", "Istanbul"]);
        assert!(visible_entries(&all, &[], "zzz").is_empty());
        assert_eq!(visible_entries(&all, &[], "   ").len(), 5);
    }

    #[test]
    fn groups_flatten_and_filter_keeps_only_matching_groups() {
        let entries = visible_entries(&[], &groups(), "");
        assert_eq!(labels(&entries), ["[Germany]", "Berlin", "Frankfurt", "[USA]", "Chicago", "Los Angeles", "[Japan]", "Kyoto"]);
        let filtered = visible_entries(&[], &groups(), "k");
        assert_eq!(labels(&filtered), ["[Germany]", "Frankfurt", "[Japan]", "Kyoto"]);
        assert!(matches!(filtered[2], Entry::Group { index: 2, .. }));
        assert!(visible_entries(&[], &groups(), "zzz").is_empty());
        // группы главнее плоских вариантов
        assert_eq!(visible_entries(&opts(), &groups(), "").len(), 8);
    }

    #[test]
    fn finds_option_by_value_even_in_groups() {
        assert_eq!(find_option(&opts(), &[], "LDN").unwrap().label, "London");
        assert_eq!(find_option(&[], &groups(), "K").unwrap().label, "Kyoto");
        assert!(find_option(&opts(), &[], "none").is_none());
    }

    #[test]
    fn navigation_skips_disabled_options_and_group_headers() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!((first_valid(&flat), last_valid(&flat)), (Some(0), Some(4)));
        assert_eq!(next_valid(&flat, 0), 2, "отключённый вариант пропущен");
        assert_eq!(prev_valid(&flat, 2), 0);
        assert_eq!(next_valid(&flat, 4), 4, "дальше некуда — индекс прежний");
        assert_eq!(prev_valid(&flat, 0), 0);
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(first_valid(&grouped), Some(1), "заголовок группы пропущен");
        assert_eq!(next_valid(&grouped, 2), 4, "через заголовок группы");
        assert_eq!(next_valid(&grouped, 4), 7, "отключённый вариант пропущен");
        assert_eq!(prev_valid(&grouped, 4), 2);
        assert_eq!(last_valid(&grouped), Some(7));
    }

    #[test]
    fn selected_and_first_focused_indexes() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(selected_index(&flat, Some("LDN")), Some(2));
        assert_eq!(selected_index(&flat, Some("RM")), None, "отключённый выбранный не ищется");
        assert_eq!(selected_index(&flat, None), None);
        assert_eq!(first_focused(&flat, Some("IST")), Some(3));
        assert_eq!(first_focused(&flat, Some("nope")), Some(0));
        assert_eq!(last_focused(&flat, None), Some(4));
        assert_eq!(last_focused(&flat, Some("NY")), Some(0));
        assert_eq!(first_focused(&[], Some("x")), None);
    }

    #[test]
    fn type_ahead_matches_prefix_of_valid_options() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(search_match(&flat, "lo"), Some(2));
        assert_eq!(search_match(&flat, "LON"), Some(2));
        assert_eq!(search_match(&flat, "ro"), None, "отключённый не ищется");
        assert_eq!(search_match(&flat, "p"), Some(4));
        assert_eq!(search_match(&flat, "zz"), None);
    }

    #[test]
    fn aria_set_and_position_ignore_group_headers() {
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(set_size(&grouped), 5);
        assert_eq!(pos_in_set(&grouped, 1), 1);
        assert_eq!(pos_in_set(&grouped, 2), 2);
        assert_eq!(pos_in_set(&grouped, 4), 3);
        assert_eq!(pos_in_set(&grouped, 7), 5);
    }

    #[test]
    fn option_builders() {
        let o = SelectOption::new("a", "b").disabled(true);
        assert_eq!((o.label.as_str(), o.value.as_str(), o.disabled), ("a", "b", true));
        assert!(!SelectOption::new("a", "b").disabled);
        assert_eq!(SelectGroup::new("g", vec![]).label, "g");
    }
}
