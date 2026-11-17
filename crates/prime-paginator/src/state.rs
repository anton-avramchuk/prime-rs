//! Pure Paginator logic: page count, link window, report, lists, step back (after PrimeNG `paginator.ts`).

use prime_rs_core::ClassNames;

/// Option of the "rows per page" list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowsPerPage {
    /// Number of rows.
    Rows(usize),
    /// "Show all" option with a label; the value is `total_records`.
    ShowAll(String),
}

impl From<usize> for RowsPerPage {
    fn from(rows: usize) -> Self {
        Self::Rows(rows)
    }
}

/// Pagination state passed to events and templates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PaginatorState {
    /// Zero-based page number.
    pub page: usize,
    /// Index of the first record.
    pub first: usize,
    /// Rows per page.
    pub rows: usize,
    /// Page count.
    pub page_count: usize,
    /// Total records.
    pub total_records: usize,
}

/// Page count; zero when `rows == 0` (no division by zero).
pub fn page_count(total: usize, rows: usize) -> usize {
    if rows == 0 { 0 } else { total.div_ceil(rows) }
}

/// Zero-based number of the current page.
pub fn page(first: usize, rows: usize) -> usize {
    first.checked_div(rows).unwrap_or(0)
}

/// Current page counted from one; zero for an empty list.
pub fn current_page(first: usize, rows: usize, total: usize) -> usize {
    if page_count(total, rows) > 0 { page(first, rows) + 1 } else { 0 }
}

/// Bounds of the link window `(start, end)` zero-based inclusive (`calculatePageLinkBoundaries`); `None` when there are no pages.
pub fn link_bounds(page: usize, page_count: usize, link_size: usize) -> Option<(usize, usize)> {
    if page_count == 0 || link_size == 0 {
        return None;
    }
    let visible = link_size.min(page_count);
    let start = (page as f64 - visible as f64 / 2.0).ceil().max(0.0) as usize;
    let end = (page_count - 1).min(start + visible - 1);
    let delta = link_size as i64 - (end as i64 - start as i64 + 1);
    let start = (start as i64 - delta).max(0) as usize;
    Some((start, end))
}

/// Page link numbers (counted from one).
pub fn links(page: usize, page_count: usize, link_size: usize) -> Vec<usize> {
    link_bounds(page, page_count, link_size).map_or_else(Vec::new, |(start, end)| (start + 1..=end + 1).collect())
}

/// Substitutes the report placeholders; each is replaced once, like `String.replace` with a string.
pub fn report(template: &str, first: usize, rows: usize, total: usize) -> String {
    let count = page_count(total, rows);
    let first_shown = if total > 0 { first + 1 } else { 0 };
    template
        .replacen("{currentPage}", &current_page(first, rows, total).to_string(), 1)
        .replacen("{totalPages}", &count.to_string(), 1)
        .replacen("{first}", &first_shown.to_string(), 1)
        .replacen("{last}", &(first + rows).min(total).to_string(), 1)
        .replacen("{rows}", &rows.to_string(), 1)
        .replacen("{totalRecords}", &total.to_string(), 1)
}

/// Options of the rows-per-page list `(label, value)`: numbers, then "show all" with the value `total`.
pub fn rows_items(options: &[RowsPerPage], total: usize) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = options
        .iter()
        .filter_map(|o| match o {
            RowsPerPage::Rows(n) => Some((n.to_string(), n.to_string())),
            RowsPerPage::ShowAll(_) => None,
        })
        .collect();
    if let Some(label) = options.iter().find_map(|o| if let RowsPerPage::ShowAll(l) = o { Some(l.clone()) } else { None }) {
        out.push((label, total.to_string()));
    }
    out
}

/// Options of the jump list: label "1"…"N", value is the page index.
pub fn page_items(count: usize) -> Vec<(String, String)> {
    (0..count).map(|i| ((i + 1).to_string(), i.to_string())).collect()
}

/// After the record count shrinks: which page to step back to if `first` went out of range (`updateFirst`).
///
/// PrimeNG goes to `page − 1` and does nothing if that page does not exist either; here the target is additionally
/// clamped to the last existing page, so the state always returns within range.
pub fn retreat_page(first: usize, rows: usize, total: usize) -> Option<usize> {
    let p = page(first, rows);
    let count = page_count(total, rows);
    (p > 0 && total > 0 && first >= total && count > 0).then(|| (p - 1).min(count - 1))
}

/// After the rows count changes: the page on which the previous `first` stays, no further than the last existing one.
pub fn page_after_rows_change(first: usize, rows: usize, total: usize) -> Option<usize> {
    let count = page_count(total, rows);
    (count > 0).then(|| page(first, rows).min(count - 1))
}

/// Navigation button classes (`first`, `prev`, `next`, `last`): the base class and `p-disabled`.
pub fn write_nav_classes(cn: &mut ClassNames, base: &str, disabled: bool) {
    cn.add(base).add_if(disabled, "p-disabled");
}

