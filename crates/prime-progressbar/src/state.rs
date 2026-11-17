//! Pure ProgressBar logic: mode, classes and `data-p` (after PrimeNG `progressbarstyle.ts` and `ProgressBar.dataP`).

use prime_rs_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Indicator mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ProgressBarMode {
    /// Determinate: width by value (default).
    #[default]
    Determinate,
    /// Indeterminate: animation without a value.
    Indeterminate,
}

impl ProgressBarMode {
    /// Value, as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Determinate => "determinate",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// Snapshot of the properties that affect classes and `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProgressBarState {
    /// Mode.
    pub mode: ProgressBarMode,
}

/// Adds the root classes in the order of `progressbarstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &ProgressBarState) {
    cn.add("p-progressbar p-component").add(match s.mode {
        ProgressBarMode::Determinate => "p-progressbar-determinate",
        ProgressBarMode::Indeterminate => "p-progressbar-indeterminate",
    });
}

/// Value of `data-p`: `determinate` or `indeterminate`.
pub fn data_p(s: &ProgressBarState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(s.mode.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_and_data_p_per_mode() {
        let cases = [
            (ProgressBarMode::Determinate, "p-progressbar p-component p-progressbar-determinate", "determinate"),
            (ProgressBarMode::Indeterminate, "p-progressbar p-component p-progressbar-indeterminate", "indeterminate"),
        ];
        for (mode, class, data) in cases {
            let s = ProgressBarState { mode };
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &s);
            assert_eq!(cn.finish(), class);
            assert_eq!(data_p(&s), data);
        }
        assert_eq!(ProgressBarMode::default(), ProgressBarMode::Determinate);
    }
}
