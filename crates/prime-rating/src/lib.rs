//! Rating — a port of PrimeNG 21.1.10's `p-rating`.
//!
//! Markup and classes as in PrimeNG: root `div.p-rating` (instead of the `<p-rating>` host), for each star a `div.p-rating-option`
//! with a hidden radio button (`span.p-hidden-accessible > input[type=radio]`) and the `p-rating-icon` icon. The stars are a group of native
//! radio buttons with a shared `name`, so the keyboard arrows work without any code of our own.
//!
//! State is a two-way signal `value: RwSignal<Option<u32>>` (star numbers from 1); without it the component keeps internal
//! state starting from `default_value`. A click on a star selects it, a repeated click on the current value resets it.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_rating::Rating;
//!
//! let value = RwSignal::new(Some(3));
//! let html = view! { <Rating value=value stars=5 /> }.to_html();
//! assert!(html.contains("p-rating-option-active") && html.contains("p-rating-on-icon") && html.contains("p-rating-off-icon"));
//! ```

mod state;

use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_icons::{StarFillIcon, StarIcon};
use prime_rs_core::ids::next_unique;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, use_prime_config};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use state::{RatingState, data_p, next_value, write_classes, write_icon_classes, write_option_classes};

/// Default number of stars.
const DEFAULT_STARS: u32 = 5;

/// Selection event: the new value and the original browser event.
#[derive(Clone, Debug)]
pub struct RatingRate {
    /// New value (`None` means reset).
    pub value: Option<u32>,
    /// Original event (`click` on a star or `change` of a radio button).
    pub original_event: web_sys::Event,
}

/// Icon template context (like `{ $implicit, class }` in PrimeNG).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RatingIcon {
    /// Star number (from 1).
    pub star: u32,
    /// Icon classes: `p-rating-icon p-rating-on-icon` or `p-rating-icon p-rating-off-icon`, `p-invalid`.
    pub class: String,
}

/// Pass-through of the Rating parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RatingPt {
    /// Root element.
    pub root: Pt,
    /// Star option (`div.p-rating-option`).
    pub option: Pt,
    /// Hidden radio button container (`span.p-hidden-accessible`).
    pub hidden_option_input_container: Pt,
    /// Hidden radio button.
    pub hidden_option_input: Pt,
    /// On icon.
    pub on_icon: Pt,
    /// Off icon.
    pub off_icon: Pt,
}

/// Star rating selection.
///
/// Every property accepts a value or a signal; `value` is a two-way signal.
#[component]
pub fn Rating(
    /// Two-way value signal.
    #[prop(optional)]
    value: Option<RwSignal<Option<u32>>>,
    /// Initial value if `value` is not passed.
    #[prop(optional, into)]
    default_value: OptProp<u32>,
    /// Number of stars (5 by default).
    #[prop(optional, into)]
    stars: OptProp<u32>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Required.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// The `name` attribute of the radio buttons (unique by default).
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Icon of an on star instead of the default star (in a `span` with the icon classes).
    #[prop(optional, into)]
    on_icon: Option<ViewFn>,
    /// Icon of an off star instead of the default star (in a `span` with the icon classes).
    #[prop(optional, into)]
    off_icon: Option<ViewFn>,
    /// Replaces the on star icon entirely.
    #[prop(optional, into)]
    on_icon_template: Option<Callback<RatingIcon, AnyView>>,
    /// Replaces the off star icon entirely.
    #[prop(optional, into)]
    off_icon_template: Option<Callback<RatingIcon, AnyView>>,
    /// Called after the user selects or resets.
    #[prop(optional, into)]
    on_rate: Option<Callback<RatingRate>>,
    /// Called when a star gets focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when a star loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<RatingPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let props = Arc::new(Props {
        value: value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked())),
        visible_focus: RwSignal::new(None),
        pointer_focus: AtomicBool::new(false),
        stars,
        readonly,
        disabled,
        invalid,
        required,
        name,
        default_name: format!("pn_id_{}_name", next_unique()),
        style,
        style_class,
        on_icon,
        off_icon,
        on_icon_template,
        off_icon_template,
        on_rate,
        on_focus,
        on_blur,
        label_one: locale.text("aria.star").to_owned(),
        label_many: locale.text("aria.stars").to_owned(),
    });

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &state_of(&props));
            props.style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let data = {
        let props = props.clone();
        move || data_p(&state_of(&props))
    };
    // The list closure reads only `stars`: reading the value here would recreate the stars and drop focus.
    let options = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let count = props.stars.get().unwrap_or(DEFAULT_STARS);
            (1..=count).map(|star| star_view(&props, &pt, star)).collect_view()
        }
    };
    let root_attrs = pt.root.attrs();

    view! {
        <div class={root_class} style={root_style} data-p={data} {..root_attrs}>
            {options}
        </div>
    }
}

