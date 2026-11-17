//! Options, groups and pure Listbox logic: flat list, filter, focus indices, single and multiple selection,
//! ranges, type-ahead search (`visibleOptions`, `onOptionSelect*`, `searchOptions` and related PrimeNG functions).

/// Selectable option.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListboxOption {
    /// Option label.
    pub label: String,
    /// Option value.
    pub value: String,
    /// Option is disabled.
    pub disabled: bool,
}

impl ListboxOption {
    /// Enabled option with a label and value.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), disabled: false }
    }

    /// Disabled option.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Option group.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListboxGroup {
    /// Group header.
    pub label: String,
    /// Group options.
    pub items: Vec<ListboxOption>,
}

impl ListboxGroup {
    /// Group with a header and options.
    pub fn new(label: impl Into<String>, items: Vec<ListboxOption>) -> Self {
        Self { label: label.into(), items }
    }
}

/// Flat list item: a group header or an option.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// Group header (`index` is the group's number in the group list).
    Group {
        /// Group number.
        index: usize,
        /// Header.
        label: String,
    },
    /// Option.
    Option(ListboxOption),
}

impl Entry {
    /// Whether the item is eligible for focus and selection: an option that is not disabled.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Option(o) if !o.disabled)
    }

    /// The option, if this is one.
    pub fn option(&self) -> Option<&ListboxOption> {
        match self {
            Self::Option(o) => Some(o),
            Self::Group { .. } => None,
        }
    }
}

/// Whether the label matches the search string: case-insensitive "contains", leading and trailing whitespace is dropped.
pub fn matches_filter(label: &str, filter: &str) -> bool {
    let needle = filter.trim().to_lowercase();
    needle.is_empty() || label.to_lowercase().contains(&needle)
}

/// Flat list of visible items: options or groups with options, filtered by the search string.
/// Groups without matching options are omitted.
pub fn visible_entries(options: &[ListboxOption], groups: &[ListboxGroup], filter: &str) -> Vec<Entry> {
    if groups.is_empty() {
        return options.iter().filter(|o| matches_filter(&o.label, filter)).cloned().map(Entry::Option).collect();
    }
    let mut out = Vec::new();
    for (index, group) in groups.iter().enumerate() {
        let items: Vec<&ListboxOption> = group.items.iter().filter(|o| matches_filter(&o.label, filter)).collect();
        if !items.is_empty() {
            out.push(Entry::Group { index, label: group.label.clone() });
            out.extend(items.into_iter().cloned().map(Entry::Option));
        }
    }
    out
}

/// Option with the given value among all options (without the filter).
pub fn find_option(options: &[ListboxOption], groups: &[ListboxGroup], value: &str) -> Option<ListboxOption> {
    if groups.is_empty() {
        options.iter().find(|o| o.value == value).cloned()
    } else {
        groups.iter().flat_map(|g| g.items.iter()).find(|o| o.value == value).cloned()
    }
}

/// Index of the first eligible item.
pub fn first_valid(entries: &[Entry]) -> Option<usize> {
    entries.iter().position(Entry::is_valid)
}

/// Index of the last eligible item.
pub fn last_valid(entries: &[Entry]) -> Option<usize> {
    entries.iter().rposition(Entry::is_valid)
}

/// Index of the next eligible item after `index`; if there is none, the previous one (`findNextOptionIndex`).
pub fn next_valid(entries: &[Entry], index: usize) -> usize {
    entries.iter().enumerate().skip(index + 1).find(|(_, e)| e.is_valid()).map_or(index, |(i, _)| i)
}

/// Index of the previous eligible item before `index`; if there is none, the previous one (`findPrevOptionIndex`).
pub fn prev_valid(entries: &[Entry], index: usize) -> usize {
    entries.iter().enumerate().take(index).rev().find(|(_, e)| e.is_valid()).map_or(index, |(i, _)| i)
}

/// Whether the option with this value is selected.
fn is_selected(selected: &[String], value: &str) -> bool {
    selected.iter().any(|s| s == value)
}

/// Index of the first eligible selected option (`findSelectedOptionIndex`).
pub fn selected_index(entries: &[Entry], selected: &[String]) -> Option<usize> {
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value)))
}

/// Index for the first focus: the selected or the first eligible one (`findFirstFocusedOptionIndex`).
pub fn first_focused(entries: &[Entry], selected: &[String]) -> Option<usize> {
    selected_index(entries, selected).or_else(|| first_valid(entries))
}

