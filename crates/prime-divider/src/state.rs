//! Pure Divider logic: classes, the inline alignment style and `data-p` (after PrimeNG `dividerstyle.ts` and `Divider.dataP`).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Divider direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerLayout {
    /// `horizontal` (default).
    #[default]
    Horizontal,
    /// `vertical`.
    Vertical,
}

impl DividerLayout {
    /// Value, as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Line type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerType {
    /// `solid` (default).
    #[default]
    Solid,
    /// `dashed`.
    Dashed,
    /// `dotted`.
    Dotted,
}

impl DividerType {
    /// Value, as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}

/// Content alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DividerAlign {
    /// `left` (for horizontal).
    Left,
    /// `center`.
    Center,
    /// `right` (for horizontal).
    Right,
    /// `top` (for vertical).
    Top,
    /// `bottom` (for vertical).
    Bottom,
}

impl DividerAlign {
    /// Value, as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Snapshot of the properties affecting classes, style and `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DividerState {
    /// Direction.
    pub layout: DividerLayout,
    /// Line type.
    pub divider_type: DividerType,
    /// Alignment (`None` - not set).
    pub align: Option<DividerAlign>,
}

/// Alignment class: only horizontal with `left`/`center`/`right` and vertical with `top`/`center`/`bottom` combinations are valid.
fn align_class(s: &DividerState) -> Option<&'static str> {
    use DividerAlign::*;
    use DividerLayout::*;
    match (s.layout, s.align) {
        (Horizontal, None | Some(Left)) => Some("p-divider-left"),
        (Horizontal, Some(Center)) => Some("p-divider-center"),
        (Horizontal, Some(Right)) => Some("p-divider-right"),
        (Vertical, Some(Top)) => Some("p-divider-top"),
        (Vertical, None | Some(Center)) => Some("p-divider-center"),
        (Vertical, Some(Bottom)) => Some("p-divider-bottom"),
        _ => None,
    }
}

/// Adds root classes in the order of `dividerstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &DividerState) {
    cn.add("p-divider p-component");
    cn.add(match s.layout {
        DividerLayout::Horizontal => "p-divider-horizontal",
        DividerLayout::Vertical => "p-divider-vertical",
    });
    cn.add(match s.divider_type {
        DividerType::Solid => "p-divider-solid",
        DividerType::Dashed => "p-divider-dashed",
        DividerType::Dotted => "p-divider-dotted",
    });
    cn.add_opt(align_class(s));
}

/// Inline alignment style: property and value (`justify-content` for horizontal, `align-items` for vertical).
pub fn align_style(s: &DividerState) -> Option<(&'static str, &'static str)> {
    use DividerAlign::*;
    use DividerLayout::*;
    match (s.layout, s.align) {
        (Horizontal, None | Some(Center)) => Some(("justify-content", "center")),
        (Horizontal, Some(Left)) => Some(("justify-content", "flex-start")),
        (Horizontal, Some(Right)) => Some(("justify-content", "flex-end")),
        (Vertical, None | Some(Center)) => Some(("align-items", "center")),
        (Vertical, Some(Top)) => Some(("align-items", "flex-start")),
        (Vertical, Some(Bottom)) => Some(("align-items", "flex-end")),
        _ => None,
    }
}

/// `data-p` value: alignment (if set), direction, type.
pub fn data_p(s: &DividerState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_opt(s.align.map(DividerAlign::as_str)).add(s.layout.as_str()).add(s.divider_type.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;
    use DividerAlign::*;
    use DividerLayout::*;
    use DividerType::*;

    fn classes(s: &DividerState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    fn st(layout: DividerLayout, divider_type: DividerType, align: Option<DividerAlign>) -> DividerState {
        DividerState { layout, divider_type, align }
    }

    #[test]
    fn defaults() {
        let s = DividerState::default();
        assert_eq!(classes(&s), "p-divider p-component p-divider-horizontal p-divider-solid p-divider-left");
        assert_eq!(data_p(&s), "horizontal solid");
        assert_eq!(align_style(&s), Some(("justify-content", "center")));
    }

    /// All combinations of direction and alignment: (layout, align, alignment class, style).
    #[test]
    fn layout_by_align_table() {
        type Row = (DividerLayout, Option<DividerAlign>, Option<&'static str>, Option<(&'static str, &'static str)>);
        let rows: Vec<Row> = vec![
            (Horizontal, None, Some("p-divider-left"), Some(("justify-content", "center"))),
            (Horizontal, Some(Left), Some("p-divider-left"), Some(("justify-content", "flex-start"))),
            (Horizontal, Some(Center), Some("p-divider-center"), Some(("justify-content", "center"))),
            (Horizontal, Some(Right), Some("p-divider-right"), Some(("justify-content", "flex-end"))),
            (Horizontal, Some(Top), None, None),
            (Horizontal, Some(Bottom), None, None),
            (Vertical, None, Some("p-divider-center"), Some(("align-items", "center"))),
            (Vertical, Some(Top), Some("p-divider-top"), Some(("align-items", "flex-start"))),
            (Vertical, Some(Center), Some("p-divider-center"), Some(("align-items", "center"))),
            (Vertical, Some(Bottom), Some("p-divider-bottom"), Some(("align-items", "flex-end"))),
            (Vertical, Some(Left), None, None),
            (Vertical, Some(Right), None, None),
        ];
        for (layout, align, class, style) in rows {
            let s = st(layout, Solid, align);
            assert_eq!(align_class(&s), class, "{layout:?} {align:?}");
            assert_eq!(align_style(&s), style, "{layout:?} {align:?}");
        }
    }

    #[test]
    fn classes_and_data_p_examples() {
        let s = st(Horizontal, Dotted, Some(Center));
        assert_eq!(classes(&s), "p-divider p-component p-divider-horizontal p-divider-dotted p-divider-center");
        assert_eq!(data_p(&s), "center horizontal dotted");
        let v = st(Vertical, Dashed, None);
        assert_eq!(classes(&v), "p-divider p-component p-divider-vertical p-divider-dashed p-divider-center");
        assert_eq!(data_p(&v), "vertical dashed");
        let bad = st(Vertical, Solid, Some(Left));
        assert_eq!(classes(&bad), "p-divider p-component p-divider-vertical p-divider-solid");
        assert_eq!(data_p(&bad), "left vertical solid");
    }
}
