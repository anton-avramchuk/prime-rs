//! Pure Table model: sorting, row selection, expansion (after PrimeNG `table.ts`, no DOM).
//!
//! Rows are identified by keys (`String`), like PrimeNG's `dataKey`; selection and expansion store keys.

use std::cmp::Ordering;

/// Sort mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SortMode {
    /// By a single column (default).
    #[default]
    Single,
    /// By several columns, added with the Ctrl/⌘ key.
    Multiple,
}

/// Row selection mode on click.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionMode {
    /// One row.
    Single,
    /// Several rows.
    Multiple,
}

/// A column and its sort order (1 - ascending, -1 - descending).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SortMeta {
    /// Column field.
    pub field: String,
    /// Order: 1 or -1.
    pub order: i32,
}

impl SortMeta {
    /// Sort by field.
    pub fn new(field: impl Into<String>, order: i32) -> Self {
        Self { field: field.into(), order }
    }
}

/// Sort order of a column: 0 if it is not sorted by.
pub fn order_of(meta: &[SortMeta], field: &str) -> i32 {
    meta.iter().find(|m| m.field == field).map_or(0, |m| m.order)
}

/// Column number in a multi-level sort (starting at 1) for the counter badge; single sort has no badge.
pub fn badge_of(mode: SortMode, meta: &[SortMeta], field: &str) -> Option<usize> {
    if mode == SortMode::Multiple && meta.len() > 1 { meta.iter().position(|m| m.field == field).map(|i| i + 1) } else { None }
}

/// New sort after a click on the `field` header (`sort`).
pub fn toggle_sort(mode: SortMode, meta: &[SortMeta], field: &str, meta_key: bool, default_order: i32) -> Vec<SortMeta> {
    match mode {
        SortMode::Single => {
            let order = match meta.first() {
                Some(m) if m.field == field => -m.order,
                _ => default_order,
            };
            vec![SortMeta::new(field, order)]
        }
        SortMode::Multiple => match meta.iter().find(|m| m.field == field) {
            Some(existing) if !meta_key => vec![SortMeta::new(field, -existing.order)],
            Some(_) => meta.iter().map(|m| if m.field == field { SortMeta::new(field, -m.order) } else { m.clone() }).collect(),
            None if !meta_key || meta.is_empty() => vec![SortMeta::new(field, default_order)],
            None => {
                let mut next = meta.to_vec();
                next.push(SortMeta::new(field, default_order));
                next
            }
        },
    }
}

/// Rows in sort order: stable, multi-level (`multisortField`); `compare` compares by field.
pub fn sort_rows<T: Clone>(rows: &[T], meta: &[SortMeta], compare: &dyn Fn(&str, &T, &T) -> Ordering) -> Vec<T> {
    let mut out = rows.to_vec();
    if meta.is_empty() {
        return out;
    }
    out.sort_by(|a, b| {
        for m in meta {
            let ordering = compare(&m.field, a, b);
            let ordering = if m.order < 0 { ordering.reverse() } else { ordering };
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    });
    out
}

/// Result of a row click: the new selection and the "row is selected" flag (for the select/unselect event).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowClick {
    /// New selection.
    pub selection: Vec<String>,
    /// The clicked row is now selected.
    pub selected: bool,
}

/// Selection on row click (`handleRowClick` without Shift range).
pub fn row_click(mode: SelectionMode, selection: &[String], key: &str, meta_selection: bool, meta_key: bool) -> RowClick {
    let was = selection.iter().any(|k| k == key);
    let without = || selection.iter().filter(|k| k.as_str() != key).cloned().collect::<Vec<_>>();
    let selection = if meta_selection {
        if was && meta_key {
            match mode {
                SelectionMode::Single => Vec::new(),
                SelectionMode::Multiple => without(),
            }
        } else {
            match mode {
                SelectionMode::Single => vec![key.to_owned()],
                SelectionMode::Multiple => {
                    let mut next = if meta_key { selection.to_vec() } else { Vec::new() };
                    next.push(key.to_owned());
                    next
                }
            }
        }
    } else {
        match (mode, was) {
            (SelectionMode::Single, true) => Vec::new(),
            (SelectionMode::Single, false) => vec![key.to_owned()],
            (SelectionMode::Multiple, true) => without(),
            (SelectionMode::Multiple, false) => {
                let mut next = selection.to_vec();
                next.push(key.to_owned());
                next
            }
        }
    };
    let selected = selection.iter().any(|k| k == key);
    RowClick { selection, selected }
}

/// All `keys` rows are selected (header checkbox).
pub fn all_selected(keys: &[String], selection: &[String]) -> bool {
    !keys.is_empty() && keys.iter().all(|k| selection.contains(k))
}

