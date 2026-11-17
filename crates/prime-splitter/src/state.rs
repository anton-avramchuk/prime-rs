//! Pure Splitter logic: panel sizes, offset, constraints, keys, classes and storage
//! (after PrimeNG `splitter.ts` and `splitterstyle.ts`).

/// Panel layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterLayout {
    /// In a row.
    #[default]
    Horizontal,
    /// In a column.
    Vertical,
}

impl SplitterLayout {
    /// Layout name: part of the class `p-splitter-<name>`, `aria-orientation` and `data-p`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// State storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterStorage {
    /// `sessionStorage`.
    #[default]
    Session,
    /// `localStorage`.
    Local,
}

/// Initial sizes in percent: given non-zero ones, the rest split equally.
pub fn initial_sizes(panel_sizes: &[f64], n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| match panel_sizes.get(i) {
            Some(&size) if size != 0.0 => size,
            _ => 100.0 / n as f64,
        })
        .collect()
}

/// Panel `flex-basis` value.
pub fn flex_basis(size: f64, n: usize, gutter: f64) -> String {
    format!("calc({size}% - {}px)", n.saturating_sub(1) as f64 * gutter)
}

/// Pointer offset as a percentage of the root size.
pub fn drag_delta(start: f64, current: f64, size: f64, rtl: bool, horizontal: bool) -> f64 {
    if horizontal && rtl { (start - current) * 100.0 / size } else { (current - start) * 100.0 / size }
}

/// Whether the new size of the pair is valid: it cannot go below its own non-zero minimum.
pub fn is_valid_resize(new_prev: f64, new_next: f64, min_prev: f64, min_next: f64) -> bool {
    !(min_prev > 0.0 && min_prev > new_prev) && !(min_next > 0.0 && min_next > new_next)
}

/// Key step: a positive one enlarges the previous panel. For the horizontal layout the left and right arrows work,
/// for the vertical one up and down.
pub fn key_step(layout: SplitterLayout, key: &str, step: f64) -> Option<f64> {
    match (layout, key) {
        (SplitterLayout::Horizontal, "ArrowRight") | (SplitterLayout::Vertical, "ArrowUp") => Some(step),
        (SplitterLayout::Horizontal, "ArrowLeft") | (SplitterLayout::Vertical, "ArrowDown") => Some(-step),
        _ => None,
    }
}

/// Whether this is an arrow (their default action is always cancelled).
pub fn is_arrow(key: &str) -> bool {
    matches!(key, "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown")
}

/// `aria-valuenow`: size with four decimals (like `toFixed(4)`).
pub fn aria_value(size: f64) -> String {
    format!("{size:.4}")
}

/// Root classes (without `p-splitter-resizing`).
pub fn root_classes(layout: SplitterLayout, resizing: bool) -> String {
    let mut out = format!("p-splitter p-component p-splitter-{}", layout.as_str());
    if resizing {
        out.push_str(" p-splitter-resizing");
    }
    out
}

/// Panel classes.
pub fn panel_classes(nested: bool) -> &'static str {
    if nested { "p-splitterpanel p-splitterpanel-nested" } else { "p-splitterpanel" }
}

/// Gutter classes.
pub fn gutter_classes(resizing: bool) -> &'static str {
    if resizing { "p-splitter-gutter p-splitter-gutter-resizing" } else { "p-splitter-gutter" }
}

/// `data-p` value of the root and gutters: layout and, when nested, `nested`.
pub fn data_p(layout: SplitterLayout, nested: bool) -> &'static str {
    match (layout, nested) {
        (SplitterLayout::Horizontal, false) => "horizontal",
        (SplitterLayout::Horizontal, true) => "horizontal nested",
        (SplitterLayout::Vertical, false) => "vertical",
        (SplitterLayout::Vertical, true) => "vertical nested",
    }
}

/// Root style.
pub fn root_style(layout: SplitterLayout) -> &'static str {
    match layout {
        SplitterLayout::Horizontal => "display: flex; flex-wrap: nowrap",
        SplitterLayout::Vertical => "display: flex; flex-wrap: nowrap; flex-direction: column",
    }
}

/// Handle style: size along the layout axis.
pub fn handle_style(layout: SplitterLayout, gutter: f64) -> String {
    match layout {
        SplitterLayout::Horizontal => format!("width: {gutter}px"),
        SplitterLayout::Vertical => format!("height: {gutter}px"),
    }
}

/// Sizes as a JSON array.
pub fn to_json(sizes: &[f64]) -> String {
    let items: Vec<String> = sizes.iter().map(f64::to_string).collect();
    format!("[{}]", items.join(","))
}

/// Parse a JSON array of numbers; anything else is `None`.
pub fn parse_json(text: &str) -> Option<Vec<f64>> {
    let inner = text.trim().strip_prefix('[')?.strip_suffix(']')?.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }
    inner.split(',').map(|part| part.trim().parse::<f64>().ok().filter(|v| v.is_finite())).collect()
}

#[cfg(test)]
mod tests {
    use super::SplitterLayout::{Horizontal, Vertical};
    use super::*;

