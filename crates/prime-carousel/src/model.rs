//! Pure Carousel model: pages, offset, remainder, clones and responsiveness (after PrimeNG `carousel.ts`).
//!
//! All numbers match the source: the `total_shifted` offset is negative and expressed in items, pages are counted from zero.

/// Scroll direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    /// Forward (`dir = -1` in PrimeNG).
    Forward,
    /// Backward (`dir = 1`).
    Backward,
}

impl Dir {
    fn sign(self) -> i32 {
        match self {
            Self::Forward => -1,
            Self::Backward => 1,
        }
    }
}

/// Responsive option (`CarouselResponsiveOptions`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Responsive {
    /// Window width limit, e.g. `"991px"` (the number at the start of the string is taken).
    pub breakpoint: String,
    /// Number of visible items.
    pub num_visible: usize,
    /// Number of items per scroll.
    pub num_scroll: usize,
}

/// Scroll state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Track {
    /// Current page.
    pub page: usize,
    /// Number of visible items.
    pub num_visible: usize,
    /// Number of items per scroll.
    pub num_scroll: usize,
    /// Logical list offset, in items (`totalShiftedItems`).
    pub total_shifted: i32,
    /// Remaining items on the last page (`remainingItems`).
    pub remaining: i32,
    /// The remainder is already included in the offset (`isRemainingItemsAdded`).
    pub remaining_added: bool,
}

/// Infinite mode is active while there are at least as many items as visible ones (`isCircular`).
pub fn is_circular(circular: bool, len: usize, num_visible: usize) -> bool {
    circular && len >= num_visible
}

/// Number of pages (`totalDots`); with an empty value and negative computations — `0`.
pub fn total_dots(len: usize, num_visible: usize, num_scroll: usize) -> usize {
    if len == 0 || num_scroll == 0 {
        return 0;
    }
    let pages = ((len as f64 - num_visible as f64) / num_scroll as f64).ceil() as i64 + 1;
    pages.max(0) as usize
}

/// Number of offset steps in percent → `translate3d` string (`translate3d(0, X%, 0)` or `translate3d(X%, 0, 0)`).
pub fn translate(shift: i32, num_visible: usize, vertical: bool) -> String {
    let mut percent = f64::from(shift) * (100.0 / num_visible.max(1) as f64);
    if percent == 0.0 {
        percent = 0.0;
    }
    if vertical { format!("translate3d(0, {percent}%, 0)") } else { format!("translate3d({percent}%, 0, 0)") }
}

/// Item width, percent (`flex: 1 0 <100/numVisible>%`).
pub fn item_flex(num_visible: usize) -> String {
    format!("1 0 {}%", 100.0 / num_visible.max(1) as f64)
}

impl Track {
    /// Initial state (PrimeNG constructor and the first `afterContentChecked` check).
    pub fn new(page: usize, num_visible: usize, num_scroll: usize, len: usize, circular: bool) -> Self {
        let mut t =
            Self { page, num_visible, num_scroll, total_shifted: -((page * num_scroll) as i32), remaining: 0, remaining_added: false };
        t.recalc(len, circular);
        t
    }

    /// Number of pages.
    pub fn dots(&self, len: usize) -> usize {
        total_dots(len, self.num_visible, self.num_scroll)
    }

    /// First visible item (`firstIndex`).
    pub fn first_index(&self, circular: bool, len: usize) -> i32 {
        if is_circular(circular, len, self.num_visible) { -(self.total_shifted + self.num_visible as i32) } else { -self.total_shifted }
    }

    /// Last visible item (`lastIndex`).
    pub fn last_index(&self, circular: bool, len: usize) -> i32 {
        self.first_index(circular, len) + self.num_visible as i32 - 1
    }

    /// "Forward" is unavailable (`isForwardNavDisabled`).
    pub fn forward_disabled(&self, circular: bool, len: usize) -> bool {
        len == 0 || (self.page + 1 >= self.dots(len) && !is_circular(circular, len, self.num_visible))
    }

    /// "Backward" is unavailable (`isBackwardNavDisabled`).
    pub fn backward_disabled(&self, circular: bool, len: usize) -> bool {
        len == 0 || (self.page == 0 && !is_circular(circular, len, self.num_visible))
    }