/// Selection after the header checkbox (`toggleRowsWithCheckbox`): when `check`, adds `keys`; with `page_only` the rest of the selection is kept.
pub fn toggle_all(keys: &[String], selection: &[String], check: bool, page_only: bool) -> Vec<String> {
    let mut next: Vec<String> = if page_only { selection.iter().filter(|k| !keys.contains(k)).cloned().collect() } else { Vec::new() };
    if check {
        next.extend(keys.iter().cloned());
    }
    next
}

/// Row expansion (`toggleRow`): in `single` mode only one row is expanded.
pub fn toggle_expanded(expanded: &[String], key: &str, single: bool) -> (Vec<String>, bool) {
    if expanded.iter().any(|k| k == key) {
        (expanded.iter().filter(|k| k.as_str() != key).cloned().collect(), false)
    } else {
        let mut next = if single { Vec::new() } else { expanded.to_vec() };
        next.push(key.to_owned());
        (next, true)
    }
}

/// Comparison of rows with gaps first (`null` is the smallest) for a column whose value is an `Option`.
pub fn compare_option<T: Ord>(a: &Option<T>, b: &Option<T>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.cmp(b),
    }
}

/// Filter match mode (PrimeNG `FilterMatchMode`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchMode {
    /// Starts with (default).
    #[default]
    StartsWith,
    /// Contains.
    Contains,
    /// Does not contain.
    NotContains,
    /// Ends with.
    EndsWith,
    /// Equals.
    Equals,
    /// Does not equal.
    NotEquals,
    /// Less than (numbers).
    Lt,
    /// Less than or equal (numbers).
    Lte,
    /// Greater than (numbers).
    Gt,
    /// Greater than or equal (numbers).
    Gte,
}

/// Column filter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilterMeta {
    /// Column field.
    pub field: String,
    /// Search value.
    pub value: String,
    /// Match mode.
    pub match_mode: MatchMode,
}

/// Whether the cell text matches the filter value. Strings are compared case-insensitively; `Equals`/`NotEquals` and order
/// modes compare numbers as numbers if both values are numeric.
pub fn matches(text: &str, value: &str, mode: MatchMode) -> bool {
    let (text, value) = (text.trim().to_lowercase(), value.trim().to_lowercase());
    let numbers = || Some((text.parse::<f64>().ok()?, value.parse::<f64>().ok()?));
    match mode {
        MatchMode::StartsWith => text.starts_with(&value),
        MatchMode::Contains => text.contains(&value),
        MatchMode::NotContains => !text.contains(&value),
        MatchMode::EndsWith => text.ends_with(&value),
        MatchMode::Equals => numbers().map_or(text == value, |(a, b)| a == b),
        MatchMode::NotEquals => numbers().map_or(text != value, |(a, b)| a != b),
        MatchMode::Lt => numbers().is_some_and(|(a, b)| a < b),
        MatchMode::Lte => numbers().is_some_and(|(a, b)| a <= b),
        MatchMode::Gt => numbers().is_some_and(|(a, b)| a > b),
        MatchMode::Gte => numbers().is_some_and(|(a, b)| a >= b),
    }
}

/// Filters after entering a value in `field`: an empty value clears the column filter.
pub fn set_filter(filters: &[FilterMeta], field: &str, value: &str, mode: MatchMode) -> Vec<FilterMeta> {
    let mut next: Vec<FilterMeta> = filters.iter().filter(|f| f.field != field).cloned().collect();
    if !value.trim().is_empty() {
        let at = filters.iter().position(|f| f.field == field).unwrap_or(next.len()).min(next.len());
        next.insert(at, FilterMeta { field: field.to_owned(), value: value.to_owned(), match_mode: mode });
    }
    next
}

/// Column filter value (empty string if there is no filter).
pub fn filter_value<'a>(filters: &'a [FilterMeta], field: &str) -> &'a str {
    filters.iter().find(|f| f.field == field).map_or("", |f| f.value.as_str())
}

