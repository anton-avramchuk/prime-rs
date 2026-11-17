//! Pure Panel logic: classes and `data-p` (based on `panelstyle.ts` and `Panel.dataP` in PrimeNG).

use prime_rs_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Icon position in the header (`iconPos`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconPos {
    /// At the start of the header.
    Start,
    /// At the end of the header.
    #[default]
    End,
    /// Centered.
    Center,
}

/// What toggles the panel (`toggler`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Toggler {
    /// By the button in the header.
    #[default]
    Icon,
    /// By clicking the whole header.
    Header,
}

/// Adds the root classes in the order of `panelstyle.ts`.
pub fn write_root_classes(cn: &mut ClassNames, toggleable: bool, collapsed: bool) {
    cn.add("p-panel p-component")
        .add_if(toggleable, "p-panel-toggleable")
        .add_if(toggleable && !collapsed, "p-panel-expanded")
        .add_if(toggleable && collapsed, "p-panel-collapsed");
}

/// Classes of the header actions block.
pub fn actions_class(pos: IconPos) -> &'static str {
    match pos {
        IconPos::Start => "p-panel-header-actions p-panel-icons-start",
        IconPos::End => "p-panel-header-actions p-panel-icons-end",
        IconPos::Center => "p-panel-header-actions p-panel-icons-center",
    }
}

/// `data-p` value of the root and header: `toggleable`.
pub fn data_p(toggleable: bool) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(toggleable, "toggleable");
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(toggleable: bool, collapsed: bool) -> String {
        let mut cn = ClassNames::new();
        write_root_classes(&mut cn, toggleable, collapsed);
        cn.finish()
    }

    #[test]
    fn root_classes_table() {
        let cases = [
            (false, false, "p-panel p-component"),
            (false, true, "p-panel p-component"),
            (true, false, "p-panel p-component p-panel-toggleable p-panel-expanded"),
            (true, true, "p-panel p-component p-panel-toggleable p-panel-collapsed"),
        ];
        for (t, c, want) in cases {
            assert_eq!(root(t, c), want, "toggleable={t} collapsed={c}");
        }
    }

    #[test]
    fn actions_and_data_p() {
        assert_eq!(actions_class(IconPos::default()), "p-panel-header-actions p-panel-icons-end");
        assert_eq!(actions_class(IconPos::Start), "p-panel-header-actions p-panel-icons-start");
        assert_eq!(actions_class(IconPos::Center), "p-panel-header-actions p-panel-icons-center");
        assert_eq!((data_p(true).as_ref(), data_p(false).as_ref()), ("toggleable", ""));
    }
}
