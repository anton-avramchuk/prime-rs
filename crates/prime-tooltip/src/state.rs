//! Pure Tooltip logic: sides and fallback order, geometry, arrow, classes, delays (per `tooltip.ts` of PrimeNG).

/// The side of the element on which the tooltip is shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Right (default).
    #[default]
    Right,
    /// Left.
    Left,
    /// Top.
    Top,
    /// Bottom.
    Bottom,
}

impl TooltipPosition {
    /// Class suffix `p-tooltip-<name>`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Events that show the tooltip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipEvent {
    /// Mouse hover and touch (default).
    #[default]
    Hover,
    /// Focus and blur.
    Focus,
    /// Both.
    Both,
}

impl TooltipEvent {
    /// Listen for hover and touch.
    pub fn hover(self) -> bool {
        matches!(self, Self::Hover | Self::Both)
    }

    /// Listen for focus.
    pub fn focus(self) -> bool {
        matches!(self, Self::Focus | Self::Both)
    }
}

/// Side order: the primary one, then the fallbacks, while the tooltip goes beyond the window bounds.
pub fn priority(position: TooltipPosition) -> [TooltipPosition; 4] {
    use TooltipPosition::{Bottom, Left, Right, Top};
    match position {
        Top => [Top, Bottom, Right, Left],
        Bottom => [Bottom, Top, Right, Left],
        Left => [Left, Right, Top, Bottom],
        Right => [Right, Left, Top, Bottom],
    }
}

/// Size in pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    /// Width.
    pub width: f64,
    /// Height.
    pub height: f64,
}

/// Tooltip offset relative to the top-left corner of the element.
pub fn place(position: TooltipPosition, host: Size, tip: Size) -> (f64, f64) {
    match position {
        TooltipPosition::Right => (host.width, (host.height - tip.height) / 2.0),
        TooltipPosition::Left => (-tip.width, (host.height - tip.height) / 2.0),
        TooltipPosition::Top => ((host.width - tip.width) / 2.0, -tip.height),
        TooltipPosition::Bottom => ((host.width - tip.width) / 2.0, host.height),
    }
}

/// Arrow position: `None` means the property is removed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArrowStyle {
    /// `top`.
    pub top: Option<String>,
    /// `right`.
    pub right: Option<String>,
    /// `bottom`.
    pub bottom: Option<String>,
    /// `left`.
    pub left: Option<String>,
}

/// Arrow at the edge facing the element; for `top`/`bottom`, at the center of the tooltip.
pub fn arrow_style(position: TooltipPosition, tip_width: f64) -> ArrowStyle {
    let center = Some(format!("{}px", tip_width / 2.0));
    match position {
        TooltipPosition::Right => ArrowStyle { top: Some("50%".into()), left: Some("0".into()), ..Default::default() },
        TooltipPosition::Left => ArrowStyle { top: Some("50%".into()), right: Some("0".into()), ..Default::default() },
        TooltipPosition::Top => ArrowStyle { bottom: Some("0".into()), left: center, ..Default::default() },
        TooltipPosition::Bottom => ArrowStyle { top: Some("0".into()), left: center, ..Default::default() },
    }
}

/// The tooltip (in window coordinates) goes beyond the window bounds.
pub fn out_of_bounds(left: f64, top: f64, width: f64, height: f64, viewport_w: f64, viewport_h: f64) -> bool {
    left + width > viewport_w || left < 0.0 || top < 0.0 || top + height > viewport_h
}

/// Root classes: `p-tooltip p-component`, the side and the user class.
pub fn root_classes(position: TooltipPosition, style_class: Option<&str>) -> String {
    let mut cn = format!("p-tooltip p-component p-tooltip-{}", position.as_str());
    if let Some(extra) = style_class.filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(extra);
    }
    cn
}

/// After how long to show the tooltip and after how long to hide it based on `life` (ms): `(show, hide)`.
///
/// Zero values are treated as "not set"; `life` is counted after `show_delay`.
pub fn activation_delay(show_delay: Option<u32>, life: Option<u32>) -> (Option<u32>, Option<u32>) {
    let show = show_delay.filter(|d| *d > 0);
    let hide = life.filter(|l| *l > 0).map(|l| l + show.unwrap_or(0));
    (show, hide)
}

/// The element content does not fit in it (`offsetWidth < scrollWidth || offsetHeight < scrollHeight`).
pub fn has_ellipsis(offset_width: f64, scroll_width: f64, offset_height: f64, scroll_height: f64) -> bool {
    offset_width < scroll_width || offset_height < scroll_height
}

/// The cursor leaving the element does not hide the tooltip: it is not auto-hiding, and the cursor moved onto it.
pub fn leave_keeps_open(auto_hide: bool, related_is_tooltip: bool) -> bool {
    !auto_hide && related_is_tooltip
}

