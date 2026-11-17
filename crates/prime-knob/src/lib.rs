//! Knob - port of PrimeNG 21.1.10 `p-knob`.
//!
//! Markup and classes as in PrimeNG: root `div.p-knob` (instead of the `<p-knob>` host), inside `<svg viewBox="0 0 100 100" role="slider">`
//! with `path.p-knob-range` (the whole arc), `path.p-knob-value` (the arc from the scale zero to the value) and `text.p-knob-text`.
//!
//! The value is a two-way signal `value: RwSignal<f64>`; without it the component keeps internal state (`default_value`, `min` by
//! default). The value is changed by clicking the scale, dragging with the mouse or touch, and with the keyboard.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_knob::Knob;
//!
//! let value = RwSignal::new(60.0);
//! let html = view! { <Knob value=value value_template="{value}%" /> }.to_html();
//! assert!(html.contains("p-knob-value") && html.contains("60%") && html.contains(r#"aria-valuenow="60""#));
//! ```

mod state;

use leptos::ev;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use state::{
    MAX_RADIANS, MID, MIN_RADIANS, RADIUS, arc_point, display_text, key_target, map_range, num, range_path, value_from_angle, value_path,
    value_radians, write_classes, zero_radians,
};

/// Default color of the value arc.
const VALUE_COLOR: &str = "var(--p-knob-value-background)";
/// Default color of the background arc.
const RANGE_COLOR: &str = "var(--p-knob-range-background)";
/// Default text color.
const TEXT_COLOR: &str = "var(--p-knob-text-color)";

/// Pass-through for Knob parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KnobPt {
    /// Root element.
    pub root: Pt,
    /// `<svg>` element.
    pub svg: Pt,
    /// Background arc (`path.p-knob-range`).
    pub range: Pt,
    /// Value arc (`path.p-knob-value`).
    pub value: Pt,
    /// Value text (`text.p-knob-text`).
    pub text: Pt,
}

