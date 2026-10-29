//! Чистая логика Slider: проценты, шаг, нормализация, клавиши, координаты указателя, классы и `data-p`
//! (по `slider.ts` и `sliderstyle.ts` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Ориентация полосы.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SliderOrientation {
    /// Горизонтальная (по умолчанию).
    #[default]
    Horizontal,
    /// Вертикальная: значение растёт снизу вверх.
    Vertical,
}

impl SliderOrientation {
    /// Строковое значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Вертикальная ли полоса.
    pub fn is_vertical(self) -> bool {
        self == Self::Vertical
    }
}

/// Значение Slider: одиночное или диапазон (меньшее первым).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SliderValue {
    /// Одиночное значение.
    Single(f64),
    /// Диапазон `[меньшее, большее]`.
    Range([f64; 2]),
}

/// Доля значения в отрезке `min..max`, 0–100 (значения вне отрезка прижимаются к краям; при `min == max` — 0).
pub fn percent(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    if value < min {
        0.0
    } else if value > max {
        100.0
    } else {
        (value - min) * 100.0 / (max - min)
    }
}

/// Значение по доле 0–100 (`getValueFromHandle`); доля не ограничивается.
pub fn value_from_percent(percent: f64, min: f64, max: f64) -> f64 {
    (max - min) * (percent / 100.0) + min
}

/// Число знаков после запятой у `step` (`getDecimalsCount`).
pub fn decimals(step: Option<f64>) -> usize {
    match step {
        Some(s) if s.is_finite() && s.fract() != 0.0 => format!("{s}").split('.').nth(1).map_or(0, str::len),
        _ => 0,
    }
}

/// Нормализация значения (`getNormalizedValue`): округление до знаков `step`, а без дробных знаков — вниз до целого.
pub fn normalize(value: f64, step: Option<f64>) -> f64 {
    match decimals(step) {
        0 => value.floor(),
        d => {
            let k = 10f64.powi(d as i32);
            (value * k).round() / k
        }
    }
}

/// Значение с шагом (`handleStepChange`): сдвиг от прежнего значения на целое число шагов в сторону нового.
pub fn step_change(new: f64, old: f64, step: f64) -> f64 {
    let diff = new - old;
    if diff < 0.0 {
        old + (new / step - old / step).ceil() * step
    } else if diff > 0.0 {
        old + (new / step - old / step).floor() * step
    } else {
        old
    }
}

/// Ограничение значения отрезком `min..=max`.
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Действие клавиши.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyMove {
    /// Изменить значение на величину.
    By(f64),
    /// Установить `min`.
    ToMin,
    /// Установить `max`.
    ToMax,
}

/// Действие по `KeyboardEvent.code` (`onKeyDown`): стрелки — `step` или 1, страницы — `step` или (10; в диапазоне 1).
pub fn key_move(code: &str, step: Option<f64>, range: bool) -> Option<KeyMove> {
    let arrow = step.unwrap_or(1.0);
    let page = step.unwrap_or(if range { 1.0 } else { 10.0 });
    match code {
        "ArrowDown" | "ArrowLeft" => Some(KeyMove::By(-arrow)),
        "ArrowUp" | "ArrowRight" => Some(KeyMove::By(arrow)),
        "PageDown" => Some(KeyMove::By(-page)),
        "PageUp" => Some(KeyMove::By(page)),
        "Home" => Some(KeyMove::ToMin),
        "End" => Some(KeyMove::ToMax),
        _ => None,
    }
}

/// Доля положения указателя вдоль полосы (`calculateHandleValue`); `start` — левый (верхний) край, `size` — ширина (высота).
pub fn bar_percent(pointer: f64, start: f64, size: f64, vertical: bool, rtl: bool) -> f64 {
    if vertical || rtl { (start + size - pointer) * 100.0 / size } else { (pointer - start) * 100.0 / size }
}

/// Доля при касании (`onDrag`): смещение пальца от точки касания в процентах (вниз до целого) плюс стартовая доля.
pub fn touch_percent(pointer: f64, touch_start: f64, size: f64, vertical: bool, start_percent: f64) -> f64 {
    let delta = if vertical { touch_start - pointer } else { pointer - touch_start };
    (delta * 100.0 / size).floor() + start_percent
}

/// Снимок свойств, влияющих на классы и `data-p` корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SliderState {
    /// Недоступен.
    pub disabled: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Ориентация.
    pub orientation: SliderOrientation,
    /// Анимация положения (не во время перетаскивания).
    pub animate: bool,
}

/// Добавляет классы корня в порядке `sliderstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &SliderState) {
    cn.add("p-slider p-component")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.invalid, "p-invalid")
        .add(if s.orientation.is_vertical() { "p-slider-vertical" } else { "p-slider-horizontal" })
        .add_if(s.animate, "p-slider-animate");
}

