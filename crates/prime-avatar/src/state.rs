//! Чистая логика Avatar: классы и `data-p` (по `avatarstyle.ts` и `Avatar.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Размер аватара.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarSize {
    /// `normal` (по умолчанию).
    #[default]
    Normal,
    /// `large`.
    Large,
    /// `xlarge`.
    XLarge,
}

impl AvatarSize {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Large => "large",
            Self::XLarge => "xlarge",
        }
    }
}

/// Форма аватара.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarShape {
    /// `square` (по умолчанию).
    #[default]
    Square,
    /// `circle`.
    Circle,
}

impl AvatarShape {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Square => "square",
            Self::Circle => "circle",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AvatarState {
    /// Задано ли изображение (`image != null`).
    pub has_image: bool,
    /// Форма.
    pub shape: AvatarShape,
    /// Размер.
    pub size: AvatarSize,
}

/// Добавляет классы корня в порядке `avatarstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &AvatarState) {
    cn.add("p-avatar p-component")
        .add_if(s.has_image, "p-avatar-image")
        .add_if(s.shape == AvatarShape::Circle, "p-avatar-circle")
        .add_if(s.size == AvatarSize::Large, "p-avatar-lg")
        .add_if(s.size == AvatarSize::XLarge, "p-avatar-xl");
}

/// Значение `data-p`: форма и размер (всегда, включая значения по умолчанию).
pub fn data_p(s: &AvatarState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(s.shape.as_str()).add(s.size.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &AvatarState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        use AvatarShape::*;
        use AvatarSize::*;
        let st = |has_image, shape, size| AvatarState { has_image, shape, size };
        let cases = [
            (AvatarState::default(), "p-avatar p-component", "square normal"),
            (st(true, Square, Normal), "p-avatar p-component p-avatar-image", "square normal"),
            (st(false, Circle, Normal), "p-avatar p-component p-avatar-circle", "circle normal"),
            (st(false, Square, Large), "p-avatar p-component p-avatar-lg", "square large"),
            (st(false, Square, XLarge), "p-avatar p-component p-avatar-xl", "square xlarge"),
            (st(true, Circle, XLarge), "p-avatar p-component p-avatar-image p-avatar-circle p-avatar-xl", "circle xlarge"),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
