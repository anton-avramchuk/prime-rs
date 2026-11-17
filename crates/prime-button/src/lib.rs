//! Button — port of `p-button` from PrimeNG 21.1.10.
//!
//! Classes, `data-p*` and content order (`children`, icon or loading indicator, label, badge) match PrimeNG.
//! Instead of the host element `<p-button>`, only `<button>` is rendered: the theme CSS is bound to classes, not to the tag.
//! Properties are [`Prop`]/[`OptProp`]: a static value creates no reactive primitives, a signal subscribes the attributes.
//! The icon is passed as `ViewFn` (for example `<CheckIcon />` from `prime-icons`), not as a font class name.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_button::{Button, ButtonSeverity};
//!
//! let html = view! { <Button label="Save" severity=ButtonSeverity::Success /> }.to_html();
//! assert!(html.contains("p-button-success") && html.contains("p-button-label"));
//! ```

mod state;

use leptos::either::{Either, EitherOf4};
use leptos::prelude::*;
use prime_badge::{Badge, BadgePt, BadgeSeverity};
#[cfg(target_arch = "wasm32")]
use prime_dom::Ripple;
use prime_icons::SpinnerIcon;
#[cfg(target_arch = "wasm32")]
use prime_rs_core::use_prime_config;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{
    ButtonSeverity, ButtonState, ButtonVariant, IconPos, icon_classes, icon_data_p, label_data_p, root_classes, root_data_p,
    write_root_classes,
};

/// Pass-through for the parts of Button: classes and attributes are added to the built-in ones, not replacing them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ButtonPt {
    /// Root `<button>`.
    pub root: Pt,
    /// Label (`span.p-button-label`).
    pub label: Pt,
    /// Icon wrapper (`span.p-button-icon`).
    pub icon: Pt,
    /// Loading indicator (`.p-button-loading-icon`).
    pub loading_icon: Pt,
    /// Badge (root of `Badge`).
    pub badge: Pt,
}

/// Autofocus and ripple: browser only (`wasm32`). On the server there is no DOM, so no `Effect` is created at all
/// (this also avoids extra cost during SSR and does not depend on the Leptos feature set).
#[cfg(target_arch = "wasm32")]
fn setup_client(node: NodeRef<leptos::html::Button>, autofocus: bool) {
    let ripple_enabled = use_prime_config().ripple;
    // Without autofocus and ripple the effect is not needed: do not create it at all (saves memory on every button).
    if !autofocus && !ripple_enabled {
        return;
    }
    let ripple = StoredValue::new_local(None::<Ripple>);
    Effect::new(move |_| {
        let Some(el) = node.get() else { return };
        if autofocus {
            let _ = el.focus();
        }
        if ripple_enabled && ripple.with_value(Option::is_none) {
            ripple.set_value(Ripple::attach(&el));
        }
    });
    on_cleanup(move || ripple.set_value(None));
}

/// Default badge severity is `secondary` (as in PrimeNG).
fn badge_severity_or_default(p: &OptProp<BadgeSeverity>) -> OptProp<BadgeSeverity> {
    match p {
        Prop::Static(v) => Prop::Static(Some(v.unwrap_or(BadgeSeverity::Secondary))),
        Prop::Reactive(_) => {
            let p = p.clone();
            Prop::Reactive(Signal::derive(move || Some(p.get().unwrap_or(BadgeSeverity::Secondary))))
        }
    }
}

