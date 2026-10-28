//! Позиционирование оверлея относительно якоря (порт `DomHandler.absolutePosition` и `relativePosition`).
//!
//! Здесь только арифметика без обращения к DOM, поэтому она одинаково работает и тестируется на любой цели.

/// Точка (отступы от верхнего левого угла).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    /// Вертикальный отступ.
    pub top: f64,
    /// Горизонтальный отступ.
    pub left: f64,
}

/// Размер элемента.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    /// Ширина.
    pub width: f64,
    /// Высота.
    pub height: f64,
}

/// Откуда «растёт» оверлей (`transform-origin`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Оверлей расположен под якорем.
    Top,
    /// Оверлей расположен над якорем.
    Bottom,
}

impl Origin {
    /// Значение для `transform-origin`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Результат: координаты для `top`/`left` и сторона раскрытия.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    /// Значение `top` в пикселях.
    pub top: f64,
    /// Значение `left` в пикселях.
    pub left: f64,
    /// Сторона раскрытия.
    pub origin: Origin,
}

/// Исходные данные для расчёта.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Input {
    /// `getBoundingClientRect()` якоря (`top`, `left` относительно окна).
    pub target_offset: Point,
    /// Размер якоря.
    pub target: Size,
    /// Размер оверлея.
    pub overlay: Size,
    /// Размер видимой области окна.
    pub viewport: Size,
    /// Прокрутка окна.
    pub scroll: Point,
}

/// Позиция для оверлея, добавленного в `body` (координаты страницы).
///
/// ```
/// use prime_dom::position::{Input, Origin, Point, Size, absolute_position};
///
/// let input = Input {
///     target_offset: Point { top: 100.0, left: 20.0 },
///     target: Size { width: 100.0, height: 30.0 },
///     overlay: Size { width: 200.0, height: 150.0 },
///     viewport: Size { width: 800.0, height: 600.0 },
///     scroll: Point::default(),
/// };
/// let placement = absolute_position(&input);
/// assert_eq!(placement.origin, Origin::Top);
/// assert_eq!(placement.top, 130.0);
/// ```
pub fn absolute_position(i: &Input) -> Placement {
    let t = i.target_offset;
    let (top, origin) = if t.top + i.target.height + i.overlay.height > i.viewport.height {
        let top = t.top + i.scroll.top - i.overlay.height;
        (if top < 0.0 { i.scroll.top } else { top }, Origin::Bottom)
    } else {
        (i.target.height + t.top + i.scroll.top, Origin::Top)
    };
    let left = if t.left + i.overlay.width > i.viewport.width {
        (t.left + i.scroll.left + i.target.width - i.overlay.width).max(0.0)
    } else {
        t.left + i.scroll.left
    };
    Placement { top, left, origin }
}

/// Позиция для оверлея внутри ближайшего предка с `position: relative`.
///
/// `relative_offset` — `getBoundingClientRect()` такого предка; если его нет, берётся `(-scroll.top, -scroll.left)`, как в PrimeNG.
pub fn relative_position(i: &Input, relative_offset: Option<Point>) -> Placement {
    let t = i.target_offset;
    let rel = relative_offset.unwrap_or(Point { top: -i.scroll.top, left: -i.scroll.left });

    let (top, origin) = if t.top + i.target.height + i.overlay.height > i.viewport.height {
        let mut top = t.top - rel.top - i.overlay.height;
        if t.top + top < 0.0 {
            top = -t.top;
        }
        (top, Origin::Bottom)
    } else {
        (i.target.height + t.top - rel.top, Origin::Top)
    };

    let horizontal_overflow = t.left + i.overlay.width - i.viewport.width;
    let left = if i.overlay.width > i.viewport.width {
        -(t.left - rel.left)
    } else if horizontal_overflow > 0.0 {
        (t.left - rel.left) - horizontal_overflow
    } else {
        t.left - rel.left
    };

    Placement { top, left, origin }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(target_top: f64, target_left: f64, overlay: Size) -> Input {
        Input {
            target_offset: Point { top: target_top, left: target_left },
            target: Size { width: 100.0, height: 30.0 },
            overlay,
            viewport: Size { width: 800.0, height: 600.0 },
            scroll: Point { top: 50.0, left: 0.0 },
        }
    }

    const OVERLAY: Size = Size { width: 200.0, height: 150.0 };

    #[test]
    fn absolute_fits_below() {
        let p = absolute_position(&input(100.0, 20.0, OVERLAY));
        assert_eq!(p, Placement { top: 30.0 + 100.0 + 50.0, left: 20.0, origin: Origin::Top });
    }

    #[test]
    fn absolute_flips_above_when_no_room_below() {
        let p = absolute_position(&input(500.0, 20.0, OVERLAY));
        assert_eq!(p.origin, Origin::Bottom);
        assert_eq!(p.top, 500.0 + 50.0 - 150.0);
    }

    #[test]
    fn absolute_flipped_clamps_to_scroll_when_above_page() {
        let mut i = input(10.0, 20.0, Size { width: 200.0, height: 700.0 });
        i.viewport.height = 600.0;
        let p = absolute_position(&i);
        assert_eq!(p.origin, Origin::Bottom);
        assert_eq!(p.top, i.scroll.top);
    }

    #[test]
    fn absolute_aligns_right_edge_when_overflowing_right() {
        let p = absolute_position(&input(100.0, 700.0, OVERLAY));
        assert_eq!(p.left, 700.0 + 100.0 - 200.0);
    }

    #[test]
    fn absolute_left_never_negative() {
        let mut i = input(100.0, 10.0, Size { width: 900.0, height: 50.0 });
        i.viewport.width = 800.0;
        assert_eq!(absolute_position(&i).left, 0.0);
    }

    #[test]
    fn relative_fits_below_without_relative_parent() {
        let p = relative_position(&input(100.0, 20.0, OVERLAY), None);
        // rel = (-50, 0): top = 30 + 100 - (-50)
        assert_eq!(p, Placement { top: 180.0, left: 20.0, origin: Origin::Top });
    }

    #[test]
    fn relative_flips_and_clamps() {
        let p = relative_position(&input(500.0, 20.0, OVERLAY), Some(Point { top: 100.0, left: 10.0 }));
        assert_eq!(p.origin, Origin::Bottom);
        assert_eq!(p.top, 500.0 - 100.0 - 150.0);
        let tall = relative_position(&input(10.0, 20.0, Size { width: 10.0, height: 700.0 }), Some(Point::default()));
        assert_eq!(tall.top, -10.0);
    }

    #[test]
    fn relative_shifts_left_on_horizontal_overflow() {
        let p = relative_position(&input(100.0, 700.0, OVERLAY), Some(Point::default()));
        assert_eq!(p.left, 700.0 - (700.0 + 200.0 - 800.0));
    }

    #[test]
    fn relative_wider_than_viewport_aligns_to_viewport_left() {
        let p = relative_position(&input(100.0, 300.0, Size { width: 900.0, height: 50.0 }), Some(Point { top: 0.0, left: 100.0 }));
        assert_eq!(p.left, -(300.0 - 100.0));
    }
}
