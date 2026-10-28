//! Чистая логика Accordion: переключение значения и выбор соседнего заголовка (по `accordion.ts` PrimeNG).

/// Список активных значений после клика по панели `value`.
///
/// В одиночном режиме открывается только эта панель (повторный клик по единственной активной закрывает её),
/// при `multiple` значение добавляется в список или убирается из него.
///
/// ```
/// use prime_accordion::toggle_value;
///
/// let v = vec!["0".to_owned()];
/// assert_eq!(toggle_value(&v, "1", false), ["1"]);
/// assert_eq!(toggle_value(&v, "1", true), ["0", "1"]);
/// assert!(toggle_value(&v, "0", false).is_empty());
/// ```
pub fn toggle_value(current: &[String], value: &str, multiple: bool) -> Vec<String> {
    let active = current.iter().any(|v| v == value);
    if multiple {
        let mut next: Vec<String> = current.to_vec();
        match next.iter().position(|v| v == value) {
            Some(i) => {
                next.remove(i);
            }
            None => next.push(value.to_owned()),
        }
        next
    } else if active && current.len() == 1 {
        Vec::new()
    } else {
        // Внешний сигнал в одиночном режиме мог содержать несколько значений: клик оставляет только эту панель.
        vec![value.to_owned()]
    }
}

/// Клавиша перемещения фокуса между заголовками.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nav {
    /// ArrowDown.
    Next,
    /// ArrowUp.
    Prev,
    /// Home.
    First,
    /// End.
    Last,
}

/// Индекс панели, чей заголовок получает фокус. `disabled[i]` — отключена ли панель `i`, `from` — текущая.
///
/// Стрелки пропускают отключённые панели; если дальше в этом направлении подходящей нет, переход идёт по кругу
/// к первой (вниз) или последней (вверх) не отключённой. `None`, когда все панели отключены.
///
/// ```
/// use prime_accordion::{Nav, nav_target};
///
/// let disabled = [false, true, false];
/// assert_eq!(nav_target(&disabled, 0, Nav::Next), Some(2));
/// assert_eq!(nav_target(&disabled, 2, Nav::Next), Some(0));
/// assert_eq!(nav_target(&disabled, 0, Nav::Prev), Some(2));
/// assert_eq!(nav_target(&disabled, 1, Nav::Last), Some(2));
/// ```
pub fn nav_target(disabled: &[bool], from: usize, nav: Nav) -> Option<usize> {
    let enabled = |i: &usize| !disabled[*i];
    let first = (0..disabled.len()).find(enabled);
    let last = (0..disabled.len()).rev().find(enabled);
    match nav {
        Nav::First => first,
        Nav::Last => last,
        Nav::Next => (from + 1..disabled.len()).find(enabled).or(first),
        Nav::Prev => (0..from.min(disabled.len())).rev().find(enabled).or(last),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn toggle_value_table() {
        let cases: [(&[&str], &str, bool, &[&str]); 9] = [
            (&[], "0", false, &["0"]),
            (&["0"], "1", false, &["1"]),
            (&["0"], "0", false, &[]),
            (&["0", "1"], "0", false, &["0"]),
            (&["0", "1"], "2", false, &["2"]),
            (&[], "0", true, &["0"]),
            (&["0"], "1", true, &["0", "1"]),
            (&["0", "1"], "0", true, &["1"]),
            (&["0"], "0", true, &[]),
        ];
        for (current, value, multiple, want) in cases {
            assert_eq!(toggle_value(&v(current), value, multiple), v(want), "{current:?} {value} multiple={multiple}");
        }
    }

    #[test]
    fn nav_target_table() {
        let none = [false; 4];
        assert_eq!(nav_target(&none, 0, Nav::Next), Some(1));
        assert_eq!(nav_target(&none, 3, Nav::Next), Some(0));
        assert_eq!(nav_target(&none, 0, Nav::Prev), Some(3));
        assert_eq!(nav_target(&none, 2, Nav::Prev), Some(1));
        assert_eq!((nav_target(&none, 2, Nav::First), nav_target(&none, 1, Nav::Last)), (Some(0), Some(3)));

        let some = [true, false, true, false, true];
        assert_eq!(nav_target(&some, 1, Nav::Next), Some(3));
        assert_eq!(nav_target(&some, 3, Nav::Next), Some(1), "после последней доступной — по кругу к первой доступной");
        assert_eq!(nav_target(&some, 1, Nav::Prev), Some(3), "перед первой доступной — по кругу к последней доступной");
        assert_eq!((nav_target(&some, 3, Nav::First), nav_target(&some, 1, Nav::Last)), (Some(1), Some(3)));

        assert_eq!(nav_target(&[true, true], 0, Nav::Next), None);
        assert_eq!(nav_target(&[], 0, Nav::First), None);
    }
}
