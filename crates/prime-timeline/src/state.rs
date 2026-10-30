//! Чистая логика Timeline: из свойств в классы и `data-p` (по `timelinestyle.ts` и `Timeline.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Положение линии относительно содержимого.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimelineAlign {
    /// `left` (вертикальная раскладка, по умолчанию).
    #[default]
    Left,
    /// `right`.
    Right,
    /// `top` (горизонтальная раскладка).
    Top,
    /// `bottom`.
    Bottom,
    /// `alternate` (содержимое по обе стороны линии).
    Alternate,
}

impl TimelineAlign {
    /// Значение, как в PrimeNG.
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

/// Ориентация таймлайна.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimelineLayout {
    /// `vertical` (по умолчанию).
    #[default]
    Vertical,
    /// `horizontal`.
    Horizontal,
}

impl TimelineLayout {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Дописывает классы корня: `p-timeline p-component p-timeline-<align> p-timeline-<layout>`.
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

/// Значение `data-p`: ориентация и положение.
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