/// Значение `data-p`: ориентация.
pub fn data_p(orientation: SliderOrientation) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(orientation.as_str());
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_clamps_and_guards_empty_range() {
        let cases = [
            (50.0, 0.0, 100.0, 50.0),
            (50.0, 0.0, 200.0, 25.0),
            (-5.0, 0.0, 100.0, 0.0),
            (150.0, 0.0, 100.0, 100.0),
            (5.0, 5.0, 5.0, 0.0),
            (15.0, 10.0, 20.0, 50.0),
        ];
        for (v, min, max, expected) in cases {
            assert_eq!(percent(v, min, max), expected, "{v} {min} {max}");
        }
        assert_eq!(value_from_percent(25.0, 0.0, 200.0), 50.0);
        assert_eq!(value_from_percent(50.0, 10.0, 20.0), 15.0);
    }

    #[test]
    fn decimals_and_normalize() {
        assert_eq!(decimals(None), 0);
        assert_eq!(decimals(Some(20.0)), 0);
        assert_eq!(decimals(Some(0.5)), 1);
        assert_eq!(decimals(Some(0.25)), 2);
        assert_eq!(normalize(33.9, None), 33.0);
        assert_eq!(normalize(33.9, Some(10.0)), 33.0);
        assert_eq!(normalize(0.3333, Some(0.5)), 0.3);
        assert_eq!(normalize(0.3333, Some(0.25)), 0.33);
    }

    #[test]
    fn step_change_moves_by_whole_steps_from_old_value() {
        let cases = [
            // (new, old, step, expected)
            (70.0, 20.0, 20.0, 60.0),
            (79.0, 20.0, 20.0, 60.0),
            (80.0, 20.0, 20.0, 80.0),
            (10.0, 40.0, 20.0, 20.0),
            (39.0, 40.0, 20.0, 40.0),
            (40.0, 40.0, 20.0, 40.0),
            (0.0, 40.0, 20.0, 0.0),
        ];
        for (new, old, step, expected) in cases {
            assert_eq!(step_change(new, old, step), expected, "{new} {old} {step}");
        }
    }

    #[test]
    fn clamp_and_key_moves() {
        assert_eq!((clamp(-1.0, 0.0, 100.0), clamp(101.0, 0.0, 100.0), clamp(5.0, 0.0, 100.0)), (0.0, 100.0, 5.0));
        let cases = [
            ("ArrowRight", None, false, Some(KeyMove::By(1.0))),
            ("ArrowUp", None, true, Some(KeyMove::By(1.0))),
            ("ArrowLeft", Some(20.0), false, Some(KeyMove::By(-20.0))),
            ("ArrowDown", None, false, Some(KeyMove::By(-1.0))),
            ("PageUp", None, false, Some(KeyMove::By(10.0))),
            ("PageDown", None, false, Some(KeyMove::By(-10.0))),
            ("PageUp", None, true, Some(KeyMove::By(1.0))),
            ("PageUp", Some(5.0), false, Some(KeyMove::By(5.0))),
            ("Home", None, false, Some(KeyMove::ToMin)),
            ("End", Some(5.0), true, Some(KeyMove::ToMax)),
            ("KeyA", None, false, None),
        ];
        for (code, step, range, expected) in cases {
            assert_eq!(key_move(code, step, range), expected, "{code} {step:?} {range}");
        }
    }

    #[test]
    fn pointer_and_touch_percent() {
        // ширина 200 px, левый край 100
        assert_eq!(bar_percent(200.0, 100.0, 200.0, false, false), 50.0);
        assert_eq!(bar_percent(200.0, 100.0, 200.0, false, true), 50.0);
        assert_eq!(bar_percent(120.0, 100.0, 200.0, false, true), 90.0);
        // высота 200 px, верхний край 100: ниже — меньше
        assert_eq!(bar_percent(250.0, 100.0, 200.0, true, false), 25.0);
        assert_eq!(touch_percent(150.0, 100.0, 200.0, false, 10.0), 35.0);
        assert_eq!(touch_percent(151.0, 100.0, 200.0, false, 10.0), 35.0);
        assert_eq!(touch_percent(100.0, 150.0, 200.0, true, 10.0), 35.0);
        assert_eq!(touch_percent(90.0, 100.0, 200.0, false, 50.0), 45.0);
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |disabled, invalid, orientation, animate| SliderState { disabled, invalid, orientation, animate };
        use SliderOrientation::*;
        let cases = [
            (st(false, false, Horizontal, false), "p-slider p-component p-slider-horizontal"),
            (st(true, false, Horizontal, false), "p-slider p-component p-disabled p-slider-horizontal"),
            (st(false, true, Vertical, false), "p-slider p-component p-invalid p-slider-vertical"),
            (st(true, true, Vertical, true), "p-slider p-component p-disabled p-invalid p-slider-vertical p-slider-animate"),
        ];
        for (s, class) in cases {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &s);
            assert_eq!(cn.finish(), class, "{s:?}");
        }
        assert_eq!(data_p(Horizontal), "horizontal");
        assert_eq!(data_p(Vertical), "vertical");
    }
}
