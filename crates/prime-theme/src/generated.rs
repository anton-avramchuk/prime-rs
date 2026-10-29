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

/// CSS компонента `buttongroup`.
#[cfg(feature = "buttongroup")]
pub const BUTTONGROUP: &str = include_str!("../css/components/buttongroup.css");

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

/// CSS компонента `floatlabel`.
#[cfg(feature = "floatlabel")]
pub const FLOATLABEL: &str = include_str!("../css/components/floatlabel.css");

/// CSS компонента `iconfield`.
#[cfg(feature = "iconfield")]
pub const ICONFIELD: &str = include_str!("../css/components/iconfield.css");

/// CSS компонента `iftalabel`.
#[cfg(feature = "iftalabel")]
pub const IFTALABEL: &str = include_str!("../css/components/iftalabel.css");

/// CSS компонента `inputgroup`.
#[cfg(feature = "inputgroup")]
pub const INPUTGROUP: &str = include_str!("../css/components/inputgroup.css");

/// CSS компонента `inputnumber`.
#[cfg(feature = "inputnumber")]
pub const INPUTNUMBER: &str = include_str!("../css/components/inputnumber.css");

/// CSS компонента `inputtext`.
#[cfg(feature = "inputtext")]
pub const INPUTTEXT: &str = include_str!("../css/components/inputtext.css");

/// CSS компонента `knob`.
#[cfg(feature = "knob")]
pub const KNOB: &str = include_str!("../css/components/knob.css");

/// CSS компонента `message`.
#[cfg(feature = "message")]
pub const MESSAGE: &str = include_str!("../css/components/message.css");

/// CSS компонента `panel`.
#[cfg(feature = "panel")]
pub const PANEL: &str = include_str!("../css/components/panel.css");

/// CSS компонента `progressbar`.
#[cfg(feature = "progressbar")]
pub const PROGRESSBAR: &str = include_str!("../css/components/progressbar.css");

/// CSS компонента `progressspinner`.
#[cfg(feature = "progressspinner")]
pub const PROGRESSSPINNER: &str = include_str!("../css/components/progressspinner.css");

/// CSS компонента `radiobutton`.
#[cfg(feature = "radiobutton")]
pub const RADIOBUTTON: &str = include_str!("../css/components/radiobutton.css");

/// CSS компонента `rating`.
#[cfg(feature = "rating")]
pub const RATING: &str = include_str!("../css/components/rating.css");

/// CSS компонента `scrollpanel`.
#[cfg(feature = "scrollpanel")]
pub const SCROLLPANEL: &str = include_str!("../css/components/scrollpanel.css");

/// CSS компонента `select`.
#[cfg(feature = "select")]
pub const SELECT: &str = include_str!("../css/components/select.css");

/// CSS компонента `selectbutton`.
#[cfg(feature = "selectbutton")]
pub const SELECTBUTTON: &str = include_str!("../css/components/selectbutton.css");

/// CSS компонента `skeleton`.
#[cfg(feature = "skeleton")]
pub const SKELETON: &str = include_str!("../css/components/skeleton.css");

/// CSS компонента `slider`.
#[cfg(feature = "slider")]
pub const SLIDER: &str = include_str!("../css/components/slider.css");

/// CSS компонента `tag`.
#[cfg(feature = "tag")]
pub const TAG: &str = include_str!("../css/components/tag.css");

/// CSS компонента `textarea`.
#[cfg(feature = "textarea")]
pub const TEXTAREA: &str = include_str!("../css/components/textarea.css");

/// CSS компонента `togglebutton`.
#[cfg(feature = "togglebutton")]
pub const TOGGLEBUTTON: &str = include_str!("../css/components/togglebutton.css");

/// CSS компонента `toggleswitch`.
#[cfg(feature = "toggleswitch")]
pub const TOGGLESWITCH: &str = include_str!("../css/components/toggleswitch.css");

/// CSS компонента `toolbar`.
#[cfg(feature = "toolbar")]
pub const TOOLBAR: &str = include_str!("../css/components/toolbar.css");

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
    #[cfg(feature = "buttongroup")]
    ("buttongroup", BUTTONGROUP),
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
    #[cfg(feature = "floatlabel")]
    ("floatlabel", FLOATLABEL),
    #[cfg(feature = "iconfield")]
    ("iconfield", ICONFIELD),
    #[cfg(feature = "iftalabel")]
    ("iftalabel", IFTALABEL),
    #[cfg(feature = "inputgroup")]
    ("inputgroup", INPUTGROUP),
    #[cfg(feature = "inputnumber")]
    ("inputnumber", INPUTNUMBER),
    #[cfg(feature = "inputtext")]
    ("inputtext", INPUTTEXT),
    #[cfg(feature = "knob")]
    ("knob", KNOB),
    #[cfg(feature = "message")]
    ("message", MESSAGE),
    #[cfg(feature = "panel")]
    ("panel", PANEL),
    #[cfg(feature = "progressbar")]
    ("progressbar", PROGRESSBAR),
    #[cfg(feature = "progressspinner")]
    ("progressspinner", PROGRESSSPINNER),
    #[cfg(feature = "radiobutton")]
    ("radiobutton", RADIOBUTTON),
    #[cfg(feature = "rating")]
    ("rating", RATING),
    #[cfg(feature = "scrollpanel")]
    ("scrollpanel", SCROLLPANEL),
    #[cfg(feature = "select")]
    ("select", SELECT),
    #[cfg(feature = "selectbutton")]
    ("selectbutton", SELECTBUTTON),
    #[cfg(feature = "skeleton")]
    ("skeleton", SKELETON),
    #[cfg(feature = "slider")]
    ("slider", SLIDER),
    #[cfg(feature = "tag")]
    ("tag", TAG),
    #[cfg(feature = "textarea")]
    ("textarea", TEXTAREA),
    #[cfg(feature = "togglebutton")]
    ("togglebutton", TOGGLEBUTTON),
    #[cfg(feature = "toggleswitch")]
    ("toggleswitch", TOGGLESWITCH),
    #[cfg(feature = "toolbar")]
    ("toolbar", TOOLBAR),
];
