//! Pure Galleria model: thumbnail window, neighboring indices, responsive visible count, classes
//! (per `galleria.ts` and `galleriastyle.ts` of PrimeNG, no DOM).

/// The side on which thumbnails or indicators are rendered.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GalleriaPosition {
    /// Bottom (default).
    #[default]
    Bottom,
    /// Top.
    Top,
    /// Left.
    Left,
    /// Right.
    Right,
}

impl GalleriaPosition {
    /// Class suffix: `bottom`, `top`, `left`, `right`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Thumbnails are laid out vertically.
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// Responsive option: up to window width `breakpoint` (px) show `num_visible` thumbnails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GalleriaResponsive {
    /// Maximum window width, px.
    pub breakpoint: u32,
    /// Number of visible thumbnails.
    pub num_visible: usize,
}

/// Number of visible thumbnails, not exceeding the number of images (`numVisibleLimit`).
pub fn effective_num_visible(num_visible: usize, len: usize) -> usize {
    if len > 0 && len < num_visible { len } else { num_visible }
}

/// Choosing the number of visible thumbnails by window width: the smallest `breakpoint` not less than the width (`calculatePosition`).
pub fn responsive_num_visible(options: &[GalleriaResponsive], window_width: u32, base: usize) -> usize {
    let mut sorted = options.to_vec();
    sorted.sort_by_key(|o| std::cmp::Reverse(o.breakpoint));
    let mut result = base;
    for option in sorted {
        if option.breakpoint >= window_width {
            result = option.num_visible;
        }
    }
    result
}

/// Index of the thumbnail the active one tends toward (`getMedianItemIndex`).
pub fn median_index(num_visible: usize) -> usize {
    let index = num_visible / 2;
    if num_visible % 2 == 1 { index } else { index.saturating_sub(1) }
}

/// Number of "pages" of the strip (`getTotalPageNumber`).
pub fn total_pages(len: usize, num_visible: usize) -> usize {
    if len > num_visible { len - num_visible + 1 } else { 0 }
}

/// Strip offset in items (<= 0) for the active index: the result of `onAfterContentChecked`.
pub fn shifted_items(len: usize, num_visible: usize, active: usize) -> i32 {
    let (len, nv, active) = (len as i32, num_visible as i32, active as i32);
    let median = median_index(num_visible) as i32;
    if active <= median {
        0
    } else if len - nv + median < active {
        nv - len
    } else if len - nv < active && nv % 2 == 0 {
        -active + median + 1
    } else {
        -active + median
    }
}

/// `transform` value of the thumbnail strip.
pub fn thumbnails_transform(shifted: i32, num_visible: usize, vertical: bool) -> String {
    let percent = f64::from(shifted) * (100.0 / num_visible.max(1) as f64);
    if vertical { format!("translate3d(0, {percent}%, 0)") } else { format!("translate3d({percent}%, 0, 0)") }
}

/// Index of the first visible thumbnail (`firstItemAciveIndex`).
pub fn first_active(shifted: i32) -> i32 {
    -shifted
}

/// Index of the last visible thumbnail (`lastItemActiveIndex`).
pub fn last_active(shifted: i32, num_visible: usize) -> i32 {
    first_active(shifted) + num_visible as i32 - 1
}

/// The thumbnail is within the visible window (`isItemActive`).
pub fn is_item_active(shifted: i32, num_visible: usize, index: usize) -> bool {
    let index = index as i32;
    first_active(shifted) <= index && last_active(shifted, num_visible) >= index
}

/// Next frame index: after the last one, `0` when `circular`.
pub fn next_index(active: usize, len: usize, circular: bool) -> usize {
    if circular && len > 0 && len - 1 == active { 0 } else { (active + 1).min(len.saturating_sub(1)) }
}

/// Previous frame index: before the first one, the last one when `circular`.
pub fn prev_index(active: usize, len: usize, circular: bool) -> usize {
    if circular && active == 0 { len.saturating_sub(1) } else { active.saturating_sub(1) }
}

/// The frame "backward" button is disabled (`GalleriaItem.isNavBackwardDisabled`).
pub fn item_backward_disabled(active: usize, circular: bool) -> bool {
    !circular && active == 0
}

/// The frame "forward" button is disabled.
pub fn item_forward_disabled(active: usize, len: usize, circular: bool) -> bool {
    !circular && active + 1 >= len
}

/// The thumbnails "backward" button is disabled.
pub fn thumb_backward_disabled(active: usize, len: usize, num_visible: usize, circular: bool) -> bool {
    (!circular && active == 0) || len <= num_visible
}

/// The thumbnails "forward" button is disabled.
pub fn thumb_forward_disabled(active: usize, len: usize, num_visible: usize, circular: bool) -> bool {
    (!circular && active + 1 >= len) || len <= num_visible
}

/// Root classes.
pub fn root_classes(
    full_screen: bool,
    inset_indicators: bool,
    hover_navigators: bool,
    thumbnails: Option<GalleriaPosition>,
    indicators: Option<GalleriaPosition>,
) -> String {
    let mut cn = String::from("p-galleria p-component");
    if full_screen {
        cn.push_str(" p-galleria-fullscreen");
    }
    if inset_indicators {
        cn.push_str(" p-galleria-inset-indicators");
    }
    if hover_navigators && !full_screen {
        cn.push_str(" p-galleria-hover-navigators");
    }
    if let Some(pos) = thumbnails {
        cn.push_str(" p-galleria-thumbnails-");
        cn.push_str(pos.as_str());
    }
    if let Some(pos) = indicators {
        cn.push_str(" p-galleria-indicators-");
        cn.push_str(pos.as_str());
    }
    cn
}

