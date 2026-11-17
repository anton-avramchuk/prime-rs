//! ToggleButton: a port of `p-togglebutton` from PrimeNG 21.1.10.
//!
//! Markup and classes as in PrimeNG: root `div.p-togglebutton[role=button]` (instead of the host `<p-togglebutton>`), inside it
//! `span.p-togglebutton-content` with an icon and `span.p-togglebutton-label`. There is no native `<input>`.
//!
//! State is a two-way signal `checked: RwSignal<bool>`; without it the component keeps internal state starting
//! from `default_checked`. Click and the Enter and Space keys toggle the state and call `on_change`. The icon is passed as a
//! `ViewFn` (for example, `<CheckIcon />` from `prime-icons`), not as a font class name.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_togglebutton::ToggleButton;
//!
//! let checked = RwSignal::new(true);
//! let html = view! { <ToggleButton checked=checked on_label="On" off_label="Off" /> }.to_html();
//! assert!(html.contains("p-togglebutton-checked") && html.contains(r#"aria-pressed="true""#) && html.contains("On"));
//! ```

mod state;

use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{IconPos, ToggleButtonState, data_p, write_classes};

/// Change event: the new state and the original browser event.
#[derive(Clone, Debug)]
pub struct ToggleButtonChange {
    /// New state.
    pub checked: bool,
    /// Original event (`click` or `keydown`).
    pub original_event: web_sys::Event,
}

/// Pass-through parts of ToggleButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToggleButtonPt {
    /// Root element.
    pub root: Pt,
    /// Content (`span.p-togglebutton-content`).
    pub content: Pt,
    /// Icon wrapper (`span.p-togglebutton-icon`).
    pub icon: Pt,
    /// Label (`span.p-togglebutton-label`).
    pub label: Pt,
}

/// Button with a boolean state.
///
/// Every property accepts a value or a signal; `checked` is a two-way signal.
#[component]
pub fn ToggleButton(
    /// Two-way state signal.
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Initial state when `checked` is not passed.
    #[prop(optional)]
    default_checked: bool,
    /// Label of the checked button ("Yes" by default).
    #[prop(optional, into)]
    on_label: OptProp<String>,
    /// Label of the unchecked button ("No" by default).
    #[prop(optional, into)]
    off_label: OptProp<String>,
    /// Icon of the checked button; rendered in `span.p-togglebutton-icon`.
    #[prop(optional, into)]
    on_icon: Option<ViewFn>,
    /// Icon of the unchecked button; rendered in `span.p-togglebutton-icon`.
    #[prop(optional, into)]
    off_icon: Option<ViewFn>,
    /// Icon position.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Whether deselecting is allowed; the restriction applies only to an explicit `false`.
    #[prop(optional, into)]
    allow_empty: OptProp<bool>,
    /// Full width.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// `tabindex` attribute (`0` by default, `-1` when `disabled`).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Replaces the default icon; receives the current state.
    #[prop(optional, into)]
    icon_template: Option<Callback<bool, AnyView>>,
    /// Replaces all content after `span.p-togglebutton-content`; receives the current state.
    #[prop(optional, into)]
    content_template: Option<Callback<bool, AnyView>>,
    /// Called after the user toggles it.
    #[prop(optional, into)]
    on_change: Option<Callback<ToggleButtonChange>>,
    /// Called when it receives focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when it loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<ToggleButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let props = Arc::new(Props {
        checked: checked.unwrap_or_else(|| RwSignal::new(default_checked)),
        on_label,
        off_label,
        on_icon,
        off_icon,
        icon_pos,
        size,
        disabled,
        invalid,
        allow_empty,
        fluid,
        tabindex,
        aria_label,
        aria_labelledby,
        style,
        style_class,
        icon_template,
        content_template,
    });

    // Like `toggle` in PrimeNG: both click and Enter/Space; `allow_empty = false` prevents switching a checked button off.
    let toggle = {
        let props = props.clone();
        move |e: web_sys::Event| {
            let was = props.checked.get_untracked();
            if props.disabled.get_untracked() || (was && props.allow_empty.get_untracked() == Some(false)) {
                return;
            }
            props.checked.set(!was);
            if let Some(cb) = on_change {
                cb.run(ToggleButtonChange { checked: !was, original_event: e });
            }
        }
    };
    let click = {
        let toggle = toggle.clone();
        move |e: web_sys::MouseEvent| toggle(e.into())
    };
    let keydown = move |e: web_sys::KeyboardEvent| {
        if matches!(e.code().as_str(), "Enter" | "Space") {
            e.prevent_default();
            toggle(e.into());
        }
    };
    let focus = move |e: web_sys::FocusEvent| {
        if let Some(cb) = on_focus {
            cb.run(e);
        }
    };
    let blur = move |e: web_sys::FocusEvent| {
        if let Some(cb) = on_blur {
            cb.run(e);
        }
    };

    let (p, t) = (&props, &pt);
    let root_class = rx(p, t, root_class);
    let root_style = rx(p, t, root_style);
    let pressed = rx(p, t, |p, _| bool_attr(p.checked.get()));
    let checked_attr = pressed.clone();
    let disabled_attr = rx(p, t, |p, _| bool_attr(p.disabled.get()));
    let data = rx(p, t, |p, _| data_p(&state_of(p)));
    let tabindex = rx(p, t, |p, _| p.tabindex.get().unwrap_or(if p.disabled.get() { -1 } else { 0 }));
    let aria_label = rx(p, t, |p, _| p.aria_label.get());
    let aria_labelledby = rx(p, t, |p, _| p.aria_labelledby.get());
    let content_class = rx(p, t, |_, t| t.content.merge_class("p-togglebutton-content"));
    let content_data = data.clone();
    let content = rx(p, t, content_of);
    let (root_attrs, content_attrs) = (pt.root.attrs(), pt.content.attrs());

    view! {
        <div
            class={root_class}
            style={root_style}
            role="button"
            aria-pressed={pressed}
            tabindex={tabindex}
            aria-label={aria_label}
            aria-labelledby={aria_labelledby}
            data-pc-name="togglebutton"
            data-p-checked={checked_attr}
            data-p-disabled={disabled_attr}
            data-p={data}
            on:click={click}
            on:keydown={keydown}
            on:focus={focus}
            on:blur={blur}
            {..root_attrs}
        >
            <span class={content_class} data-p={content_data} {..content_attrs}>{content}</span>
        </div>
    }
}

