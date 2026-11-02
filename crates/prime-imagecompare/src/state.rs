//! Чистая логика ImageCompare: форма обрезки по значению ползунка (по `onSlide` PrimeNG).

/// Процент без хвостовых нулей: `30` → `30`, `33.5` → `33.5`.
fn percent(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if rounded.fract() == 0.0 { format!("{}", rounded as i64) } else { format!("{rounded}") }
}

/// Значение `clip-path` для значения ползунка `value` (0–100, прижимается): слева направо или справа налево (`rtl`).
pub fn clip_path(value: f64, rtl: bool) -> String {
    let v = if value.is_nan() { 0.0 } else { value.clamp(0.0, 100.0) };
    if rtl {
        let from = percent(100.0 - v);
        format!("polygon({from}% 0, 100% 0, 100% 100%, {from}% 100%)")
    } else {
        let to = percent(v);
        format!("polygon(0 0, {to}% 0, {to}% 100%, 0 100%)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_to_right_table() {
        assert_eq!(clip_path(50.0, false), "polygon(0 0, 50% 0, 50% 100%, 0 100%)");
        assert_eq!(clip_path(30.0, false), "polygon(0 0, 30% 0, 30% 100%, 0 100%)");
        assert_eq!(clip_path(0.0, false), "polygon(0 0, 0% 0, 0% 100%, 0 100%)");
        assert_eq!(clip_path(100.0, false), "polygon(0 0, 100% 0, 100% 100%, 0 100%)");
        assert_eq!(clip_path(33.5, false), "polygon(0 0, 33.5% 0, 33.5% 100%, 0 100%)");
    }

    #[test]
    fn right_to_left_mirrors_the_value() {
        assert_eq!(clip_path(30.0, true), "polygon(70% 0, 100% 0, 100% 100%, 70% 100%)");
        assert_eq!(clip_path(100.0, true), "polygon(0% 0, 100% 0, 100% 100%, 0% 100%)");
        assert_eq!(clip_path(0.0, true), "polygon(100% 0, 100% 0, 100% 100%, 100% 100%)");
        assert_eq!(clip_path(33.5, true), "polygon(66.5% 0, 100% 0, 100% 100%, 66.5% 100%)");
    }

    #[test]
    fn out_of_range_and_nan_are_clamped() {
        assert_eq!(clip_path(150.0, false), clip_path(100.0, false));
        assert_eq!(clip_path(-5.0, false), clip_path(0.0, false));
        assert_eq!(clip_path(f64::NAN, false), clip_path(0.0, false));
        assert_eq!(clip_path(150.0, true), clip_path(100.0, true));
    }

    #[test]
    fn fractions_are_rounded_to_hundredths() {
        assert_eq!(clip_path(33.333_333, false), "polygon(0 0, 33.33% 0, 33.33% 100%, 0 100%)");
        assert_eq!(clip_path(49.999, false), "polygon(0 0, 50% 0, 50% 100%, 0 100%)");
    }
}