/// Classes of a thumbnail in the strip.
pub fn thumbnail_item_classes(index: usize, active: usize, shifted: i32, num_visible: usize) -> String {
    let mut cn = String::from("p-galleria-thumbnail-item");
    if active == index {
        cn.push_str(" p-galleria-thumbnail-item-current");
    }
    if is_item_active(shifted, num_visible, index) {
        cn.push_str(" p-galleria-thumbnail-item-active");
    }
    if first_active(shifted) == index as i32 {
        cn.push_str(" p-galleria-thumbnail-item-start");
    }
    if last_active(shifted, num_visible) == index as i32 {
        cn.push_str(" p-galleria-thumbnail-item-end");
    }
    cn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_table() {
        for (n, want) in [(0, 0), (1, 0), (2, 0), (3, 1), (4, 1), (5, 2), (6, 2)] {
            assert_eq!(median_index(n), want, "n = {n}");
        }
    }

    #[test]
    fn shifted_table() {
        // (len, n_visible, active, offset)
        let cases = [
            (6, 3, 0, 0),
            (6, 3, 1, 0),
            (6, 3, 2, -1),
            (6, 3, 3, -2),
            (6, 3, 4, -3),
            (6, 3, 5, -3),
            (6, 4, 0, 0),
            (6, 4, 1, 0),
            (6, 4, 2, -1),
            (6, 4, 3, -1),
            (6, 4, 4, -2),
            (6, 4, 5, -2),
            (3, 3, 2, 0),
            (2, 5, 1, 0),
        ];
        for (len, nv, active, want) in cases {
            assert_eq!(shifted_items(len, nv, active), want, "len {len} nv {nv} active {active}");
        }
    }

    #[test]
    fn window_membership() {
        let shifted = shifted_items(6, 3, 3);
        assert_eq!((first_active(shifted), last_active(shifted, 3)), (2, 4));
        assert!(is_item_active(shifted, 3, 2) && is_item_active(shifted, 3, 4));
        assert!(!is_item_active(shifted, 3, 1) && !is_item_active(shifted, 3, 5));
    }

    #[test]
    fn pages() {
        assert_eq!(total_pages(6, 3), 4);
        assert_eq!(total_pages(3, 3), 0);
        assert_eq!(total_pages(2, 5), 0);
    }

    #[test]
    fn transform_string() {
        assert_eq!(thumbnails_transform(-2, 4, false), "translate3d(-50%, 0, 0)");
        assert_eq!(thumbnails_transform(0, 3, true), "translate3d(0, 0%, 0)");
        assert_eq!(thumbnails_transform(-1, 4, true), "translate3d(0, -25%, 0)");
    }

    #[test]
    fn neighbours() {
        assert_eq!(next_index(0, 3, false), 1);
        assert_eq!(next_index(2, 3, false), 2);
        assert_eq!(next_index(2, 3, true), 0);
        assert_eq!(prev_index(0, 3, false), 0);
        assert_eq!(prev_index(0, 3, true), 2);
        assert_eq!(prev_index(2, 3, true), 1);
    }

    #[test]
    fn disabled_flags() {
        assert!(item_backward_disabled(0, false) && !item_backward_disabled(0, true));
        assert!(item_forward_disabled(2, 3, false) && !item_forward_disabled(2, 3, true));
        assert!(thumb_backward_disabled(0, 6, 3, false) && !thumb_backward_disabled(1, 6, 3, false));
        assert!(thumb_backward_disabled(3, 3, 3, true), "no more thumbnails than visible");
        assert!(thumb_forward_disabled(5, 6, 3, false) && !thumb_forward_disabled(5, 6, 3, true));
    }

    #[test]
    fn responsive_table() {
        let options = [GalleriaResponsive { breakpoint: 960, num_visible: 4 }, GalleriaResponsive { breakpoint: 640, num_visible: 2 }];
        assert_eq!(responsive_num_visible(&options, 1200, 5), 5);
        assert_eq!(responsive_num_visible(&options, 960, 5), 4);
        assert_eq!(responsive_num_visible(&options, 800, 5), 4);
        assert_eq!(responsive_num_visible(&options, 600, 5), 2);
        assert_eq!(responsive_num_visible(&[], 600, 5), 5);
    }

    #[test]
    fn num_visible_limited_by_len() {
        assert_eq!(effective_num_visible(5, 3), 3);
        assert_eq!(effective_num_visible(3, 6), 3);
        assert_eq!(effective_num_visible(3, 0), 3);
    }

    #[test]
    fn root_class_table() {
        use GalleriaPosition::*;
        assert_eq!(root_classes(false, false, false, Some(Bottom), None), "p-galleria p-component p-galleria-thumbnails-bottom");
        assert_eq!(
            root_classes(true, true, true, Some(Left), Some(Top)),
            "p-galleria p-component p-galleria-fullscreen p-galleria-inset-indicators p-galleria-thumbnails-left p-galleria-indicators-top"
        );
        assert_eq!(
            root_classes(false, false, true, None, Some(Right)),
            "p-galleria p-component p-galleria-hover-navigators p-galleria-indicators-right"
        );
    }

    #[test]
    fn thumbnail_item_class_table() {
        let shifted = shifted_items(6, 3, 3);
        assert_eq!(
            thumbnail_item_classes(3, 3, shifted, 3),
            "p-galleria-thumbnail-item p-galleria-thumbnail-item-current p-galleria-thumbnail-item-active"
        );
        assert_eq!(
            thumbnail_item_classes(2, 3, shifted, 3),
            "p-galleria-thumbnail-item p-galleria-thumbnail-item-active p-galleria-thumbnail-item-start"
        );
        assert_eq!(thumbnail_item_classes(0, 3, shifted, 3), "p-galleria-thumbnail-item");
    }
}
