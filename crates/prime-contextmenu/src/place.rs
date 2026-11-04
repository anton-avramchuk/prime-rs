//! Положение панели у курсора (`position()` из PrimeNG).

/// Размеры окна и прокрутка страницы.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// Ширина окна без полосы прокрутки.
    pub width: f64,
    /// Высота окна.
    pub height: f64,
    /// Горизонтальная прокрутка страницы.
    pub scroll_x: f64,
    /// Вертикальная прокрутка страницы.
    pub scroll_y: f64,
}

/// Координаты левого верхнего угла панели размером `width × height` для курсора в точке (`page_x`, `page_y`) страницы:
/// сдвиг на 1 пиксель, переворот влево/вверх, если панель не помещается справа/снизу, затем прижатие к левому/верхнему краю окна.
pub fn place(page_x: f64, page_y: f64, width: f64, height: f64, view: Viewport) -> (f64, f64) {
    let (mut left, mut top) = (page_x + 1.0, page_y + 1.0);
    if left + width - view.scroll_x > view.width {
        left -= width;
    }
    if top + height - view.scroll_y > view.height {
        top -= height;
    }
    if left < view.scroll_x {
        left = view.scroll_x;
    }
    if top < view.scroll_y {
        top = view.scroll_y;
    }
    (left, top)
}

#[cfg(test)]
mod tests {
    use super::*;

    const V: Viewport = Viewport { width: 1000.0, height: 800.0, scroll_x: 0.0, scroll_y: 0.0 };

    #[test]
    fn offsets_by_one_pixel_when_it_fits() {
        assert_eq!(place(100.0, 50.0, 200.0, 100.0, V), (101.0, 51.0));
    }

    #[test]
    fn flips_left_and_up_near_the_far_edges() {
        assert_eq!(place(900.0, 50.0, 200.0, 100.0, V), (701.0, 51.0));
        assert_eq!(place(100.0, 750.0, 200.0, 100.0, V), (101.0, 651.0));
        assert_eq!(place(900.0, 750.0, 200.0, 100.0, V), (701.0, 651.0));
    }

    #[test]
    fn exact_fit_does_not_flip() {
        assert_eq!(place(799.0, 0.0, 200.0, 100.0, V), (800.0, 1.0));
        assert_eq!(place(800.0, 0.0, 200.0, 100.0, V), (801.0 - 200.0, 1.0));
    }

    #[test]
    fn clamps_to_the_near_edges_after_flipping() {
        assert_eq!(place(50.0, 20.0, 400.0, 300.0, Viewport { width: 300.0, height: 200.0, ..V }), (0.0, 0.0));
    }

    #[test]
    fn accounts_for_page_scroll() {
        let scrolled = Viewport { scroll_x: 500.0, scroll_y: 400.0, ..V };
        assert_eq!(place(600.0, 450.0, 200.0, 100.0, scrolled), (601.0, 451.0));
        assert_eq!(place(1450.0, 1150.0, 200.0, 100.0, scrolled), (1251.0, 1051.0));
        assert_eq!(place(10.0, 10.0, 200.0, 100.0, scrolled), (500.0, 400.0));
    }
}