/// Button.
///
/// Each property accepts a value or a signal; a value creates no reactive primitives. If `children` are set,
/// the label and badge are not rendered, and `children` come first, like projected content in PrimeNG.
///
/// ```
/// use leptos::prelude::*;
/// use prime_button::{Button, ButtonPt, IconPos};
/// use prime_rs_core::Pt;
/// use prime_icons::CheckIcon;
///
/// // icon on the right, badge and pass-through for the label
/// let pt = ButtonPt { label: Pt::new().class("my-label"), ..Default::default() };
/// let html = view! {
///     <Button label="Done" icon=ViewFn::from(|| view! { <CheckIcon /> }) icon_pos=IconPos::Right badge="2" pt=pt />
/// }
/// .to_html();
/// assert!(html.contains("p-button-icon-right") && html.contains("my-label") && html.contains("p-badge"));
///
/// // loading state changed by a signal
/// let loading = RwSignal::new(true);
/// let html = view! { <Button label="Sending" loading=loading /> }.to_html();
/// assert!(html.contains("p-button-loading") && html.contains("disabled"));
/// ```
#[component]
pub fn Button(
    /// Label text.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Icon; rendered in `span.p-button-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Icon position.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Loading: the button is unavailable, an indicator is shown instead of the icon.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Custom loading indicator (default `SpinnerIcon`).
    #[prop(optional, into)]
    loading_icon: Option<ViewFn>,
    /// Severity.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// Variant: outlined or text.
    #[prop(optional, into)]
    variant: OptProp<ButtonVariant>,
    /// Text button.
    #[prop(optional, into)]
    text: Prop<bool>,
    /// Outlined button.
    #[prop(optional, into)]
    outlined: Prop<bool>,
    /// Plain text.
    #[prop(optional, into)]
    plain: Prop<bool>,
    /// Link.
    #[prop(optional, into)]
    link: Prop<bool>,
    /// With shadow.
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Rounded.
    #[prop(optional, into)]
    rounded: Prop<bool>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Full width of the container.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Badge value.
    #[prop(optional, into)]
    badge: OptProp<String>,
    /// Badge severity (default `secondary`).
    #[prop(optional, into)]
    badge_severity: OptProp<BadgeSeverity>,
    /// Unavailable.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// `type` attribute (default `button`).
    #[prop(optional, into)]
    button_type: OptProp<String>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `tabindex` attribute.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Receive focus after mounting in the browser (not set on the server).
    #[prop(optional)]
    autofocus: bool,
    /// Called on click of an available button.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Called when focus is received.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when focus is lost.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ButtonPt>,
    /// Content instead of the label and badge.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let has_children = children.is_some();
    let content = children.map(|c| c());
    let badge_severity = badge_severity_or_default(&badge_severity);
    let fluid = prime_rs_core::fluid::fluid_prop(fluid);
    let is_static = all_static!(
        label,
        badge,
        loading,
        severity,
        variant,
        size,
        icon_pos,
        text,
        outlined,
        plain,
        link,
        raised,
        rounded,
        fluid,
        disabled,
        button_type,
        aria_label,
        tabindex,
        style,
        style_class,
        badge_severity
    );
    let has_icon = icon.is_some() || loading_icon.is_some();

    // All properties in one `Arc`: attribute closures clone the reference counter, not the value strings.
    let props = Props {
        label,
        badge,
        badge_severity,
        loading,
        severity,
        variant,
        size,
        icon_pos,
        text,
        outlined,
        plain,
        link,
        raised,
        rounded,
        fluid,
        disabled,
        button_type,
        aria_label,
        tabindex,
        style,
        style_class,
        icon,
        loading_icon,
        has_icon,
        has_children,
    };

    // `NodeRef` is needed only by the client (autofocus, ripple): it is not created at all during SSR.
    #[cfg(target_arch = "wasm32")]
    let node = {
        let node = NodeRef::<leptos::html::Button>::new();
        setup_client(node, autofocus);
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Button, _>(node)
    };
    // `()` — an empty attribute instead of `node_ref`, so the markup is shared by both targets.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let node = {
        let _ = autofocus;
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
    let attrs = pt.root.attrs();

    if is_static {
        // Static path: values and blocks are computed once, there are no closures and no `RenderEffect`.
        let off = props.is_disabled();
        let click = move |e: web_sys::MouseEvent| {
            if let (Some(cb), false) = (on_click, off) {
                cb.run(e);
            }
        };
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let (ty, aria, tab, disabled) = (type_of(&props), props.aria_label.get(), props.tabindex.get(), props.is_disabled());
        let (data_disabled, data_severity) = (disabled.then_some("true"), props.severity.get().map(ButtonSeverity::as_str));
        let (indicator, label, badge) = (indicator_of(&props, &pt), label_of(&props, &pt), badge_of(&props, &pt));
        Either::Left(button_markup!(
            ty,
            class,
            style,
            aria,
            tab,
            disabled,
            node,
            click,
            focus,
            blur,
            data,
            data_disabled,
            data_severity,
            attrs,
            content,
            indicator,
            label,
            badge
        ))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let click = {
            let props = props.clone();
            move |e: web_sys::MouseEvent| {
                if let (Some(cb), false) = (on_click, props.is_disabled()) {
                    cb.run(e);
                }
            }
        };
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let ty = {
            let props = props.clone();
            move || type_of(&props)
        };
        let aria = {
            let props = props.clone();
            move || props.aria_label.get()
        };
        let tab = {
            let props = props.clone();
            move || props.tabindex.get()
        };
        let disabled = {
            let props = props.clone();
            move || props.is_disabled()
        };
        let data_disabled = {
            let props = props.clone();
            move || props.is_disabled().then_some("true")
        };
        let data_severity = {
            let props = props.clone();
            move || props.severity.get().map(ButtonSeverity::as_str)
        };
        let indicator = {
            let (props, pt) = (props.clone(), pt.clone());
            move || indicator_of(&props, &pt)
        };
        let label = {
            let (props, pt) = (props.clone(), pt.clone());
            move || label_of(&props, &pt)
        };
        let badge = move || badge_of(&props, &pt);
        Either::Right(button_markup!(
            ty,
            class,
            style,
            aria,
            tab,
            disabled,
            node,
            click,
            focus,
            blur,
            data,
            data_disabled,
            data_severity,
            attrs,
            content,
            indicator,
            label,
            badge
        ))
    }
}