/// Closure over the instance properties: `Arc` clones and a function pointer without capture.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<ToggleButtonPt>,
    f: fn(&Props, &ToggleButtonPt) -> T,
) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

fn state_of(props: &Props) -> ToggleButtonState {
    ToggleButtonState {
        checked: props.checked.get(),
        disabled: props.disabled.get(),
        invalid: props.invalid.get(),
        size: props.size.get(),
        fluid: props.fluid.get(),
    }
}

fn root_class(props: &Props, pt: &ToggleButtonPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

/// Root style; an empty result is `None`, so that `style=""` is not rendered.
fn root_style(props: &Props, pt: &ToggleButtonPt) -> Option<String> {
    let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
    (!style.is_empty()).then_some(style)
}

/// Label with a default fallback; an empty label is a single space, as in the PrimeNG template.
fn label_of(props: &Props) -> String {
    let (label, default) = if props.checked.get() { (props.on_label.get(), "Yes") } else { (props.off_label.get(), "No") };
    match label {
        None => default.to_owned(),
        Some(l) if l.is_empty() => " ".to_owned(),
        Some(l) => l,
    }
}

fn content_of(props: &Props, pt: &ToggleButtonPt) -> AnyView {
    let checked = props.checked.get();
    if let Some(template) = props.content_template {
        return template.run(checked);
    }
    let icon = match props.icon_template {
        Some(template) => Some(template.run(checked)),
        None if props.on_icon.is_some() || props.off_icon.is_some() => {
            let icon = if checked { &props.on_icon } else { &props.off_icon };
            Some(
                view! {
                    <span class=pt.icon.merge_class(props.icon_pos.get().class()) {..pt.icon.attrs()}>
                        {icon.as_ref().map(ViewFn::run)}
                    </span>
                }
                .into_any(),
            )
        }
        None => None,
    };
    view! {
        {icon}
        <span class=pt.label.merge_class("p-togglebutton-label") {..pt.label.attrs()}>{label_of(props)}</span>
    }
    .into_any()
}

/// Instance properties.
struct Props {
    checked: RwSignal<bool>,
    on_label: OptProp<String>,
    off_label: OptProp<String>,
    on_icon: Option<ViewFn>,
    off_icon: Option<ViewFn>,
    icon_pos: Prop<IconPos>,
    size: OptProp<Size>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    allow_empty: OptProp<bool>,
    fluid: Prop<bool>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon_template: Option<Callback<bool, AnyView>>,
    content_template: Option<Callback<bool, AnyView>>,
}
