//! ColorPicker — port of `p-colorpicker` from PrimeNG 21.1.10.
//!
//! Color selection: a swatch button opens a panel with a saturation/brightness field and a hue scale (or the panel is embedded,
//! `inline`). The value is a [`ColorValue`] in `hex`, `rgb` or `hsb` format. The color math lives in [`color`].
//!
//! ```
//! use leptos::prelude::*;
//! use prime_colorpicker::ColorPicker;
//!
//! let html = view! { <ColorPicker /> }.to_html();
//! assert!(html.contains("p-colorpicker-preview"));
//! // the panel is closed: it is not in the markup
//! assert!(!html.contains("p-colorpicker-panel"));
//! ```

pub mod color;

use color::{Hsb, Rgb, field_handle, hex_to_hsb, hsb_to_hex, hsb_to_rgb, hue_handle, pick_field, pick_hue, rgb_to_hsb, validate_hsb};
use leptos::leptos_dom::helpers::WindowListenerHandle;
use leptos::prelude::*;
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::set_timeout_with_handle;
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use prime_rs_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Value format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorFormat {
    /// String `#rrggbb`.
    #[default]
    Hex,
    /// Channels `r`, `g`, `b`.
    Rgb,
    /// Hue, saturation, brightness.
    Hsb,
}

/// Color value.
#[derive(Clone, Debug, PartialEq)]
pub enum ColorValue {
    /// String `#rrggbb` (without `#` is also accepted on input).
    Hex(String),
    /// RGB channels.
    Rgb(Rgb),
    /// Hue, saturation, brightness.
    Hsb(Hsb),
}

impl ColorValue {
    /// Value in HSB, as the panel sees it.
    pub fn to_hsb(&self) -> Hsb {
        match self {
            Self::Hex(hex) => hex_to_hsb(hex),
            Self::Rgb(rgb) => rgb_to_hsb(*rgb),
            Self::Hsb(hsb) => *hsb,
        }
    }

    /// Value from HSB in the given format (`getValueToUpdate`).
    pub fn from_hsb(hsb: Hsb, format: ColorFormat) -> Self {
        match format {
            ColorFormat::Hex => Self::Hex(format!("#{}", hsb_to_hex(hsb))),
            ColorFormat::Rgb => Self::Rgb(hsb_to_rgb(hsb)),
            ColorFormat::Hsb => Self::Hsb(hsb),
        }
    }
}

/// Pass-through of [`ColorPicker`] parts.
#[derive(Clone, Debug, Default)]
pub struct ColorPickerPt {
    /// Root.
    pub root: Pt,
    /// Swatch field.
    pub preview: Pt,
    /// Panel.
    pub panel: Pt,
    /// Panel content.
    pub content: Pt,
    /// Saturation/brightness field.
    pub color_selector: Pt,
    /// Field background.
    pub color_background: Pt,
    /// Field pointer.
    pub color_handle: Pt,
    /// Hue scale.
    pub hue: Pt,
    /// Scale pointer.
    pub hue_handle: Pt,
}

/// Handle object: show and hide the panel from code.
#[derive(Clone, Copy)]
pub struct ColorPickerRef {
    visible: RwSignal<bool>,
}

impl Default for ColorPickerRef {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorPickerRef {
    /// Creates a hidden panel. Call inside a component (the signal owner).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false) }
    }

    /// Panel is open (with subscription).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// Shows the panel.
    pub fn show(&self) {
        self.visible.try_set(true);
    }

    /// Hides the panel.
    pub fn hide(&self) {
        self.visible.try_set(false);
    }

    /// Toggles the panel.
    pub fn toggle(&self) {
        self.visible.try_update(|v| *v = !*v);
    }
}

/// Which area is being dragged right now.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Drag {
    None,
    Field,
    Hue,
}

/// Root classes.
pub fn root_classes(inline: bool, dragging: bool) -> &'static str {
    match (inline, dragging) {
        (true, false) => "p-colorpicker p-component",
        (false, false) => "p-colorpicker p-component p-colorpicker-overlay",
        (true, true) => "p-colorpicker p-component p-colorpicker-dragging",
        (false, true) => "p-colorpicker p-component p-colorpicker-overlay p-colorpicker-dragging",
    }
}

/// Panel classes.
pub fn panel_classes(inline: bool, disabled: bool) -> &'static str {
    match (inline, disabled) {
        (false, false) => "p-colorpicker-panel",
        (true, false) => "p-colorpicker-panel p-colorpicker-panel-inline",
        (false, true) => "p-colorpicker-panel p-disabled",
        (true, true) => "p-colorpicker-panel p-colorpicker-panel-inline p-disabled",
    }
}

