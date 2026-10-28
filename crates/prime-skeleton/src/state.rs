//! Чистая логика Skeleton: формы, анимация, классы, `data-p` и встроенный стиль (по `skeleton.ts` и `skeletonstyle.ts` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;
use std::fmt::Write;

/// Форма заглушки (`shape`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SkeletonShape {
    /// Прямоугольник (по умолчанию).
    #[default]
    Rectangle,
    /// Круг.
    Circle,
}

impl SkeletonShape {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rectangle => "rectangle",
            Self::Circle => "circle",
        }
    }
}

/// Анимация заглушки (`animation`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SkeletonAnimation {
    /// Волна (по умолчанию).
    #[default]
    Wave,
    /// Без анимации.
    None,
}

/// Добавляет классы корня в порядке `skeletonstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, shape: SkeletonShape, animation: SkeletonAnimation) {
    cn.add("p-skeleton p-component")
        .add_if(shape == SkeletonShape::Circle, "p-skeleton-circle")
        .add_if(animation == SkeletonAnimation::None, "p-skeleton-animation-none");
}

/// Значение `data-p`: имя формы.
pub fn data_p(shape: SkeletonShape) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(shape.as_str());
    d.finish_cow()
}

/// Встроенный стиль (без завершающей `;`, её добавляет Leptos): `position: relative`, `width`, `height`, `border-radius`; при `size` ширина и высота равны ему.
pub fn container_style(size: Option<&str>, width: Option<&str>, height: Option<&str>, border_radius: Option<&str>) -> String {
    let mut out = String::with_capacity(80);
    out.push_str("position: relative;");
    let (w, h) = match size {
        Some(size) => (size, size),
        None => (width.unwrap_or("100%"), height.unwrap_or("1rem")),
    };
    let _ = write!(out, " width: {w}; height: {h}");
    if let Some(r) = border_radius {
        let _ = write!(out, "; border-radius: {r}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_and_data_p_table() {
        use SkeletonAnimation::{None as NoAnim, Wave};
        use SkeletonShape::{Circle, Rectangle};
        let cases = [
            (Rectangle, Wave, "p-skeleton p-component", "rectangle"),
            (Circle, Wave, "p-skeleton p-component p-skeleton-circle", "circle"),
            (Rectangle, NoAnim, "p-skeleton p-component p-skeleton-animation-none", "rectangle"),
            (Circle, NoAnim, "p-skeleton p-component p-skeleton-circle p-skeleton-animation-none", "circle"),
        ];
        for (shape, animation, class, data) in cases {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, shape, animation);
            assert_eq!(cn.finish(), class);
            assert_eq!(data_p(shape), data);
        }
    }

    #[test]
    fn style_defaults_size_and_radius() {
        assert_eq!(container_style(None, None, None, None), "position: relative; width: 100%; height: 1rem");
        assert_eq!(container_style(None, Some("10rem"), Some("4rem"), None), "position: relative; width: 10rem; height: 4rem");
        assert_eq!(container_style(Some("3rem"), Some("9rem"), Some("9rem"), None), "position: relative; width: 3rem; height: 3rem");
        assert_eq!(container_style(None, None, None, Some("16px")), "position: relative; width: 100%; height: 1rem; border-radius: 16px");
    }
}
