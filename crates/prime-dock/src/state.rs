//! Чистая логика Dock: положение, классы, идентификаторы, клавиши и шаги фокуса (по `dock.ts` и `dockstyle.ts` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Положение дока.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DockPosition {
    /// У нижнего края (по умолчанию).
    #[default]
    Bottom,
    /// У верхнего края.
    Top,
    /// У левого края.
    Left,
    /// У правого края.
    Right,
}

impl DockPosition {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Горизонтальный ли док.
    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Bottom | Self::Top)
    }
}

/// Значение `aria-orientation` списка.
pub fn orientation(position: DockPosition) -> &'static str {
    if position.is_horizontal() { "horizontal" } else { "vertical" }
}

/// Классы корня: `p-dock p-component p-dock-<положение>` и `p-dock-mobile`.
pub fn root_classes(position: DockPosition, mobile: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-dock p-component").add(match position {
        DockPosition::Bottom => "p-dock-bottom",
        DockPosition::Top => "p-dock-top",
        DockPosition::Left => "p-dock-left",
        DockPosition::Right => "p-dock-right",
    });
    cn.add_if(mobile, "p-dock-mobile").add_user(extra);
    cn.finish_cow()
}

/// Классы пункта: `p-dock-item`, `p-focus`, `p-disabled` и пользовательский класс.
pub fn item_classes(focused: bool, disabled: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-dock-item").add_if(focused, "p-focus").add_if(disabled, "p-disabled").add_user(extra);
    cn.finish_cow()
}

/// Идентификатор пункта: собственный `id` или `<id списка>_<индекс>`.
pub fn item_id(list_id: &str, own: Option<&str>, index: usize) -> String {
    match own {
        Some(id) if !id.is_empty() => id.to_owned(),
        _ => format!("{list_id}_{index}"),
    }
}

/// Действие клавиши.
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
    /// Клик по ссылке пункта.
    Activate,
    /// Клавиша перехватывается без действия (стрелка поперёк дока).
    Noop,
}

/// Действие клавиши `code` в доке с положением `position`; `None` — клавиша не обрабатывается.
pub fn key_action(position: DockPosition, code: &str) -> Option<Action> {
    let horizontal = position.is_horizontal();
    Some(match code {
        "ArrowRight" => {
            if horizontal {
                Action::Next
            } else {
                Action::Noop
            }
        }
        "ArrowLeft" => {
            if horizontal {
                Action::Prev
            } else {
                Action::Noop
            }
        }
        "ArrowDown" => {
            if horizontal {
                Action::Noop
            } else {
                Action::Next
            }
        }
        "ArrowUp" => {
            if horizontal {
                Action::Noop
            } else {
                Action::Prev
            }
        }
        "Home" => Action::First,
        "End" => Action::Last,
        "Enter" | "Space" => Action::Activate,
        _ => return None,
    })
}

/// Новая позиция фокуса среди `count` доступных пунктов (`changeFocusedOptionIndex`): `Next` и `Prev` исходят из текущей
/// позиции (без неё — `0`) и зажимаются в границах; `First`/`Last` — крайние. Без пунктов — `None`.
pub fn step(count: usize, current: Option<usize>, action: Action) -> Option<usize> {
    let last = count.checked_sub(1)?;
    match action {
        Action::Next => Some(current.map_or(0, |c| c + 1).min(last)),
        Action::Prev => Some(current.map_or(0, |c| c.saturating_sub(1)).min(last)),
        Action::First => Some(0),
        Action::Last => Some(last),
        Action::Activate | Action::Noop => current,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DockPosition::*;

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(Bottom, false, None), "p-dock p-component p-dock-bottom");
        assert_eq!(root_classes(Top, true, None), "p-dock p-component p-dock-top p-dock-mobile");
        assert_eq!(root_classes(Left, false, Some("mine")), "p-dock p-component p-dock-left mine");
        assert_eq!(root_classes(Right, true, Some("mine")), "p-dock p-component p-dock-right p-dock-mobile mine");
    }

    #[test]
    fn orientation_and_defaults() {
        assert_eq!((orientation(Bottom), orientation(Top)), ("horizontal", "horizontal"));
        assert_eq!((orientation(Left), orientation(Right)), ("vertical", "vertical"));
        assert_eq!(DockPosition::default(), Bottom);
        assert_eq!(Left.as_str(), "left");
    }

    #[test]
    fn item_classes_and_ids() {
        assert_eq!(item_classes(false, false, None), "p-dock-item");
        assert_eq!(item_classes(true, true, Some("x")), "p-dock-item p-focus p-disabled x");
        assert_eq!(item_id("d", None, 2), "d_2");
        assert_eq!(item_id("d", Some("own"), 2), "own");
        assert_eq!(item_id("d", Some(""), 2), "d_2");
    }

    #[test]
    fn keys_follow_the_orientation() {
        assert_eq!(key_action(Bottom, "ArrowRight"), Some(Action::Next));
        assert_eq!(key_action(Top, "ArrowLeft"), Some(Action::Prev));
        assert_eq!(key_action(Bottom, "ArrowDown"), Some(Action::Noop), "поперёк дока — перехват без действия");
        assert_eq!(key_action(Left, "ArrowDown"), Some(Action::Next));
        assert_eq!(key_action(Right, "ArrowUp"), Some(Action::Prev));
        assert_eq!(key_action(Right, "ArrowLeft"), Some(Action::Noop));
        for p in [Bottom, Top, Left, Right] {
            assert_eq!(key_action(p, "Home"), Some(Action::First));
            assert_eq!(key_action(p, "End"), Some(Action::Last));
            assert_eq!(key_action(p, "Enter"), Some(Action::Activate));
            assert_eq!(key_action(p, "Space"), Some(Action::Activate));
            assert_eq!(key_action(p, "KeyA"), None);
            assert_eq!(key_action(p, "Tab"), None);
        }
    }

    #[test]
    fn step_table() {
        assert_eq!(step(0, None, Action::Next), None, "нет доступных пунктов");
        assert_eq!(step(3, None, Action::Next), Some(0), "без фокуса — первый");
        assert_eq!(step(3, None, Action::Prev), Some(0));
        assert_eq!(step(3, Some(0), Action::Next), Some(1));
        assert_eq!(step(3, Some(2), Action::Next), Some(2), "на границе остаётся");
        assert_eq!(step(3, Some(0), Action::Prev), Some(0));
        assert_eq!(step(3, Some(2), Action::Prev), Some(1));
        assert_eq!(step(3, Some(1), Action::First), Some(0));
        assert_eq!(step(3, Some(1), Action::Last), Some(2));
        assert_eq!(step(3, Some(1), Action::Noop), Some(1));
        assert_eq!(step(3, Some(1), Action::Activate), Some(1));
        assert_eq!(step(1, Some(0), Action::Next), Some(0));
    }
}
