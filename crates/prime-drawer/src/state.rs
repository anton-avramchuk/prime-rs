//! Чистая логика Drawer: позиция, классы корня и маски, слои (по `drawer.ts` и `drawerstyle.ts` PrimeNG).

/// Край экрана, у которого показывается панель.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerPosition {
    /// Слева (по умолчанию).
    #[default]
    Left,
    /// Справа.
    Right,
    /// Сверху.
    Top,
    /// Снизу.
    Bottom,
    /// На весь экран.
    Full,
}

impl DrawerPosition {
    /// Суффикс классов: `left`, `right`, `top`, `bottom`, `full`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Full => "full",
        }
    }
}

/// Классы корня: позиция, `p-drawer-full` при `full_screen`, `p-drawer-open` пока панель не закрывается.
pub fn root_classes(position: DrawerPosition, full_screen: bool, open: bool) -> String {
    let mut cn = String::from("p-drawer p-component");
    if full_screen && position != DrawerPosition::Full {
        cn.push_str(" p-drawer-full");
    }
    if open {
        cn.push_str(" p-drawer-open");
    }
    cn.push_str(" p-drawer-");
    cn.push_str(position.as_str());
    cn
}

/// Класс анимации: `p-drawer-enter-<позиция>` или `p-drawer-leave-<позиция>` (`full` при `full_screen`).
pub fn animation_class(position: DrawerPosition, full_screen: bool, leaving: bool) -> String {
    let side = if full_screen { "full" } else { position.as_str() };
    format!("p-drawer-{}-{side}", if leaving { "leave" } else { "enter" })
}

/// Классы маски (она всегда модальная): состояние анимации и `p-drawer-full`.
pub fn mask_classes(full_screen: bool, leaving: bool) -> String {
    let mut cn = String::from("p-drawer-mask p-overlay-mask ");
    cn.push_str(if leaving { "p-overlay-mask-leave-active" } else { "p-overlay-mask-enter-active" });
    if full_screen {
        cn.push_str(" p-drawer-full");
    }
    cn
}

/// Значение `data-p`: перечень признаков через пробел.
pub fn data_p(position: DrawerPosition, open: bool, modal: bool) -> String {
    let mut parts = Vec::new();
    if position == DrawerPosition::Full {
        parts.push("full-screen");
    }
    parts.push(position.as_str());
    if open {
        parts.push("open");
    }
    if modal {
        parts.push("modal");
    }
    parts.join(" ")
}

/// Эта панель — верхняя по `z-index` (только она закрывается по Escape).
pub fn is_topmost(own_z: i32, top_z: i32) -> bool {
    own_z >= top_z
}

/// `z-index` маски: на единицу ниже панели.
pub fn mask_z(panel_z: i32) -> i32 {
    panel_z - 1
}

#[cfg(test)]
mod tests {
    use super::DrawerPosition::*;
    use super::*;

    #[test]
    fn root_classes_table() {
        // (позиция, full_screen, open, ожидание)
        let cases = [
            (Left, false, false, "p-drawer p-component p-drawer-left"),
            (Right, false, true, "p-drawer p-component p-drawer-open p-drawer-right"),
            (Top, true, true, "p-drawer p-component p-drawer-full p-drawer-open p-drawer-top"),
            (Bottom, false, false, "p-drawer p-component p-drawer-bottom"),
            (Full, false, true, "p-drawer p-component p-drawer-open p-drawer-full"),
            (Full, true, false, "p-drawer p-component p-drawer-full"),
        ];
        for (p, f, o, want) in cases {
            assert_eq!(root_classes(p, f, o), want, "{p:?} {f} {o}");
        }
    }

    #[test]
    fn animation_class_table() {
        assert_eq!(animation_class(Left, false, false), "p-drawer-enter-left");
        assert_eq!(animation_class(Bottom, false, true), "p-drawer-leave-bottom");
        assert_eq!(animation_class(Right, true, false), "p-drawer-enter-full");
        assert_eq!(animation_class(Full, false, true), "p-drawer-leave-full");
    }

    #[test]
    fn mask_classes_table() {
        assert_eq!(mask_classes(false, false), "p-drawer-mask p-overlay-mask p-overlay-mask-enter-active");
        assert_eq!(mask_classes(false, true), "p-drawer-mask p-overlay-mask p-overlay-mask-leave-active");
        assert_eq!(mask_classes(true, false), "p-drawer-mask p-overlay-mask p-overlay-mask-enter-active p-drawer-full");
    }

    #[test]
    fn data_p_table() {
        assert_eq!(data_p(Left, true, true), "left open modal");
        assert_eq!(data_p(Right, false, false), "right");
        assert_eq!(data_p(Full, true, false), "full-screen full open");
    }

    #[test]
    fn layering() {
        assert!(is_topmost(1102, 1102));
        assert!(is_topmost(1104, 1102));
        assert!(!is_topmost(1100, 1102));
        assert_eq!(mask_z(1102), 1101);
    }
}
