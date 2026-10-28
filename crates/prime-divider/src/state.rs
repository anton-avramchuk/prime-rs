//! Чистая логика Divider: классы, inline-стиль выравнивания и `data-p` (по `dividerstyle.ts` и `Divider.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Направление разделителя.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerLayout {
    /// `horizontal` (по умолчанию).
    #[default]
    Horizontal,
    /// `vertical`.
    Vertical,
}

impl DividerLayout {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Тип линии.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DividerType {
    /// `solid` (по умолчанию).
    #[default]
    Solid,
    /// `dashed`.
    Dashed,
    /// `dotted`.
    Dotted,
}

impl DividerType {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}

/// Выравнивание содержимого.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DividerAlign {
    /// `left` (для горизонтального).
    Left,
    /// `center`.
    Center,
    /// `right` (для горизонтального).
    Right,
    /// `top` (для вертикального).
    Top,
    /// `bottom` (для вертикального).
    Bottom,
}

impl DividerAlign {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Снимок свойств, влияющих на классы, стиль и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DividerState {
    /// Направление.
    pub layout: DividerLayout,
    /// Тип линии.
    pub divider_type: DividerType,
    /// Выравнивание (`None` — не задано).
    pub align: Option<DividerAlign>,
}

/// Класс выравнивания: допустимы только сочетания горизонтального с `left`/`center`/`right` и вертикального с `top`/`center`/`bottom`.
fn align_class(s: &DividerState) -> Option<&'static str> {
    use DividerAlign::*;
    use DividerLayout::*;
    match (s.layout, s.align) {
        (Horizontal, None | Some(Left)) => Some("p-divider-left"),
        (Horizontal, Some(Center)) => Some("p-divider-center"),
        (Horizontal, Some(Right)) => Some("p-divider-right"),
        (Vertical, Some(Top)) => Some("p-divider-top"),
        (Vertical, None | Some(Center)) => Some("p-divider-center"),
        (Vertical, Some(Bottom)) => Some("p-divider-bottom"),
        _ => None,
    }
}

/// Добавляет классы корня в порядке `dividerstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &DividerState) {
    cn.add("p-divider p-component");
    cn.add(match s.layout {
        DividerLayout::Horizontal => "p-divider-horizontal",
        DividerLayout::Vertical => "p-divider-vertical",
    });
    cn.add(match s.divider_type {
        DividerType::Solid => "p-divider-solid",
        DividerType::Dashed => "p-divider-dashed",
        DividerType::Dotted => "p-divider-dotted",
    });
    cn.add_opt(align_class(s));
}

/// Inline-стиль выравнивания: свойство и значение (`justify-content` для горизонтального, `align-items` для вертикального).
pub fn align_style(s: &DividerState) -> Option<(&'static str, &'static str)> {
    use DividerAlign::*;
    use DividerLayout::*;
    match (s.layout, s.align) {
        (Horizontal, None | Some(Center)) => Some(("justify-content", "center")),
        (Horizontal, Some(Left)) => Some(("justify-content", "flex-start")),
        (Horizontal, Some(Right)) => Some(("justify-content", "flex-end")),
        (Vertical, None | Some(Center)) => Some(("align-items", "center")),
        (Vertical, Some(Top)) => Some(("align-items", "flex-start")),
        (Vertical, Some(Bottom)) => Some(("align-items", "flex-end")),
        _ => None,
    }
}

/// Значение `data-p`: выравнивание (если задано), направление, тип.
pub fn data_p(s: &DividerState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_opt(s.align.map(DividerAlign::as_str)).add(s.layout.as_str()).add(s.divider_type.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;
    use DividerAlign::*;
    use DividerLayout::*;
    use DividerType::*;

    fn classes(s: &DividerState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    fn st(layout: DividerLayout, divider_type: DividerType, align: Option<DividerAlign>) -> DividerState {
        DividerState { layout, divider_type, align }
    }

    #[test]
    fn defaults() {
        let s = DividerState::default();
        assert_eq!(classes(&s), "p-divider p-component p-divider-horizontal p-divider-solid p-divider-left");
        assert_eq!(data_p(&s), "horizontal solid");
        assert_eq!(align_style(&s), Some(("justify-content", "center")));
    }

    /// Все сочетания направления и выравнивания: (layout, align, класс выравнивания, стиль).
    #[test]
    fn layout_by_align_table() {
        type Row = (DividerLayout, Option<DividerAlign>, Option<&'static str>, Option<(&'static str, &'static str)>);
        let rows: Vec<Row> = vec![
            (Horizontal, None, Some("p-divider-left"), Some(("justify-content", "center"))),
            (Horizontal, Some(Left), Some("p-divider-left"), Some(("justify-content", "flex-start"))),
            (Horizontal, Some(Center), Some("p-divider-center"), Some(("justify-content", "center"))),
            (Horizontal, Some(Right), Some("p-divider-right"), Some(("justify-content", "flex-end"))),
            (Horizontal, Some(Top), None, None),
            (Horizontal, Some(Bottom), None, None),
            (Vertical, None, Some("p-divider-center"), Some(("align-items", "center"))),
            (Vertical, Some(Top), Some("p-divider-top"), Some(("align-items", "flex-start"))),
            (Vertical, Some(Center), Some("p-divider-center"), Some(("align-items", "center"))),
            (Vertical, Some(Bottom), Some("p-divider-bottom"), Some(("align-items", "flex-end"))),
            (Vertical, Some(Left), None, None),
            (Vertical, Some(Right), None, None),
        ];
        for (layout, align, class, style) in rows {
            let s = st(layout, Solid, align);
            assert_eq!(align_class(&s), class, "{layout:?} {align:?}");
            assert_eq!(align_style(&s), style, "{layout:?} {align:?}");
        }
    }

    #[test]
    fn classes_and_data_p_examples() {
        let s = st(Horizontal, Dotted, Some(Center));
        assert_eq!(classes(&s), "p-divider p-component p-divider-horizontal p-divider-dotted p-divider-center");
        assert_eq!(data_p(&s), "center horizontal dotted");
        let v = st(Vertical, Dashed, None);
        assert_eq!(classes(&v), "p-divider p-component p-divider-vertical p-divider-dashed p-divider-center");
        assert_eq!(data_p(&v), "vertical dashed");
        let bad = st(Vertical, Solid, Some(Left));
        assert_eq!(classes(&bad), "p-divider p-component p-divider-vertical p-divider-solid");
        assert_eq!(data_p(&bad), "left vertical solid");
    }
}
