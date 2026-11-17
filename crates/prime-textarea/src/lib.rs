//! Textarea — port of the `pTextarea` directive from PrimeNG 21.1.10.
//!
//! In PrimeNG this is a directive on a native `<textarea>`; here the component renders a `<textarea>` with the same classes
//! (`p-textarea p-component`, `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, sizes, `p-invalid`).
//!
//! The value is an optional two-way `value: RwSignal<String>`: during SSR it is rendered as the child text of `<textarea>`, on the client
//! it is updated through `prop:value`. Without `value` the field is uncontrolled, and `p-filled` is not tracked.
//!
//! `auto_resize` adjusts the height by `scrollHeight` (taking the inline `max-height` into account) in the browser; `on_resize` is called after the adjustment.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_textarea::Textarea;
//!
//! let value = RwSignal::new("line".to_owned());
//! let html = view! { <Textarea value=value rows=3u32 auto_resize=true /> }.to_html();
//! assert!(html.contains("p-textarea-resizable") && html.contains("p-filled") && html.contains(r#"rows="3""#) && html.contains(">line</textarea>"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{Fit, TextareaState, fit_height, write_classes};

/// Height adjustment to the content (browser only).
#[cfg(target_arch = "wasm32")]
fn resize(el: &web_sys::HtmlTextAreaElement, on_resize: Option<Callback<Option<web_sys::Event>>>, event: Option<web_sys::Event>) {
    let style = web_sys::HtmlElement::style(AsRef::<web_sys::HtmlElement>::as_ref(el));
    let _ = style.set_property("height", "auto");
    let max = style.get_property_value("max-height").ok().and_then(|v| v.trim().trim_end_matches("px").parse::<f64>().ok());
    let fit = fit_height(f64::from(el.scroll_height()), max);
    let _ = style.set_property("height", &format!("{}px", fit.height));
    if fit.scroll {
        let _ = style.set_property("overflow-y", "scroll");
    } else {
        let _ = style.set_property("overflow", "hidden");
    }
    if let Some(cb) = on_resize {
        cb.run(event);
    }
}

