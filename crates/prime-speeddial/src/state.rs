//! Чистая логика SpeedDial: классы, раскладка, задержки, клавиши и шаги фокуса (по `speeddial.ts` и `speeddialstyle.ts` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;
use std::f64::consts::PI;

/// Тип раскладки пунктов.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpeedDialType {
    /// По прямой (по умолчанию).
    #[default]
    Linear,
    /// По кругу вокруг кнопки.
    Circle,
    /// По полукругу.
    SemiCircle,
    /// По четверти круга.
    QuarterCircle,
}

impl SpeedDialType {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Circle => "circle",
            Self::SemiCircle => "semi-circle",
            Self::QuarterCircle => "quarter-circle",
        }
    }
}

/// Направление раскрытия.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpeedDialDirection {
    /// Вверх (по умолчанию).
    #[default]
    Up,
    /// Вниз.
    Down,
    /// Влево.
    Left,
    /// Вправо.
    Right,
    /// Вверх и влево.
    UpLeft,
    /// Вверх и вправо.
    UpRight,
    /// Вниз и влево.
    DownLeft,
    /// Вниз и вправо.
    DownRight,
}

impl SpeedDialDirection {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
            Self::UpLeft => "up-left",
            Self::UpRight => "up-right",
            Self::DownLeft => "down-left",
            Self::DownRight => "down-right",
        }
    }
}

/// Классы корня: `p-speeddial p-component p-speeddial-<тип>`, `p-speeddial-direction-<направление>` (кроме круга),
/// `p-speeddial-open`, `p-disabled`.
pub fn root_classes(
    kind: SpeedDialType,
    direction: SpeedDialDirection,
    open: bool,
    disabled: bool,
    extra: Option<&str>,
) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-speeddial p-component").add(&format!("p-speeddial-{}", kind.as_str()));
    if kind != SpeedDialType::Circle {
        cn.add(&format!("p-speeddial-direction-{}", direction.as_str()));
    }
    cn.add_if(open, "p-speeddial-open").add_if(disabled, "p-disabled").add_user(extra);
    cn.finish_cow()
}

fn flex_direction(direction: SpeedDialDirection) -> Option<&'static str> {
    match direction {
        SpeedDialDirection::Up => Some("column-reverse"),
        SpeedDialDirection::Down => Some("column"),
        SpeedDialDirection::Left => Some("row-reverse"),
        SpeedDialDirection::Right => Some("row"),
        _ => None,
    }
}

/// Инлайн-стиль корня: выравнивание и `flex-direction` по направлению.
pub fn root_inline_style(direction: SpeedDialDirection) -> String {
    let mut parts = Vec::new();
    if matches!(direction, SpeedDialDirection::Up | SpeedDialDirection::Down) {
        parts.push("align-items: center".to_owned());
    }
    if matches!(direction, SpeedDialDirection::Left | SpeedDialDirection::Right) {
        parts.push("justify-content: center".to_owned());
    }
    if let Some(d) = flex_direction(direction) {
        parts.push(format!("flex-direction: {d}"));
    }
    parts.join("; ")
}

/// Инлайн-стиль списка: `flex-direction` по направлению.
pub fn list_inline_style(direction: SpeedDialDirection) -> String {
    flex_direction(direction).map(|d| format!("flex-direction: {d}")).unwrap_or_default()
}

/// Классы пункта: `p-speeddial-item`, `p-hidden` (`visible = false`), `p-focus`, пользовательский.
pub fn item_classes(hidden: bool, focused: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-speeddial-item").add_if(hidden, "p-hidden").add_if(focused, "p-focus").add_user(extra);
    cn.finish_cow()
}

/// Радиус раскладки: `radius`, а при нуле — `20 × число пунктов`.
pub fn effective_radius(radius: f64, length: usize) -> f64 {
    if radius == 0.0 { length as f64 * 20.0 } else { radius }
}

fn calc(value: f64, var: &str) -> String {
    format!("calc({value}px + var({var}, 0px))")
}