/// State shared by the handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    value: RwSignal<Option<ColorValue>>,
    hsb: RwSignal<Hsb>,
    format: Prop<ColorFormat>,
    inline: Prop<bool>,
    default_color: OptProp<String>,
    disabled: Prop<bool>,
    auto_z_index: Prop<bool>,
    r: ColorPickerRef,
    drag: RwSignal<Drag>,
    root: NodeRef<leptos::html::Div>,
    panel: NodeRef<leptos::html::Div>,
    field: NodeRef<leptos::html::Div>,
    hue: NodeRef<leptos::html::Div>,
    /// The last value written by the component itself: it is not external, so HSB must not be recomputed from it.
    last_written: StoredValue<Option<ColorValue>>,
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_visible: StoredValue<bool>,
    on_change: Option<Callback<ColorValue>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
}

/// Default color: `default_color` or `ff0000`.
fn default_hsb(default_color: &OptProp<String>) -> Hsb {
    default_color.with(|c| hex_to_hsb(c.as_deref().unwrap_or("ff0000")))
}

impl Core {
    fn current(&self) -> Hsb {
        self.hsb.get_untracked()
    }

    /// Writes the selected color: HSB, the value in the given format, `on_change`.
    fn apply(&self, hsb: Hsb) {
        let hsb = validate_hsb(hsb);
        let value = ColorValue::from_hsb(hsb, self.format.get_untracked());
        self.hsb.set(hsb);
        self.last_written.set_value(Some(value.clone()));
        self.value.set(Some(value.clone()));
        if let Some(cb) = self.on_change {
            cb.run(value);
        }
    }

    /// Pointer coordinates relative to the top-left corner of the area.
    fn local(area: NodeRef<leptos::html::Div>, client_x: f64, client_y: f64) -> Option<(f64, f64)> {
        let rect = area.get_untracked()?.get_bounding_client_rect();
        Some((client_x - rect.left(), client_y - rect.top()))
    }

    fn pick_color(&self, client_x: f64, client_y: f64) {
        if let Some((x, y)) = Self::local(self.field, client_x, client_y) {
            let (s, b) = pick_field(x, y);
            self.apply(Hsb { h: self.current().h, s, b });
        }
    }

    fn pick_hue(&self, client_y: f64) {
        if let Some((_, y)) = Self::local(self.hue, 0.0, client_y) {
            self.apply(Hsb { h: pick_hue(y), ..self.current() });
        }
    }

    fn mouse_down(self: &Arc<Self>, e: web_sys::MouseEvent, drag: Drag) {
        if self.disabled.get_untracked() {
            return;
        }
        self.bind_drag();
        self.drag.set(drag);
        self.pick(drag, f64::from(e.client_x()), f64::from(e.client_y()));
    }

    fn pick(&self, drag: Drag, x: f64, y: f64) {
        match drag {
            Drag::Field => self.pick_color(x, y),
            Drag::Hue => self.pick_hue(y),
            Drag::None => {}
        }
    }

    fn touch_start(&self, e: web_sys::TouchEvent, drag: Drag) {
        if self.disabled.get_untracked() {
            return;
        }
        self.drag.set(drag);
        if let Some(t) = e.changed_touches().get(0) {
            self.pick(drag, f64::from(t.client_x()), f64::from(t.client_y()));
        }
    }

    fn touch_move(&self, e: web_sys::TouchEvent) {
        let drag = self.drag.get_untracked();
        if drag == Drag::None {
            return;
        }
        if let Some(t) = e.changed_touches().get(0) {
            self.pick(drag, f64::from(t.client_x()), f64::from(t.client_y()));
        }
        e.prevent_default();
    }

    fn drag_end(&self) {
        self.drag.try_set(Drag::None);
        self.unbind_drag();
    }

    /// The window `mousemove`/`mouseup` listeners live only during a mouse drag.
    fn bind_drag(self: &Arc<Self>) {
        self.unbind_drag();
        let (on_move, on_up) = (self.clone(), self.clone());
        let move_handle = window_event_listener(leptos::ev::mousemove, move |e: web_sys::MouseEvent| {
            on_move.pick(on_move.drag.get_untracked(), f64::from(e.client_x()), f64::from(e.client_y()));
        });
        let up_handle = window_event_listener(leptos::ev::mouseup, move |_| on_up.drag_end());
        self.listeners.update_value(|l| l.extend([move_handle, up_handle]));
    }

    fn unbind_drag(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
    }