/// Index for focusing from the end: the last selected eligible or the last eligible one (`findLastFocusedOptionIndex`).
pub fn last_focused(entries: &[Entry], selected: &[String]) -> Option<usize> {
    entries
        .iter()
        .rposition(|e| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value)))
        .or_else(|| last_valid(entries))
}

/// Selected option nearest to `index` (`findNearestSelectedOptionIndex`): with `first_check_up` up first, then down,
/// otherwise the reverse; if there are no selected ones, `index` itself.
pub fn nearest_selected(entries: &[Entry], selected: &[String], index: usize, first_check_up: bool) -> usize {
    let chosen = |e: &Entry| e.is_valid() && e.option().is_some_and(|o| is_selected(selected, &o.value));
    let prev = || entries.iter().enumerate().take(index).rev().find(|(_, e)| chosen(e)).map(|(i, _)| i);
    let next = || entries.iter().enumerate().skip(index + 1).find(|(_, e)| chosen(e)).map(|(i, _)| i);
    let found = if first_check_up { prev().or_else(next) } else { next().or_else(prev) };
    found.unwrap_or(index)
}

/// First eligible option whose label starts with the typed string, case-insensitive; the search goes from `from`
/// (inclusive) to the end, then from the start (`searchOptions`).
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

/// Number of options without group headers (`aria-setsize`).
pub fn set_size(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.option().is_some()).count()
}

/// Position of the option among options without group headers (`aria-posinset`, one-based).
pub fn pos_in_set(entries: &[Entry], index: usize) -> usize {
    entries.iter().take(index + 1).filter(|e| e.option().is_some()).count()
}

/// Single selection (`onOptionSelectSingle`). `None` means no change, `Some(v)` is the new value.
/// `meta` means `meta_key_selection` is on (and no touch), `meta_key` means `Ctrl`/`⌘` is pressed.
pub fn select_single(current: Option<&str>, value: &str, meta: bool, meta_key: bool) -> Option<Option<String>> {
    let selected = current == Some(value);
    if meta {
        if selected { meta_key.then_some(None) } else { Some(Some(value.to_owned())) }
    } else if selected {
        Some(None)
    } else {
        Some(Some(value.to_owned()))
    }
}

/// Multiple selection (`onOptionSelectMultiple`): the new list of values.
pub fn select_multiple(current: &[String], value: &str, meta: bool, meta_key: bool) -> Vec<String> {
    let selected = is_selected(current, value);
    let without = || current.iter().filter(|v| v.as_str() != value).cloned().collect::<Vec<_>>();
    if meta {
        if selected {
            if meta_key { without() } else { vec![value.to_owned()] }
        } else {
            let mut out = if meta_key { current.to_vec() } else { Vec::new() };
            out.push(value.to_owned());
            out
        }
    } else if selected {
        without()
    } else {
        let mut out = current.to_vec();
        out.push(value.to_owned());
        out
    }
}

/// Eligible options in the index range `start..=end` (bounds in any order), `onOptionSelectRange`.
pub fn select_range(entries: &[Entry], start: usize, end: usize) -> Vec<String> {
    let (lo, hi) = (start.min(end), start.max(end));
    entries
        .iter()
        .enumerate()
        .filter(|(i, e)| (lo..=hi).contains(i) && e.is_valid())
        .filter_map(|(_, e)| e.option().map(|o| o.value.clone()))
        .collect()
}

/// All eligible visible options (`Ctrl+A`, "select all").
pub fn select_all(entries: &[Entry]) -> Vec<String> {
    entries.iter().filter(|e| e.is_valid()).filter_map(|e| e.option().map(|o| o.value.clone())).collect()
}

/// Whether all visible options except disabled ones are selected (`allSelected`); an empty list means no.
pub fn all_selected(entries: &[Entry], selected: &[String]) -> bool {
    !entries.is_empty() && entries.iter().all(|e| !e.is_valid() || e.option().is_some_and(|o| is_selected(selected, &o.value)))
}

/// Root classes in the order of `listboxstyle.ts`.
pub fn write_root_classes(cn: &mut prime_rs_core::ClassNames, striped: bool, disabled: bool, invalid: bool, fluid: bool) {
    cn.add("p-listbox p-component")
        .add_if(striped, "p-listbox-striped")
        .add_if(disabled, "p-disabled")
        .add_if(invalid, "p-invalid")
        .add_if(fluid, "p-listbox-fluid");
}

/// Option classes.
pub fn write_option_classes(cn: &mut prime_rs_core::ClassNames, highlighted: bool, focused: bool, disabled: bool) {
    cn.add("p-listbox-option").add_if(highlighted, "p-listbox-option-selected").add_if(focused, "p-focus").add_if(disabled, "p-disabled");
}