/// Положение пункта `index` из `length` по раскладке (`calculatePointStyle`); у `linear` стиля нет.
pub fn point_style(kind: SpeedDialType, direction: SpeedDialDirection, index: usize, length: usize, radius: f64) -> String {
    use SpeedDialDirection as D;
    if kind == SpeedDialType::Linear || length == 0 {
        return String::new();
    }
    let radius = effective_radius(radius, length);
    let i = index as f64;
    let step_for = |full: f64| if length > 1 { full / (length - 1) as f64 } else { 0.0 };
    let xy = |step: f64| (calc(radius * (step * i).cos(), "--item-diff-x"), calc(radius * (step * i).sin(), "--item-diff-y"));
    match kind {
        SpeedDialType::Circle => {
            let (x, y) = xy(2.0 * PI / length as f64);
            format!("left: {x}; top: {y}")
        }
        SpeedDialType::SemiCircle => {
            let (x, y) = xy(step_for(PI));
            match direction {
                D::Up => format!("left: {x}; bottom: {y}"),
                D::Down => format!("left: {x}; top: {y}"),
                D::Left => format!("right: {y}; top: {x}"),
                D::Right => format!("left: {y}; top: {x}"),
                _ => String::new(),
            }
        }
        SpeedDialType::QuarterCircle => {
            let (x, y) = xy(step_for(PI / 2.0));
            match direction {
                D::UpLeft => format!("right: {x}; bottom: {y}"),
                D::UpRight => format!("left: {x}; bottom: {y}"),
                D::DownLeft => format!("right: {y}; top: {x}"),
                D::DownRight => format!("left: {y}; top: {x}"),
                _ => String::new(),
            }
        }
        SpeedDialType::Linear => String::new(),
    }
}

/// Задержка перехода пункта, мс: при показе растёт к концу списка, при скрытии — к началу.
pub fn transition_delay(visible: bool, index: usize, length: usize, step: u32) -> u64 {
    let order = if visible { index } else { length.saturating_sub(index + 1) };
    order as u64 * u64::from(step)
}

/// Инлайн-стиль пункта: задержка перехода и положение.
pub fn item_style(
    visible: bool,
    kind: SpeedDialType,
    direction: SpeedDialDirection,
    index: usize,
    length: usize,
    radius: f64,
    step: u32,
) -> String {
    let delay = transition_delay(visible, index, length, step);
    let point = point_style(kind, direction, index, length, radius);
    if point.is_empty() { format!("transition-delay: {delay}ms") } else { format!("transition-delay: {delay}ms; {point}") }
}

/// Шаг фокуса по списку.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Следующий доступный пункт.
    Next,
    /// Предыдущий доступный пункт.
    Prev,
    /// Первый пункт.
    First,
    /// Последний пункт.
    Last,
    /// Активировать пункт под фокусом.
    Activate,
    /// Скрыть список.
    Hide,
}

fn left_valid(d: SpeedDialDirection) -> bool {
    matches!(d, SpeedDialDirection::Left | SpeedDialDirection::UpRight | SpeedDialDirection::DownLeft)
}

/// Действие клавиши в списке; `None` — клавиша не обрабатывается.
pub fn list_action(direction: SpeedDialDirection, code: &str) -> Option<Action> {
    use SpeedDialDirection as D;
    Some(match code {
        "ArrowUp" => match direction {
            D::Down => Action::Prev,
            _ => Action::Next,
        },
        "ArrowDown" => match direction {
            D::Down => Action::Next,
            _ => Action::Prev,
        },
        "ArrowLeft" => {
            if left_valid(direction) {
                Action::Next
            } else {
                Action::Prev
            }
        }
        "ArrowRight" => {
            if left_valid(direction) {
                Action::Prev
            } else {
                Action::Next
            }
        }
        "Home" => Action::First,
        "End" => Action::Last,
        "Enter" | "Space" => Action::Activate,
        "Escape" => Action::Hide,
        _ => return None,
    })
}

/// Что делает клавиша на кнопке переключателя.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TogglerAction {
    /// Открыть список и перейти на следующий пункт.
    OpenNext,
    /// Открыть список и перейти на предыдущий пункт (на последний при пустом фокусе).
    OpenPrev,
    /// Скрыть список.
    Hide,
}

/// Действие клавиши на кнопке; `None` — не обрабатывается.
pub fn toggler_action(code: &str) -> Option<TogglerAction> {
    match code {
        "ArrowDown" | "ArrowLeft" => Some(TogglerAction::OpenNext),
        "ArrowUp" | "ArrowRight" => Some(TogglerAction::OpenPrev),
        "Escape" => Some(TogglerAction::Hide),
        _ => None,
    }
}

/// Новая позиция фокуса среди `count` доступных пунктов; на границах остаётся. Без пунктов — `None`.
pub fn step(count: usize, current: Option<usize>, action: Action) -> Option<usize> {
    let last = count.checked_sub(1)?;
    match action {
        Action::Next => Some(current.map_or(0, |c| c + 1).min(last)),
        Action::Prev => Some(current.map_or(last, |c| c.saturating_sub(1)).min(last)),
        Action::First => Some(0),
        Action::Last => Some(last),
        Action::Activate | Action::Hide => current,
    }
}

/// Какой значок кнопки показать.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconChoice {
    /// Значок по умолчанию («плюс»).
    Default,
    /// `show_icon`.
    Show,
    /// `hide_icon`.
    Hide,
}

