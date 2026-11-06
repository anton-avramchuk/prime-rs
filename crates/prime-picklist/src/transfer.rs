//! Перенос элементов между списками PickList (по `moveRight`, `moveAllRight`, `moveLeft`, `moveAllLeft` PrimeNG, без DOM).

use prime_listbox::ListboxOption;

fn has(list: &[ListboxOption], value: &str) -> bool {
    list.iter().any(|o| o.value == value)
}

/// Переносит выбранные элементы в конец `to` в порядке выбора; элементы, значения которых уже есть в `to`, остаются.
/// Возвращает перенесённые элементы.
pub fn move_selected(from: &mut Vec<ListboxOption>, to: &mut Vec<ListboxOption>, selection: &[String]) -> Vec<ListboxOption> {
    let mut moved = Vec::new();
    for value in selection {
        if has(to, value) {
            continue;
        }
        if let Some(pos) = from.iter().position(|o| &o.value == value) {
            let item = from.remove(pos);
            to.push(item.clone());
            moved.push(item);
        }
    }
    moved
}

/// Переносит в конец `to` все элементы `from`, для которых `visible` истинно, сохраняя порядок. Возвращает перенесённые.
pub fn move_all(
    from: &mut Vec<ListboxOption>,
    to: &mut Vec<ListboxOption>,
    visible: impl Fn(&ListboxOption) -> bool,
) -> Vec<ListboxOption> {
    let mut moved = Vec::new();
    let mut i = 0;
    while i < from.len() {
        if visible(&from[i]) {
            let item = from.remove(i);
            to.push(item.clone());
            moved.push(item);
        } else {
            i += 1;
        }
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(s: &str) -> Vec<ListboxOption> {
        s.split_whitespace().map(|n| ListboxOption::new(n, n.to_lowercase())).collect()
    }

    fn labels(l: &[ListboxOption]) -> String {
        l.iter().map(|o| o.label.as_str()).collect::<Vec<_>>().join(" ")
    }

    fn sel(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn selected_follow_selection_order() {
        let (mut from, mut to) = (list("A B C D"), list("X"));
        let moved = move_selected(&mut from, &mut to, &sel("c a"));
        assert_eq!((labels(&from).as_str(), labels(&to).as_str(), labels(&moved).as_str()), ("B D", "X C A", "C A"));
    }

    #[test]
    fn duplicates_stay() {
        let (mut from, mut to) = (list("A B"), list("A"));
        let moved = move_selected(&mut from, &mut to, &sel("a b"));
        assert_eq!((labels(&from).as_str(), labels(&to).as_str(), labels(&moved).as_str()), ("A", "A B", "B"));
    }

    #[test]
    fn unknown_and_empty_selection() {
        let (mut from, mut to) = (list("A B"), list(""));
        assert!(move_selected(&mut from, &mut to, &sel("zzz")).is_empty());
        assert!(move_selected(&mut from, &mut to, &[]).is_empty());
        assert_eq!((labels(&from).as_str(), labels(&to).as_str()), ("A B", ""));
    }

    #[test]
    fn all_visible_only() {
        let (mut from, mut to) = (list("Apple Banana Avocado"), list(""));
        let moved = move_all(&mut from, &mut to, |o| o.label.to_lowercase().contains("ap"));
        assert_eq!((labels(&from).as_str(), labels(&to).as_str(), labels(&moved).as_str()), ("Banana Avocado", "Apple", "Apple"));
        let moved = move_all(&mut from, &mut to, |_| true);
        assert_eq!((labels(&from).as_str(), labels(&to).as_str(), labels(&moved).as_str()), ("", "Apple Banana Avocado", "Banana Avocado"));
    }

    #[test]
    fn all_keeps_every_element() {
        let (mut from, mut to) = (list("A B C D E"), list("X Y"));
        move_all(&mut from, &mut to, |o| o.value != "c");
        assert_eq!((labels(&from).as_str(), labels(&to).as_str()), ("C", "X Y A B D E"));
    }
}
