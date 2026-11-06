//! Чистая логика DataView: фильтр, сортировка, страница и итог (по `dataview.ts` PrimeNG, без DOM).

use std::cmp::Ordering;

/// Сравнение двух элементов для сортировки.
pub type Compare<'a, T> = &'a dyn Fn(&T, &T) -> Ordering;

/// Первая запись последней непустой страницы; `first`, вышедший за итог, возвращается к ней.
pub fn clamp_first(first: usize, total: usize, rows: usize) -> usize {
    if rows == 0 || total == 0 {
        return 0;
    }
    first.min((total - 1) / rows * rows)
}

/// Элементы после фильтра и сортировки (`filter`, `sort`): `order` — 1 по возрастанию, -1 по убыванию, 0 — без сортировки.
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

/// Страница подготовленных элементов; в режиме `lazy` значение уже содержит нужную страницу (`slice` не нужен).
pub fn page<T: Clone>(prepared: &[T], paginator: bool, lazy: bool, first: usize, rows: usize) -> Vec<T> {
    if !paginator || lazy {
        return prepared.to_vec();
    }
    prepared.iter().skip(first).take(rows).cloned().collect()
}

/// Всего записей для пагинатора: `total_records` в режиме `lazy`, иначе число подготовленных элементов.
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
        assert_eq!(prepare(&items, None, None, 1), items, "без сравнения порядок прежний");
        assert_eq!(prepare(&items, None, Some(&cmp), 0), items, "порядок 0 — без сортировки");
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
        assert_eq!(page(&items, false, false, 3, 3), items, "без пагинатора — всё");
        assert_eq!(page(&items, true, true, 3, 3), items, "lazy — значение уже страница");
    }

    #[test]
    fn totals() {
        assert_eq!(total(7, false, 100), 7);
        assert_eq!(total(7, true, 100), 100);
    }
}
