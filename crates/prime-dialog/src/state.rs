//! Чистая логика Dialog: положение, классы, инлайн-стили, расчёт перетаскивания и размера (по `dialog.ts` и `dialogstyle.ts` PrimeNG).

/// Положение окна на экране.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogPosition {
    /// По центру (по умолчанию).
    #[default]
    Center,
    /// Сверху по центру.
    Top,
    /// Снизу по центру.
    Bottom,
    /// Слева по центру.
    Left,
    /// Справа по центру.
    Right,
    /// Левый верхний угол.
    TopLeft,
    /// Правый верхний угол.
    TopRight,
    /// Левый нижний угол.
    BottomLeft,
    /// Правый нижний угол.
    BottomRight,
}

impl DialogPosition {
    /// Суффикс класса маски (`p-dialog-<имя>`); у центра класса нет.
    pub fn class_suffix(self) -> Option<&'static str> {
        match self {
            Self::Center => None,
            Self::Top => Some("top"),
            Self::Bottom => Some("bottom"),
            Self::Left => Some("left"),
            Self::Right => Some("right"),
            Self::TopLeft => Some("topleft"),
            Self::TopRight => Some("topright"),
            Self::BottomLeft => Some("bottomleft"),
            Self::BottomRight => Some("bottomright"),
        }
    }

    fn justify(self) -> &'static str {
        match self {
            Self::Left | Self::TopLeft | Self::BottomLeft => "flex-start",
            Self::Right | Self::TopRight | Self::BottomRight => "flex-end",
            _ => "center",
        }
    }

    fn align(self) -> &'static str {
        match self {
            Self::Top | Self::TopLeft | Self::TopRight => "flex-start",
            Self::Bottom | Self::BottomLeft | Self::BottomRight => "flex-end",
            _ => "center",
        }
    }
}

/// Классы маски: `p-dialog-mask`, `p-overlay-mask` при `modal`, `p-dialog-<положение>`.
pub fn mask_classes(position: DialogPosition, modal: bool) -> String {
    let mut cn = String::from("p-dialog-mask");
    if modal {
        cn.push_str(" p-overlay-mask");
    }
    if let Some(suffix) = position.class_suffix() {
        cn.push_str(" p-dialog-");
        cn.push_str(suffix);
    }
    cn
}

/// Классы маски на время анимации: у модальной — `p-overlay-mask-enter-active` / `-leave-active`.
pub fn mask_animation_class(modal: bool, leaving: bool) -> &'static str {
    match (modal, leaving) {
        (false, _) => "",
        (true, false) => "p-overlay-mask-enter-active",
        (true, true) => "p-overlay-mask-leave-active",
    }
}

/// Классы окна: `p-dialog p-component`, `p-dialog-maximized` и класс анимации.
pub fn root_classes(maximized: bool, leaving: bool) -> String {
    let mut cn = String::from("p-dialog p-component");
    if maximized {
        cn.push_str(" p-dialog-maximized");
    }
    cn.push_str(if leaving { " p-dialog-leave-active" } else { " p-dialog-enter-active" });
    cn
}

/// Инлайн-стиль маски: на весь экран, выравнивание окна по положению, клики только у модальной.
pub fn mask_inline_style(position: DialogPosition, modal: bool) -> String {
    format!(
        "position: fixed; height: 100%; width: 100%; left: 0; top: 0; display: flex; justify-content: {}; align-items: {}; pointer-events: {}",
        position.justify(),
        position.align(),
        if modal { "auto" } else { "none" }
    )
}

/// Инлайн-стиль окна.
pub fn root_inline_style() -> &'static str {
    "display: flex; flex-direction: column; pointer-events: auto"
}

/// Новые `left` и `top` окна при перетаскивании: прямоугольник окна плюс смещение указателя минус поля.
pub fn drag_position(rect_left: f64, rect_top: f64, margin_left: f64, margin_top: f64, dx: f64, dy: f64) -> (f64, f64) {
    (rect_left + dx - margin_left, rect_top + dy - margin_top)
}