    /// Removes the window subscriptions and releases the z-index.
    fn release(&self) {
        self.unbind_drag();
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_rs_core::zindex::revert(z);
        }
    }

    /// The panel has just been shown: z-index, alignment to the root, subscriptions, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(root)) = (self.panel.get_untracked(), self.root.get_untracked()) else { return };
        let (panel, root): (web_sys::HtmlElement, web_sys::HtmlElement) = (panel.unchecked_into(), root.unchecked_into());
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_rs_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        align_overlay(&panel, &root, AlignMode::Relative, false);

        // Subscriptions go on the next tick: the click that opened the panel is still bubbling to the window and would close it itself.
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.r.visible.is_disposed() || !this.r.visible.get_untracked() {
                    return;
                }
                this.bind_window();
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Outside click and window resize hide the panel; an ancestor scroll does too.
    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let inside = this.root.get_untracked().is_some_and(|r| r.contains(node.as_ref()));
            if !inside {
                this.r.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.r.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || resize.remove()));
        });
        let this = self.clone();
        if let Some(root) = self.root.get_untracked()
            && let Some(unwatch) = prime_dom::watch_scroll(root.unchecked_into(), move || this.r.hide())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
    }

    /// The panel has just been hidden.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    fn key_down(&self, e: &web_sys::KeyboardEvent) {
        match e.code().as_str() {
            "Space" => {
                self.r.toggle();
                e.prevent_default();
            }
            "Escape" | "Tab" => self.r.hide(),
            _ => {}
        }
    }
}

/// Color selection.
///
/// Each property accepts a value or a signal; `value` is a two-way signal. The panel is rendered inside the root
/// (not in `body`), so ancestors must not have `overflow: hidden`.
#[component]
pub fn ColorPicker(
    /// Two-way value signal; `None` shows `default_color`.
    #[prop(optional)]
    value: Option<RwSignal<Option<ColorValue>>>,
    /// Value format.
    #[prop(optional, into)]
    format: Prop<ColorFormat>,
    /// Embedded panel without a swatch.
    #[prop(optional, into)]
    inline: Prop<bool>,
    /// Color while there is no value (HEX, default `ff0000`).
    #[prop(optional, into)]
    default_color: OptProp<String>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// `tabindex` attribute of the swatch.
    #[prop(optional, into)]
    tabindex: OptProp<String>,
    /// `id` attribute of the swatch (for `label for`).
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Swatch focus on load.
    #[prop(optional, into)]
    autofocus: Prop<bool>,
    /// Automatically assign the panel `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// `id` attribute of the root.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called on every color change by the user.
    #[prop(optional, into)]
    on_change: Option<Callback<ColorValue>>,
    /// Called after the panel is shown.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Called after the panel is hidden.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Handle object of the panel.
    #[prop(optional)]
    color_picker_ref: Option<ColorPickerRef>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<ColorPickerPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let value = value.unwrap_or_else(|| RwSignal::new(None));
    let r = color_picker_ref.unwrap_or_default();
    let initial = value.with_untracked(|v| v.as_ref().map(ColorValue::to_hsb)).unwrap_or_else(|| default_hsb(&default_color));
    let hsb = RwSignal::new(initial);
    let core = Arc::new(Core {
        value,
        hsb,
        format,
        inline: inline.clone(),
        default_color: default_color.clone(),
        disabled: disabled.clone(),
        auto_z_index,
        r,
        drag: RwSignal::new(Drag::None),
        root: NodeRef::new(),
        panel: NodeRef::new(),
        field: NodeRef::new(),
        hue: NodeRef::new(),
        last_written: StoredValue::new(None),
        listeners: StoredValue::new_local(Vec::new()),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_visible: StoredValue::new(false),
        on_change,
        on_show,
        on_hide,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }

    // An external value change (not our own write) recomputes HSB — in the browser only.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        let sync = core.clone();
        Effect::new(move |_| {
            let core = &sync;
            let external = core.value.get();
            if external == core.last_written.get_value() && external.is_some() {
                return;
            }
            let next = external.as_ref().map(ColorValue::to_hsb).unwrap_or_else(|| default_hsb(&core.default_color));
            core.hsb.set(next);
        });
        let core = core.clone();
        Effect::new(move |_| {
            let visible = core.r.visible.get();
            // The panel enters the DOM after this effect, so we also watch the reference to it.
            let mounted = core.panel.get().is_some();
            let was = core.was_visible.get_value();
            if visible && mounted && !was {
                core.was_visible.set_value(true);
                core.opened();
            } else if !visible && was {
                core.was_visible.set_value(false);
                core.closed();
            }
        });
    }

    let root_class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let mut cn = String::from(root_classes(core.inline.get(), core.drag.get() != Drag::None));
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let dragging = {
        let core = core.clone();
        move || if core.drag.get() == Drag::None { "false" } else { "true" }
    };

    let preview = {
        let (core, pt) = (core.clone(), pt.clone());
        let disabled = disabled.clone();
        move || {
            core.inline.get().not_then(|| {
                let (c_click, c_key, c_bg) = (core.clone(), core.clone(), core.clone());
                let (disabled_class, disabled_attr) = (disabled.clone(), disabled.clone());
                let (tabindex, input_id) = (tabindex.clone(), input_id.clone());
                let pt = pt.clone();
                view! {
                    <input
                        type="text"
                        class={move || {
                            let base = if disabled_class.get() { "p-colorpicker-preview p-disabled" } else { "p-colorpicker-preview" };
                            pt.preview.merge_class(base).into_owned()
                        }}
                        readonly
                        aria-label="Select Color"
                        tabindex={move || tabindex.get()}
                        id={move || input_id.get()}
                        disabled={move || disabled_attr.get().then_some("")}
                        autofocus={autofocus.get_untracked()}
                        style:background-color={move || format!("#{}", hsb_to_hex(c_bg.hsb.get()))}
                        on:click={move |_| c_click.r.toggle()}
                        on:keydown={move |e: web_sys::KeyboardEvent| c_key.key_down(&e)}
                    />
                }
            })
        }
    };

    let panel = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let show = core.inline.get() || core.r.visible.get();
            show.then(|| panel_view(&core, &pt))
        }
    };
    let root = core.root;
    let id_attr = move || id.get();
    let root_attrs = pt.root.attrs();

    view! {
        <div
            node_ref={root}
            class={root_class}
            style={root_style}
            id={id_attr}
            data-pc-name="colorpicker"
            data-pc-section="root"
            data-p-colorpicker-dragging={dragging}
            {..root_attrs}
        >
            {preview}
            {panel}
        </div>
    }
}