/// Value of the root `data-p`: `invalid`, `disabled`.
pub fn data_p(invalid: bool, disabled: bool) -> std::borrow::Cow<'static, str> {
    let mut d = prime_rs_core::DataP::new();
    d.add_if(invalid, "invalid").add_if(disabled, "disabled");
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<ListboxOption> {
        vec![
            ListboxOption::new("New York", "NY"),
            ListboxOption::new("Rome", "RM").disabled(true),
            ListboxOption::new("London", "LDN"),
            ListboxOption::new("Istanbul", "IST"),
            ListboxOption::new("Paris", "PRS"),
        ]
    }

    fn groups() -> Vec<ListboxGroup> {
        vec![
            ListboxGroup::new("Germany", vec![ListboxOption::new("Berlin", "B"), ListboxOption::new("Frankfurt", "F")]),
            ListboxGroup::new("USA", vec![ListboxOption::new("Chicago", "C"), ListboxOption::new("Los Angeles", "LA").disabled(true)]),
            ListboxGroup::new("Japan", vec![ListboxOption::new("Kyoto", "K")]),
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
        assert!(visible_entries(&[], &groups(), "zzz").is_empty());
        assert_eq!(visible_entries(&opts(), &groups(), "").len(), 8, "groups take precedence over flat options");
    }

    #[test]
    fn navigation_skips_disabled_options_and_group_headers() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!((first_valid(&flat), last_valid(&flat)), (Some(0), Some(4)));
        assert_eq!(next_valid(&flat, 0), 2, "disabled option is skipped");
        assert_eq!(prev_valid(&flat, 2), 0);
        assert_eq!(next_valid(&flat, 4), 4, "nowhere further, index unchanged");
        assert_eq!(prev_valid(&flat, 0), 0);
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(first_valid(&grouped), Some(1), "group header is skipped");
        assert_eq!(next_valid(&grouped, 2), 4, "across a group header");
        assert_eq!(next_valid(&grouped, 4), 7, "disabled option is skipped");
        assert_eq!(last_valid(&grouped), Some(7));
    }

    #[test]
    fn focus_indexes_follow_selection() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(selected_index(&flat, &sel(&["LDN"])), Some(2));
        assert_eq!(selected_index(&flat, &sel(&["RM"])), None, "disabled selected one is not searched");
        assert_eq!(selected_index(&flat, &[]), None);
        assert_eq!(selected_index(&flat, &sel(&["PRS", "IST"])), Some(3), "first in list order");
        assert_eq!(first_focused(&flat, &sel(&["IST"])), Some(3));
        assert_eq!(first_focused(&flat, &sel(&["nope"])), Some(0));
        assert_eq!(last_focused(&flat, &[]), Some(4));
        assert_eq!(last_focused(&flat, &sel(&["NY", "LDN"])), Some(2));
        assert_eq!(first_focused(&[], &sel(&["x"])), None);
    }

    #[test]
    fn nearest_selected_checks_requested_direction_first() {
        let flat = visible_entries(&opts(), &[], "");
        let s = sel(&["NY", "PRS"]);
        assert_eq!(nearest_selected(&flat, &s, 2, true), 0, "up first");
        assert_eq!(nearest_selected(&flat, &s, 2, false), 4, "down first");
        assert_eq!(nearest_selected(&flat, &s, 4, false), 0, "nothing down, so up");
        assert_eq!(nearest_selected(&flat, &[], 2, true), 2, "no selected ones, same index");
    }

    #[test]
    fn type_ahead_starts_at_focus_and_wraps() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(search_match_from(&flat, "lo", None), Some(2));
        assert_eq!(search_match_from(&flat, "LON", Some(3)), Some(2), "after the end, from the start");
        assert_eq!(search_match_from(&flat, "p", Some(0)), Some(4));
        assert_eq!(search_match_from(&flat, "ro", None), None, "disabled one is not searched");
        assert_eq!(search_match_from(&flat, "zz", Some(1)), None);
        assert_eq!(search_match_from(&flat, "n", Some(0)), Some(0), "from the focused one inclusive");
    }

    #[test]
    fn aria_set_and_position_ignore_group_headers() {
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(set_size(&grouped), 5);
        assert_eq!(pos_in_set(&grouped, 1), 1);
        assert_eq!(pos_in_set(&grouped, 4), 3);
        assert_eq!(pos_in_set(&grouped, 7), 5);
    }

    #[test]
    fn single_selection_table() {
        // (current, value, meta, meta_key) -> result
        type Case = (Option<&'static str>, &'static str, bool, bool, Option<Option<&'static str>>);
        let cases: [Case; 8] = [
            (None, "a", false, false, Some(Some("a"))),
            (Some("a"), "a", false, false, Some(None)),
            (Some("b"), "a", false, false, Some(Some("a"))),
            (Some("a"), "a", true, false, None),
            (Some("a"), "a", true, true, Some(None)),
            (None, "a", true, false, Some(Some("a"))),
            (Some("b"), "a", true, true, Some(Some("a"))),
            (None, "a", false, true, Some(Some("a"))),
        ];
        for (current, value, meta, meta_key, expected) in cases {
            let expected = expected.map(|o| o.map(str::to_owned));
            assert_eq!(select_single(current, value, meta, meta_key), expected, "{current:?} {value} {meta} {meta_key}");
        }
    }

    #[test]
    fn multiple_selection_table() {
        assert_eq!(select_multiple(&sel(&["a"]), "b", false, false), sel(&["a", "b"]));
        assert_eq!(select_multiple(&sel(&["a", "b"]), "a", false, false), sel(&["b"]));
        assert_eq!(select_multiple(&[], "a", false, false), sel(&["a"]));
        // meta: without Ctrl replaces, with Ctrl adds or removes
        assert_eq!(select_multiple(&sel(&["a", "b"]), "c", true, false), sel(&["c"]));
        assert_eq!(select_multiple(&sel(&["a", "b"]), "c", true, true), sel(&["a", "b", "c"]));
        assert_eq!(select_multiple(&sel(&["a", "b"]), "a", true, false), sel(&["a"]), "keeps only the clicked one");
        assert_eq!(select_multiple(&sel(&["a", "b"]), "a", true, true), sel(&["b"]));
    }

    #[test]
    fn range_and_select_all_skip_disabled_and_headers() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(select_range(&flat, 0, 3), sel(&["NY", "LDN", "IST"]));
        assert_eq!(select_range(&flat, 3, 0), sel(&["NY", "LDN", "IST"]), "bound order does not matter");
        assert_eq!(select_range(&flat, 2, 2), sel(&["LDN"]));
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(select_range(&grouped, 0, 4), sel(&["B", "F", "C"]));
        assert_eq!(select_all(&grouped), sel(&["B", "F", "C", "K"]));
        assert_eq!(select_all(&flat), sel(&["NY", "LDN", "IST", "PRS"]));
        assert!(select_all(&[]).is_empty());
    }

    #[test]
    fn all_selected_ignores_disabled() {
        let flat = visible_entries(&opts(), &[], "");
        assert!(all_selected(&flat, &sel(&["NY", "LDN", "IST", "PRS"])));
        assert!(!all_selected(&flat, &sel(&["NY", "LDN"])));
        assert!(!all_selected(&[], &[]), "empty list, not all");
        let filtered = visible_entries(&opts(), &[], "n");
        assert!(all_selected(&filtered, &sel(&["NY", "LDN", "IST"])));
    }

    #[test]
    fn classes_and_data_p() {
        let mut cn = prime_rs_core::ClassNames::new();
        write_root_classes(&mut cn, true, true, true, true);
        assert_eq!(cn.finish(), "p-listbox p-component p-listbox-striped p-disabled p-invalid p-listbox-fluid");
        let mut cn = prime_rs_core::ClassNames::new();
        write_root_classes(&mut cn, false, false, false, false);
        assert_eq!(cn.finish(), "p-listbox p-component");
        let mut cn = prime_rs_core::ClassNames::new();
        write_option_classes(&mut cn, true, true, true);
        assert_eq!(cn.finish(), "p-listbox-option p-listbox-option-selected p-focus p-disabled");
        assert_eq!(data_p(true, true), "invalid disabled");
        assert_eq!(data_p(false, false), "");
        assert_eq!(data_p(false, true), "disabled");
    }

    #[test]
    fn option_builders() {
        let o = ListboxOption::new("a", "b").disabled(true);
        assert_eq!((o.label.as_str(), o.value.as_str(), o.disabled), ("a", "b", true));
        assert!(!ListboxOption::new("a", "b").disabled);
        assert_eq!(ListboxGroup::new("g", vec![]).label, "g");
        assert_eq!(find_option(&opts(), &[], "LDN").unwrap().label, "London");
        assert_eq!(find_option(&[], &groups(), "K").unwrap().label, "Kyoto");
        assert!(find_option(&opts(), &[], "none").is_none());
    }
}
