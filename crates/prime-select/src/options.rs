//! Options, groups and pure Select list logic: flat list, filter, focus indices, type-ahead search
//! (`visibleOptions`, `findNextOptionIndex` and related PrimeNG functions).

/// Select option.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectOption {
    /// Option label.
    pub label: String,
    /// Option value.
    pub value: String,
    /// The option is disabled.
    pub disabled: bool,
}

impl SelectOption {
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
pub struct SelectGroup {
    /// Group title.
    pub label: String,
    /// Group options.
    pub items: Vec<SelectOption>,
}

impl SelectGroup {
    /// Group with a title and options.
    pub fn new(label: impl Into<String>, items: Vec<SelectOption>) -> Self {
        Self { label: label.into(), items }
    }
}

/// Flat list item: group header or option.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// Group header (`index` is the group number in the group list).
    Group {
        /// Group number.
        index: usize,
        /// Title.
        label: String,
    },
    /// Option.
    Option(SelectOption),
}

impl Entry {
    /// Whether the item is eligible for focus and selection: an option that is not disabled.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Option(o) if !o.disabled)
    }

    /// The option, if this is one.
    pub fn option(&self) -> Option<&SelectOption> {
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

/// Option with the given value among all options (no filter).
pub fn find_option(options: &[SelectOption], groups: &[SelectGroup], value: &str) -> Option<SelectOption> {
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

/// Index of the selected eligible option.
pub fn selected_index(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    let value = value?;
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| o.value == value))
}

/// Index for the first focus: the selected or the first eligible one (`findFirstFocusedOptionIndex`).
pub fn first_focused(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    selected_index(entries, value).or_else(|| first_valid(entries))
}

/// Index for focusing from the end: the selected or the last eligible one (`findLastFocusedOptionIndex`).
pub fn last_focused(entries: &[Entry], value: Option<&str>) -> Option<usize> {
    selected_index(entries, value).or_else(|| last_valid(entries))
}

/// First eligible option whose label starts with the typed string, case-insensitive (`searchOptions`).
pub fn search_match(entries: &[Entry], typed: &str) -> Option<usize> {
    let typed = typed.to_lowercase();
    entries.iter().position(|e| e.is_valid() && e.option().is_some_and(|o| o.label.to_lowercase().starts_with(&typed)))
}

/// Number of options without group headers (`aria-setsize`).
pub fn set_size(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.option().is_some()).count()
}

/// Option number among options without group headers (`aria-posinset`, one-based).
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
        // groups take precedence over flat options
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
        assert_eq!(next_valid(&flat, 0), 2, "disabled option skipped");
        assert_eq!(prev_valid(&flat, 2), 0);
        assert_eq!(next_valid(&flat, 4), 4, "nowhere further: index unchanged");
        assert_eq!(prev_valid(&flat, 0), 0);
        let grouped = visible_entries(&[], &groups(), "");
        assert_eq!(first_valid(&grouped), Some(1), "group header skipped");
        assert_eq!(next_valid(&grouped, 2), 4, "across the group header");
        assert_eq!(next_valid(&grouped, 4), 7, "disabled option skipped");
        assert_eq!(prev_valid(&grouped, 4), 2);
        assert_eq!(last_valid(&grouped), Some(7));
    }

    #[test]
    fn selected_and_first_focused_indexes() {
        let flat = visible_entries(&opts(), &[], "");
        assert_eq!(selected_index(&flat, Some("LDN")), Some(2));
        assert_eq!(selected_index(&flat, Some("RM")), None, "disabled selected is not searched");
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
        assert_eq!(search_match(&flat, "ro"), None, "disabled is not searched");
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
