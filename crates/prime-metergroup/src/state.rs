//! Pure MeterGroup logic: percentages, classes and `data-p` (per `metergroup.ts` and `metergroupstyle.ts` of PrimeNG).

use prime_rs_core::ClassNames;

/// Meter orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MeterOrientation {
    /// `horizontal` (default).
    #[default]
    Horizontal,
    /// `vertical`.
    Vertical,
}

impl MeterOrientation {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Label position relative to the meter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LabelPosition {
    /// Before the meter.
    Start,
    /// After the meter (default).
    #[default]
    End,
}

/// Percentage of a value in the `min`–`max` range: rounded, clamped to 0–100; when `min == max` it is 100.
pub fn percent(value: f64, min: f64, max: f64) -> u32 {
    if max == min {
        return 100;
    }
    let p = (value - min) / (max - min) * 100.0;
    p.clamp(0.0, 100.0).round() as u32
}

/// Percentage of the sum of all values.
pub fn total_percent(values: impl IntoIterator<Item = f64>, min: f64, max: f64) -> u32 {
    percent(values.into_iter().sum(), min, max)
}

/// Cumulative sums of the values (like `percentages()` in PrimeNG).
pub fn cumulative(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut sum = 0.0;
    values
        .into_iter()
        .map(|v| {
            sum += v;
            sum
        })
        .collect()
}

/// Appends the root classes.
pub fn write_classes(cn: &mut ClassNames, orientation: MeterOrientation) {
    cn.add("p-metergroup p-component");
    cn.add(match orientation {
        MeterOrientation::Horizontal => "p-metergroup-horizontal",
        MeterOrientation::Vertical => "p-metergroup-vertical",
    });
}

/// Label list classes.
pub fn label_list_classes(cn: &mut ClassNames, orientation: MeterOrientation) {
    cn.add("p-metergroup-label-list");
    cn.add(match orientation {
        MeterOrientation::Horizontal => "p-metergroup-label-list-horizontal",
        MeterOrientation::Vertical => "p-metergroup-label-list-vertical",
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_table() {
        let table = [
            (50.0, 0.0, 200.0, 25),
            (150.0, 0.0, 100.0, 100),
            (-10.0, 0.0, 100.0, 0),
            (0.0, 0.0, 100.0, 0),
            (16.0, 0.0, 100.0, 16),
            (1.0, 0.0, 3.0, 33),
            (2.0, 0.0, 3.0, 67),
            (5.0, 5.0, 5.0, 100),
            (15.0, 10.0, 20.0, 50),
            (0.5, 0.0, 100.0, 1),
        ];
        for (v, min, max, want) in table {
            assert_eq!(percent(v, min, max), want, "{v} in {min}..{max}");
        }
    }

    #[test]
    fn total_and_cumulative() {
        assert_eq!(total_percent([20.0, 30.0], 0.0, 100.0), 50);
        assert_eq!(total_percent([], 0.0, 100.0), 0);
        assert_eq!(total_percent([80.0, 80.0], 0.0, 100.0), 100);
        assert_eq!(cumulative([20.0, 30.0, 5.0]), vec![20.0, 50.0, 55.0]);
        assert!(cumulative([]).is_empty());
    }

    #[test]
    fn classes() {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, MeterOrientation::Vertical);
        assert_eq!(cn.finish(), "p-metergroup p-component p-metergroup-vertical");
        let mut cn = ClassNames::new();
        label_list_classes(&mut cn, MeterOrientation::Horizontal);
        assert_eq!(cn.finish(), "p-metergroup-label-list p-metergroup-label-list-horizontal");
    }
}
