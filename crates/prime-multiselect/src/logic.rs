//! Варианты, группы и чистая логика MultiSelect: плоский список, фильтр, индексы фокуса, выбор, диапазоны, «выбрать все»,
//! предел выбора, метка и чипы, поиск набором (`visibleOptions`, `onOptionSelect`, `onToggleAll`, `label` и родственные функции PrimeNG).

/// Вариант выбора.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MultiSelectOption {
    /// Метка варианта.
    pub label: String,
    /// Значение варианта.
    pub value: String,
    /// Вариант недоступен.
    pub disabled: bool,
}

impl MultiSelectOption {
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
pub struct MultiSelectGroup {
    /// Заголовок группы.
    pub label: String,
    /// Варианты группы.
    pub items: Vec<MultiSelectOption>,
}

impl MultiSelectGroup {
    /// Группа с заголовком и вариантами.
    pub fn new(label: impl Into<String>, items: Vec<MultiSelectOption>) -> Self {
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
    Option(MultiSelectOption),
}

impl Entry {
    /// Допустим ли элемент для фокуса и выбора: вариант, не отключённый.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Option(o) if !o.disabled)
    }

    /// Вариант, если это он.
    pub fn option(&self) -> Option<&MultiSelectOption> {
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
pub fn visible_entries(options: &[MultiSelectOption], groups: &[MultiSelectGroup], filter: &str) -> Vec<Entry> {
    if groups.is_empty() {
        return options.iter().filter(|o| matches_filter(&o.label, filter)).cloned().map(Entry::Option).collect();
    }
    let mut out = Vec::new();
    for (index, group) in groups.iter().enumerate() {
        let items: Vec<&MultiSelectOption> = group.items.iter().filter(|o| matches_filter(&o.label, filter)).collect();
        if !items.is_empty() {
            out.push(Entry::Group { index, label: group.label.clone() });
            out.extend(items.into_iter().cloned().map(Entry::Option));
        }
    }
    out
}

/// Вариант с заданным значением среди всех вариантов (без фильтра).
pub fn find_option(options: &[MultiSelectOption], groups: &[MultiSelectGroup], value: &str) -> Option<MultiSelectOption> {
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

/// Выбран ли вариант с таким значением.
fn is_selected(selected: &[String], value: &str) -> bool {
    selected.iter().any(|s| s == value)
}

/// Индекс первого допустимого выбранного варианта (`findSelectedOptionIndex`).
pub fn selected_index(entries: &[Entry], selected: &[String]) -> Option<usize> {
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value)))
}

/// Индекс для первой фокусировки: выбранный или первый допустимый (`findFirstFocusedOptionIndex`).
pub fn first_focused(entries: &[Entry], selected: &[String]) -> Option<usize> {
    selected_index(entries, selected).or_else(|| first_valid(entries))
}

/// Индекс для фокусировки с конца: последний выбранный допустимый или последний допустимый (`findLastFocusedOptionIndex`).
pub fn last_focused(entries: &[Entry], selected: &[String]) -> Option<usize> {
    entries
        .iter()
        .rposition(|e| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value)))
        .or_else(|| last_valid(entries))
}

/// Ближайший выбранный вариант к `index` (`findNearestSelectedOptionIndex`): при `first_check_up` сначала вверх, затем вниз,
/// иначе наоборот; если выбранных нет — сам `index`.
pub fn nearest_selected(entries: &[Entry], selected: &[String], index: usize, first_check_up: bool) -> usize {
    let chosen = |e: &Entry| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value));
    let prev = || entries.iter().enumerate().take(index).rev().find(|(_, e)| chosen(e)).map(|(i, _)| i);
    let next = || entries.iter().enumerate().skip(index + 1).find(|(_, e)| chosen(e)).map(|(i, _)| i);
    let found = if first_check_up { prev().or_else(next) } else { next().or_else(prev) };
    found.unwrap_or(index)
}

