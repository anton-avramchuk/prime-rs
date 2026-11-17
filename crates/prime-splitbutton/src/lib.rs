//! SplitButton — port of `p-splitbutton` from PrimeNG 21.1.10.
//!
//! A button with a default action (`p-splitbutton-button`, which is a `Button`), an arrow button (`p-splitbutton-dropdown`) and a popup
//! command menu (`Menu` in `popup` mode) attached to the whole component. In PrimeNG the menu is a `TieredMenu` with submenus; here for now it is a
//! flat `Menu` with groups, nested submenus will arrive together with `TieredMenu`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::MenuItem;
//! use prime_splitbutton::SplitButton;
//!
//! let model = vec![MenuItem::new().label("Update"), MenuItem::new().label("Delete")];
//! let html = Owner::new().with(|| view! { <SplitButton label="Save" model=model /> }.to_html());
//! assert!(html.contains("p-splitbutton-button") && html.contains("p-splitbutton-dropdown") && html.contains("aria-expanded=\"false\""));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity, ButtonState, IconPos, root_classes as button_classes, root_data_p as button_data_p};
use prime_core::{ClassNames, OptProp, Prop, Pt, Size};
use prime_icons::ChevronDownIcon;
use prime_menu::{Menu, MenuItem, MenuRef};
use std::sync::Arc;

pub use state::{is_toggle_key, root_classes};

/// Pass-through parts of SplitButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SplitButtonPt {
    /// Root.
    pub root: Pt,
    /// Main button (classes and attributes are added to `Button`).
    pub button: Pt,
    /// Dropdown button.
    pub dropdown: Pt,
}