/// Circular number input.
///
/// Each property accepts a value or a signal; `value` is a two-way signal.
#[component]
pub fn Knob(
    /// Two-way value signal.
    #[prop(optional)]
    value: Option<RwSignal<f64>>,
    /// Initial value if `value` is not passed (`min` by default).
    #[prop(optional, into)]
    default_value: OptProp<f64>,
    /// Lower bound (0 by default).
    #[prop(optional, into)]
    min: OptProp<f64>,
    /// Upper bound (100 by default).
    #[prop(optional, into)]
    max: OptProp<f64>,
    /// Value step for pointer input (1 by default).
    #[prop(optional, into)]
    step: OptProp<f64>,
    /// Size in pixels (100 by default).
    #[prop(optional, into)]
    size: OptProp<u32>,
    /// Arc stroke width (14 by default).
    #[prop(optional, into)]
    stroke_width: OptProp<f64>,
    /// Show the value inside (`true` by default).
    #[prop(optional, into)]
    show_value: OptProp<bool>,
    /// Text template; `{value}` is replaced by the value (`{value}` by default).
    #[prop(optional, into)]
    value_template: OptProp<String>,
    /// Value arc color.
    #[prop(optional, into)]
    value_color: OptProp<String>,
    /// Background arc color.
    #[prop(optional, into)]
    range_color: OptProp<String>,
    /// Text color.
    #[prop(optional, into)]
    text_color: OptProp<String>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Required.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// `name` attribute of the value text.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// `tabindex` attribute (0 by default; `-1` with `readonly` and `disabled`).
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
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called on every value change by the user.
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,
    /// Pass-through for parts.
    #[prop(optional)]
    pt: Option<KnobPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let initial = default_value.get_untracked().unwrap_or_else(|| min.get_untracked().unwrap_or(0.0));
    let props = Arc::new(Props {
        value: value.unwrap_or_else(|| RwSignal::new(initial)),
        min,
        max,
        step,
        size,
        stroke_width,
        show_value,
        value_template,
        value_color,
        range_color,
        text_color,
        readonly,
        disabled,
        required,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        style,
        style_class,
        on_change,
        moved: AtomicBool::new(false),
        svg_ref: NodeRef::new(),
        listeners: StoredValue::new_local(Vec::new()),
    });
    // `window` listeners live only while pressed; they are removed on unmount.
    {
        let props = props.clone();
        on_cleanup(move || props.unbind_window());
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, props.disabled.get());
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
    let svg_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let size = props.size.get().unwrap_or(100);
            pt.svg.merge_style_parts(&[&format!("width: {size}px; height: {size}px")])
        }
    };
    let svg_class = pt.svg.extra_class().map(str::to_owned);
    let (min_attr, max_attr, now) = (
        {
            let props = props.clone();
            move || num(props.min_t())
        },
        {
            let props = props.clone();
            move || num(props.max_t())
        },
        {
            let props = props.clone();
            move || num(props.value.get())
        },
    );
    let required = {
        let props = props.clone();
        move || props.required.get().then_some("")
    };
    let aria_label = {
        let props = props.clone();
        move || props.aria_label.get()
    };
    let aria_labelledby = {
        let props = props.clone();
        move || props.aria_labelledby.get()
    };
    let tabindex = {
        let props = props.clone();
        move || if props.blocked() { -1 } else { props.tabindex.get().unwrap_or(0) }
    };
    let range_d = range_path();
    let range_stroke = {
        let props = props.clone();
        move || props.range_color.get().unwrap_or_else(|| RANGE_COLOR.to_owned())
    };
    let value_stroke = {
        let props = props.clone();
        move || props.value_color.get().unwrap_or_else(|| VALUE_COLOR.to_owned())
    };
    let stroke_width = {
        let props = props.clone();
        move || num(props.stroke_width.get().unwrap_or(14.0))
    };
    let stroke_width_value = stroke_width.clone();
    let value_d = {
        let props = props.clone();
        move || value_path(props.value.get(), props.min_t(), props.max_t())
    };
    let (range_class, value_class) = (pt.range.merge_class("p-knob-range"), pt.value.merge_class("p-knob-value"));
    let text = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            props.show_value.get().unwrap_or(true).then(|| {
                let fill = props.text_color.get().unwrap_or_else(|| TEXT_COLOR.to_owned());
                let label = props.value_template.with(|t| display_text(t.as_deref().unwrap_or("{value}"), props.value.get()));
                view! {
                    <text x="50" y="57" text-anchor="middle" fill={fill} class={pt.text.merge_class("p-knob-text")} name={props.name.get()} {..pt.text.attrs()}>
                        {label}
                    </text>
                }
            })
        }
    };
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.click(e)
    };
    let key_down = {
        let props = props.clone();
        move |e: web_sys::KeyboardEvent| props.key_down(e)
    };
    let mouse_down = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.mouse_down(e)
    };
    let touch_start = {
        let props = props.clone();
        move |e: web_sys::TouchEvent| props.touch_start(e)
    };
    let (root_attrs, svg_attrs, range_attrs, value_attrs) = (pt.root.attrs(), pt.svg.attrs(), pt.range.attrs(), pt.value.attrs());
    let svg_ref = props.svg_ref;

    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            <svg
                node_ref={svg_ref}
                viewBox="0 0 100 100"
                role="slider"
                class={svg_class}
                style={svg_style}
                aria-valuemin={min_attr}
                aria-valuemax={max_attr}
                aria-valuenow={now}
                aria-labelledby={aria_labelledby}
                aria-label={aria_label}
                required={required}
                tabindex={tabindex}
                on:click={click}
                on:keydown={key_down}
                on:mousedown={mouse_down}
                on:touchstart={touch_start}
                {..svg_attrs}
            >
                <path d={range_d} stroke-width={stroke_width} stroke={range_stroke} class={range_class} {..range_attrs}></path>
                <path d={value_d} stroke-width={stroke_width_value} stroke={value_stroke} class={value_class} {..value_attrs}></path>
                {text}
            </svg>
        </div>
    }
}

/// Properties and state of the instance.
struct Props {
    value: RwSignal<f64>,
    min: OptProp<f64>,
    max: OptProp<f64>,
    step: OptProp<f64>,
    size: OptProp<u32>,
    stroke_width: OptProp<f64>,
    show_value: OptProp<bool>,
    value_template: OptProp<String>,
    value_color: OptProp<String>,
    range_color: OptProp<String>,
    text_color: OptProp<String>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    required: Prop<bool>,
    name: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    on_change: Option<Callback<f64>>,
    /// While pressed, the value was changed by the pointer: the next `click` does not repeat the change.
    moved: AtomicBool,
    svg_ref: NodeRef<leptos::svg::Svg>,
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>, LocalStorage>,
}