#[cfg(test)]
mod tests {
    use super::TooltipPosition::*;
    use super::*;

    const HOST: Size = Size { width: 100.0, height: 40.0 };
    const TIP: Size = Size { width: 60.0, height: 20.0 };

    #[test]
    fn priority_table() {
        assert_eq!(priority(Top), [Top, Bottom, Right, Left]);
        assert_eq!(priority(Bottom), [Bottom, Top, Right, Left]);
        assert_eq!(priority(Left), [Left, Right, Top, Bottom]);
        assert_eq!(priority(Right), [Right, Left, Top, Bottom]);
        for p in [Top, Bottom, Left, Right] {
            let order = priority(p);
            assert_eq!(order[0], p, "the primary side comes first");
            for q in [Top, Bottom, Left, Right] {
                assert_eq!(order.iter().filter(|x| **x == q).count(), 1, "{p:?}: {q:?} exactly once");
            }
        }
    }

    #[test]
    fn place_table() {
        // (side, expected offset)
        let cases = [(Right, (100.0, 10.0)), (Left, (-60.0, 10.0)), (Top, (20.0, -20.0)), (Bottom, (20.0, 40.0))];
        for (p, want) in cases {
            assert_eq!(place(p, HOST, TIP), want, "{p:?}");
        }
        // the tooltip is wider than the element: centered with a negative shift
        assert_eq!(place(Top, Size { width: 40.0, height: 10.0 }, TIP), (-10.0, -20.0));
    }

    #[test]
    fn arrow_style_table() {
        let right = arrow_style(Right, 60.0);
        assert_eq!((right.top.as_deref(), right.left.as_deref(), right.right, right.bottom), (Some("50%"), Some("0"), None, None));
        let left = arrow_style(Left, 60.0);
        assert_eq!((left.top.as_deref(), left.right.as_deref(), left.left, left.bottom), (Some("50%"), Some("0"), None, None));
        let top = arrow_style(Top, 60.0);
        assert_eq!((top.bottom.as_deref(), top.left.as_deref(), top.top, top.right), (Some("0"), Some("30px"), None, None));
        let bottom = arrow_style(Bottom, 61.0);
        assert_eq!((bottom.top.as_deref(), bottom.left.as_deref(), bottom.bottom, bottom.right), (Some("0"), Some("30.5px"), None, None));
    }

    #[test]
    fn out_of_bounds_table() {
        // window 1000x800, tooltip 100x50
        let oob = |l, t| out_of_bounds(l, t, 100.0, 50.0, 1000.0, 800.0);
        assert!(!oob(10.0, 10.0));
        assert!(!oob(900.0, 750.0), "touching the right and bottom edge still fits");
        assert!(oob(901.0, 10.0), "overflows to the right");
        assert!(oob(10.0, 751.0), "overflows downward");
        assert!(oob(-1.0, 10.0), "overflows to the left");
        assert!(oob(10.0, -1.0), "overflows upward");
    }

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(Right, None), "p-tooltip p-component p-tooltip-right");
        assert_eq!(root_classes(Top, Some("custom wide")), "p-tooltip p-component p-tooltip-top custom wide");
        assert_eq!(root_classes(Left, Some("")), "p-tooltip p-component p-tooltip-left");
    }

    #[test]
    fn activation_delay_table() {
        // (show_delay, life, expectation)
        let cases = [
            (None, None, (None, None)),
            (Some(300), None, (Some(300), None)),
            (None, Some(2000), (None, Some(2000))),
            (Some(300), Some(2000), (Some(300), Some(2300))),
            (Some(0), Some(0), (None, None)),
            (Some(0), Some(500), (None, Some(500))),
        ];
        for (s, l, want) in cases {
            assert_eq!(activation_delay(s, l), want, "{s:?} {l:?}");
        }
    }

    #[test]
    fn ellipsis_and_leave() {
        assert!(!has_ellipsis(100.0, 100.0, 20.0, 20.0));
        assert!(has_ellipsis(100.0, 140.0, 20.0, 20.0), "by width");
        assert!(has_ellipsis(100.0, 100.0, 20.0, 35.0), "by height");
        assert!(leave_keeps_open(false, true));
        assert!(!leave_keeps_open(false, false));
        assert!(!leave_keeps_open(true, true), "with auto_hide the tooltip is always hidden");
    }

    #[test]
    fn events() {
        assert!(TooltipEvent::Hover.hover() && !TooltipEvent::Hover.focus());
        assert!(!TooltipEvent::Focus.hover() && TooltipEvent::Focus.focus());
        assert!(TooltipEvent::Both.hover() && TooltipEvent::Both.focus());
    }
}
