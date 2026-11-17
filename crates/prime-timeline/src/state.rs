//! Pure Timeline logic: from properties to classes and `data-p` (after PrimeNG `timelinestyle.ts` and `Timeline.dataP`).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Line position relative to the content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimelineAlign {
    /// `left` (vertical layout, default).
    #[default]
    Left,
    /// `right`.
    Right,
    /// `top` (horizontal layout).
    Top,
    /// `bottom`.
    Bottom,
    /// `alternate` (content on both sides of the line).
    Alternate,
}

impl TimelineAlign {
    /// The value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Alternate => "alternate",
        }
    }
}

/// Timeline orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimelineLayout {
    /// `vertical` (default).
    #[default]
    Vertical,
    /// `horizontal`.
    Horizontal,
}

impl TimelineLayout {
    /// The value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Appends the root classes: `p-timeline p-component p-timeline-<align> p-timeline-<layout>`.
pub fn write_classes(cn: &mut ClassNames, align: TimelineAlign, layout: TimelineLayout) {
    cn.add("p-timeline p-component");
    cn.add(match align {
        TimelineAlign::Left => "p-timeline-left",
        TimelineAlign::Right => "p-timeline-right",
        TimelineAlign::Top => "p-timeline-top",
        TimelineAlign::Bottom => "p-timeline-bottom",
        TimelineAlign::Alternate => "p-timeline-alternate",
    });
    cn.add(match layout {
        TimelineLayout::Vertical => "p-timeline-vertical",
        TimelineLayout::Horizontal => "p-timeline-horizontal",
    });
}

/// The `data-p` value: orientation and position.
pub fn data_p(align: TimelineAlign, layout: TimelineLayout) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(layout.as_str()).add(align.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;
    use TimelineAlign::*;
    use TimelineLayout::*;

    #[test]
    fn classes_and_data_p_table() {
        let table = [
            (Left, Vertical, "p-timeline p-component p-timeline-left p-timeline-vertical", "vertical left"),
            (Right, Vertical, "p-timeline p-component p-timeline-right p-timeline-vertical", "vertical right"),
            (Top, Horizontal, "p-timeline p-component p-timeline-top p-timeline-horizontal", "horizontal top"),
            (Bottom, Horizontal, "p-timeline p-component p-timeline-bottom p-timeline-horizontal", "horizontal bottom"),
            (Alternate, Vertical, "p-timeline p-component p-timeline-alternate p-timeline-vertical", "vertical alternate"),
        ];
        for (align, layout, classes, data) in table {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, align, layout);
            assert_eq!(cn.finish(), classes);
            assert_eq!(data_p(align, layout), data);
        }
    }
}