/// Первый допустимый вариант, метка которого начинается с набранной строки без учёта регистра; поиск идёт от `from`
/// (включительно) до конца, затем с начала (`searchOptions`).
pub fn search_match_from(entries: &[Entry], typed: &str, from: Option<usize>) -> Option<usize> {
    let typed = typed.to_lowercase();
    let hit = |e: &Entry| e.is_valid() && e.option().is_some_and(|o| o.label.to_lowercase().starts_with(&typed));
    match from {
        Some(start) => {
            let start = start.min(entries.len());
            entries[start..].iter().position(hit).map(|i| i + start).or_else(|| entries[..start].iter().position(hit))
        }
        None => entries.iter().position(hit),
    }
}

/// Число вариантов без заголовков групп (`aria-setsize`).
pub fn set_size(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.option().is_some()).count()
}

/// Номер варианта среди вариантов без заголовков групп (`aria-posinset`, с единицы).
pub fn pos_in_set(entries: &[Entry], index: usize) -> usize {
    entries.iter().take(index + 1).filter(|e| e.option().is_some()).count()
}

/// Переключает значение: убирает его, если оно выбрано, иначе добавляет в конец (`onOptionSelect`).
pub fn toggle_value(current: &[String], value: &str) -> Vec<String> {
    if is_selected(current, value) {
        current.iter().filter(|v| v.as_str() != value).cloned().collect()
    } else {
        let mut out = current.to_vec();
        out.push(value.to_owned());
        out
    }
}

/// Допустимые варианты в диапазоне индексов `start..=end` (порядок границ любой), `onOptionSelectRange`.
pub fn select_range(entries: &[Entry], start: usize, end: usize) -> Vec<String> {
    let (lo, hi) = (start.min(end), start.max(end));
    entries
        .iter()
        .enumerate()
        .filter(|(i, e)| (lo..=hi).contains(i) && e.is_valid())
        .filter_map(|(_, e)| e.option().map(|o| o.value.clone()))
        .collect()
}

/// Все допустимые видимые варианты (`Ctrl+A`).
pub fn select_all(entries: &[Entry]) -> Vec<String> {
    entries.iter().filter(|e| e.is_valid()).filter_map(|e| e.option().map(|o| o.value.clone())).collect()
}

/// Выбраны ли все видимые допустимые варианты (`allSelected`); пустой список — нет.
pub fn all_selected(entries: &[Entry], selected: &[String]) -> bool {
    !entries.is_empty() && entries.iter().all(|e| !e.is_valid() || e.option().is_some_and(|o| is_selected(selected, &o.value)))
}

/// Все варианты без заголовков групп (до фильтра).
pub fn all_options(options: &[MultiSelectOption], groups: &[MultiSelectGroup]) -> Vec<MultiSelectOption> {
    if groups.is_empty() { options.to_vec() } else { groups.iter().flat_map(|g| g.items.iter().cloned()).collect() }
}

/// Предел выбора достигнут: выбрано ровно `limit` значений.
pub fn limit_reached(selected: &[String], limit: Option<usize>) -> bool {
    limit.is_some_and(|l| l > 0 && selected.len() == l)
}

/// Недоступен ли вариант с учётом предела выбора: собственный `disabled` либо предел достигнут, а вариант не выбран.
pub fn option_disabled(option: &MultiSelectOption, selected: &[String], limit: Option<usize>) -> bool {
    option.disabled || (limit_reached(selected, limit) && !is_selected(selected, &option.value))
}

/// Применяет предел выбора к плоскому списку: невыбранные варианты при достигнутом пределе становятся недоступными.
pub fn apply_limit(entries: Vec<Entry>, selected: &[String], limit: Option<usize>) -> Vec<Entry> {
    if !limit_reached(selected, limit) {
        return entries;
    }
    entries
        .into_iter()
        .map(|e| match e {
            Entry::Option(mut o) => {
                o.disabled = option_disabled(&o, selected, limit);
                Entry::Option(o)
            }
            group => group,
        })
        .collect()
}

