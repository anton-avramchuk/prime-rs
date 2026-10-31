//! Чистая логика Tooltip: стороны и порядок переноса, геометрия, стрелка, классы, задержки (по `tooltip.ts` PrimeNG).

/// Сторона элемента, с которой показывается подсказка.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Справа (по умолчанию).
    #[default]
    Right,
    /// Слева.
    Left,
    /// Сверху.
    Top,
    /// Снизу.
    Bottom,
}

impl TooltipPosition {
    /// Суффикс класса `p-tooltip-<имя>`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// События, по которым показывается подсказка.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipEvent {
    /// Наведение мыши и касание (по умолчанию).
    #[default]
    Hover,
    /// Фокус и потеря фокуса.
    Focus,
    /// И то и другое.
    Both,
}

impl TooltipEvent {
    /// Слушать наведение и касание.
    pub fn hover(self) -> bool {
        matches!(self, Self::Hover | Self::Both)
    }

    /// Слушать фокус.
    pub fn focus(self) -> bool {
        matches!(self, Self::Focus | Self::Both)
    }
}

/// Порядок сторон: основная, затем запасные, пока подсказка выходит за границы окна.
pub fn priority(position: TooltipPosition) -> [TooltipPosition; 4] {
    use TooltipPosition::{Bottom, Left, Right, Top};
    match position {
        Top => [Top, Bottom, Right, Left],
        Bottom => [Bottom, Top, Right, Left],
        Left => [Left, Right, Top, Bottom],
        Right => [Right, Left, Top, Bottom],
    }
}

/// Размер в пикселях.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    /// Ширина.
    pub width: f64,
    /// Высота.
    pub height: f64,
}

/// Смещение подсказки относительно левого верхнего угла элемента.
pub fn place(position: TooltipPosition, host: Size, tip: Size) -> (f64, f64) {
    match position {
        TooltipPosition::Right => (host.width, (host.height - tip.height) / 2.0),
        TooltipPosition::Left => (-tip.width, (host.height - tip.height) / 2.0),
        TooltipPosition::Top => ((host.width - tip.width) / 2.0, -tip.height),
        TooltipPosition::Bottom => ((host.width - tip.width) / 2.0, host.height),
    }
}

/// Положение стрелки: `None` — свойство снимается.
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

/// Стрелка у края, обращённого к элементу; у `top`/`bottom` — по центру подсказки.
pub fn arrow_style(position: TooltipPosition, tip_width: f64) -> ArrowStyle {
    let center = Some(format!("{}px", tip_width / 2.0));
    match position {
        TooltipPosition::Right => ArrowStyle { top: Some("50%".into()), left: Some("0".into()), ..Default::default() },
        TooltipPosition::Left => ArrowStyle { top: Some("50%".into()), right: Some("0".into()), ..Default::default() },
        TooltipPosition::Top => ArrowStyle { bottom: Some("0".into()), left: center, ..Default::default() },
        TooltipPosition::Bottom => ArrowStyle { top: Some("0".into()), left: center, ..Default::default() },
    }
}

/// Подсказка (в координатах окна) выходит за границы окна.
pub fn out_of_bounds(left: f64, top: f64, width: f64, height: f64, viewport_w: f64, viewport_h: f64) -> bool {
    left + width > viewport_w || left < 0.0 || top < 0.0 || top + height > viewport_h
}

/// Классы корня: `p-tooltip p-component`, сторона и пользовательский класс.
pub fn root_classes(position: TooltipPosition, style_class: Option<&str>) -> String {
    let mut cn = format!("p-tooltip p-component p-tooltip-{}", position.as_str());
    if let Some(extra) = style_class.filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(extra);
    }
    cn
}

/// Через сколько показать подсказку и через сколько скрыть по `life` (мс): `(показ, скрытие)`.
///
/// Нулевые значения считаются «не задано»; `life` отсчитывается после `show_delay`.
pub fn activation_delay(show_delay: Option<u32>, life: Option<u32>) -> (Option<u32>, Option<u32>) {
    let show = show_delay.filter(|d| *d > 0);
    let hide = life.filter(|l| *l > 0).map(|l| l + show.unwrap_or(0));
    (show, hide)
}

/// Содержимое элемента не помещается в него (`offsetWidth < scrollWidth || offsetHeight < scrollHeight`).
pub fn has_ellipsis(offset_width: f64, scroll_width: f64, offset_height: f64, scroll_height: f64) -> bool {
    offset_width < scroll_width || offset_height < scroll_height
}

/// Уход курсора с элемента не скрывает подсказку: она не автоскрываемая, а курсор перешёл на неё.
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
            assert_eq!(order[0], p, "первой идёт основная сторона");
            for q in [Top, Bottom, Left, Right] {
                assert_eq!(order.iter().filter(|x| **x == q).count(), 1, "{p:?}: {q:?} ровно один раз");
            }
        }
    }

    #[test]
    fn place_table() {
        // (сторона, ожидаемое смещение)
        let cases = [(Right, (100.0, 10.0)), (Left, (-60.0, 10.0)), (Top, (20.0, -20.0)), (Bottom, (20.0, 40.0))];
        for (p, want) in cases {
            assert_eq!(place(p, HOST, TIP), want, "{p:?}");
        }
        // подсказка шире элемента: центрируется с отрицательным сдвигом
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
        // окно 1000x800, подсказка 100x50
        let oob = |l, t| out_of_bounds(l, t, 100.0, 50.0, 1000.0, 800.0);
        assert!(!oob(10.0, 10.0));
        assert!(!oob(900.0, 750.0), "касается правого и нижнего края — ещё помещается");
        assert!(oob(901.0, 10.0), "вылезает вправо");
        assert!(oob(10.0, 751.0), "вылезает вниз");
        assert!(oob(-1.0, 10.0), "вылезает влево");
        assert!(oob(10.0, -1.0), "вылезает вверх");
    }

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(Right, None), "p-tooltip p-component p-tooltip-right");
        assert_eq!(root_classes(Top, Some("custom wide")), "p-tooltip p-component p-tooltip-top custom wide");
        assert_eq!(root_classes(Left, Some("")), "p-tooltip p-component p-tooltip-left");
    }

    #[test]
    fn activation_delay_table() {
        // (show_delay, life, ожидание)
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
        assert!(has_ellipsis(100.0, 140.0, 20.0, 20.0), "по ширине");
        assert!(has_ellipsis(100.0, 100.0, 20.0, 35.0), "по высоте");
        assert!(leave_keeps_open(false, true));
        assert!(!leave_keeps_open(false, false));
        assert!(!leave_keeps_open(true, true), "при auto_hide подсказка всегда скрывается");
    }

    #[test]
    fn events() {
        assert!(TooltipEvent::Hover.hover() && !TooltipEvent::Hover.focus());
        assert!(!TooltipEvent::Focus.hover() && TooltipEvent::Focus.focus());
        assert!(TooltipEvent::Both.hover() && TooltipEvent::Both.focus());
    }
}