/// A small extension for `bool`: the value when the condition is not met.
trait NotThen {
    fn not_then<T>(self, f: impl FnOnce() -> T) -> Option<T>;
}

impl NotThen for bool {
    fn not_then<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        (!self).then(f)
    }
}

fn panel_view(core: &Arc<Core>, pt: &Arc<ColorPickerPt>) -> impl IntoView + use<> {
    let hsb = core.hsb;
    let (panel_ref, field_ref, hue_ref) = (core.panel, core.field, core.hue);
    let c_panel_class = core.clone();
    let selector_color = move || format!("#{}", hsb_to_hex(Hsb { s: 100.0, b: 100.0, ..hsb.get() }));
    let handle_style = move || {
        let (left, top) = field_handle(hsb.get());
        format!("left: {left}px; top: {top}px")
    };
    let hue_style = move || format!("top: {}px", hue_handle(hsb.get()));
    let panel_class = {
        let pt = pt.clone();
        move || pt.panel.merge_class(panel_classes(c_panel_class.inline.get(), c_panel_class.disabled.get())).into_owned()
    };
    let (m_field, t_field, m_hue, t_hue) = (core.clone(), core.clone(), core.clone(), core.clone());
    let (tm_field, tm_hue, te_field, te_hue) = (core.clone(), core.clone(), core.clone(), core.clone());
    view! {
        <div
            node_ref=panel_ref
            class={panel_class}
            data-pc-section="panel"
            {..pt.panel.attrs()}
        >
            <div class={pt.content.merge_class("p-colorpicker-content")} {..pt.content.attrs()}>
                <div
                    node_ref=field_ref
                    class={pt.color_selector.merge_class("p-colorpicker-color-selector")}
                    style:background-color={selector_color}
                    on:mousedown={move |e: web_sys::MouseEvent| m_field.mouse_down(e, Drag::Field)}
                    on:touchstart={move |e: web_sys::TouchEvent| t_field.touch_start(e, Drag::Field)}
                    on:touchmove={move |e: web_sys::TouchEvent| tm_field.touch_move(e)}
                    on:touchend={move |_| te_field.drag_end()}
                    {..pt.color_selector.attrs()}
                >
                    <div class={pt.color_background.merge_class("p-colorpicker-color-background")} {..pt.color_background.attrs()}>
                        <div class={pt.color_handle.merge_class("p-colorpicker-color-handle")} style={handle_style} {..pt.color_handle.attrs()}></div>
                    </div>
                </div>
                <div
                    node_ref=hue_ref
                    class={pt.hue.merge_class("p-colorpicker-hue")}
                    on:mousedown={move |e: web_sys::MouseEvent| m_hue.mouse_down(e, Drag::Hue)}
                    on:touchstart={move |e: web_sys::TouchEvent| t_hue.touch_start(e, Drag::Hue)}
                    on:touchmove={move |e: web_sys::TouchEvent| tm_hue.touch_move(e)}
                    on:touchend={move |_| te_hue.drag_end()}
                    {..pt.hue.attrs()}
                >
                    <div class={pt.hue_handle.merge_class("p-colorpicker-hue-handle")} style={hue_style} {..pt.hue_handle.attrs()}></div>
                </div>
            </div>
        </div>
    }
}