/// Итоговые значения после клика по «выбрать все» (`onToggleAll`): выбранные недоступные остаются всегда; при выборе добавляются
/// все доступные видимые, а при включённом поле поиска сохраняются и выбранные вне результата поиска; при снятии остаются
/// только выбранные недоступные видимые. Повторы убираются, порядок: прежние вне поиска, недоступные, видимые.
pub fn toggle_all(all: &[MultiSelectOption], visible: &[Entry], selected: &[String], filter_enabled: bool) -> Vec<String> {
    let all_checked = all_selected(visible, selected);
    let chosen = |o: &MultiSelectOption| is_selected(selected, &o.value);
    let disabled_selected = all.iter().filter(|o| chosen(o) && o.disabled);
    let visible_part =
        visible.iter().filter_map(Entry::option).filter(|o| if all_checked { o.disabled && chosen(o) } else { chosen(o) || !o.disabled });
    let before_search = all.iter().filter(|o| filter_enabled && !all_checked && chosen(o) && !o.disabled);
    let mut out: Vec<String> = Vec::new();
    for o in before_search.chain(disabled_selected).chain(visible_part) {
        if !out.contains(&o.value) {
            out.push(o.value.clone());
        }
    }
    out
}

/// Подставляет число в шаблон: первое `{…}` заменяется на `count` (`getSelectedItemsLabel`); без скобок шаблон не меняется.
pub fn summary(template: &str, count: usize) -> String {
    match (template.find('{'), template.find('}')) {
        (Some(open), Some(close)) if close > open => format!("{}{}{}", &template[..open], count, &template[close + 1..]),
        _ => template.to_owned(),
    }
}

/// Метки выбранных вариантов в порядке выбора; значения, которых нет среди вариантов, пропускаются.
pub fn selected_labels(all: &[MultiSelectOption], selected: &[String]) -> Vec<String> {
    selected.iter().filter_map(|v| all.iter().find(|o| &o.value == v).map(|o| o.label.clone())).collect()
}

/// Что показывает метка.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabelContent {
    /// Метки через запятую.
    Joined(String),
    /// Сводка «N items selected».
    Summary(String),
    /// Подсказка `placeholder` (или пусто).
    Placeholder(Option<String>),
}

/// Метка режима `comma` (`label()`): список меток, сводка при превышении `max` либо подсказка.
pub fn comma_label(labels: &[String], max: Option<usize>, template: &str, placeholder: Option<&str>, show_labels: bool) -> LabelContent {
    if !labels.is_empty() && show_labels {
        if max.is_some_and(|m| labels.len() > m) {
            LabelContent::Summary(summary(template, labels.len()))
        } else {
            LabelContent::Joined(labels.join(", "))
        }
    } else {
        LabelContent::Placeholder(placeholder.filter(|p| !p.is_empty()).map(str::to_owned))
    }
}

/// Что показывает метка в режиме `chip`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChipContent {
    /// Чипы: значения выбранных вариантов (не больше `max`).
    Chips(Vec<String>),
    /// Сводка: выбрано не меньше `max` (так делает PrimeNG: `chipSelectedItems().length === maxSelectedLabels`).
    Summary(String),
    /// Подсказка.
    Placeholder(Option<String>),
}

/// Метка режима `chip` (`chipSelectedItems()`): чипы, сводка при `len >= max` либо подсказка.
pub fn chip_content(selected: &[String], max: Option<usize>, template: &str, placeholder: Option<&str>) -> ChipContent {
    if selected.is_empty() {
        return ChipContent::Placeholder(placeholder.filter(|p| !p.is_empty()).map(str::to_owned));
    }
    match max {
        Some(m) if selected.len() >= m => ChipContent::Summary(summary(template, selected.len())),
        _ => ChipContent::Chips(selected.to_vec()),
    }
}