/// Page link classes.
pub fn write_page_classes(cn: &mut ClassNames, selected: bool) {
    cn.add("p-paginator-page").add_if(selected, "p-paginator-page-selected");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_arithmetic() {
        assert_eq!(page_count(120, 10), 12);
        assert_eq!(page_count(121, 10), 13);
        assert_eq!(page_count(0, 10), 0);
        assert_eq!(page_count(50, 0), 0, "rows = 0 does not divide by zero");
        assert_eq!((page(0, 10), page(25, 10), page(30, 10), page(5, 0)), (0, 2, 3, 0));
        assert_eq!((current_page(20, 10, 120), current_page(0, 10, 0), current_page(0, 0, 50)), (3, 0, 0));
    }

    #[test]
    fn link_window_table() {
        // (page, page count, window size) -> links
        let cases: [(usize, usize, usize, &[usize]); 11] = [
            (0, 12, 5, &[1, 2, 3, 4, 5]),
            (1, 12, 5, &[1, 2, 3, 4, 5]),
            (2, 12, 5, &[1, 2, 3, 4, 5]),
            (3, 12, 5, &[2, 3, 4, 5, 6]),
            (5, 12, 5, &[4, 5, 6, 7, 8]),
            (9, 12, 5, &[8, 9, 10, 11, 12]),
            (11, 12, 5, &[8, 9, 10, 11, 12]),
            (0, 3, 5, &[1, 2, 3]),
            (2, 3, 5, &[1, 2, 3]),
            (0, 1, 5, &[1]),
            (4, 10, 1, &[5]),
        ];
        for (page, count, size, expected) in cases {
            assert_eq!(links(page, count, size), expected, "page={page} count={count} size={size}");
        }
        assert!(links(0, 0, 5).is_empty());
        assert!(links(0, 5, 0).is_empty());
        assert_eq!(link_bounds(5, 12, 5), Some((3, 7)));
    }

    #[test]
    fn report_substitutes_each_placeholder_once() {
        assert_eq!(report("{currentPage} of {totalPages}", 0, 10, 120), "1 of 12");
        assert_eq!(report("Showing {first} to {last} of {totalRecords}", 10, 10, 120), "Showing 11 to 20 of 120");
        assert_eq!(report("{first} - {last} of {totalRecords}", 110, 10, 115), "111 - 115 of 115", "the last page is truncated");
        assert_eq!(report("{rows} rows", 0, 25, 100), "25 rows");
        assert_eq!(report("{currentPage}/{totalPages}", 0, 10, 0), "0/0", "empty");
        assert_eq!(report("{first}-{last}", 0, 10, 0), "0-0");
        assert_eq!(report("{rows}{rows}", 0, 7, 70), "7{rows}", "only the first occurrence");
        assert_eq!(report("no placeholders", 0, 10, 10), "no placeholders");
    }

    #[test]
    fn rows_and_page_items() {
        let options = [RowsPerPage::Rows(10), RowsPerPage::ShowAll("All".into()), RowsPerPage::Rows(20)];
        assert_eq!(
            rows_items(&options, 120),
            [("10".to_owned(), "10".to_owned()), ("20".to_owned(), "20".to_owned()), ("All".to_owned(), "120".to_owned())],
            "\"show all\" is always last"
        );
        assert!(rows_items(&[], 5).is_empty());
        assert_eq!(page_items(3), [("1".to_owned(), "0".to_owned()), ("2".to_owned(), "1".to_owned()), ("3".to_owned(), "2".to_owned())]);
        assert!(page_items(0).is_empty());
        assert_eq!(RowsPerPage::from(10), RowsPerPage::Rows(10));
    }

    #[test]
    fn retreat_goes_back_one_page_at_a_time() {
        assert_eq!(retreat_page(110, 10, 50), Some(4), "beyond the last page — to the last one");
        assert_eq!(retreat_page(110, 10, 105), Some(10), "one page back, as in PrimeNG");
        assert_eq!(retreat_page(40, 10, 50), None, "first within range");
        assert_eq!(retreat_page(50, 10, 50), Some(4));
        assert_eq!(retreat_page(0, 10, 0), None, "empty list");
        assert_eq!(retreat_page(10, 10, 0), None, "total = 0 does not step back");
        assert_eq!(retreat_page(0, 10, 5), None, "first page");
    }

    #[test]
    fn page_after_rows_change_keeps_first_and_clamps() {
        assert_eq!(page_after_rows_change(20, 20, 120), Some(1));
        assert_eq!(page_after_rows_change(20, 30, 120), Some(0));
        assert_eq!(page_after_rows_change(110, 120, 120), Some(0));
        assert_eq!(page_after_rows_change(130, 10, 120), Some(11), "beyond the last — to the last one");
        assert_eq!(page_after_rows_change(0, 10, 0), None);
        assert_eq!(page_after_rows_change(0, 0, 10), None);
    }

    #[test]
    fn classes() {
        let mut cn = ClassNames::new();
        write_nav_classes(&mut cn, "p-paginator-first", false);
        assert_eq!(cn.finish(), "p-paginator-first");
        let mut cn = ClassNames::new();
        write_nav_classes(&mut cn, "p-paginator-next", true);
        assert_eq!(cn.finish(), "p-paginator-next p-disabled");
        let mut cn = ClassNames::new();
        write_page_classes(&mut cn, true);
        assert_eq!(cn.finish(), "p-paginator-page p-paginator-page-selected");
        let mut cn = ClassNames::new();
        write_page_classes(&mut cn, false);
        assert_eq!(cn.finish(), "p-paginator-page");
    }
}