/// Выбор значка кнопки (`buttonIconClass`).
pub fn icon_choice(visible: bool, has_show: bool, has_hide: bool) -> IconChoice {
    if visible && has_hide {
        IconChoice::Hide
    } else if has_show {
        IconChoice::Show
    } else {
        IconChoice::Default
    }
}

/// Дополнительные классы кнопки переключателя.
pub fn button_extra_classes(rotate: bool, has_hide_icon: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-speeddial-button").add_if(rotate && !has_hide_icon, "p-speeddial-rotate").add_user(extra);
    cn.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::SpeedDialDirection as D;
    use super::SpeedDialType as T;
    use super::*;

    #[test]
    fn root_classes_table() {
        assert_eq!(
            root_classes(T::Linear, D::Up, false, false, None),
            "p-speeddial p-component p-speeddial-linear p-speeddial-direction-up"
        );
        assert_eq!(root_classes(T::Circle, D::Up, true, false, None), "p-speeddial p-component p-speeddial-circle p-speeddial-open");
        assert_eq!(
            root_classes(T::QuarterCircle, D::DownLeft, true, true, Some("mine")),
            "p-speeddial p-component p-speeddial-quarter-circle p-speeddial-direction-down-left p-speeddial-open p-disabled mine"
        );
        assert_eq!(T::SemiCircle.as_str(), "semi-circle");
    }

    #[test]
    fn flex_styles() {
        assert_eq!(root_inline_style(D::Up), "align-items: center; flex-direction: column-reverse");
        assert_eq!(root_inline_style(D::Down), "align-items: center; flex-direction: column");
        assert_eq!(root_inline_style(D::Left), "justify-content: center; flex-direction: row-reverse");
        assert_eq!(root_inline_style(D::Right), "justify-content: center; flex-direction: row");
        assert_eq!(root_inline_style(D::UpLeft), "");
        assert_eq!(list_inline_style(D::Up), "flex-direction: column-reverse");
        assert_eq!(list_inline_style(D::DownRight), "");
    }

    #[test]
    fn item_classes_table() {
        assert_eq!(item_classes(false, false, None), "p-speeddial-item");
        assert_eq!(item_classes(true, true, Some("x")), "p-speeddial-item p-hidden p-focus x");
    }

    #[test]
    fn radius_and_delay() {
        assert_eq!(effective_radius(0.0, 4), 80.0);
        assert_eq!(effective_radius(50.0, 4), 50.0);
        assert_eq!((0..3).map(|i| transition_delay(true, i, 3, 30)).collect::<Vec<_>>(), [0, 30, 60]);
        assert_eq!((0..3).map(|i| transition_delay(false, i, 3, 30)).collect::<Vec<_>>(), [60, 30, 0]);
        assert_eq!(transition_delay(false, 0, 0, 30), 0);
    }

    #[test]
    fn point_styles() {
        assert_eq!(point_style(T::Linear, D::Up, 0, 3, 0.0), "");
        assert_eq!(
            point_style(T::Circle, D::Up, 0, 4, 80.0),
            "left: calc(80px + var(--item-diff-x, 0px)); top: calc(0px + var(--item-diff-y, 0px))"
        );
        // полукруг вверх, 3 пункта: углы 0, π/2, π
        let s = point_style(T::SemiCircle, D::Up, 0, 3, 80.0);
        assert_eq!(s, "left: calc(80px + var(--item-diff-x, 0px)); bottom: calc(0px + var(--item-diff-y, 0px))");
        assert!(point_style(T::SemiCircle, D::Up, 2, 3, 80.0).starts_with("left: calc(-80px"));
        assert!(point_style(T::SemiCircle, D::Down, 0, 3, 80.0).contains("top:"));
        assert!(point_style(T::SemiCircle, D::Left, 0, 3, 80.0).starts_with("right: calc(0px"));
        assert!(point_style(T::SemiCircle, D::Right, 0, 3, 80.0).starts_with("left: calc(0px"));
        assert_eq!(point_style(T::SemiCircle, D::UpLeft, 0, 3, 80.0), "", "диагональ для полукруга не задана");
        for (d, first, second) in [
            (D::UpLeft, "right: calc(80px", "bottom: calc(0px"),
            (D::UpRight, "left: calc(80px", "bottom: calc(0px"),
            (D::DownLeft, "right: calc(0px", "top: calc(80px"),
            (D::DownRight, "left: calc(0px", "top: calc(80px"),
        ] {
            let s = point_style(T::QuarterCircle, d, 0, 3, 80.0);
            assert!(s.starts_with(first) && s.contains(second), "{d:?}: {s}");
        }
        // конечный пункт четверти круга лежит на другой оси
        let last = point_style(T::QuarterCircle, D::UpRight, 2, 3, 80.0);
        assert!(last.contains("left: calc(0") || last.contains("left: calc(0.0") || last.contains("e-"), "{last}");
        assert!(!point_style(T::SemiCircle, D::Up, 0, 1, 80.0).contains("NaN"), "один пункт не даёт NaN");
        assert_eq!(point_style(T::Circle, D::Up, 0, 0, 80.0), "");
    }

    #[test]
    fn item_style_joins_delay_and_point() {
        assert_eq!(item_style(true, T::Linear, D::Up, 2, 3, 0.0, 30), "transition-delay: 60ms");
        assert_eq!(
            item_style(false, T::Circle, D::Up, 0, 4, 80.0, 30),
            "transition-delay: 90ms; left: calc(80px + var(--item-diff-x, 0px)); top: calc(0px + var(--item-diff-y, 0px))"
        );
    }

    #[test]
    fn list_keys_follow_the_direction() {
        let table = [
            (D::Up, "ArrowUp", Action::Next),
            (D::Up, "ArrowDown", Action::Prev),
            (D::Down, "ArrowUp", Action::Prev),
            (D::Down, "ArrowDown", Action::Next),
            (D::Left, "ArrowLeft", Action::Next),
            (D::Left, "ArrowRight", Action::Prev),
            (D::UpRight, "ArrowLeft", Action::Next),
            (D::DownLeft, "ArrowRight", Action::Prev),
            (D::Right, "ArrowLeft", Action::Prev),
            (D::Right, "ArrowRight", Action::Next),
            (D::UpLeft, "ArrowRight", Action::Next),
            (D::DownRight, "ArrowLeft", Action::Prev),
            (D::Left, "ArrowUp", Action::Next),
            (D::Right, "ArrowDown", Action::Prev),
            (D::Up, "Home", Action::First),
            (D::Up, "End", Action::Last),
            (D::Up, "Enter", Action::Activate),
            (D::Up, "Space", Action::Activate),
            (D::Up, "Escape", Action::Hide),
        ];
        for (d, code, want) in table {
            assert_eq!(list_action(d, code), Some(want), "{d:?} {code}");
        }
        assert_eq!(list_action(D::Up, "KeyA"), None);
        assert_eq!(list_action(D::Up, "Tab"), None);
    }

    #[test]
    fn toggler_keys() {
        assert_eq!(toggler_action("ArrowDown"), Some(TogglerAction::OpenNext));
        assert_eq!(toggler_action("ArrowLeft"), Some(TogglerAction::OpenNext));
        assert_eq!(toggler_action("ArrowUp"), Some(TogglerAction::OpenPrev));
        assert_eq!(toggler_action("ArrowRight"), Some(TogglerAction::OpenPrev));
        assert_eq!(toggler_action("Escape"), Some(TogglerAction::Hide));
        assert_eq!(toggler_action("Enter"), None);
    }

    #[test]
    fn step_table() {
        assert_eq!(step(0, None, Action::Next), None);
        assert_eq!(step(3, None, Action::Next), Some(0));
        assert_eq!(step(3, None, Action::Prev), Some(2), "без фокуса назад — последний");
        assert_eq!(step(3, Some(0), Action::Next), Some(1));
        assert_eq!(step(3, Some(2), Action::Next), Some(2), "на границе остаётся");
        assert_eq!(step(3, Some(0), Action::Prev), Some(0));
        assert_eq!(step(3, Some(2), Action::Prev), Some(1));
        assert_eq!(step(3, Some(1), Action::First), Some(0));
        assert_eq!(step(3, Some(1), Action::Last), Some(2));
        assert_eq!(step(3, Some(1), Action::Activate), Some(1));
        assert_eq!(step(3, Some(1), Action::Hide), Some(1));
    }

    #[test]
    fn icon_choice_and_button_classes() {
        assert_eq!(icon_choice(false, false, false), IconChoice::Default);
        assert_eq!(icon_choice(true, false, false), IconChoice::Default);
        assert_eq!(icon_choice(false, true, true), IconChoice::Show);
        assert_eq!(icon_choice(true, true, true), IconChoice::Hide);
        assert_eq!(icon_choice(true, true, false), IconChoice::Show);
        assert_eq!(icon_choice(true, false, true), IconChoice::Hide);
        assert_eq!(button_extra_classes(true, false, None), "p-speeddial-button p-speeddial-rotate");
        assert_eq!(button_extra_classes(true, true, None), "p-speeddial-button");
        assert_eq!(button_extra_classes(false, false, Some("mine")), "p-speeddial-button mine");
    }
}