    /// Recompute on a change of `num_visible`, `num_scroll` or length (the block in `onAfterContentChecked`).
    /// Returns `true` if the page was clamped and has to be reported.
    pub fn recalc(&mut self, len: usize, circular: bool) -> bool {
        let circ = is_circular(circular, len, self.num_visible);
        let scroll = self.num_scroll.max(1) as i32;
        self.remaining = (len as i32 - self.num_visible as i32) % scroll;
        let dots = self.dots(len);
        let mut clamped = false;
        if dots != 0 && self.page >= dots {
            self.page = dots - 1;
            clamped = true;
        }
        let mut total = -((self.page * self.num_scroll) as i32);
        if circ {
            total -= self.num_visible as i32;
        }
        if dots != 0 && self.page == dots - 1 && self.remaining > 0 {
            total += -self.remaining + self.num_scroll as i32;
            self.remaining_added = true;
        } else {
            self.remaining_added = false;
        }
        self.total_shifted = total;
        self.settle(len, circular);
        clamped
    }

    /// Settling the logical offset after a step (end of `onAfterContentChecked`): page `0` → `−numVisible`,
    /// zero offset → `−length`.
    pub fn settle(&mut self, len: usize, circular: bool) {
        if !is_circular(circular, len, self.num_visible) {
            return;
        }
        let mut total = self.total_shifted;
        if self.page == 0 {
            total = -(self.num_visible as i32);
        } else if total == 0 {
            total = -(len as i32);
            if self.remaining > 0 {
                self.remaining_added = true;
            }
        }
        self.total_shifted = total;
    }

    /// Step to the adjacent page or to `page` (`step`). Returns the offset for `transform` right after the step (before
    /// [`Track::settle`], which the caller does next).
    pub fn step(&mut self, len: usize, circular: bool, dir: Dir, page: Option<usize>) -> i32 {
        let circ = is_circular(circular, len, self.num_visible);
        let (nv, ns) = (self.num_visible as i32, self.num_scroll.max(1) as i32);
        let sign = dir.sign();
        let dots = self.dots(len);
        let mut total = self.total_shifted;
        let mut next_page: i64;
        match page {
            Some(p) => {
                total = -(ns * p as i32);
                if circ {
                    total -= nv;
                }
                self.remaining_added = false;
                next_page = p as i64;
            }
            None => {
                total += ns * sign;
                if self.remaining_added {
                    total += self.remaining - ns * sign;
                    self.remaining_added = false;
                }
                let original = if circ { total + nv } else { total };
                next_page = (f64::from(original) / f64::from(ns)).floor().abs() as i64;
            }
        }
        let last = dots as i64 - 1;
        if circ && self.page as i64 == last && dir == Dir::Forward {
            total = -(len as i32 + nv);
            next_page = 0;
        } else if circ && self.page == 0 && dir == Dir::Backward {
            total = 0;
            next_page = last;
        } else if next_page == last && self.remaining > 0 {
            total += -self.remaining - ns * sign;
            self.remaining_added = true;
        }
        self.total_shifted = total;
        self.page = next_page.max(0) as usize;
        total
    }

    /// Offset for `transform` after the transition ends (`onTransitionEnd`): in infinite mode on the edge pages
    /// the list snaps to the logical offset without animation; otherwise `None` (nothing changes).
    pub fn after_transition(&self, len: usize, circular: bool) -> Option<i32> {
        let dots = self.dots(len);
        let edge = self.page == 0 || (dots > 0 && self.page == dots - 1);
        (edge && is_circular(circular, len, self.num_visible)).then_some(self.total_shifted)
    }

    /// Applies a responsive option (`calculatePosition`): recomputes the page, step and visible count and settles the offset.
    /// `defaults` — the component's prop values. Returns `true` if the page has to be reported.
    pub fn apply_responsive(
        &mut self,
        options: &[Responsive],
        window_width: f64,
        defaults: (usize, usize),
        len: usize,
        circular: bool,
    ) -> bool {
        let (matched_nv, matched_ns) = match_responsive(options, window_width, defaults);
        let mut notify = false;
        if self.num_scroll != matched_ns {
            self.page = (self.page * self.num_scroll) / matched_ns.max(1);
            self.num_scroll = matched_ns;
            notify = true;
        }
        if self.num_visible != matched_nv {
            self.num_visible = matched_nv;
        }
        notify | self.recalc(len, circular)
    }
}

