//! Менеджер z-index оверлеев (порт `ZIndexUtils` из PrimeNG).
//!
//! Состояние хранится в потоке: в браузере поток один; на сервере (SSR) оверлеи не открываются.

use std::cell::RefCell;

/// Группа оверлеев.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZGroup {
    /// Модальные окна (Dialog, Drawer).
    Modal,
    /// Оверлеи (Select, Popover).
    Overlay,
    /// Меню.
    Menu,
    /// Подсказки.
    Tooltip,
}

thread_local! {
    static STACK: RefCell<Vec<(ZGroup, i32)>> = const { RefCell::new(Vec::new()) };
}

/// Выдаёт z-index для группы: следующий выше предыдущего, при смене группы добавляется её база.
pub fn generate(group: ZGroup, base: i32) -> i32 {
    STACK.with_borrow_mut(|stack| {
        let (last_group, last_value) = stack.last().copied().unwrap_or((group, base));
        let value = last_value + if last_group == group { 0 } else { base } + 2;
        stack.push((group, value));
        value
    })
}

/// Освобождает ранее выданный z-index.
pub fn revert(z_index: i32) {
    STACK.with_borrow_mut(|stack| stack.retain(|&(_, v)| v != z_index));
}

/// Текущий верхний z-index или 0.
pub fn current() -> i32 {
    STACK.with_borrow(|stack| stack.last().map_or(0, |&(_, v)| v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_overlays_increase_and_release() {
        let a = generate(ZGroup::Overlay, 1000);
        let b = generate(ZGroup::Overlay, 1000);
        assert_eq!(a, 1002);
        assert_eq!(b, 1004);
        assert_eq!(current(), b);
        revert(b);
        assert_eq!(current(), a);
        revert(a);
        assert_eq!(current(), 0);
    }

    #[test]
    fn group_switch_adds_base() {
        let a = generate(ZGroup::Overlay, 1000);
        let m = generate(ZGroup::Modal, 1100);
        assert_eq!(m, a + 1100 + 2);
        revert(m);
        revert(a);
    }
}