impl Props {
    fn min_t(&self) -> f64 {
        self.min.get().unwrap_or(0.0)
    }

    fn max_t(&self) -> f64 {
        self.max.get().unwrap_or(100.0)
    }

    fn bounds(&self) -> (f64, f64) {
        (self.min.get_untracked().unwrap_or(0.0), self.max.get_untracked().unwrap_or(100.0))
    }

    /// Read-only or disabled: input is blocked.
    fn blocked(&self) -> bool {
        self.readonly.get() || self.disabled.get()
    }

    fn blocked_untracked(&self) -> bool {
        self.readonly.get_untracked() || self.disabled.get_untracked()
    }

    /// Writes the value and calls `on_change` (on every application, like PrimeNG `updateModelValue`).
    fn apply(&self, value: f64) {
        self.value.set(value);
        if let Some(cb) = self.on_change {
            cb.run(value);
        }
    }

    /// Value from the client coordinates of the pointer (`updateValue`); returns whether it was applied.
    fn update_from_pointer(&self, client_x: f64, client_y: f64) -> bool {
        let Some(svg) = self.svg_ref.get_untracked() else { return false };
        let rect = svg.unchecked_ref::<web_sys::Element>().get_bounding_client_rect();
        let (dx, dy) = (client_x - rect.left() - rect.width() / 2.0, rect.height() / 2.0 - (client_y - rect.top()));
        let (min, max) = self.bounds();
        match value_from_angle(dy.atan2(dx), min, max, self.step.get_untracked().unwrap_or(1.0)) {
            Some(value) => {
                self.apply(value);
                true
            }
            None => false,
        }
    }

    fn click(&self, e: web_sys::MouseEvent) {
        if self.blocked_untracked() {
            return;
        }
        // a `click` after dragging would only repeat the change
        if !self.moved.swap(false, Ordering::Relaxed) {
            self.update_from_pointer(e.client_x() as f64, e.client_y() as f64);
        }
    }

    fn key_down(&self, e: web_sys::KeyboardEvent) {
        if self.blocked_untracked() {
            return;
        }
        let (min, max) = self.bounds();
        if let Some(value) = key_target(&e.code(), self.value.get_untracked(), min, max) {
            e.prevent_default();
            self.apply(value);
        }
    }

    fn focus(&self) {
        if let Some(svg) = self.svg_ref.get_untracked() {
            let _ = svg.unchecked_ref::<web_sys::HtmlElement>().focus();
        }
    }

    fn mouse_down(self: &Arc<Self>, e: web_sys::MouseEvent) {
        if self.blocked_untracked() {
            return;
        }
        self.moved.store(false, Ordering::Relaxed);
        self.unbind_window();
        let (on_move, on_up) = (self.clone(), self.clone());
        let move_handle = window_event_listener(ev::mousemove, move |e: web_sys::MouseEvent| {
            if !on_move.blocked_untracked() && on_move.update_from_pointer(e.client_x() as f64, e.client_y() as f64) {
                on_move.moved.store(true, Ordering::Relaxed);
                e.prevent_default();
            }
        });
        let up_handle = window_event_listener(ev::mouseup, move |e: web_sys::MouseEvent| {
            on_up.unbind_window();
            e.prevent_default();
        });
        self.listeners.update_value(|l| l.extend([move_handle, up_handle]));
        self.focus();
        e.prevent_default();
    }

    fn touch_start(self: &Arc<Self>, e: web_sys::TouchEvent) {
        if self.blocked_untracked() {
            return;
        }
        self.unbind_window();
        let (on_move, on_end) = (self.clone(), self.clone());
        let move_handle = window_event_listener(ev::touchmove, move |e: web_sys::TouchEvent| {
            if !on_move.blocked_untracked()
                && e.touches().length() == 1
                && let Some(touch) = e.target_touches().get(0)
            {
                on_move.update_from_pointer(touch.client_x() as f64, touch.client_y() as f64);
            }
        });
        let end_handle = window_event_listener(ev::touchend, move |e: web_sys::TouchEvent| {
            on_end.unbind_window();
            e.prevent_default();
        });
        self.listeners.update_value(|l| l.extend([move_handle, end_handle]));
        e.prevent_default();
    }

    fn unbind_window(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
    }
}