/// One star: an option with a hidden radio button and an icon.
fn star_view(props: &Arc<Props>, pt: &Arc<RatingPt>, star: u32) -> impl IntoView + use<> {
    let input_ref = NodeRef::<html::Input>::new();
    let option_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            let active = props.value.get().is_some_and(|v| star <= v);
            write_option_classes(&mut cn, active, props.visible_focus.get() == Some(star));
            cn.add_user(pt.option.extra_class());
            cn.finish_cow()
        }
    };
    let option_style = {
        let pt = pt.clone();
        move || pt.option.has_style().then(|| pt.option.merge_style(""))
    };
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            // A click on the radio button itself (arrows, programmatic `click()`) arrives together with `change`, which `change` handles.
            if e.target().is_some_and(|t| t.is_instance_of::<web_sys::HtmlInputElement>()) {
                return;
            }
            props.choose(star, e.into(), true);
            props.visible_focus.set(None);
            if let Some(input) = input_ref.get() {
                // focus after a click is not considered visible (`isFocusVisibleItem = false` in `onOptionClick`)
                props.pointer_focus.store(true, Ordering::Relaxed);
                let _ = input.focus();
                props.pointer_focus.store(false, Ordering::Relaxed);
            }
        }
    };
    let change = {
        let props = props.clone();
        move |e: web_sys::Event| props.choose(star, e, false)
    };
    let focus = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| {
            if props.readonly.get_untracked() || props.disabled.get_untracked() {
                return;
            }
            let visible = !props.pointer_focus.load(Ordering::Relaxed)
                && input_ref.get().is_some_and(|input| input.matches(":focus-visible").unwrap_or(false));
            props.visible_focus.set(visible.then_some(star));
            if let Some(cb) = props.on_focus {
                cb.run(e);
            }
        }
    };
    let blur = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| {
            props.visible_focus.set(None);
            if let Some(cb) = props.on_blur {
                cb.run(e);
            }
        }
    };
    let input_name = {
        let props = props.clone();
        move || props.name.get().unwrap_or_else(|| props.default_name.clone())
    };
    let checked = {
        let props = props.clone();
        move || props.value.get() == Some(star)
    };
    let required = {
        let props = props.clone();
        move || props.required.get()
    };
    let readonly = {
        let props = props.clone();
        move || props.readonly.get().then_some("")
    };
    let disabled = {
        let props = props.clone();
        move || props.disabled.get()
    };
    let aria_label = props.star_label(star);
    let icon = {
        let (props, pt) = (props.clone(), pt.clone());
        move || icon_of(&props, &pt, star)
    };
    let container_class = pt.hidden_option_input_container.merge_class("p-hidden-accessible");
    // The radio button has no class of its own: `class` is output only from `pt` (as an attribute, otherwise it would be `class=""`).
    let input_pt = match pt.hidden_option_input.extra_class() {
        Some(class) => pt.hidden_option_input.clone().attr("class", class),
        None => pt.hidden_option_input.clone(),
    };
    let (option_attrs, container_attrs, input_attrs) = (pt.option.attrs(), pt.hidden_option_input_container.attrs(), input_pt.attrs());

    view! {
        <div class={option_class} style={option_style} on:click={click} {..option_attrs}>
            <span class={container_class} data-p-hidden-accessible="true" {..container_attrs}>
                <input
                    node_ref={input_ref}
                    type="radio"
                    name={input_name}
                    value={star}
                    checked={checked.clone()}
                    prop:checked={checked}
                    required={required}
                    readonly={readonly}
                    disabled={disabled}
                    aria-label={aria_label}
                    on:focus={focus}
                    on:blur={blur}
                    on:change={change}
                    {..input_attrs}
                />
            </span>
            {icon}
        </div>
    }
}

/// Star icon: the template, then `on_icon`/`off_icon`, then the default star.
fn icon_of(props: &Props, pt: &RatingPt, star: u32) -> AnyView {
    let on = props.value.get().is_some_and(|v| star <= v);
    let mut cn = ClassNames::new();
    write_icon_classes(&mut cn, on, props.invalid.get());
    let (template, custom, icon_pt) =
        if on { (props.on_icon_template, &props.on_icon, &pt.on_icon) } else { (props.off_icon_template, &props.off_icon, &pt.off_icon) };
    let class = icon_pt.merge_class(&cn.finish());
    if let Some(template) = template {
        return template.run(RatingIcon { star, class: class.into_owned() });
    }
    match (custom, on) {
        (Some(icon), _) => view! { <span class={class} {..icon_pt.attrs()}>{icon.run()}</span> }.into_any(),
        (None, true) => view! { <StarFillIcon class=class.into_owned() {..icon_pt.attrs()} /> }.into_any(),
        (None, false) => view! { <StarIcon class=class.into_owned() {..icon_pt.attrs()} /> }.into_any(),
    }
}

fn state_of(props: &Props) -> RatingState {
    RatingState { readonly: props.readonly.get(), disabled: props.disabled.get() }
}

/// Instance properties.
struct Props {
    value: RwSignal<Option<u32>>,
    /// The star with keyboard (visible) focus.
    visible_focus: RwSignal<Option<u32>>,
    /// A programmatic focus after a click on a star is in progress.
    pointer_focus: AtomicBool,
    stars: OptProp<u32>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    required: Prop<bool>,
    name: OptProp<String>,
    default_name: String,
    style: OptProp<String>,
    style_class: OptProp<String>,
    on_icon: Option<ViewFn>,
    off_icon: Option<ViewFn>,
    on_icon_template: Option<Callback<RatingIcon, AnyView>>,
    off_icon_template: Option<Callback<RatingIcon, AnyView>>,
    on_rate: Option<Callback<RatingRate>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    label_one: String,
    label_many: String,
}

impl Props {
    /// A star's `aria-label`: `aria.star` for the first, otherwise `aria.stars` with `{star}` substituted.
    fn star_label(&self, star: u32) -> String {
        if star == 1 { self.label_one.clone() } else { self.label_many.replace("{star}", &star.to_string()) }
    }

    /// Star selection. `toggle` is whether to reset on repeated selection of the current value (click; `change` always brings a new value).
    fn choose(&self, star: u32, original_event: web_sys::Event, toggle: bool) {
        if self.readonly.get_untracked() || self.disabled.get_untracked() {
            return;
        }
        let current = self.value.get_untracked();
        let next = if toggle { next_value(current, star) } else { Some(star) };
        self.value.set(next);
        if let Some(cb) = self.on_rate {
            cb.run(RatingRate { value: next, original_event });
        }
    }
}