/// Responsive option selection: options are sorted by descending limit, the last one with `limit ≥ window width` is taken;
/// otherwise the defaults.
pub fn match_responsive(options: &[Responsive], window_width: f64, defaults: (usize, usize)) -> (usize, usize) {
    let mut sorted: Vec<&Responsive> = options.iter().collect();
    sorted.sort_by_key(|o| std::cmp::Reverse(parse_int(&o.breakpoint)));
    let mut matched = defaults;
    for option in sorted {
        if f64::from(parse_int(&option.breakpoint)) >= window_width {
            matched = (option.num_visible, option.num_scroll);
        }
    }
    matched
}

/// JavaScript `parseInt`: the integer at the start of the string, otherwise `0`.
fn parse_int(s: &str) -> i32 {
    let s = s.trim();
    let digits: String = s.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().unwrap_or(0)
}

/// Swipe threshold, px.
pub const SWIPE_THRESHOLD: f64 = 20.0;

/// Swipe direction by the coordinate difference (`changePageOnTouch`): left/up (`< 0`) — forward, right/down — backward.
pub fn swipe(diff: f64) -> Option<Dir> {
    if diff.abs() > SWIPE_THRESHOLD { Some(if diff < 0.0 { Dir::Forward } else { Dir::Backward }) } else { None }
}

/// Next autoplay page: from the last one — to zero; without pages — `None`.
pub fn autoplay_next(page: usize, dots: usize) -> Option<usize> {
    (dots > 0).then(|| if page + 1 >= dots { 0 } else { page + 1 })
}

/// Substitution into a translation string (`{slideNumber}`, `{page}`).
pub fn fill(template: &str, key: &str, value: usize) -> String {
    template.replace(&format!("{{{key}}}"), &value.to_string())
}