/// Button markup: one for the static and reactive branches (only the attribute and block expressions differ).
macro_rules! button_markup {
    (
        $ty:expr, $class:expr, $style:expr, $aria:expr, $tab:expr, $disabled:expr, $node:expr, $click:expr, $focus:expr, $blur:expr,
        $data:expr, $data_disabled:expr, $data_severity:expr, $attrs:expr, $content:expr, $indicator:expr, $label:expr, $badge:expr
    ) => {
        view! {
            <button
                type={$ty}
                class={$class}
                style={$style}
                aria-label={$aria}
                tabindex={$tab}
                disabled={$disabled}
                                on:click={$click}
                on:focus={$focus}
                on:blur={$blur}
                data-p={$data}
                data-p-disabled={$data_disabled}
                data-p-severity={$data_severity}
                {..$attrs}
                {..$node}
            >
                {$content}
                {$indicator}
                {$label}
                {$badge}
            </button>
        }
    };
}
use button_markup;

fn class_of(props: &Props, pt: &ButtonPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    props.with_state(|s| write_root_classes(&mut cn, s));
    cn.add("p-ripple");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &ButtonPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    props.with_state(root_data_p)
}

fn type_of(props: &Props) -> Cow<'static, str> {
    props.button_type.with(|t| t.as_deref().map_or(Cow::Borrowed("button"), |s| Cow::Owned(s.to_owned())))
}

/// Icon or loading indicator.
fn indicator_of(props: &Props, pt: &ButtonPt) -> impl IntoView + use<> {
    if props.loading.get() {
        match props.loading_icon.clone() {
            Some(custom) => EitherOf4::A(view! {
                <span class=pt.loading_icon.merge_class("p-button-loading-icon") aria-hidden="true" {..pt.loading_icon.attrs()}>
                    {custom.run()}
                </span>
            }),
            None => EitherOf4::B(
                view! { <SpinnerIcon spin=true class=pt.loading_icon.merge_class("p-button-loading-icon") {..pt.loading_icon.attrs()} /> },
            ),
        }
    } else if let Some(icon) = props.icon.clone() {
        let (class, data) = props.with_state(|s| (icon_classes(s), icon_data_p(s)));
        EitherOf4::C(view! {
            <span class=pt.icon.merge_class(&class) data-p={data} {..pt.icon.attrs()}>
                {icon.run()}
            </span>
        })
    } else {
        EitherOf4::D(())
    }
}

/// Label (`None` if there are `children` or the label is empty).
fn label_of(props: &Props, pt: &ButtonPt) -> Option<impl IntoView + use<>> {
    if props.has_children {
        return None;
    }
    let data = props.with_state(|s| s.has_label().then(|| label_data_p(s)))?;
    Some(view! {
        <span class=pt.label.merge_class("p-button-label") data-p={data} {..pt.label.attrs()}>
            {props.label.get()}
        </span>
    })
}

/// Badge (`None` if there are `children` or the badge is empty).
fn badge_of(props: &Props, pt: &ButtonPt) -> Option<impl IntoView + use<>> {
    let present = !props.has_children && props.with_state(|s| s.has_badge());
    present
        .then(|| view! { <Badge value=props.badge.clone() severity=props.badge_severity.clone() pt=BadgePt { root: pt.badge.clone() } /> })
}

/// Instance properties.
struct Props {
    label: OptProp<String>,
    badge: OptProp<String>,
    badge_severity: OptProp<BadgeSeverity>,
    loading: Prop<bool>,
    severity: OptProp<ButtonSeverity>,
    variant: OptProp<ButtonVariant>,
    size: OptProp<Size>,
    icon_pos: Prop<IconPos>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    link: Prop<bool>,
    raised: Prop<bool>,
    rounded: Prop<bool>,
    fluid: Prop<bool>,
    disabled: Prop<bool>,
    button_type: OptProp<String>,
    aria_label: OptProp<String>,
    tabindex: OptProp<i32>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon: Option<ViewFn>,
    loading_icon: Option<ViewFn>,
    has_icon: bool,
    has_children: bool,
}

impl Props {
    /// Unavailable: `disabled` or loading.
    fn is_disabled(&self) -> bool {
        self.disabled.get() || self.loading.get()
    }

    /// Calls `f` with a borrowed state snapshot (no string copies).
    fn with_state<R>(&self, f: impl FnOnce(&ButtonState) -> R) -> R {
        self.label.with(|l| {
            self.badge.with(|b| {
                f(&ButtonState {
                    label: l.as_deref(),
                    has_icon: self.has_icon,
                    badge: b.as_deref(),
                    loading: self.loading.get(),
                    severity: self.severity.get(),
                    variant: self.variant.get(),
                    text: self.text.get(),
                    outlined: self.outlined.get(),
                    plain: self.plain.get(),
                    link: self.link.get(),
                    raised: self.raised.get(),
                    rounded: self.rounded.get(),
                    size: self.size.get(),
                    fluid: self.fluid.get(),
                    icon_pos: self.icon_pos.get(),
                })
            })
        })
    }
}
