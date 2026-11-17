//! Reordering the selected list items (after PrimeNG `moveUp`, `moveTop`, `moveDown`, `moveBottom`, no DOM).
//!
//! Unlike PrimeNG, the selected items keep their relative order: in PrimeNG adjacent selected items at the edge
//! swap places on `moveUp`, and `moveTop`/`moveBottom` depend on the order of clicks on the items.

fn is_selected(selected: &[String], key: &str) -> bool {
    selected.iter().any(|s| s == key)
}

/// Moves every selected item up by one position if the one above is not selected.
pub fn move_up(items: &[String], selected: &[String]) -> Vec<String> {
    let mut out = items.to_vec();
    for i in 1..out.len() {
        if is_selected(selected, &out[i]) && !is_selected(selected, &out[i - 1]) {
            out.swap(i, i - 1);
        }
    }
    out
}

/// Moves every selected item down by one position if the one below is not selected.
pub fn move_down(items: &[String], selected: &[String]) -> Vec<String> {
    let mut out = items.to_vec();
    for i in (0..out.len().saturating_sub(1)).rev() {
        if is_selected(selected, &out[i]) && !is_selected(selected, &out[i + 1]) {
            out.swap(i, i + 1);
        }
    }
    out
}

/// Moves all selected items to the top, keeping the order.
pub fn move_top(items: &[String], selected: &[String]) -> Vec<String> {
    let (mut picked, rest): (Vec<_>, Vec<_>) = items.iter().cloned().partition(|k| is_selected(selected, k));
    picked.extend(rest);
    picked
}

/// Moves all selected items to the bottom, keeping the order.
pub fn move_bottom(items: &[String], selected: &[String]) -> Vec<String> {
    let (picked, mut rest): (Vec<_>, Vec<_>) = items.iter().cloned().partition(|k| is_selected(selected, k));
    rest.extend(picked);
    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_owned).collect()
    }

    fn join(items: Vec<String>) -> String {
        items.join(" ")
    }

    #[test]
    fn up_table() {
        // (list, selected, expectation)
        for (items, sel, want) in [
            ("A B C D", "C", "A C B D"),
            ("A B C D", "A", "A B C D"),
            ("A B C", "A B", "A B C"),
            ("A B C D E", "B D", "B A D C E"),
            ("A B C D", "B C", "B C A D"),
            ("A B C D", "A C", "A C B D"),
            ("A B C", "", "A B C"),
            ("A", "A", "A"),
            ("", "A", ""),
        ] {
            assert_eq!(join(move_up(&v(items), &v(sel))), want, "{items} / {sel}");
        }
    }

    #[test]
    fn down_table() {
        for (items, sel, want) in [
            ("A B C D", "B", "A C B D"),
            ("A B C D", "D", "A B C D"),
            ("A B C", "B C", "A B C"),
            ("A B C D E", "B D", "A C B E D"),
            ("A B C D", "B C", "A D B C"),
            ("A B C D", "B D", "A C B D"),
            ("A B C", "", "A B C"),
            ("A", "A", "A"),
        ] {
            assert_eq!(join(move_down(&v(items), &v(sel))), want, "{items} / {sel}");
        }
    }

    #[test]
    fn top_and_bottom_table() {
        assert_eq!(join(move_top(&v("A B C D"), &v("C"))), "C A B D");
        assert_eq!(join(move_top(&v("A B C D"), &v("D B"))), "B D A C", "click order does not matter");
        assert_eq!(join(move_top(&v("A B C"), &v("A"))), "A B C");
        assert_eq!(join(move_top(&v("A B C"), &v(""))), "A B C");
        assert_eq!(join(move_bottom(&v("A B C D"), &v("A C"))), "B D A C");
        assert_eq!(join(move_bottom(&v("A B C D"), &v("C A"))), "B D A C", "click order does not matter");
        assert_eq!(join(move_bottom(&v("A B C"), &v("C"))), "A B C");
    }

    #[test]
    fn keeps_every_element() {
        let items = v("A B C D E F");
        for sel in ["", "A", "B D", "A B C D E F", "F"] {
            for f in [move_up, move_down, move_top, move_bottom] {
                let mut out = f(&items, &v(sel));
                out.sort();
                assert_eq!(out, v("A B C D E F"), "{sel}");
            }
        }
    }
}