/// Multi-line input field.
///
/// Each property accepts a value or a signal. `value` is a two-way binding (`RwSignal<String>`).
#[component]
pub fn Textarea(
    /// Two-way bound value; without it the field is uncontrolled.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// `id` attribute.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `name` attribute.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Hint in the empty field.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Number of rows (`rows`).
    #[prop(optional, into)]
    rows: OptProp<u32>,
    /// Number of columns (`cols`).
    #[prop(optional, into)]
    cols: OptProp<u32>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-describedby` attribute.
    #[prop(optional, into)]
    aria_describedby: OptProp<String>,
    /// Unavailable.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Required.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Maximum length (`maxlength`).
    #[prop(optional, into)]
    maxlength: OptProp<i32>,
    /// Automatic height adjustment to the content.
    #[prop(optional)]
    auto_resize: bool,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Variant; if not set, `PrimeConfig.input_variant` is used.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Full width of the container.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called on input, after the value is written to `value`.
    #[prop(optional, into)]
    on_input: Option<Callback<web_sys::Event>>,
    /// Called when the value is committed (focus lost).
    #[prop(optional, into)]
    on_change: Option<Callback<web_sys::Event>>,
    /// Called when focus is received.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when focus is lost.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Called after the height adjustment (`auto_resize`); the event is passed if the adjustment was caused by input.
    #[prop(optional, into)]
    on_resize: Option<Callback<Option<web_sys::Event>>>,
    /// Pass-through of the root.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let is_static = all_static!(
        id,
        name,
        placeholder,
        rows,
        cols,
        aria_label,
        aria_describedby,
        disabled,
        readonly,
        required,
        maxlength,
        size,
        variant,
        invalid,
        fluid,
        style,
        style_class
    ) && value.is_none();
    let props = Props {
        value,
        auto_resize,
        size,
        variant,
        invalid,
        fluid,
        style,
        style_class,
        id,
        name,
        placeholder,
        rows,
        cols,
        aria_label,
        aria_describedby,
        disabled,
        readonly,
        required,
        maxlength,
        config_variant: use_prime_config().input_variant,
    };
    let attrs = pt.attrs();

    // `NodeRef` and `Effect` are needed only by the client with `auto_resize`; during SSR they are absent altogether.
    #[cfg(target_arch = "wasm32")]
    let node = {
        let node = NodeRef::<leptos::html::Textarea>::new();
        if auto_resize {
            Effect::new(move |_| {
                if let Some(v) = value {
                    v.track();
                }
                if let Some(el) = node.get() {
                    resize(&el, on_resize, None);
                }
            });
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Textarea, _>(node)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let node = {
        let _ = on_resize;
    };

    let input = move |e: web_sys::Event| {
        if let Some(sig) = value {
            sig.set(event_target_value(&e));
        } else {
            #[cfg(target_arch = "wasm32")]
            if auto_resize {
                resize(&event_target::<web_sys::HtmlTextAreaElement>(&e), on_resize, Some(e.clone()));
            }
        }
        if let Some(cb) = on_input {
            cb.run(e);
        }
    };
    let change = move |e: web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(e);
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

    if is_static {
        let (class, style) = (class_of(&props, &pt), style_of(&props, &pt));
        let (id, name, placeholder, rows, cols) =
            (props.id.get(), props.name.get(), props.placeholder.get(), props.rows.get(), props.cols.get());
        let (aria_label, aria_describedby) = (props.aria_label.get(), props.aria_describedby.get());
        let (disabled, readonly, required, maxlength) =
            (props.disabled.get(), props.readonly.get(), props.required.get(), props.maxlength.get());
        let (current, initial) = (None::<String>, None::<String>);
        Either::Left(textarea_markup!(
            class,
            style,
            id,
            name,
            placeholder,
            rows,
            cols,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            initial,
            node,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        macro_rules! reactive {
            ($($field:ident),+) => {
                $(let $field = { let props = props.clone(); move || props.$field.get() };)+
            };
        }
        reactive!(id, name, placeholder, rows, cols, aria_label, aria_describedby, disabled, readonly, required, maxlength);
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let initial = value.map(|v| v.with_untracked(|s| initial_text(s)));
        let current = move || props.value.map(|v| v.get());
        Either::Right(textarea_markup!(
            class,
            style,
            id,
            name,
            placeholder,
            rows,
            cols,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            initial,
            node,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    }
}

/// Textarea markup: one for the static and reactive branches (only the attribute expressions differ).
macro_rules! textarea_markup {
    (
        $class:expr, $style:expr, $id:expr, $name:expr, $placeholder:expr, $rows:expr, $cols:expr, $aria_label:expr, $aria_describedby:expr,
        $disabled:expr, $readonly:expr, $required:expr, $maxlength:expr, $current:expr, $initial:expr, $node:expr, $input:expr, $change:expr,
        $focus:expr, $blur:expr, $attrs:expr
    ) => {
        view! {
            <textarea
                id={$id}
                name={$name}
                placeholder={$placeholder}
                rows={$rows}
                cols={$cols}
                aria-label={$aria_label}
                aria-describedby={$aria_describedby}
                disabled={$disabled}
                readonly={$readonly}
                required={$required}
                maxlength={$maxlength}
                class={$class}
                style={$style}
                prop:value={$current}
                on:input={$input}
                on:change={$change}
                on:focus={$focus}
                on:blur={$blur}
                {..$attrs}
                {..$node}
            >
                {$initial}
            </textarea>
        }
    };
}
use textarea_markup;

/// Initial text of the `<textarea>` for SSR and hydration.
///
/// Leptos renders the children of `<textarea>` without escaping (like `<script>`), so `&`, `<` and `>` are escaped here, otherwise a value like
/// `</textarea><script>` would break the markup. The HTML parser drops the first line break inside `<textarea>`, so a leading line break of the value is protected by an extra one.
/// In a pure client build (CSR) the text stays as is: the field value is set by `prop:value`.
fn initial_text(value: &str) -> String {
    if cfg!(all(target_arch = "wasm32", not(feature = "hydrate"))) {
        return value.to_owned();
    }
    let mut out = String::with_capacity(value.len() + 8);
    if value.starts_with('\n') || value.starts_with("\r\n") {
        out.push('\n');
    }
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

fn state_of(props: &Props) -> TextareaState {
    TextareaState {
        filled: props.value.is_some_and(|v| v.with(|s| !s.is_empty())),
        auto_resize: props.auto_resize,
        variant: props.variant.get().or(props.config_variant),
        fluid: props.fluid.get(),
        size: props.size.get(),
        invalid: props.invalid.get(),
    }
}

fn class_of(props: &Props, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &Pt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.has_style()).then(|| pt.merge_style(st.as_deref().unwrap_or(""))))
}

/// Instance properties.
struct Props {
    value: Option<RwSignal<String>>,
    auto_resize: bool,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    invalid: Prop<bool>,
    fluid: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    id: OptProp<String>,
    name: OptProp<String>,
    placeholder: OptProp<String>,
    rows: OptProp<u32>,
    cols: OptProp<u32>,
    aria_label: OptProp<String>,
    aria_describedby: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    required: Prop<bool>,
    maxlength: OptProp<i32>,
    config_variant: Option<Variant>,
}