/// Принимаются ли новые координаты по осям `(x, y)`.
///
/// При `keep_in_viewport` ось принимается, только если окно остаётся в пределах `min_*` и экрана; оси независимы.
#[allow(clippy::too_many_arguments)]
pub fn drag_accept(
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    viewport_w: f64,
    viewport_h: f64,
    min_x: f64,
    min_y: f64,
    keep_in_viewport: bool,
) -> (bool, bool) {
    if !keep_in_viewport {
        return (true, true);
    }
    (left >= min_x && left + width < viewport_w, top >= min_y && top + height < viewport_h)
}

/// Новые размеры окна при изменении размера: `(ширина, высота)`, принятые по осям.
///
/// Пока окно не перетаскивали (`not_dragged`), оно центрировано и растёт в обе стороны, поэтому смещение удваивается.
#[allow(clippy::too_many_arguments)]
pub fn resize_to(
    width: f64,
    height: f64,
    dx: f64,
    dy: f64,
    not_dragged: bool,
    min_width: Option<f64>,
    min_height: Option<f64>,
    left: f64,
    top: f64,
    viewport_w: f64,
    viewport_h: f64,
) -> (Option<f64>, Option<f64>) {
    let (mut new_width, mut new_height) = (width + dx, height + dy);
    if not_dragged {
        new_width += dx;
        new_height += dy;
    }
    let w = (min_width.is_none_or(|m| new_width > m) && left + new_width < viewport_w).then_some(new_width);
    let h = (min_height.is_none_or(|m| new_height > m) && top + new_height < viewport_h).then_some(new_height);
    (w, h)
}

/// Правила `@media` для адаптивной ширины окна; `attr` — имя атрибута, поставленного на окно.
pub fn breakpoints_css(attr: &str, breakpoints: &[(String, String)]) -> String {
    breakpoints
        .iter()
        .map(|(bp, width)| {
            format!(
                "@media screen and (max-width: {bp}) {{ .p-dialog[{attr}]:not(.p-dialog-maximized) {{ width: {width} !important; }} }}\n"
            )
        })
        .collect()
}

/// Эта панель — верхняя по `z-index` (только она закрывается по Escape).
pub fn is_topmost(own_z: i32, top_z: i32) -> bool {
    own_z >= top_z
}

/// `z-index` маски: на единицу ниже окна.
pub fn mask_z(panel_z: i32) -> i32 {
    panel_z - 1
}