/// Состояние корня для классов и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RootState {
    /// Режим чипов.
    pub chip: bool,
    /// Недоступен.
    pub disabled: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Вариант `filled`.
    pub variant_filled: bool,
    /// Поле в фокусе.
    pub focused: bool,
    /// Есть выбранные значения.
    pub filled: bool,
    /// Панель открыта.
    pub open: bool,
    /// На всю ширину.
    pub fluid: bool,
    /// Размер.
    pub size: Option<prime_core::Size>,
}

/// Классы корня в порядке `multiselectstyle.ts`.
pub fn write_root_classes(cn: &mut prime_core::ClassNames, s: &RootState) {
    use prime_core::Size;
    cn.add("p-multiselect p-component p-inputwrapper")
        .add_if(s.chip, "p-multiselect-display-chip")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.variant_filled, "p-variant-filled")
        .add_if(s.focused, "p-focus")
        .add_if(s.filled, "p-inputwrapper-filled")
        .add_if(s.focused || s.open, "p-inputwrapper-focus")
        .add_if(s.open, "p-multiselect-open")
        .add_if(s.fluid, "p-multiselect-fluid")
        .add_if(s.size == Some(Size::Small), "p-multiselect-sm p-inputfield-sm")
        .add_if(s.size == Some(Size::Large), "p-multiselect-lg p-inputfield-lg");
}

/// Значение `data-p` корня: `invalid`, `disabled`, `focus`, `fluid`, `filled`, размер.
pub fn data_p(s: &RootState) -> std::borrow::Cow<'static, str> {
    let mut d = prime_core::DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.disabled, "disabled")
        .add_if(s.focused, "focus")
        .add_if(s.fluid, "fluid")
        .add_if(s.variant_filled, "filled")
        .add_opt(s.size.map(prime_core::Size::as_str));
    d.finish_cow()
}

/// Классы метки: `p-placeholder` при подсказке, `p-multiselect-label-empty` без подсказки и без выбора.
pub fn write_label_classes(cn: &mut prime_core::ClassNames, placeholder_shown: bool, empty: bool) {
    cn.add("p-multiselect-label").add_if(placeholder_shown, "p-placeholder").add_if(empty, "p-multiselect-label-empty");
}