/// Rows matching all column filters and the global filter (`Contains` mode on any of `global_fields`).
/// `text_of(field, row)` gives the text of a row field.
pub fn filter_rows<T: Clone>(
    rows: &[T],
    filters: &[FilterMeta],
    global: &str,
    global_fields: &[String],
    text_of: &dyn Fn(&str, &T) -> String,
) -> Vec<T> {
    let global = global.trim();
    if filters.is_empty() && global.is_empty() {
        return rows.to_vec();
    }
    rows.iter()
        .filter(|row| {
            filters.iter().all(|f| matches(&text_of(&f.field, row), &f.value, f.match_mode))
                && (global.is_empty() || global_fields.iter().any(|field| matches(&text_of(field, row), global, MatchMode::Contains)))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    fn m(field: &str, order: i32) -> SortMeta {
        SortMeta::new(field, order)
    }

    #[test]
    fn single_sort_cycle() {
        let first = toggle_sort(SortMode::Single, &[], "a", false, 1);
        assert_eq!(first, [m("a", 1)]);
        let second = toggle_sort(SortMode::Single, &first, "a", false, 1);
        assert_eq!(second, [m("a", -1)]);
        assert_eq!(toggle_sort(SortMode::Single, &second, "a", false, 1), [m("a", 1)]);
        assert_eq!(toggle_sort(SortMode::Single, &second, "b", false, 1), [m("b", 1)], "other column: default order");
        assert_eq!(toggle_sort(SortMode::Single, &second, "b", false, -1), [m("b", -1)]);
    }

    #[test]
    fn multiple_sort_table() {
        use SortMode::Multiple;
        assert_eq!(toggle_sort(Multiple, &[], "a", false, 1), [m("a", 1)]);
        assert_eq!(toggle_sort(Multiple, &[m("a", 1)], "b", false, 1), [m("b", 1)], "without key: starts over");
        assert_eq!(toggle_sort(Multiple, &[m("a", 1)], "b", true, 1), [m("a", 1), m("b", 1)], "with key: appended");
        assert_eq!(toggle_sort(Multiple, &[m("a", 1), m("b", 1)], "a", false, 1), [m("a", -1)], "without key a single column remains");
        assert_eq!(
            toggle_sort(Multiple, &[m("a", 1), m("b", 1)], "a", true, 1),
            [m("a", -1), m("b", 1)],
            "with key the order changes in place"
        );
        assert_eq!(toggle_sort(Multiple, &[], "a", true, -1), [m("a", -1)]);
    }

    #[test]
    fn order_and_badge() {
        let meta = [m("a", 1), m("b", -1)];
        assert_eq!((order_of(&meta, "a"), order_of(&meta, "b"), order_of(&meta, "c")), (1, -1, 0));
        assert_eq!(badge_of(SortMode::Multiple, &meta, "b"), Some(2));
        assert_eq!(badge_of(SortMode::Multiple, &meta[..1], "a"), None, "one column: no counter");
        assert_eq!(badge_of(SortMode::Single, &meta, "a"), None);
        assert_eq!(badge_of(SortMode::Multiple, &meta, "c"), None);
    }

    #[test]
    fn sort_rows_is_multilevel_and_stable() {
        let rows = vec![(2, 'b'), (1, 'z'), (2, 'a'), (1, 'a'), (3, 'c')];
        let cmp = |field: &str, a: &(i32, char), b: &(i32, char)| if field == "n" { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) };
        assert_eq!(sort_rows(&rows, &[m("n", 1)], &cmp), [(1, 'z'), (1, 'a'), (2, 'b'), (2, 'a'), (3, 'c')], "stable");
        assert_eq!(sort_rows(&rows, &[m("n", -1), m("c", 1)], &cmp), [(3, 'c'), (2, 'a'), (2, 'b'), (1, 'a'), (1, 'z')]);
        assert_eq!(sort_rows(&rows, &[], &cmp), rows);
    }

    #[test]
    fn row_click_without_meta_key_selection() {
        use SelectionMode::*;
        let r = row_click(Single, &[], "a", false, false);
        assert_eq!((r.selection, r.selected), (keys("a"), true));
        let r = row_click(Single, &keys("a"), "b", false, false);
        assert_eq!(r.selection, keys("b"));
        let r = row_click(Single, &keys("a"), "a", false, false);
        assert_eq!((r.selection, r.selected), (vec![], false), "repeated click deselects");
        let r = row_click(Multiple, &keys("a"), "b", false, false);
        assert_eq!(r.selection, keys("a b"));
        let r = row_click(Multiple, &keys("a b"), "a", false, false);
        assert_eq!((r.selection, r.selected), (keys("b"), false));
    }

    #[test]
    fn row_click_with_meta_key_selection() {
        use SelectionMode::*;
        let r = row_click(Multiple, &keys("a b"), "c", true, false);
        assert_eq!(r.selection, keys("c"), "without key the selection is replaced");
        let r = row_click(Multiple, &keys("a b"), "c", true, true);
        assert_eq!(r.selection, keys("a b c"), "with key it is extended");
        let r = row_click(Multiple, &keys("a b"), "a", true, true);
        assert_eq!((r.selection, r.selected), (keys("b"), false), "with key a selected row is deselected");
        let r = row_click(Multiple, &keys("a b"), "a", true, false);
        assert_eq!(r.selection, keys("a"), "without key only the clicked one remains");
        let r = row_click(Single, &keys("a"), "a", true, true);
        assert!(r.selection.is_empty() && !r.selected);
        let r = row_click(Single, &keys("a"), "b", true, false);
        assert_eq!(r.selection, keys("b"));
    }

    #[test]
    fn header_checkbox() {
        let all = keys("a b c");
        assert!(all_selected(&all, &keys("c b a x")));
        assert!(!all_selected(&all, &keys("a b")));
        assert!(!all_selected(&[], &keys("a")), "an empty set is not considered selected");
        assert_eq!(toggle_all(&all, &[], true, false), all);
        assert_eq!(toggle_all(&all, &keys("a b c"), false, false), Vec::<String>::new());
        assert_eq!(toggle_all(&keys("a b"), &keys("c x"), true, true), keys("c x a b"), "with page_only the rest of the selection is kept");
        assert_eq!(toggle_all(&keys("a b"), &keys("a c"), false, true), keys("c"));
        assert_eq!(toggle_all(&keys("a b"), &keys("a c"), true, false), keys("a b"), "without page_only the selection is replaced");
    }

    #[test]
    fn expansion() {
        let (e, open) = toggle_expanded(&[], "a", false);
        assert_eq!((e.clone(), open), (keys("a"), true));
        let (e2, open) = toggle_expanded(&e, "b", false);
        assert_eq!((e2.clone(), open), (keys("a b"), true));
        let (e3, open) = toggle_expanded(&e2, "a", false);
        assert_eq!((e3, open), (keys("b"), false));
        let (e4, open) = toggle_expanded(&e2, "c", true);
        assert_eq!((e4, open), (keys("c"), true), "single mode");
    }

    #[test]
    fn option_ordering() {
        assert_eq!(compare_option(&None, &Some(1)), Ordering::Less);
        assert_eq!(compare_option(&Some(1), &None), Ordering::Greater);
        assert_eq!(compare_option::<i32>(&None, &None), Ordering::Equal);
        assert_eq!(compare_option(&Some(1), &Some(2)), Ordering::Less);
    }

    #[test]
    fn match_modes() {
        use MatchMode::*;
        for (text, value, mode, want) in [
            ("Bamboo Watch", "bam", StartsWith, true),
            ("Bamboo Watch", "watch", StartsWith, false),
            ("Bamboo Watch", "boo W", Contains, true),
            ("Bamboo Watch", "x", NotContains, true),
            ("Bamboo Watch", "WATCH", EndsWith, true),
            ("Bamboo", "bamboo", Equals, true),
            ("10", "10.0", Equals, true),
            ("10", "9", NotEquals, true),
            ("abc", "ABD", NotEquals, true),
            ("5", "10", Lt, true),
            ("10", "10", Lt, false),
            ("10", "10", Lte, true),
            ("11", "10", Gt, true),
            ("10", "10", Gte, true),
            ("abc", "10", Gt, false),
        ] {
            assert_eq!(matches(text, value, mode), want, "{text} {mode:?} {value}");
        }
    }

    #[test]
    fn set_and_read_filters() {
        let a = set_filter(&[], "name", "bam", MatchMode::StartsWith);
        let b = set_filter(&a, "code", "f2", MatchMode::Contains);
        assert_eq!(b.len(), 2);
        let c = set_filter(&b, "name", "blue", MatchMode::StartsWith);
        assert_eq!((c[0].field.as_str(), filter_value(&c, "name")), ("name", "blue"), "order is preserved");
        let d = set_filter(&c, "name", "  ", MatchMode::StartsWith);
        assert_eq!(d.len(), 1, "an empty value clears the filter");
        assert_eq!(filter_value(&d, "name"), "");
    }

    #[test]
    fn filtering_rows() {
        let rows = vec![("Bamboo Watch", "Accessories", 65), ("Black Watch", "Accessories", 72), ("Blue Band", "Fitness", 79)];
        let text = |field: &str, r: &(&str, &str, i32)| match field {
            "name" => r.0.to_owned(),
            "category" => r.1.to_owned(),
            "price" => r.2.to_string(),
            _ => String::new(),
        };
        let f = |filters: &[FilterMeta], global: &str| {
            filter_rows(&rows, filters, global, &["name".to_owned(), "category".to_owned()], &text).iter().map(|r| r.0).collect::<Vec<_>>()
        };
        assert_eq!(f(&[], "").len(), 3);
        let by_name = set_filter(&[], "name", "bl", MatchMode::StartsWith);
        assert_eq!(f(&by_name, ""), ["Black Watch", "Blue Band"]);
        let both = set_filter(&by_name, "price", "75", MatchMode::Lt);
        assert_eq!(f(&both, ""), ["Black Watch"], "conditions are combined with AND");
        assert_eq!(f(&[], "fit"), ["Blue Band"], "global filter: any of the fields");
        assert_eq!(f(&by_name, "watch"), ["Black Watch"], "global and column filters together");
        assert!(f(&[], "zzz").is_empty());
    }
}
