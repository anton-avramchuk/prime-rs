//! InputMask — port of `p-inputmask` from PrimeNG 21.1.10.
//!
//! Masked input field: `9` — digit, `a` — letter (`character_pattern`), `*` — letter or digit, `?` — start of an optional
//! part, other characters are constants. The value is a `RwSignal<String>`: the masked text or (with `unmask`) only the entered
//! characters. The mask logic is the pure engine [`Mask`] (buffer, character checks, shifts on input and deletion).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inputmask::InputMask;
//!
//! let value = RwSignal::new("5551234567".to_owned());
//! let html = Owner::new().with(|| view! { <InputMask mask="(999) 999-9999" value=value /> }.to_html());
//! assert!(html.contains("p-inputmask") && html.contains("value=\"(555) 123-4567\""));
//! ```

mod mask;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::prelude::*;
use prime_icons::TimesIcon;
use prime_inputtext::{InputTextState, write_classes as input_classes};
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use std::sync::Arc;

pub use mask::{CharTest, Deleted, Focused, Mask, Typed, ascii_letters};

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Pass-through for the parts of [`InputMask`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputMaskPt {
    /// Root.
    pub root: Pt,
    /// Input field.
    pub input: Pt,
    /// Clear icon.
    pub clear_icon: Pt,
}

/// State shared by the handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    mask: StoredValue<Mask>,
    mask_text: Prop<String>,
    value: RwSignal<String>,
    focus_text: StoredValue<String>,
    /// Input has already happened after focus: the deferred buffer and caret write is not needed.
    touched: StoredValue<bool>,
    unmask: Prop<bool>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    slot_char: Prop<String>,
    auto_clear: Prop<bool>,
    keep_buffer: Prop<bool>,
    alpha: CharTest,
    input_ref: NodeRef<html::Input>,
    on_complete: Option<Callback<()>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    on_input: Option<Callback<()>>,
    on_clear: Option<Callback<()>>,
    on_unmasked_change: Option<Callback<String>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn text(&self) -> String {
        self.input_ref.get_untracked().map(|i| i.value()).unwrap_or_default()
    }

    fn set_text(&self, text: &str) {
        if let Some(i) = self.input_ref.get_untracked() {
            i.set_value(text);
        }
    }

    fn caret(&self) -> (usize, usize) {
        self.input_ref
            .get_untracked()
            .map(|i| (i.selection_start().ok().flatten().unwrap_or(0) as usize, i.selection_end().ok().flatten().unwrap_or(0) as usize))
            .unwrap_or((0, 0))
    }

    /// The caret is set only on the field that is currently focused (`caret()` of the original).
    fn set_caret(&self, begin: usize, end: usize) {
        #[cfg(target_arch = "wasm32")]
        if let Some(i) = self.input_ref.get_untracked() {
            let node: &web_sys::Node = i.unchecked_ref();
            let focused =
                web_sys::window().and_then(|w| w.document()).and_then(|d| d.active_element()).is_some_and(|a| a.is_same_node(Some(node)));
            if focused {
                let _ = i.set_selection_range(begin as u32, end as u32);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (begin, end);
    }

    /// `updateModel`: the value from the field (or the entered characters with `unmask`).
    fn update_model(&self) {
        let updated = if self.unmask.get_untracked() { self.mask.with_value(Mask::unmasked) } else { self.text() };
        if self.value.get_untracked() != updated {
            self.value.set(updated);
        }
    }

    fn emit_unmasked(&self) {
        if let Some(cb) = self.on_unmasked_change {
            cb.run(self.mask.with_value(Mask::unmasked));
        }
    }

    fn apply(&self, text: Option<String>, caret: Option<usize>) {
        if let Some(t) = text {
            self.set_text(&t);
        }
        if let Some(c) = caret {
            self.set_caret(c, c);
        }
    }

    fn focus(self: &Arc<Self>, e: web_sys::FocusEvent) {
        if self.readonly.get_untracked() {
            return;
        }
        let text = self.text();
        self.focus_text.set_value(text.clone());
        self.touched.set_value(false);
        let res = self.mask.update_value_return(|m| m.focus(&text));
        self.set_text(&res.text);
        #[cfg(target_arch = "wasm32")]
        {
            let this = self.clone();
            set_timeout(
                move || {
                    if this.touched.get_value() {
                        return;
                    }
                    this.set_text(&res.buffer_text);
                    this.set_caret(res.begin, res.end);
                },
                Duration::from_millis(10),
            );
        }
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn blur(&self, e: Option<web_sys::FocusEvent>) {
        let text = self.text();
        let finished = self.mask.update_value_return(|m| m.blur(&text));
        self.set_text(&finished);
        if let (Some(cb), Some(e)) = (self.on_blur, e) {
            cb.run(e);
        }
        self.update_model();
    }

    fn key_down(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        if self.readonly.get_untracked() || self.disabled.get_untracked() || e.is_composing() {
            return;
        }
        self.touched.set_value(true);
        let key = e.key();
        match key.as_str() {
            "Backspace" | "Delete" => {
                let (begin, end) = self.caret();
                let deleted = self.mask.update_value_return(|m| m.delete(begin, end, key == "Delete"));
                self.apply(deleted.text, deleted.caret);
                self.update_model();
                self.emit_unmasked();
                if let Some(cb) = self.on_input {
                    cb.run(());
                }
                e.prevent_default();
            }
            "Enter" => {
                self.blur(None);
                self.update_model();
            }
            "Escape" => {
                let focus_text = self.focus_text.get_value();
                let (text, pos) = self.mask.update_value_return(|m| m.check_val(&focus_text, false));
                self.set_text(&text);
                self.set_caret(0, pos);
                self.update_model();
                e.prevent_default();
            }
            _ => {
                if e.ctrl_key() || e.alt_key() || e.meta_key() || key.chars().count() != 1 {
                    return;
                }
                let ch = key.chars().next().unwrap_or(' ');
                let (begin, end) = self.caret();
                let typed = self.mask.update_value_return(|m| m.key_press(ch, begin, end));
                self.apply(typed.text, typed.caret);
                if typed.accepted {
                    if let Some(cb) = self.on_input {
                        cb.run(());
                    }
                    self.emit_unmasked();
                }
                e.prevent_default();
                self.update_model();
                if typed.completed
                    && let Some(cb) = self.on_complete
                {
                    cb.run(());
                }
            }
        }
    }

    /// Input not via keys (paste, drag and drop, IME): the value is laid out by the mask.
    fn input_changed(self: &Arc<Self>) {
        if self.readonly.get_untracked() {
            return;
        }
        self.touched.set_value(true);
        #[cfg(target_arch = "wasm32")]
        {
            let this = self.clone();
            set_timeout(
                move || {
                    let text = this.text();
                    let (checked, pos) = this.mask.update_value_return(|m| m.check_val(&text, true));
                    this.set_text(&checked);
                    this.set_caret(pos, pos);
                    this.update_model();
                    this.emit_unmasked();
                    if this.mask.with_value(Mask::is_completed)
                        && let Some(cb) = this.on_complete
                    {
                        cb.run(());
                    }
                },
                Duration::ZERO,
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = self;
    }

    fn clear(&self) {
        self.mask.update_value(|m| {
            m.check_val("", true);
        });
        self.set_text("");
        self.value.set(String::new());
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }

    /// Rebuilds the mask when its properties change and clears the value (like the `mask` setter of the original).
    fn rebuild(&self) {
        let m = Mask::new(
            &self.mask_text.get_untracked(),
            &self.slot_char.get_untracked(),
            self.auto_clear.get_untracked(),
            self.keep_buffer.get_untracked(),
            self.alpha.clone(),
        );
        self.mask.set_value(m);
        self.set_text("");
        self.value.set(String::new());
    }
}

trait UpdateReturn<T> {
    fn update_value_return<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
}

impl<T: Send + Sync + 'static> UpdateReturn<T> for StoredValue<T> {
    fn update_value_return<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut out = None;
        self.update_value(|v| out = Some(f(v)));
        out.expect("StoredValue::update_value calls the closure")
    }
}

/// Masked input field.
///
/// Properties accept a value or a signal.
#[component]
pub fn InputMask(
    /// Mask: `9` — digit, `a` — letter, `*` — letter or digit, `?` — start of an optional part.
    #[prop(into)]
    mask: Prop<String>,
    /// Value: the masked text or (with `unmask`) only the entered characters.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Placeholder character for empty positions.
    #[prop(optional, default = Prop::Static("_".to_owned()), into)]
    slot_char: Prop<String>,
    /// Clear an incomplete value on blur.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_clear: Prop<bool>,
    /// Clear icon.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// The value contains only the entered characters.
    #[prop(optional, into)]
    unmask: Prop<bool>,
    /// Keep the buffer on blur and do not overwrite the entered text.
    #[prop(optional, into)]
    keep_buffer: Prop<bool>,
    /// "Letter" predicate for the `a` and `*` templates (default: ASCII letters).
    #[prop(optional)]
    character_pattern: Option<CharTest>,
    /// Input type.
    #[prop(optional, into)]
    input_type: OptProp<String>,
    /// Placeholder of the empty field.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Unavailable.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Style variant.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Full width.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// `id` attribute of the field.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Tab order.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// `title` attribute.
    #[prop(optional, into)]
    title: OptProp<String>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// `aria-required` attribute.
    #[prop(optional, into)]
    aria_required: Prop<bool>,
    /// `autocomplete` attribute.
    #[prop(optional, into)]
    autocomplete: OptProp<String>,
    /// `id` attribute of the root.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the field.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<InputMaskPt>,
    /// The required part of the mask is filled.
    #[prop(optional)]
    on_complete: Option<Callback<()>>,
    /// The field received focus.
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// The field lost focus.
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// A character was entered or deleted.
    #[prop(optional)]
    on_input: Option<Callback<()>>,
    /// The value was cleared by the icon.
    #[prop(optional)]
    on_clear: Option<Callback<()>>,
    /// The entered characters changed (without constants).
    #[prop(optional)]
    on_unmasked_change: Option<Callback<String>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let alpha = character_pattern.unwrap_or_else(ascii_letters);
    let fluid = prime_rs_core::fluid::fluid_prop(fluid);
    let value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let mut engine = Mask::new(
        &mask.get_untracked(),
        &slot_char.get_untracked(),
        auto_clear.get_untracked(),
        keep_buffer.get_untracked(),
        alpha.clone(),
    );
    let initial = {
        let v = value.get_untracked();
        if v.is_empty() { String::new() } else { engine.check_val(&v, false).0 }
    };
    let core = Arc::new(Core {
        mask: StoredValue::new(engine),
        mask_text: mask.clone(),
        value,
        focus_text: StoredValue::new(initial.clone()),
        touched: StoredValue::new(false),
        unmask: unmask.clone(),
        readonly: readonly.clone(),
        disabled: disabled.clone(),
        slot_char: slot_char.clone(),
        auto_clear: auto_clear.clone(),
        keep_buffer: keep_buffer.clone(),
        alpha,
        input_ref: NodeRef::new(),
        on_complete,
        on_focus,
        on_blur,
        on_input,
        on_clear,
        on_unmasked_change,
    });
    #[cfg(target_arch = "wasm32")]
    {
        // A value set from outside is laid out by the mask; our own changes (the field already matches) are skipped.
        let c = core.clone();
        Effect::new(move |_| {
            let v = c.value.get();
            let current = if c.unmask.get_untracked() { c.mask.with_value(Mask::unmasked) } else { c.text() };
            if v != current {
                let text = c.mask.update_value_return(|m| {
                    if v.is_empty() {
                        m.check_val("", true);
                        String::new()
                    } else {
                        m.check_val(&v, false).0
                    }
                });
                c.set_text(&text);
            }
        });
        // Changing the mask or its settings clears the value.
        let c = core.clone();
        let first = StoredValue::new(true);
        Effect::new(move |_| {
            let _ = (c.mask_text.get(), c.slot_char.get(), c.auto_clear.get(), c.keep_buffer.get());
            if first.get_value() {
                first.set_value(false);
            } else {
                c.rebuild();
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &core.alpha;

    let filled = {
        let value = core.value;
        move || value.with(|v| !v.is_empty())
    };
    let root_class = {
        let (pt, variant) = (pt.clone(), variant.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-inputmask p-component p-inputwrapper")
                .add_if(variant.get().or(use_prime_config().input_variant) == Some(Variant::Filled), "p-variant-filled");
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (pt, style, fluid) = (pt.clone(), style.clone(), fluid.clone());
        move || {
            let fluid_style = if fluid.get() { " width: 100%;" } else { "" };
            let st = style.with(|s| {
                pt.root
                    .merge_style(&format!("position: relative;{fluid_style}{}", s.as_deref().map(|s| format!(" {s}")).unwrap_or_default()))
            });
            (!st.is_empty()).then_some(st)
        }
    };
    let input_class = {
        let (pt, filled, invalid, variant, size, fluid, style_class) =
            (pt.clone(), filled, invalid.clone(), variant.clone(), size.clone(), fluid.clone(), style_class.clone());
        move || {
            let variant = variant.get().or(use_prime_config().input_variant);
            let mut cn = ClassNames::new();
            input_classes(
                &mut cn,
                &InputTextState { filled: filled(), size: size.get(), invalid: invalid.get(), variant, fluid: fluid.get() },
            );
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.input.extra_class());
            cn.finish_cow()
        }
    };
    let attrs: Vec<AnyAttribute> = {
        let attr = |name: &'static str, f: Box<dyn Fn() -> Option<String> + Send + Sync>| custom_attribute(name, f).into_any_attr();
        let (
            disabled,
            readonly,
            input_type,
            input_id,
            placeholder,
            title,
            autocomplete,
            tabindex,
            aria_label,
            aria_labelledby,
            aria_required,
        ) = (
            disabled.clone(),
            readonly.clone(),
            input_type.clone(),
            input_id.clone(),
            placeholder.clone(),
            title.clone(),
            autocomplete.clone(),
            tabindex.clone(),
            aria_label.clone(),
            aria_labelledby.clone(),
            aria_required.clone(),
        );
        vec![
            attr("type", Box::new(move || Some(input_type.get().unwrap_or_else(|| "text".to_owned())))),
            attr("id", Box::new(move || input_id.get())),
            attr("placeholder", Box::new(move || placeholder.get())),
            attr("title", Box::new(move || title.get())),
            attr("autocomplete", Box::new(move || autocomplete.get())),
            attr("aria-label", Box::new(move || aria_label.get())),
            attr("aria-labelledby", Box::new(move || aria_labelledby.get())),
            attr("aria-required", Box::new(move || aria_required.get().then(|| "true".to_owned()))),
            attr("readonly", Box::new(move || readonly.get().then(String::new))),
            attr("disabled", {
                let disabled = disabled.clone();
                Box::new(move || disabled.get().then(String::new))
            }),
            attr("tabindex", Box::new(move || tabindex.get().map(|t| t.to_string()))),
        ]
    };
    let clear = {
        let (core, pt, filled, disabled, show_clear) = (core.clone(), pt.clone(), filled, disabled.clone(), show_clear.clone());
        move || {
            (filled() && !disabled.get() && show_clear.get()).then(|| {
                let core = core.clone();
                let class = pt.clear_icon.merge_class("p-inputmask-clear-icon").into_owned();
                view! { <TimesIcon class={class} on:click={move |_| core.clear()} {..pt.clear_icon.attrs()} /> }
            })
        }
    };
    let (focus_core, blur_core, key_core, input_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let (root_attrs, input_attrs, input_ref) = (pt.root.attrs(), pt.input.attrs(), core.input_ref);
    view! {
        <span id={move || id.get()} class={root_class} style={root_style} {..root_attrs}>
            <input
                node_ref={input_ref}
                class={input_class}
                value={initial}
                on:focus={move |e| focus_core.focus(e)}
                on:blur={move |e| blur_core.blur(Some(e))}
                on:keydown={move |e| key_core.key_down(&e)}
                on:input={move |_| input_core.input_changed()}
                {..attrs}
                {..input_attrs}
            />
            {clear}
        </span>
    }
}