/// Where the indicator focus moves on an arrow: `delta = +1` — right, `-1` — left; stays at the edges.
pub fn indicator_focus(current: usize, count: usize, delta: i32) -> usize {
    let last = count.saturating_sub(1);
    ((current as i64 + i64::from(delta)).clamp(0, last as i64)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(page: usize, nv: usize, ns: usize, len: usize, circular: bool) -> Track {
        Track::new(page, nv, ns, len, circular)
    }

    #[test]
    fn dots_table() {
        // (length, visible, step, pages)
        for (len, nv, ns, want) in
            [(0, 3, 1, 0), (5, 2, 1, 4), (9, 3, 3, 3), (9, 3, 2, 4), (3, 3, 1, 1), (2, 3, 1, 0), (1, 3, 1, 0), (10, 1, 1, 10), (7, 3, 3, 3)]
        {
            assert_eq!(total_dots(len, nv, ns), want, "{len} {nv} {ns}");
        }
    }

    #[test]
    fn circular_needs_enough_items() {
        assert!(is_circular(true, 5, 3));
        assert!(is_circular(true, 3, 3));
        assert!(!is_circular(true, 2, 3));
        assert!(!is_circular(false, 5, 3));
    }

    #[test]
    fn initial_state_and_remainder() {
        let a = t(0, 3, 1, 6, false);
        assert_eq!((a.total_shifted, a.remaining, a.remaining_added, a.dots(6)), (0, 0, false, 4));
        let b = t(0, 3, 2, 9, false);
        assert_eq!((b.remaining, b.dots(9)), (0, 4), "(9−3) % 2 = 0");
        let c = t(0, 3, 2, 8, false);
        assert_eq!((c.remaining, c.dots(8)), (1, 4), "(8−3) % 2 = 1");
        let circular = t(0, 3, 1, 6, true);
        assert_eq!(circular.total_shifted, -3, "in infinite mode the list sits behind the start clones");
    }

    #[test]
    fn plain_forward_and_back() {
        let mut a = t(0, 3, 1, 6, false);
        assert_eq!(a.step(6, false, Dir::Forward, None), -1);
        a.settle(6, false);
        assert_eq!((a.page, a.total_shifted), (1, -1));
        assert_eq!(a.step(6, false, Dir::Forward, None), -2);
        assert_eq!(a.step(6, false, Dir::Forward, None), -3);
        assert_eq!(a.page, 3);
        assert!(a.forward_disabled(false, 6) && !a.backward_disabled(false, 6));
        assert_eq!(a.step(6, false, Dir::Backward, None), -2);
        assert_eq!(a.page, 2);
        assert_eq!((a.first_index(false, 6), a.last_index(false, 6)), (2, 4));
    }

    #[test]
    fn remainder_page_shows_the_last_items() {
        // 8 items, 3 visible, step 2: pages 0..3, shift 5 on the last one (items 5-7 visible)
        let mut a = t(0, 3, 2, 8, false);
        let mut shifts = Vec::new();
        for _ in 0..3 {
            shifts.push(a.step(8, false, Dir::Forward, None));
        }
        assert_eq!(a.page, 3);
        assert_eq!(shifts, [-2, -4, -5], "the last step is shorter by the remainder");
        assert!(a.remaining_added);
        assert_eq!((a.first_index(false, 8), a.last_index(false, 8)), (5, 7));
        // backward returns exactly one step
        assert_eq!(a.step(8, false, Dir::Backward, None), -4);
        assert_eq!(a.page, 2);
        assert!(!a.remaining_added);
    }

    #[test]
    fn jump_to_a_page_including_the_remainder_page() {
        let mut a = t(0, 3, 2, 8, false);
        assert_eq!(a.step(8, false, Dir::Forward, Some(2)), -4);
        assert_eq!(a.page, 2);
        assert_eq!(a.step(8, false, Dir::Forward, Some(3)), -5);
        assert_eq!((a.page, a.remaining_added), (3, true));
        assert_eq!(a.step(8, false, Dir::Backward, Some(0)), 0);
        assert_eq!((a.page, a.remaining_added), (0, false));
    }

    #[test]
    fn circular_wraps_through_the_clones() {
        let len = 6;
        let mut a = t(0, 3, 1, len, true);
        assert_eq!(a.total_shifted, -3);
        // forward to the last page
        for want in [-4, -5, -6] {
            let applied = a.step(len, true, Dir::Forward, None);
            a.settle(len, true);
            assert_eq!(applied, want);
        }
        assert_eq!(a.page, 3);
        assert_eq!(a.after_transition(len, true), Some(-6));
        // from the last page forward — to the end clones
        let applied = a.step(len, true, Dir::Forward, None);
        assert_eq!(applied, -9, "−(length + visible)");
        a.settle(len, true);
        assert_eq!(a.page, 0);
        assert_eq!(a.total_shifted, -3, "after the transition the list snaps to the start without animation");
        assert_eq!(a.after_transition(len, true), Some(-3));
        // from zero backward — to the start clones
        let applied = a.step(len, true, Dir::Backward, None);
        assert_eq!(applied, 0);
        a.settle(len, true);
        assert_eq!(a.page, 3);
        assert_eq!(a.total_shifted, -6, "zero offset settles to −length");
        assert!(!a.forward_disabled(true, len) && !a.backward_disabled(true, len));
    }

    #[test]
    fn circular_with_remainder_sets_the_flag_on_return() {
        let len = 8;
        let mut a = t(0, 3, 2, len, true);
        a.step(len, true, Dir::Backward, None);
        a.settle(len, true);
        assert_eq!(a.page, 3);
        assert!(a.remaining_added || a.remaining > 0);
    }

    #[test]
    fn page_is_clamped_when_pages_shrink() {
        let mut a = t(3, 3, 1, 6, false);
        assert_eq!(a.page, 3);
        assert!(!a.recalc(6, false));
        assert!(a.recalc(4, false), "now 4 items: two pages, the page is clamped");
        assert_eq!((a.page, a.total_shifted), (1, -1));
        assert!(a.recalc(0, false) || a.page == 1, "an empty value leaves the page alone");
    }

    #[test]
    fn nav_disabled_flags() {
        let empty = t(0, 3, 1, 0, false);
        assert!(empty.forward_disabled(false, 0) && empty.backward_disabled(false, 0));
        let one_page = t(0, 3, 1, 3, false);
        assert!(one_page.forward_disabled(false, 3) && one_page.backward_disabled(false, 3));
        let circ = t(0, 3, 1, 5, true);
        assert!(!circ.forward_disabled(true, 5) && !circ.backward_disabled(true, 5));
    }

    #[test]
    fn active_range_follows_the_shift() {
        let mut a = t(0, 2, 1, 5, false);
        assert_eq!((a.first_index(false, 5), a.last_index(false, 5)), (0, 1));
        a.step(5, false, Dir::Forward, None);
        assert_eq!((a.first_index(false, 5), a.last_index(false, 5)), (1, 2));
        let c = t(0, 2, 1, 5, true);
        assert_eq!((c.first_index(true, 5), c.last_index(true, 5)), (0, 1), "in infinite mode the offset accounts for clones");
    }

    #[test]
    fn translate_and_flex_strings() {
        assert_eq!(translate(0, 3, false), "translate3d(0%, 0, 0)");
        assert_eq!(translate(-1, 4, false), "translate3d(-25%, 0, 0)");
        assert_eq!(translate(-2, 1, true), "translate3d(0, -200%, 0)");
        assert_eq!(item_flex(4), "1 0 25%");
        assert_eq!(item_flex(1), "1 0 100%");
        assert_eq!(item_flex(3), "1 0 33.333333333333336%");
    }

    fn options() -> Vec<Responsive> {
        vec![
            Responsive { breakpoint: "1400px".into(), num_visible: 2, num_scroll: 1 },
            Responsive { breakpoint: "1199px".into(), num_visible: 3, num_scroll: 1 },
            Responsive { breakpoint: "767px".into(), num_visible: 2, num_scroll: 1 },
            Responsive { breakpoint: "575px".into(), num_visible: 1, num_scroll: 1 },
        ]
    }

    #[test]
    fn responsive_matching() {
        let defaults = (3, 3);
        assert_eq!(match_responsive(&options(), 1920.0, defaults), (3, 3), "wider than all limits — defaults");
        assert_eq!(match_responsive(&options(), 1300.0, defaults), (2, 1), "1400");
        assert_eq!(match_responsive(&options(), 1100.0, defaults), (3, 1), "1199");
        assert_eq!(match_responsive(&options(), 700.0, defaults), (2, 1), "767");
        assert_eq!(match_responsive(&options(), 500.0, defaults), (1, 1), "575");
        assert_eq!(match_responsive(&options(), 575.0, defaults), (1, 1), "limit is inclusive");
        assert_eq!(match_responsive(&[], 500.0, defaults), defaults);
    }

    #[test]
    fn responsive_recomputes_page_and_shift() {
        let len = 9;
        let mut a = t(0, 3, 3, len, false);
        a.step(len, false, Dir::Forward, None);
        assert_eq!((a.page, a.total_shifted), (1, -3));
        let notify = a.apply_responsive(&options(), 500.0, (3, 3), len, false);
        assert!(notify);
        assert_eq!((a.num_visible, a.num_scroll), (1, 1));
        assert_eq!(a.page, 3, "⌊1·3/1⌋");
        assert_eq!(a.total_shifted, -3);
        let notify = a.apply_responsive(&options(), 1920.0, (3, 3), len, false);
        assert!(notify);
        assert_eq!((a.num_visible, a.num_scroll, a.page), (3, 3, 1), "⌊3·1/3⌋");
        assert!(!a.apply_responsive(&options(), 1920.0, (3, 3), len, false), "no changes, no event needed");
    }

    #[test]
    fn swipe_threshold() {
        assert_eq!(swipe(-21.0), Some(Dir::Forward));
        assert_eq!(swipe(21.0), Some(Dir::Backward));
        assert_eq!(swipe(20.0), None);
        assert_eq!(swipe(-20.0), None);
        assert_eq!(swipe(0.0), None);
    }

    #[test]
    fn autoplay_and_indicator_focus() {
        assert_eq!(autoplay_next(0, 4), Some(1));
        assert_eq!(autoplay_next(3, 4), Some(0));
        assert_eq!(autoplay_next(0, 0), None);
        assert_eq!(autoplay_next(0, 1), Some(0));
        assert_eq!(indicator_focus(0, 4, -1), 0);
        assert_eq!(indicator_focus(0, 4, 1), 1);
        assert_eq!(indicator_focus(3, 4, 1), 3);
        assert_eq!(indicator_focus(2, 4, -1), 1);
        assert_eq!(indicator_focus(0, 0, 1), 0);
    }

    #[test]
    fn aria_templates_fill() {
        assert_eq!(fill("{slideNumber}", "slideNumber", 3), "3");
        assert_eq!(fill("Page {page} of x", "page", 2), "Page 2 of x");
        assert_eq!(fill("no placeholder", "page", 2), "no placeholder");
    }
}
