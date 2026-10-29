//! Чистая логика Knob: углы, значения, пути дуг, клавиши и классы (по `knob.ts` и `knobstyle.ts` PrimeNG).

use prime_core::ClassNames;
use std::f64::consts::PI;

/// Радиус дуги в единицах `viewBox` (100 × 100).
pub const RADIUS: f64 = 40.0;
/// Центр дуги по обеим осям.
pub const MID: f64 = 50.0;
/// Угол начала шкалы (`min`): нижний левый.
pub const MIN_RADIANS: f64 = 4.0 * PI / 3.0;
/// Угол конца шкалы (`max`): нижний правый.
pub const MAX_RADIANS: f64 = -PI / 3.0;

/// Линейное отображение `x` из `in_min..in_max` в `out_min..out_max` (`mapRange`); при пустом входном отрезке — `out_min`.
pub fn map_range(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    if in_max == in_min {
        return out_min;
    }
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Округление как `Math.round` в JavaScript (половины — к большему).
fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Значение по углу точки (`updateModel`): точки в разрыве внизу шкалы дают `None`, остальные округляются до шага от `min`
/// и ограничиваются отрезком `min..=max`.
pub fn value_from_angle(angle: f64, min: f64, max: f64, step: f64) -> Option<f64> {
    let start = -PI / 2.0 - PI / 6.0;
    let mapped = if angle > MAX_RADIANS {
        map_range(angle, MIN_RADIANS, MAX_RADIANS, min, max)
    } else if angle < start {
        map_range(angle + 2.0 * PI, MIN_RADIANS, MAX_RADIANS, min, max)
    } else {
        return None;
    };
    let stepped = if step > 0.0 { js_round((mapped - min) / step) * step + min } else { mapped };
    Some(stepped.clamp(min.min(max), max.max(min)))
}

/// Угол нуля шкалы (`zeroRadians`): при `min > 0 && max > 0` — положение `min`.
pub fn zero_radians(min: f64, max: f64) -> f64 {
    let zero = if min > 0.0 && max > 0.0 { min } else { 0.0 };
    map_range(zero, min, max, MIN_RADIANS, MAX_RADIANS)
}

/// Угол значения (`valueRadians`); значение вне отрезка прижимается к ближайшему краю.
pub fn value_radians(value: f64, min: f64, max: f64) -> f64 {
    map_range(value.clamp(min.min(max), max.max(min)), min, max, MIN_RADIANS, MAX_RADIANS)
}

/// Точка дуги под углом.
pub fn arc_point(radians: f64) -> (f64, f64) {
    (MID + radians.cos() * RADIUS, MID - radians.sin() * RADIUS)
}

/// Число с тремя знаками после запятой без хвостовых нулей (и без «-0»).
pub fn num(v: f64) -> String {
    let r = (v * 1000.0).round() / 1000.0;
    format!("{}", if r == 0.0 { 0.0 } else { r })
}

/// Контур фоновой дуги (`rangePath`).
pub fn range_path() -> String {
    let ((x0, y0), (x1, y1)) = (arc_point(MIN_RADIANS), arc_point(MAX_RADIANS));
    format!("M {} {} A {RADIUS} {RADIUS} 0 1 1 {} {}", num(x0), num(y0), num(x1), num(y1))
}

/// Контур дуги значения от нуля шкалы (`valuePath`).
pub fn value_path(value: f64, min: f64, max: f64) -> String {
    let (zero, current) = (zero_radians(min, max), value_radians(value, min, max));
    let (large, sweep) = (u8::from((zero - current).abs() >= PI), u8::from(current <= zero));
    let ((zx, zy), (vx, vy)) = (arc_point(zero), arc_point(current));
    format!("M {} {} A {RADIUS} {RADIUS} 0 {large} {sweep} {} {}", num(zx), num(zy), num(vx), num(vy))
}

/// Новое значение по клавише (`onKeyDown`): стрелки ±1, страницы ±10, `Home`/`End` — границы; результат ограничен отрезком.
pub fn key_target(code: &str, current: f64, min: f64, max: f64) -> Option<f64> {
    let target = match code {
        "ArrowRight" | "ArrowUp" => current + 1.0,
        "ArrowLeft" | "ArrowDown" => current - 1.0,
        "PageUp" => current + 10.0,
        "PageDown" => current - 10.0,
        "Home" => min,
        "End" => max,
        _ => return None,
    };
    Some(target.clamp(min.min(max), max.max(min)))
}

/// Текст значения: первое `{value}` в шаблоне заменяется значением (как `String.replace` в JavaScript).
pub fn display_text(template: &str, value: f64) -> String {
    template.replacen("{value}", &value.to_string(), 1)
}

/// Добавляет классы корня в порядке `knobstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, disabled: bool) {
    cn.add("p-knob p-component").add_if(disabled, "p-disabled");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn map_range_and_guard() {
        assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range(5.0, 5.0, 5.0, 7.0, 9.0), 7.0);
    }

    #[test]
    fn value_from_angle_table() {
        let up = PI / 2.0;
        let cases = [
            // (угол, min, max, step, ожидание)
            (up, 0.0, 100.0, 1.0, Some(50.0)),
            (PI, 0.0, 100.0, 1.0, Some(20.0)),
            (0.0, 0.0, 100.0, 1.0, Some(80.0)),
            (-2.0 * PI / 3.0 - 0.001, 0.0, 100.0, 1.0, Some(0.0)),
            (-2.0 * PI / 3.0 + 0.001, 0.0, 100.0, 1.0, None),
            (-PI / 2.0, 0.0, 100.0, 1.0, None),
            (-PI / 2.0 - 0.3, 0.0, 100.0, 1.0, None),
            (up, -50.0, 50.0, 1.0, Some(0.0)),
            (up, 0.0, 100.0, 40.0, Some(40.0)),
            (PI, 0.0, 100.0, 10.0, Some(20.0)),
        ];
        for (angle, min, max, step, expected) in cases {
            match (value_from_angle(angle, min, max, step), expected) {
                (Some(v), Some(e)) => assert!(close(v, e), "{angle} {min} {max} {step}: {v} != {e}"),
                (None, None) => {}
                (got, want) => panic!("{angle} {min} {max} {step}: {got:?} != {want:?}"),
            }
        }
    }

    #[test]
    fn value_from_angle_never_leaves_the_range() {
        // в углах шкалы округление по шагу не должно выводить за границы
        for step in [1.0, 7.0, 30.0] {
            for i in 0..720 {
                let angle = -PI + i as f64 * PI / 360.0;
                if let Some(v) = value_from_angle(angle, 0.0, 100.0, step) {
                    assert!((0.0..=100.0).contains(&v), "{angle} {step}: {v}");
                }
            }
        }
    }

    #[test]
    fn radians_zero_value_and_points() {
        assert!(close(value_radians(0.0, 0.0, 100.0), MIN_RADIANS));
        assert!(close(value_radians(100.0, 0.0, 100.0), MAX_RADIANS));
        assert!(close(value_radians(50.0, 0.0, 100.0), PI / 2.0));
        assert!(close(value_radians(500.0, 0.0, 100.0), MAX_RADIANS), "прижимается к max");
        assert!(close(value_radians(-5.0, 0.0, 100.0), MIN_RADIANS), "прижимается к min");
        assert!(close(zero_radians(0.0, 100.0), MIN_RADIANS));
        assert!(close(zero_radians(-50.0, 50.0), PI / 2.0));
        assert!(close(zero_radians(20.0, 100.0), MIN_RADIANS), "при min > 0 ноль — это min");
        let (x, y) = arc_point(PI / 2.0);
        assert!(close(x, 50.0) && close(y, 10.0));
    }

    #[test]
    fn paths_have_expected_flags() {
        assert_eq!(range_path(), "M 30 84.641 A 40 40 0 1 1 70 84.641");
        // верх шкалы при диапазоне -50..50: дуга нулевой длины
        assert_eq!(value_path(0.0, -50.0, 50.0), "M 50 10 A 40 40 0 0 1 50 10");
        // по умолчанию ноль — слева внизу: значение 50 лежит на пол-оборота по часовой стрелке (меньшая дуга, sweep = 1)
        assert_eq!(value_path(50.0, 0.0, 100.0), "M 30 84.641 A 40 40 0 0 1 50 10");
        // от нуля до max — большая дуга
        assert_eq!(value_path(100.0, 0.0, 100.0), "M 30 84.641 A 40 40 0 1 1 70 84.641");
        // правее нуля при диапазоне -50..50: угол значения меньше угла нуля, sweep = 1
        let (x, y) = arc_point(value_radians(25.0, -50.0, 50.0));
        assert_eq!(value_path(25.0, -50.0, 50.0), format!("M 50 10 A 40 40 0 0 1 {} {}", num(x), num(y)));
        assert_eq!(num(-0.0001), "0");
        assert_eq!(num(1.23456), "1.235");
    }

    #[test]
    fn key_targets_table() {
        let cases = [
            ("ArrowRight", 50.0, Some(51.0)),
            ("ArrowUp", 50.0, Some(51.0)),
            ("ArrowLeft", 50.0, Some(49.0)),
            ("ArrowDown", 0.0, Some(0.0)),
            ("PageUp", 95.0, Some(100.0)),
            ("PageDown", 5.0, Some(0.0)),
            ("PageUp", 50.0, Some(60.0)),
            ("Home", 50.0, Some(0.0)),
            ("End", 50.0, Some(100.0)),
            ("KeyA", 50.0, None),
        ];
        for (code, current, expected) in cases {
            assert_eq!(key_target(code, current, 0.0, 100.0), expected, "{code} {current}");
        }
    }

    #[test]
    fn display_text_replaces_first_placeholder_only() {
        assert_eq!(display_text("{value}", 60.0), "60");
        assert_eq!(display_text("{value}%", 60.0), "60%");
        assert_eq!(display_text("{value}/{value}", 5.5), "5.5/{value}");
        assert_eq!(display_text("n/a", 5.0), "n/a");
    }

    #[test]
    fn classes_table() {
        let built = |disabled| {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, disabled);
            cn.finish()
        };
        assert_eq!(built(false), "p-knob p-component");
        assert_eq!(built(true), "p-knob p-component p-disabled");
    }
}