/// A button with a default action and a dropdown command menu.
///
/// Every property accepts a value or a signal.
#[component]
pub fn SplitButton(
    /// Menu commands.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Severity of both buttons.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// With a shadow (root class).
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Rounded edges (root class).
    #[prop(optional, into)]
    rounded: Prop<bool>,
    /// Text buttons.
    #[prop(optional, into)]
    text: Prop<bool>,
    /// Buttons with a border.
    #[prop(optional, into)]
    outlined: Prop<bool>,
    /// Plain text.
    #[prop(optional, into)]
    plain: Prop<bool>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Icon of the main button.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Position of the main button icon.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Label of the main button.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// `aria-label` attribute of the main button.
    #[prop(optional, into)]
    button_aria_label: OptProp<String>,
    /// `aria-label` attribute of the dropdown button.
    #[prop(optional, into)]
    expand_aria_label: OptProp<String>,
    /// Icon of the dropdown button instead of the arrow.
    #[prop(optional, into)]
    dropdown_icon: Option<ViewFn>,
    /// Content of the main button instead of the label and icon.
    #[prop(optional, into)]
    content: Option<ViewFn>,
    /// Both buttons are unavailable.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// The main button is unavailable.
    #[prop(optional, into)]
    button_disabled: Prop<bool>,
    /// The dropdown button is unavailable.
    #[prop(optional, into)]
    menu_button_disabled: Prop<bool>,
    /// `tabindex` attribute of the main button.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Focus the main button on show.
    #[prop(optional)]
    autofocus: bool,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Inline style of the menu.
    #[prop(optional, into)]
    menu_style: OptProp<String>,
    /// Extra class of the menu.
    #[prop(optional, into)]
    menu_style_class: OptProp<String>,
    /// Click on the main button.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Click on the dropdown button.
    #[prop(optional, into)]
    on_dropdown_click: Option<Callback<web_sys::MouseEvent>>,
    /// Menu shown.
    #[prop(optional, into)]
    on_menu_show: Option<Callback<()>>,
    /// Menu hidden.
    #[prop(optional, into)]
    on_menu_hide: Option<Callback<()>>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<SplitButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = format!("pn_id_{}", prime_core::ids::next_unique());
    let menu_ref = MenuRef::new();
    let expanded = RwSignal::new(false);
    let root = NodeRef::<html::Div>::new();

    let toggle = move || {
        if menu_ref.visible_untracked() {
            menu_ref.hide();
        } else if let Some(el) = root.get_untracked() {
            menu_ref.show_at(&el);
        }
    };
    let main_click = Callback::new(move |e: web_sys::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        menu_ref.hide();
    });
    let dropdown_click = move |e: web_sys::MouseEvent| {
        if let Some(cb) = on_dropdown_click {
            cb.run(e);
        }
        toggle();
    };
    let dropdown_key = move |e: web_sys::KeyboardEvent| {
        if is_toggle_key(&e.code()) {
            toggle();
            e.prevent_default();
        }
    };
    let on_show = Callback::new(move |()| {
        expanded.set(true);
        if let Some(cb) = on_menu_show {
            cb.run(());
        }
    });
    let on_hide = Callback::new(move |()| {
        expanded.set(false);
        if let Some(cb) = on_menu_hide {
            cb.run(());
        }
    });

    let class = {
        let (pt, outlined, text, size) = (pt.clone(), outlined.clone(), text.clone(), size.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add(&root_classes(raised.get(), rounded.get(), outlined.get(), text.get(), size.get()));
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let data_severity = {
        let severity = severity.clone();
        move || severity.get().map(ButtonSeverity::as_str)
    };
    let main_disabled = {
        let disabled = disabled.clone();
        Signal::derive(move || disabled.get() || button_disabled.get())
    };
    let menu_disabled = Signal::derive(move || disabled.get() || menu_button_disabled.get());
    let main_class = pt.button.merge_class("p-splitbutton-button").into_owned();
    let root_attrs = pt.root.attrs();

    let main = main_button(
        MainParts {
            label: label.clone(),
            severity: severity.clone(),
            text: text.clone(),
            outlined: outlined.clone(),
            plain: plain.clone(),
            size: size.clone(),
            icon_pos: icon_pos.clone(),
            tabindex,
            button_aria_label: button_aria_label.clone(),
            disabled: main_disabled,
            autofocus,
            class: main_class,
            on_click: main_click,
        },
        icon.clone(),
        content.clone(),
    );
    let dropdown = dropdown_button(
        DropdownParts {
            severity,
            text,
            outlined,
            plain,
            size,
            expand_aria_label,
            expanded,
            menu_id: menu_id.clone(),
            disabled: menu_disabled,
            pt: pt.clone(),
            dropdown_icon,
        },
        dropdown_click,
        dropdown_key,
    );

    view! {
        <div node_ref=root class={class} style={root_style} data-p-severity={data_severity} {..root_attrs}>
            {main}
            {dropdown}
            <Menu popup=true menu_ref=menu_ref model=model id=menu_id style=menu_style style_class=menu_style_class on_show=on_show on_hide=on_hide />
        </div>
    }
}

/// Properties of the main button.
struct MainParts {
    label: OptProp<String>,
    severity: OptProp<ButtonSeverity>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    size: OptProp<Size>,
    icon_pos: Prop<IconPos>,
    tabindex: OptProp<i32>,
    button_aria_label: OptProp<String>,
    disabled: Signal<bool>,
    autofocus: bool,
    class: String,
    on_click: Callback<web_sys::MouseEvent>,
}

/// Main button: `Button` taking the optional `icon` and `content` into account (four markup variants).
fn main_button(p: MainParts, icon: Option<ViewFn>, content: Option<ViewFn>) -> AnyView {
    macro_rules! button {
        ($($extra:tt)*) => {
            view! {
                <Button
                    label=p.label.clone()
                    severity=p.severity.clone()
                    text=p.text.clone()
                    outlined=p.outlined.clone()
                    plain=p.plain.clone()
                    size=p.size.clone()
                    icon_pos=p.icon_pos.clone()
                    tabindex=p.tabindex.clone()
                    aria_label=p.button_aria_label.clone()
                    disabled=p.disabled
                    autofocus=p.autofocus
                    style_class=p.class.clone()
                    on_click=p.on_click
                    $($extra)*
                />
            }
            .into_any()
        };
    }
    match (icon, content) {
        (Some(icon), Some(content)) => view! {
            <Button
                severity=p.severity.clone()
                text=p.text.clone()
                outlined=p.outlined.clone()
                plain=p.plain.clone()
                size=p.size.clone()
                icon_pos=p.icon_pos.clone()
                tabindex=p.tabindex.clone()
                aria_label=p.button_aria_label.clone()
                disabled=p.disabled
                autofocus=p.autofocus
                style_class=p.class.clone()
                on_click=p.on_click
                icon=icon
            >
                {content.run()}
            </Button>
        }
        .into_any(),
        (None, Some(content)) => view! {
            <Button
                severity=p.severity.clone()
                text=p.text.clone()
                outlined=p.outlined.clone()
                plain=p.plain.clone()
                size=p.size.clone()
                icon_pos=p.icon_pos.clone()
                tabindex=p.tabindex.clone()
                aria_label=p.button_aria_label.clone()
                disabled=p.disabled
                autofocus=p.autofocus
                style_class=p.class.clone()
                on_click=p.on_click
            >
                {content.run()}
            </Button>
        }
        .into_any(),
        (Some(icon), None) => button!(icon = icon),
        (None, None) => button!(),
    }
}

/// Properties of the dropdown button.
struct DropdownParts {
    severity: OptProp<ButtonSeverity>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    size: OptProp<Size>,
    expand_aria_label: OptProp<String>,
    expanded: RwSignal<bool>,
    menu_id: String,
    disabled: Signal<bool>,
    pt: Arc<SplitButtonPt>,
    dropdown_icon: Option<ViewFn>,
}

/// Dropdown button with its own markup: `Button` does not accept reactive `aria-expanded` and `aria-controls`.
fn dropdown_button(
    p: DropdownParts,
    click: impl Fn(web_sys::MouseEvent) + Send + Sync + 'static,
    key: impl Fn(web_sys::KeyboardEvent) + Send + Sync + 'static,
) -> impl IntoView {
    let state = {
        let (severity, text, outlined, plain, size) =
            (p.severity.clone(), p.text.clone(), p.outlined.clone(), p.plain.clone(), p.size.clone());
        move || {
            // the arrow button has no label and badge: only an icon
            ButtonState {
                has_icon: true,
                severity: severity.get(),
                text: text.get(),
                outlined: outlined.get(),
                plain: plain.get(),
                size: size.get(),
                ..ButtonState::default()
            }
        }
    };
    let class = {
        let (state, pt) = (state.clone(), p.pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add(&button_classes(&state())).add("p-splitbutton-dropdown p-ripple").add_user(pt.dropdown.extra_class());
            cn.finish_cow()
        }
    };
    let data = {
        let state = state.clone();
        move || button_data_p(&state())
    };
    let (disabled, expanded) = (p.disabled, p.expanded);
    let attrs: Vec<AnyAttribute> = {
        let expand_label = p.expand_aria_label.clone();
        let menu_id = p.menu_id.clone();
        vec![
            custom_attribute("aria-label", move || expand_label.get()).into_any_attr(),
            custom_attribute("aria-expanded", move || Some(expanded.get().to_string())).into_any_attr(),
            custom_attribute("aria-controls", move || Some(menu_id.clone())).into_any_attr(),
        ]
    };
    let pt_attrs = p.pt.dropdown.attrs();
    let data_severity = {
        let severity = p.severity.clone();
        move || severity.get().map(ButtonSeverity::as_str)
    };
    let icon = match p.dropdown_icon {
        Some(icon) => icon.run().into_any(),
        None => view! { <ChevronDownIcon /> }.into_any(),
    };
    // `NodeRef` is needed only by the client (ripple): on SSR it is absent, and `()` is an empty attribute instead of `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let node = {
        let node = NodeRef::<html::Button>::new();
        if prime_core::use_prime_config().ripple {
            let ripple = StoredValue::new_local(None::<prime_dom::ripple::Ripple>);
            Effect::new(move |_| {
                if let Some(el) = node.get()
                    && ripple.with_value(Option::is_none)
                {
                    ripple.set_value(prime_dom::ripple::Ripple::attach(&el));
                }
            });
            on_cleanup(move || ripple.set_value(None));
        }
        leptos::tachys::html::node_ref::node_ref::<html::Button, _>(node)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let node = {};
    view! {
        <button
            {..node}
            type="button"
            class={class}
            data-p={data}
            data-p-severity={data_severity}
            data-p-disabled={move || disabled.get().then_some("true")}
            aria-haspopup="true"
            disabled={move || disabled.get().then_some("")}
            on:click={click}
            on:keydown={key}
            {..attrs}
            {..pt_attrs}
        >
            {icon}
        </button>
    }
}