/// Значение `data-p`: признаки через пробел.
pub fn data_p(maximized: bool, modal: bool) -> String {
    let mut parts = Vec::new();
    if maximized {
        parts.push("maximized");
    }
    if modal {
        parts.push("modal");
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::DialogPosition::*;
    use super::*;

    #[test]
    fn mask_classes_table() {
        // (положение, modal, ожидание)
        let cases = [
            (Center, false, "p-dialog-mask"),
            (Center, true, "p-dialog-mask p-overlay-mask"),
            (Top, false, "p-dialog-mask p-dialog-top"),
            (BottomRight, true, "p-dialog-mask p-overlay-mask p-dialog-bottomright"),
            (TopLeft, false, "p-dialog-mask p-dialog-topleft"),
        ];
        for (p, m, want) in cases {
            assert_eq!(mask_classes(p, m), want, "{p:?} {m}");
        }
    }

    #[test]
    fn animation_and_root_classes() {
        assert_eq!(mask_animation_class(true, false), "p-overlay-mask-enter-active");
        assert_eq!(mask_animation_class(true, true), "p-overlay-mask-leave-active");
        assert_eq!(mask_animation_class(false, true), "");
        assert_eq!(root_classes(false, false), "p-dialog p-component p-dialog-enter-active");
        assert_eq!(root_classes(true, true), "p-dialog p-component p-dialog-maximized p-dialog-leave-active");
    }

    #[test]
    fn mask_inline_style_table() {
        // (положение, justify, align)
        let cases = [
            (Center, "center", "center"),
            (Top, "center", "flex-start"),
            (Bottom, "center", "flex-end"),
            (Left, "flex-start", "center"),
            (Right, "flex-end", "center"),
            (TopLeft, "flex-start", "flex-start"),
            (TopRight, "flex-end", "flex-start"),
            (BottomLeft, "flex-start", "flex-end"),
            (BottomRight, "flex-end", "flex-end"),
        ];
        for (p, j, a) in cases {
            let css = mask_inline_style(p, true);
            assert!(css.contains(&format!("justify-content: {j}")) && css.contains(&format!("align-items: {a}")), "{p:?}: {css}");
            assert!(css.contains("pointer-events: auto") && css.starts_with("position: fixed"));
        }
        assert!(mask_inline_style(Center, false).ends_with("pointer-events: none"));
        assert!(root_inline_style().contains("flex-direction: column") && root_inline_style().contains("pointer-events: auto"));
    }

    #[test]
    fn drag_position_subtracts_margins() {
        assert_eq!(drag_position(100.0, 50.0, 16.0, 8.0, 10.0, -5.0), (94.0, 37.0));
        assert_eq!(drag_position(100.0, 50.0, 0.0, 0.0, 0.0, 0.0), (100.0, 50.0));
    }

    #[test]
    fn drag_accept_table() {
        // viewport 1000x800, окно 300x200, min 0/0
        let accept = |l, t, keep| drag_accept(l, t, 300.0, 200.0, 1000.0, 800.0, 0.0, 0.0, keep);
        assert_eq!(accept(100.0, 100.0, true), (true, true));
        assert_eq!(accept(-5.0, 100.0, true), (false, true), "левее min_x");
        assert_eq!(accept(100.0, -5.0, true), (true, false), "выше min_y");
        assert_eq!(accept(700.0, 100.0, true), (false, true), "правый край в границе экрана не помещается");
        assert_eq!(accept(699.0, 599.0, true), (true, true));
        assert_eq!(accept(-500.0, 5000.0, false), (true, true), "без keep_in_viewport — всегда");
        assert_eq!(drag_accept(5.0, 5.0, 100.0, 100.0, 1000.0, 800.0, 10.0, 20.0, true), (false, false), "min_x и min_y");
    }

    #[test]
    fn resize_to_table() {
        // окно 400x300 в экране 1000x800, левый верх (100, 100)
        let go =
            |dx, dy, not_dragged, min_w, min_h| resize_to(400.0, 300.0, dx, dy, not_dragged, min_w, min_h, 100.0, 100.0, 1000.0, 800.0);
        assert_eq!(go(40.0, 20.0, false, None, None), (Some(440.0), Some(320.0)));
        assert_eq!(go(40.0, 20.0, true, None, None), (Some(480.0), Some(340.0)), "не перетаскивали: смещение удваивается");
        assert_eq!(go(-100.0, 0.0, false, Some(350.0), None), (None, Some(300.0)), "уже минимальной ширины");
        assert_eq!(go(0.0, -100.0, false, None, Some(250.0)), (Some(400.0), None));
        assert_eq!(go(600.0, 0.0, false, None, None), (None, Some(300.0)), "не помещается по ширине");
        assert_eq!(go(0.0, 500.0, false, None, None), (Some(400.0), None), "не помещается по высоте");
    }

    #[test]
    fn breakpoints_css_table() {
        assert_eq!(breakpoints_css("data-dlg-1", &[]), "");
        let css = breakpoints_css("data-dlg-7", &[("1199px".into(), "75vw".into()), ("575px".into(), "90vw".into())]);
        assert_eq!(css.lines().count(), 2);
        assert!(css.contains(
            "@media screen and (max-width: 1199px) { .p-dialog[data-dlg-7]:not(.p-dialog-maximized) { width: 75vw !important; } }"
        ));
        assert!(css.contains("max-width: 575px") && css.contains("width: 90vw !important"));
    }

    #[test]
    fn layering_and_data_p() {
        assert!(is_topmost(1102, 1102) && is_topmost(1104, 1102) && !is_topmost(1100, 1102));
        assert_eq!(mask_z(1102), 1101);
        assert_eq!(data_p(true, true), "maximized modal");
        assert_eq!(data_p(false, true), "modal");
        assert_eq!(data_p(false, false), "");
    }
}