/// Классы варианта.
pub fn write_option_classes(cn: &mut prime_core::ClassNames, highlighted: bool, focused: bool, disabled: bool) {
    cn.add("p-multiselect-option")
        .add_if(highlighted, "p-multiselect-option-selected")
        .add_if(focused, "p-focus")
        .add_if(disabled, "p-disabled");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<MultiSelectOption> {
        vec![
            MultiSelectOption::new("New York", "NY"),
            MultiSelectOption::new("Rome", "RM").disabled(true),
            MultiSelectOption::new("London", "LDN"),
            MultiSelectOption::new("Istanbul", "IST"),
            MultiSelectOption::new("Paris", "PRS"),
        ]
    }

    fn groups() -> Vec<MultiSelectGroup> {
        vec![
            MultiSelectGroup::new("Germany", vec![MultiSelectOption::new("Berlin", "B"), MultiSelectOption::new("Frankfurt", "F")]),
            MultiSelectGroup::new(
                "USA",
                vec![MultiSelectOption::new("Chicago", "C"), MultiSelectOption::new("Los Angeles", "LA").disabled(true)],
            ),
            MultiSelectGroup::new("Japan", vec![MultiSelectOption::new("Kyoto", "K")]),
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

    fn sel(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| (*v).to_owned()).collect()
    }

    #[test]
    fn flat_list_filter_and_groups() {
        let entries = visible_entries(&opts(), &[], "");
        assert_eq!(labels(&entries), ["New York", "Rome", "London", "Istanbul", "Paris"]);
        assert_eq!(labels(&visible_entries(&opts(), &[], "  RO ")), ["Rome"]);
        assert_eq!(labels(&visible_entries(&opts(), &[], "n")), ["New York", "London", "Istanbul"]);
        assert!(visible_entries(&opts(), &[], "zzz").is_empty());
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(labels(&grouped), ["[Germany]", "Berlin", "Frankfurt", "[USA]", "Chicago", "Los Angeles", "[Japan]", "Kyoto"]);
        assert_eq!(labels(&visible_entries(&[], &groups(), "k")), ["[Germany]", "Frankfurt", "[Japan]", "Kyoto"]);
        assert_eq!(set_size(&grouped), 5);
        assert_eq!(pos_in_set(&grouped, 7), 5);
    }

    #[test]
    fn navigation_skips_disabled_and_headers() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!((first_valid(&flat), last_valid(&flat)), (Some(0), Some(4)));
        assert_eq!(next_valid(&flat, 0), 2);
        assert_eq!(prev_valid(&flat, 2), 0);
        assert_eq!(next_valid(&flat, 4), 4);
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(first_valid(&grouped), Some(1));
        assert_eq!(next_valid(&grouped, 4), 7);
    }

    #[test]
    fn focus_indexes_follow_selection() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(selected_index(&flat, &sel(&["LDN"])), Some(2));
        assert_eq!(selected_index(&flat, &sel(&["RM"])), None);
        assert_eq!(first_focused(&flat, &sel(&["IST"])), Some(3));
        assert_eq!(first_focused(&flat, &sel(&["nope"])), Some(0));
        assert_eq!(last_focused(&flat, &sel(&["NY", "LDN"])), Some(2));
        assert_eq!(last_focused(&flat, &[]), Some(4));
        assert_eq!(nearest_selected(&flat, &sel(&["NY", "PRS"]), 2, true), 0);
        assert_eq!(nearest_selected(&flat, &sel(&["NY", "PRS"]), 2, false), 4);
        assert_eq!(nearest_selected(&flat, &[], 2, true), 2);
    }

    #[test]
    fn type_ahead_starts_at_focus_and_wraps() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(search_match_from(&flat, "lo", None), Some(2));
        assert_eq!(search_match_from(&flat, "LON", Some(3)), Some(2));
        assert_eq!(search_match_from(&flat, "ro", None), None);
        assert_eq!(search_match_from(&flat, "n", Some(0)), Some(0));
    }

    #[test]
    fn toggle_value_adds_and_removes() {
        assert_eq!(toggle_value(&sel(&["a"]), "b"), sel(&["a", "b"]));
        assert_eq!(toggle_value(&sel(&["a", "b"]), "a"), sel(&["b"]));
        assert_eq!(toggle_value(&[], "a"), sel(&["a"]));
    }

    #[test]
    fn range_and_select_all_skip_disabled_and_headers() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(select_range(&flat, 0, 3), sel(&["NY", "LDN", "IST"]));
        assert_eq!(select_range(&flat, 3, 0), sel(&["NY", "LDN", "IST"]));
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(select_range(&grouped, 0, 4), sel(&["B", "F", "C"]));
        assert_eq!(select_all(&grouped), sel(&["B", "F", "C", "K"]));
        assert!(select_all(&[]).is_empty());
    }

    #[test]
    fn all_selected_ignores_disabled() {
        let flat = visible_entries(&opts(), &[], "");
        assert!(all_selected(&flat, &sel(&["NY", "LDN", "IST", "PRS"])));
        assert!(!all_selected(&flat, &sel(&["NY"])));
        assert!(!all_selected(&[], &[]));
        assert!(all_selected(&visible_entries(&opts(), &[], "n"), &sel(&["NY", "LDN", "IST"])));
    }

    #[test]
    fn selection_limit_disables_unselected_only_at_the_limit() {
        let o = MultiSelectOption::new("a", "a");
        assert!(!limit_reached(&sel(&["a"]), Some(2)));
        assert!(limit_reached(&sel(&["a", "b"]), Some(2)));
        assert!(!limit_reached(&sel(&["a", "b"]), None));
        assert!(!limit_reached(&sel(&["a", "b"]), Some(0)), "ноль — без предела");
        assert!(!option_disabled(&o, &sel(&["a", "b"]), Some(2)), "выбранный остаётся доступным");
        assert!(option_disabled(&MultiSelectOption::new("c", "c"), &sel(&["a", "b"]), Some(2)));
        assert!(option_disabled(&MultiSelectOption::new("c", "c").disabled(true), &[], Some(2)));
        let flat = visible_entries(&opts(), &[], "");
        let limited = apply_limit(flat.clone(), &sel(&["NY", "LDN"]), Some(2));
        assert_eq!(first_valid(&limited), Some(0));
        assert_eq!(next_valid(&limited, 0), 2);
        assert_eq!(next_valid(&limited, 2), 2, "остальные недоступны");
        assert_eq!(apply_limit(flat.clone(), &sel(&["NY"]), Some(2)), flat, "предел не достигнут");
    }

    #[test]
    fn toggle_all_table() {
        let all = all_options(&opts(), &[]);
        let visible = visible_entries(&opts(), &[], "");
        // ничего не выбрано: все доступные видимые
        assert_eq!(toggle_all(&all, &visible, &[], true), sel(&["NY", "LDN", "IST", "PRS"]));
        // выбран недоступный Rome: остаётся, порядок — недоступные перед видимыми
        assert_eq!(toggle_all(&all, &visible, &sel(&["RM"]), true), sel(&["RM", "NY", "LDN", "IST", "PRS"]));
        // все выбраны: снимает всё, кроме недоступных выбранных
        assert_eq!(toggle_all(&all, &visible, &sel(&["NY", "LDN", "IST", "PRS", "RM"]), true), sel(&["RM"]));
        assert!(toggle_all(&all, &visible, &sel(&["NY", "LDN", "IST", "PRS"]), true).is_empty());
        // поиск «n»: видимые NY, LDN, IST; прежний выбор вне поиска (PRS) сохраняется при включённом поле поиска
        let found = visible_entries(&opts(), &[], "n");
        assert_eq!(toggle_all(&all, &found, &sel(&["PRS"]), true), sel(&["PRS", "NY", "LDN", "IST"]));
        assert_eq!(toggle_all(&all, &found, &sel(&["PRS"]), false), sel(&["NY", "LDN", "IST"]));
        // снятие при поиске не трогает выбранные вне результата?  — PrimeNG: вне поиска они не сохраняются при снятии
        assert!(toggle_all(&all, &found, &sel(&["NY", "LDN", "IST", "PRS"]), true).is_empty());
    }

    #[test]
    fn summary_substitutes_the_first_placeholder() {
        assert_eq!(summary("{0} items selected", 4), "4 items selected");
        assert_eq!(summary("Chosen: {n}!", 2), "Chosen: 2!");
        assert_eq!(summary("none", 3), "none");
        assert_eq!(summary("{0} of {1}", 3), "3 of {1}");
    }

    #[test]
    fn labels_follow_selection_order_and_skip_unknown() {
        let all = all_options(&opts(), &[]);
        assert_eq!(selected_labels(&all, &sel(&["LDN", "zzz", "NY"])), ["London", "New York"]);
        assert!(selected_labels(&all, &[]).is_empty());
    }

    #[test]
    fn comma_label_table() {
        let l = |v: &[&str]| v.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        let t = "{0} items selected";
        assert_eq!(comma_label(&l(&["a", "b"]), Some(3), t, None, true), LabelContent::Joined("a, b".into()));
        assert_eq!(comma_label(&l(&["a", "b", "c"]), Some(3), t, None, true), LabelContent::Joined("a, b, c".into()));
        assert_eq!(comma_label(&l(&["a", "b", "c", "d"]), Some(3), t, None, true), LabelContent::Summary("4 items selected".into()));
        assert_eq!(comma_label(&l(&["a", "b", "c", "d"]), None, t, None, true), LabelContent::Joined("a, b, c, d".into()));
        assert_eq!(comma_label(&[], Some(3), t, Some("Pick"), true), LabelContent::Placeholder(Some("Pick".into())));
        assert_eq!(comma_label(&[], Some(3), t, Some(""), true), LabelContent::Placeholder(None));
        assert_eq!(comma_label(&l(&["a"]), Some(3), t, Some("Pick"), false), LabelContent::Placeholder(Some("Pick".into())));
    }

    #[test]
    fn chip_content_table() {
        let t = "{0} items selected";
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
        assert_eq!(chip_content(&s(&["a", "b"]), Some(3), t, None), ChipContent::Chips(s(&["a", "b"])));
        assert_eq!(
            chip_content(&s(&["a", "b", "c"]), Some(3), t, None),
            ChipContent::Summary("3 items selected".into()),
            "как в PrimeNG: len == max"
        );
        assert_eq!(chip_content(&s(&["a", "b", "c", "d"]), Some(3), t, None), ChipContent::Summary("4 items selected".into()));
        assert_eq!(chip_content(&s(&["a", "b", "c", "d"]), None, t, None), ChipContent::Chips(s(&["a", "b", "c", "d"])));
        assert_eq!(chip_content(&[], Some(3), t, Some("Pick")), ChipContent::Placeholder(Some("Pick".into())));
    }

    #[test]
    fn root_classes_and_data_p() {
        let mut cn = prime_core::ClassNames::new();
        write_root_classes(&mut cn, &RootState::default());
        assert_eq!(cn.finish(), "p-multiselect p-component p-inputwrapper");
        let full = RootState {
            chip: true,
            disabled: true,
            invalid: true,
            variant_filled: true,
            focused: true,
            filled: true,
            open: true,
            fluid: true,
            size: Some(prime_core::Size::Large),
        };
        let mut cn = prime_core::ClassNames::new();
        write_root_classes(&mut cn, &full);
        assert_eq!(
            cn.finish(),
            "p-multiselect p-component p-inputwrapper p-multiselect-display-chip p-disabled p-invalid p-variant-filled p-focus p-inputwrapper-filled p-inputwrapper-focus p-multiselect-open p-multiselect-fluid p-multiselect-lg p-inputfield-lg"
        );
        assert_eq!(data_p(&full), "invalid disabled focus fluid filled large");
        assert_eq!(data_p(&RootState::default()), "");
        let focus_only = RootState { open: true, ..RootState::default() };
        let mut cn = prime_core::ClassNames::new();
        write_root_classes(&mut cn, &focus_only);
        assert!(cn.finish().contains("p-inputwrapper-focus"), "открытая панель даёт p-inputwrapper-focus");
    }

    #[test]
    fn label_and_option_classes() {
        let mut cn = prime_core::ClassNames::new();
        write_label_classes(&mut cn, true, false);
        assert_eq!(cn.finish(), "p-multiselect-label p-placeholder");
        let mut cn = prime_core::ClassNames::new();
        write_label_classes(&mut cn, false, true);
        assert_eq!(cn.finish(), "p-multiselect-label p-multiselect-label-empty");
        let mut cn = prime_core::ClassNames::new();
        write_option_classes(&mut cn, true, true, true);
        assert_eq!(cn.finish(), "p-multiselect-option p-multiselect-option-selected p-focus p-disabled");
    }

    #[test]
    fn builders() {
        let o = MultiSelectOption::new("a", "b").disabled(true);
        assert_eq!((o.label.as_str(), o.value.as_str(), o.disabled), ("a", "b", true));
        assert_eq!(find_option(&opts(), &[], "LDN").unwrap().label, "London");
        assert_eq!(find_option(&[], &groups(), "K").unwrap().label, "Kyoto");
        assert_eq!(all_options(&[], &groups()).len(), 5);
        assert_eq!(all_options(&opts(), &[]).len(), 5);
    }
}
