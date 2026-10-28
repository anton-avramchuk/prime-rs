//! Чистая логика ProgressBar: режим, классы и `data-p` (по `progressbarstyle.ts` и `ProgressBar.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Режим индикатора.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ProgressBarMode {
    /// Определённый: ширина по значению (по умолчанию).
    #[default]
    Determinate,
    /// Неопределённый: анимация без значения.
    Indeterminate,
}

impl ProgressBarMode {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Determinate => "determinate",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProgressBarState {
    /// Режим.
    pub mode: ProgressBarMode,
}

/// Добавляет классы корня в порядке `progressbarstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &ProgressBarState) {
    cn.add("p-progressbar p-component").add(match s.mode {
        ProgressBarMode::Determinate => "p-progressbar-determinate",
        ProgressBarMode::Indeterminate => "p-progressbar-indeterminate",
    });
}

/// Значение `data-p`: `determinate` или `indeterminate`.
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