    #[test]
    fn initial_sizes_table() {
        assert_eq!(initial_sizes(&[], 2), [50.0, 50.0]);
        assert_eq!(initial_sizes(&[20.0, 80.0], 2), [20.0, 80.0]);
        assert_eq!(initial_sizes(&[25.0], 4), [25.0, 25.0, 25.0, 25.0]);
        assert_eq!(initial_sizes(&[0.0, 70.0], 2), [50.0, 70.0]);
        assert_eq!(initial_sizes(&[10.0, 20.0, 30.0], 2), [10.0, 20.0]);
        assert!(initial_sizes(&[], 0).is_empty());
    }

    #[test]
    fn flex_basis_table() {
        assert_eq!(flex_basis(50.0, 2, 4.0), "calc(50% - 4px)");
        assert_eq!(flex_basis(20.0, 3, 4.0), "calc(20% - 8px)");
        assert_eq!(flex_basis(100.0, 1, 4.0), "calc(100% - 0px)");
        assert_eq!(flex_basis(33.5, 2, 6.0), "calc(33.5% - 6px)");
    }

    #[test]
    fn drag_delta_table() {
        assert_eq!(drag_delta(100.0, 140.0, 400.0, false, true), 10.0);
        assert_eq!(drag_delta(100.0, 60.0, 400.0, false, true), -10.0);
        assert_eq!(drag_delta(100.0, 140.0, 400.0, true, true), -10.0);
        assert_eq!(drag_delta(100.0, 140.0, 400.0, true, false), 10.0, "rtl affects only the horizontal layout");
        assert_eq!(drag_delta(0.0, 30.0, 300.0, false, false), 10.0);
    }

    #[test]
    fn is_valid_resize_table() {
        // (new prev, new next, min prev, min next, expected)
        let cases = [
            (60.0, 40.0, 10.0, 0.0, true),
            (5.0, 95.0, 10.0, 0.0, false),
            (10.0, 90.0, 10.0, 0.0, true),
            (95.0, 5.0, 0.0, 10.0, false),
            (50.0, 50.0, 0.0, 0.0, true),
            (1.0, 99.0, 0.0, 0.0, true),
            (5.0, 5.0, 10.0, 10.0, false),
        ];
        for (prev, next, min_prev, min_next, expected) in cases {
            assert_eq!(is_valid_resize(prev, next, min_prev, min_next), expected, "{prev} {next} {min_prev} {min_next}");
        }
    }

    #[test]
    fn key_step_table() {
        assert_eq!(key_step(Horizontal, "ArrowRight", 5.0), Some(5.0));
        assert_eq!(key_step(Horizontal, "ArrowLeft", 5.0), Some(-5.0));
        assert_eq!(key_step(Horizontal, "ArrowUp", 5.0), None);
        assert_eq!(key_step(Horizontal, "ArrowDown", 5.0), None);
        assert_eq!(key_step(Vertical, "ArrowUp", 5.0), Some(5.0));
        assert_eq!(key_step(Vertical, "ArrowDown", 5.0), Some(-5.0));
        assert_eq!(key_step(Vertical, "ArrowLeft", 5.0), None);
        assert_eq!(key_step(Horizontal, "Enter", 5.0), None);
        assert!(is_arrow("ArrowUp") && is_arrow("ArrowLeft") && !is_arrow("Enter") && !is_arrow("a"));
    }

    #[test]
    fn classes_and_data_p_table() {
        assert_eq!(root_classes(Horizontal, false), "p-splitter p-component p-splitter-horizontal");
        assert_eq!(root_classes(Vertical, true), "p-splitter p-component p-splitter-vertical p-splitter-resizing");
        assert_eq!(panel_classes(false), "p-splitterpanel");
        assert_eq!(panel_classes(true), "p-splitterpanel p-splitterpanel-nested");
        assert_eq!(gutter_classes(false), "p-splitter-gutter");
        assert_eq!(gutter_classes(true), "p-splitter-gutter p-splitter-gutter-resizing");
        assert_eq!(data_p(Horizontal, false), "horizontal");
        assert_eq!(data_p(Vertical, true), "vertical nested");
        assert_eq!(root_style(Horizontal), "display: flex; flex-wrap: nowrap");
        assert_eq!(root_style(Vertical), "display: flex; flex-wrap: nowrap; flex-direction: column");
        assert_eq!(handle_style(Horizontal, 4.0), "width: 4px");
        assert_eq!(handle_style(Vertical, 6.0), "height: 6px");
        assert_eq!(aria_value(50.0), "50.0000");
        assert_eq!(aria_value(33.333_333), "33.3333");
    }

    #[test]
    fn json_round_trip_table() {
        assert_eq!(to_json(&[50.0, 50.0]), "[50,50]");
        assert_eq!(to_json(&[33.5, 66.5]), "[33.5,66.5]");
        assert_eq!(parse_json("[30,70]"), Some(vec![30.0, 70.0]));
        assert_eq!(parse_json(" [ 12.5 , 87.5 ] "), Some(vec![12.5, 87.5]));
        assert_eq!(parse_json("[]"), Some(vec![]));
        for bad in ["", "30,70", "[30,", "[a,b]", "[1,,2]", "{\"a\":1}", "[NaN]"] {
            assert_eq!(parse_json(bad), None, "{bad:?}");
        }
        let sizes = [33.333_333_333_333_336, 66.666_666_666_666_66];
        assert_eq!(parse_json(&to_json(&sizes)).unwrap(), sizes);
    }
}
