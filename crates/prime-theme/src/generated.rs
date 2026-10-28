// Файл сгенерирован `cargo xtask gen-theme`; не редактировать вручную.

/// CSS компонента `accordion`.
#[cfg(feature = "accordion")]
pub const ACCORDION: &str = include_str!("../css/components/accordion.css");

/// CSS компонента `avatar`.
#[cfg(feature = "avatar")]
pub const AVATAR: &str = include_str!("../css/components/avatar.css");

/// CSS компонента `badge`.
#[cfg(feature = "badge")]
pub const BADGE: &str = include_str!("../css/components/badge.css");

/// CSS компонента `button`.
#[cfg(feature = "button")]
pub const BUTTON: &str = include_str!("../css/components/button.css");

/// CSS компонента `card`.
#[cfg(feature = "card")]
pub const CARD: &str = include_str!("../css/components/card.css");

/// CSS компонента `checkbox`.
#[cfg(feature = "checkbox")]
pub const CHECKBOX: &str = include_str!("../css/components/checkbox.css");

/// CSS компонента `chip`.
#[cfg(feature = "chip")]
pub const CHIP: &str = include_str!("../css/components/chip.css");

/// CSS компонента `divider`.
#[cfg(feature = "divider")]
pub const DIVIDER: &str = include_str!("../css/components/divider.css");

/// CSS компонента `fieldset`.
#[cfg(feature = "fieldset")]
pub const FIELDSET: &str = include_str!("../css/components/fieldset.css");

/// CSS компонента `inputtext`.
#[cfg(feature = "inputtext")]
pub const INPUTTEXT: &str = include_str!("../css/components/inputtext.css");

/// CSS компонента `message`.
#[cfg(feature = "message")]
pub const MESSAGE: &str = include_str!("../css/components/message.css");

/// CSS компонента `panel`.
#[cfg(feature = "panel")]
pub const PANEL: &str = include_str!("../css/components/panel.css");

/// CSS компонента `progressbar`.
#[cfg(feature = "progressbar")]
pub const PROGRESSBAR: &str = include_str!("../css/components/progressbar.css");

/// CSS компонента `radiobutton`.
#[cfg(feature = "radiobutton")]
pub const RADIOBUTTON: &str = include_str!("../css/components/radiobutton.css");

/// CSS компонента `skeleton`.
#[cfg(feature = "skeleton")]
pub const SKELETON: &str = include_str!("../css/components/skeleton.css");

/// CSS компонента `tag`.
#[cfg(feature = "tag")]
pub const TAG: &str = include_str!("../css/components/tag.css");

/// CSS компонента `textarea`.
#[cfg(feature = "textarea")]
pub const TEXTAREA: &str = include_str!("../css/components/textarea.css");

/// CSS компонента `toggleswitch`.
#[cfg(feature = "toggleswitch")]
pub const TOGGLESWITCH: &str = include_str!("../css/components/toggleswitch.css");

/// Пары «имя — CSS» для включённых компонентов.
pub(crate) static COMPONENTS: &[(&str, &str)] = &[
    #[cfg(feature = "accordion")]
    ("accordion", ACCORDION),
    #[cfg(feature = "avatar")]
    ("avatar", AVATAR),
    #[cfg(feature = "badge")]
    ("badge", BADGE),
    #[cfg(feature = "button")]
    ("button", BUTTON),
    #[cfg(feature = "card")]
    ("card", CARD),
    #[cfg(feature = "checkbox")]
    ("checkbox", CHECKBOX),
    #[cfg(feature = "chip")]
    ("chip", CHIP),
    #[cfg(feature = "divider")]
    ("divider", DIVIDER),
    #[cfg(feature = "fieldset")]
    ("fieldset", FIELDSET),
    #[cfg(feature = "inputtext")]
    ("inputtext", INPUTTEXT),
    #[cfg(feature = "message")]
    ("message", MESSAGE),
    #[cfg(feature = "panel")]
    ("panel", PANEL),
    #[cfg(feature = "progressbar")]
    ("progressbar", PROGRESSBAR),
    #[cfg(feature = "radiobutton")]
    ("radiobutton", RADIOBUTTON),
    #[cfg(feature = "skeleton")]
    ("skeleton", SKELETON),
    #[cfg(feature = "tag")]
    ("tag", TAG),
    #[cfg(feature = "textarea")]
    ("textarea", TEXTAREA),
    #[cfg(feature = "toggleswitch")]
    ("toggleswitch", TOGGLESWITCH),
];
