//! Чистая логика ScrollTop: порог видимости, классы корня и фаза показа (по `scrolltop.ts` и `scrolltopstyle.ts` PrimeNG).

/// Что прокручивает кнопка.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollTopTarget {
    /// Окно (кнопка `fixed`).
    #[default]
    Window,
    /// Родительский элемент (кнопка `sticky`).
    Parent,
}

/// Поведение прокрутки по клику.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollTopBehavior {
    /// Плавная прокрутка.
    #[default]
    Smooth,
    /// Мгновенный переход.
    Auto,
}

/// Фаза показа кнопки.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Не показана.
    Hidden,
    /// Появляется (идёт анимация входа).
    Entering,
    /// Показана.
    Shown,
    /// Исчезает (идёт анимация выхода).
    Leaving,
}

/// Кнопка должна быть видна: прокрутка строго больше порога.
pub fn is_visible(scroll: f64, threshold: f64) -> bool {
    scroll > threshold
}

/// Классы корня: `p-scrolltop` и `p-scrolltop-sticky` для родителя.
pub fn root_classes(target: ScrollTopTarget) -> &'static str {
    match target {
        ScrollTopTarget::Window => "p-scrolltop",
        ScrollTopTarget::Parent => "p-scrolltop p-scrolltop-sticky",
    }
}

/// Классы анимации входа: сначала начальное состояние, потом (на следующем кадре) только активный переход.
pub fn enter_classes(started: bool) -> &'static str {
    if started { "p-scrolltop-enter-active p-scrolltop-enter-to" } else { "p-scrolltop-enter-from p-scrolltop-enter-active" }
}

/// Классы анимации выхода.
pub fn leave_classes(started: bool) -> &'static str {
    if started { "p-scrolltop-leave-active p-scrolltop-leave-to" } else { "p-scrolltop-leave-from p-scrolltop-leave-active" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_is_strictly_above_threshold() {
        let cases = [
            (0.0, 400.0, false),
            (400.0, 400.0, false),
            (400.5, 400.0, true),
            (401.0, 400.0, true),
            (150.0, 100.0, true),
            (100.0, 100.0, false),
        ];
        for (scroll, threshold, want) in cases {
            assert_eq!(is_visible(scroll, threshold), want, "{scroll} > {threshold}");
        }
    }

    #[test]
    fn root_classes_by_target() {
        assert_eq!(root_classes(ScrollTopTarget::Window), "p-scrolltop");
        assert_eq!(root_classes(ScrollTopTarget::Parent), "p-scrolltop p-scrolltop-sticky");
    }

    #[test]
    fn animation_classes() {
        assert_eq!(enter_classes(false), "p-scrolltop-enter-from p-scrolltop-enter-active");
        assert_eq!(enter_classes(true), "p-scrolltop-enter-active p-scrolltop-enter-to");
        assert_eq!(leave_classes(false), "p-scrolltop-leave-from p-scrolltop-leave-active");
        assert_eq!(leave_classes(true), "p-scrolltop-leave-active p-scrolltop-leave-to");
    }

    #[test]
    fn defaults() {
        assert_eq!(ScrollTopTarget::default(), ScrollTopTarget::Window);
        assert_eq!(ScrollTopBehavior::default(), ScrollTopBehavior::Smooth);
    }
}
