//! Pure DataView logic: filter, sort, page and total (after PrimeNG `dataview.ts`, no DOM).

use std::cmp::Ordering;

/// Comparison of two items for sorting.
pub type Compare<'a, T> = &'a dyn Fn(&T, &T) -> Ordering;

/// First record of the last non-empty page; a `first` beyond the total is returned to it.
pub fn clamp_first(first: usize, total: usize, rows: usize) -> usize {
    if rows == 0 || total == 0 {
        return 0;
    }
    first.min((total - 1) / rows * rows)
}

/// Items after filter and sort (`filter`, `sort`): `order` is 1 ascending, -1 descending, 0 no sorting.
pub fn prepare<T: Clone>(items: &[T], filter: Option<&dyn Fn(&T) -> bool>, compare: Option<Compare<T>>, order: i32) -> Vec<T> {
    let mut out: Vec<T> = match filter {
        Some(f) => items.iter().filter(|i| f(i)).cloned().collect(),
        None => items.to_vec(),
    };
    if let (Some(compare), true) = (compare, order != 0) {
        out.sort_by(|a, b| {
            let ordering = compare(a, b);
            if order < 0 { ordering.reverse() } else { ordering }
        });
    }
    out
}

/// Page of prepared items; in `lazy` mode the value already holds the needed page (`slice` is not needed).
pub fn page<T: Clone>(prepared: &[T], paginator: bool, lazy: bool, first: usize, rows: usize) -> Vec<T> {
    if !paginator || lazy {
        return prepared.to_vec();
    }
    prepared.iter().skip(first).take(rows).cloned().collect()
}

/// Total records for the paginator: `total_records` in `lazy` mode, otherwise the number of prepared items.
pub fn total(prepared_len: usize, lazy: bool, total_records: usize) -> usize {
    if lazy { total_records } else { prepared_len }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_table() {
        for (first, total, rows, want) in
            [(0, 10, 3, 0), (3, 10, 3, 3), (9, 10, 3, 9), (30, 10, 3, 9), (5, 0, 3, 0), (5, 10, 0, 0), (4, 12, 4, 4), (12, 12, 4, 8)]
        {
            assert_eq!(clamp_first(first, total, rows), want, "first {first} total {total} rows {rows}");
        }
    }

    #[test]
    fn prepare_filters_and_sorts() {
        let items = vec![5, 3, 8, 1, 9, 2];
        let even_or_big = |n: &i32| *n > 2;
        let cmp = |a: &i32, b: &i32| a.cmp(b);
        assert_eq!(prepare(&items, None, None, 1), items, "without comparison the order is unchanged");
        assert_eq!(prepare(&items, None, Some(&cmp), 0), items, "order 0: no sorting");
        assert_eq!(prepare(&items, None, Some(&cmp), 1), [1, 2, 3, 5, 8, 9]);
        assert_eq!(prepare(&items, None, Some(&cmp), -1), [9, 8, 5, 3, 2, 1]);
        assert_eq!(prepare(&items, Some(&even_or_big), Some(&cmp), 1), [3, 5, 8, 9]);
        assert_eq!(prepare(&items, Some(&even_or_big), None, 1), [5, 3, 8, 9]);
    }

    #[test]
    fn sort_is_stable() {
        let items = vec![(1, 'a'), (0, 'b'), (1, 'c'), (0, 'd')];
        let cmp = |a: &(i32, char), b: &(i32, char)| a.0.cmp(&b.0);
        assert_eq!(prepare(&items, None, Some(&cmp), 1), [(0, 'b'), (0, 'd'), (1, 'a'), (1, 'c')]);
        assert_eq!(prepare(&items, None, Some(&cmp), -1), [(1, 'a'), (1, 'c'), (0, 'b'), (0, 'd')]);
    }

    #[test]
    fn page_slices() {
        let items: Vec<i32> = (0..10).collect();
        assert_eq!(page(&items, true, false, 3, 3), [3, 4, 5]);
        assert_eq!(page(&items, true, false, 9, 3), [9]);
        assert_eq!(page(&items, true, false, 30, 3), Vec::<i32>::new());
        assert_eq!(page(&items, false, false, 3, 3), items, "without a paginator: everything");
        assert_eq!(page(&items, true, true, 3, 3), items, "lazy: the value is already a page");
    }

    #[test]
    fn totals() {
        assert_eq!(total(7, false, 100), 7);
        assert_eq!(total(7, true, 100), 100);
    }
}
