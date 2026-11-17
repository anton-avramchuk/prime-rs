//! Overlay z-index manager (port of `ZIndexUtils` from PrimeNG).
//!
//! State is kept per thread: in the browser there is one thread; on the server (SSR) overlays are not opened.

use std::cell::RefCell;

/// Overlay group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZGroup {
    /// Modal windows (Dialog, Drawer).
    Modal,
    /// Overlays (Select, Popover).
    Overlay,
    /// Menus.
    Menu,
    /// Tooltips.
    Tooltip,
}

thread_local! {
    static STACK: RefCell<Vec<(ZGroup, i32)>> = const { RefCell::new(Vec::new()) };
}

/// Issues a z-index for a group: the next one is higher than the previous, on a group change its base is added.
pub fn generate(group: ZGroup, base: i32) -> i32 {
    STACK.with_borrow_mut(|stack| {
        let (last_group, last_value) = stack.last().copied().unwrap_or((group, base));
        let value = last_value + if last_group == group { 0 } else { base } + 2;
        stack.push((group, value));
        value
    })
}

/// Releases a previously issued z-index.
pub fn revert(z_index: i32) {
    STACK.with_borrow_mut(|stack| stack.retain(|&(_, v)| v != z_index));
}

/// Current top z-index or 0.
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
