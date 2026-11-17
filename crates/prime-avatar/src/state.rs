//! Pure Avatar logic: classes and `data-p` (per `avatarstyle.ts` and `Avatar.dataP` of PrimeNG).

use prime_rs_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Avatar size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarSize {
    /// `normal` (default).
    #[default]
    Normal,
    /// `large`.
    Large,
    /// `xlarge`.
    XLarge,
}

impl AvatarSize {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Large => "large",
            Self::XLarge => "xlarge",
        }
    }
}

/// Avatar shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AvatarShape {
    /// `square` (default).
    #[default]
    Square,
    /// `circle`.
    Circle,
}

impl AvatarShape {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Square => "square",
            Self::Circle => "circle",
        }
    }
}

/// Snapshot of the properties that affect classes and `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AvatarState {
    /// Whether an image is set (`image != null`).
    pub has_image: bool,
    /// Shape.
    pub shape: AvatarShape,
    /// Size.
    pub size: AvatarSize,
}

/// Appends the root classes in the order of `avatarstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &AvatarState) {
    cn.add("p-avatar p-component")
        .add_if(s.has_image, "p-avatar-image")
        .add_if(s.shape == AvatarShape::Circle, "p-avatar-circle")
        .add_if(s.size == AvatarSize::Large, "p-avatar-lg")
        .add_if(s.size == AvatarSize::XLarge, "p-avatar-xl");
}

/// `data-p` value: shape and size (always, including defaults).
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
