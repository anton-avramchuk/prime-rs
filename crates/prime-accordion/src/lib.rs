//! Accordion — port of PrimeNG 21.1.10 `p-accordion`.
//!
//! A composite component of four parts, as in PrimeNG: [`Accordion`] keeps the list of active values,
//! [`AccordionPanel`] sets the panel's `value`, [`AccordionHeader`] is the header button, [`AccordionContent`] is the collapsible content.
//! The parts communicate through the Leptos context and do not take values from each other.
//!
//! The value is a two-way `RwSignal<Vec<String>>` signal: in single mode it holds 0 or 1 value, with `multiple` — any number.
//! Expanding and collapsing animate the `p-collapsible-enter-active`/`p-collapsible-leave-active` classes via
//! [`prime_core::collapsible`], as in Panel and Fieldset.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel};
//!
//! let html = Owner::new().with(|| view! {
//!     <Accordion default_value=vec!["0".to_owned()]>
//!         <AccordionPanel value="0">
//!             <AccordionHeader>"Header I"</AccordionHeader>
//!             <AccordionContent><p>"Content I"</p></AccordionContent>
//!         </AccordionPanel>
//!         <AccordionPanel value="1">
//!             <AccordionHeader>"Header II"</AccordionHeader>
//!             <AccordionContent><p>"Content II"</p></AccordionContent>
//!         </AccordionPanel>
//!     </Accordion>
//! }
//! .to_html());
//! assert!(html.contains("p-accordionpanel-active") && html.contains(r#"aria-expanded="true""#) && html.contains("Header II"));
//! ```

mod state;

use leptos::either::EitherOf3;
use leptos::prelude::*;
use prime_core::collapsible::{CollapseIds, hidden_style};
use prime_core::{ClassNames, OptProp, Prop, Pt};
use prime_icons::{ChevronDownIcon, ChevronUpIcon};
use std::sync::Arc;

pub use state::{Nav, nav_target, toggle_value};

/// `on_open` and `on_close` event: the original event and the panel value.
#[derive(Clone, Debug)]
pub struct AccordionToggle {
    /// Original `click` event.
    pub original_event: web_sys::Event,
    /// Panel value (`value` of [`AccordionPanel`]).
    pub index: String,
}

/// Pass-through parts of Accordion.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AccordionPt {
    /// Root element (`div.p-accordion`).
    pub root: Pt,
    /// Panel (`div.p-accordionpanel`).
    pub panel: Pt,
    /// Header (`div.p-accordionheader`).
    pub header: Pt,
    /// Toggle icon: only the class is applied.
    pub toggle_icon: Pt,
    /// Content container (`div.p-accordioncontent`).
    pub content_container: Pt,
    /// Content wrapper (`div.p-accordioncontent-wrapper`).
    pub content_wrapper: Pt,
    /// Content (`div.p-accordioncontent-content`).
    pub content: Pt,
}

/// Shared group state available to the panels.
#[derive(Clone)]
struct AccordionCtx {
    id: String,
    value: RwSignal<Vec<String>>,
    multiple: Prop<bool>,
    select_on_focus: Prop<bool>,
    on_open: Option<Callback<AccordionToggle>>,
    on_close: Option<Callback<AccordionToggle>>,
    pt: Arc<AccordionPt>,
}

/// Panel state available to the header and content.
#[derive(Clone)]
struct PanelCtx {
    value: String,
    disabled: Prop<bool>,
    active: Signal<bool>,
}

const NO_ACCORDION: &str = "AccordionPanel, AccordionHeader and AccordionContent must be inside Accordion";
const NO_PANEL: &str = "AccordionHeader and AccordionContent must be inside AccordionPanel";

fn header_id(ctx: &AccordionCtx, value: &str) -> String {
    format!("{}_accordionheader_{value}", ctx.id)
}

fn content_id(ctx: &AccordionCtx, value: &str) -> String {
    format!("{}_accordioncontent_{value}", ctx.id)
}

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// Tab group: keeps the list of active values.
///
/// `value` is a two-way signal; without it the state is internal, initially `default_value`.
#[component]
pub fn Accordion(
    /// `id` attribute base for header and content identifiers. Unique by default.
    #[prop(optional, into)]
    id: Option<String>,
    /// Two-way signal of active values.
    #[prop(optional)]
    value: Option<RwSignal<Vec<String>>>,
    /// Initial values when `value` is not passed.
    #[prop(optional)]
    default_value: Vec<String>,
    /// Whether several panels can be open at the same time.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Toggle the panel when its header receives focus.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called when a panel is opened by a click on the header.
    #[prop(optional, into)]
    on_open: Option<Callback<AccordionToggle>>,
    /// Called when a panel is closed by a click on the header.
    #[prop(optional, into)]
    on_close: Option<Callback<AccordionToggle>>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<AccordionPt>,
    /// Panels.
    children: Children,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let ctx = AccordionCtx {
        id: CollapseIds::new(id.as_deref()).root,
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        multiple,
        select_on_focus,
        on_open,
        on_close,
        pt: pt.clone(),
    };
    provide_context(ctx);
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordion p-component");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        pt.root.has_style().then(|| pt.root.merge_style(""))
    };
    view! {
        <div class={class} style={style} {..pt.root.attrs()}>
            {children()}
        </div>
    }
}

/// Panel: sets `value` and the active state for the header and content.
#[component]
pub fn AccordionPanel(
    /// Unique panel value in the group.
    #[prop(into)]
    value: String,
    /// The panel is not available for interaction.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Extra class of the panel.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Header and content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let active = {
        let (list, own) = (ctx.value, value.clone());
        Signal::derive(move || list.with(|v| v.contains(&own)))
    };
    provide_context(PanelCtx { value, disabled: disabled.clone(), active });
    let pt = ctx.pt.clone();
    let class = {
        let (pt, disabled) = (pt.clone(), disabled.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordionpanel").add_if(active.get(), "p-accordionpanel-active").add_if(disabled.get(), "p-disabled");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.panel.extra_class());
            cn.finish_cow()
        }
    };
    let data_disabled = move || bool_attr(disabled.get());
    let data_active = move || bool_attr(active.get());
    view! {
        <div class={class} data-p-disabled={data_disabled} data-p-active={data_active} {..pt.panel.attrs()}>
            {children()}
        </div>
    }
}

/// Panel header: a button with a toggle icon.
#[component]
pub fn AccordionHeader(
    /// Extra class of the header.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Toggle icon slot; receives `active` and replaces the chevron.
    #[prop(optional, into)]
    toggle_icon_template: Option<Callback<bool, AnyView>>,
    /// Header content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let panel = use_context::<PanelCtx>().expect(NO_PANEL);
    let (active, disabled) = (panel.active, panel.disabled.clone());
    let (id, controls) = (header_id(&ctx, &panel.value), content_id(&ctx, &panel.value));
    let pt = ctx.pt.clone();

    let toggle = {
        let (ctx, value, disabled) = (ctx.clone(), panel.value.clone(), disabled.clone());
        move |e: web_sys::Event, emit: bool| {
            if disabled.get_untracked() {
                return;
            }
            let was_active = active.get_untracked();
            let multiple = ctx.multiple.get_untracked();
            ctx.value.update(|list| *list = toggle_value(list, &value, multiple));
            let is_active = active.get_untracked();
            if !emit {
                return;
            }
            let event = AccordionToggle { original_event: e, index: value.clone() };
            match (was_active, is_active) {
                (false, true) => ctx.on_open.inspect(|cb| cb.run(event)),
                (true, false) => ctx.on_close.inspect(|cb| cb.run(event)),
                _ => None,
            };
        }
    };
    let click = {
        let toggle = toggle.clone();
        move |e: web_sys::MouseEvent| toggle(e.into(), true)
    };
    let focus = {
        let (toggle, ctx, disabled) = (toggle.clone(), ctx.clone(), disabled.clone());
        move |e: web_sys::FocusEvent| {
            if !disabled.get_untracked() && ctx.select_on_focus.get_untracked() {
                toggle(e.into(), false);
            }
        }
    };
    let keydown = move |e: web_sys::KeyboardEvent| {
        let nav = match e.code().as_str() {
            "ArrowDown" => Some(Nav::Next),
            "ArrowUp" => Some(Nav::Prev),
            "Home" => Some(Nav::First),
            "End" => Some(Nav::Last),
            "Enter" | "NumpadEnter" | "Space" => {
                toggle(e.clone().into(), false);
                e.prevent_default();
                return;
            }
            _ => return,
        };
        if let Some(nav) = nav {
            focus_neighbor(&e, nav);
            e.prevent_default();
        }
    };

    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordionheader");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.header.extra_class());
            cn.finish_cow()
        }
    };
    let style = pt.header.merge_style("user-select: none");
    let icon_class = pt.toggle_icon.merge_class("p-accordionheader-toggle-icon").into_owned();
    let icon = move || {
        let is_active = active.get();
        match toggle_icon_template {
            Some(template) => EitherOf3::A(template.run(is_active)),
            None if is_active => EitherOf3::B(view! { <ChevronUpIcon class=icon_class.clone() /> }),
            None => EitherOf3::C(view! { <ChevronDownIcon class=icon_class.clone() /> }),
        }
    };
    let expanded = move || bool_attr(active.get());
    let aria_disabled = {
        let disabled = disabled.clone();
        move || bool_attr(disabled.get())
    };
    let tabindex = {
        let disabled = disabled.clone();
        move || if disabled.get() { "-1" } else { "0" }
    };
    let data_disabled = move || bool_attr(disabled.get());
    let data_active = move || bool_attr(active.get());
    let data = move || active.get().then_some("active");
    view! {
        <div
            id={id}
            class={class}
            style={style}
            role="button"
            aria-expanded={expanded}
            aria-controls={controls}
            aria-disabled={aria_disabled}
            tabindex={tabindex}
            data-p-active={data_active}
            data-p-disabled={data_disabled}
            data-p={data}
            on:click={click}
            on:focus={focus}
            on:keydown={keydown}
            {..pt.header.attrs()}
        >
            {children()}
            {icon}
        </div>
    }
}

/// Collapsible panel content.
#[component]
pub fn AccordionContent(
    /// Extra class of the content container.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let panel = use_context::<PanelCtx>().expect(NO_PANEL);
    let active = panel.active;
    let (id, labelled_by) = (content_id(&ctx, &panel.value), header_id(&ctx, &panel.value));
    let pt = ctx.pt.clone();

    // `NodeRef` is needed only by the client (animation and `tabindex`): there is none in SSR, and `()` is an empty attribute instead of `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        prime_core::collapsible::bind_region_with(region, move || !active.get(), None);
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {};

    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordioncontent");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.content_container.extra_class());
            cn.finish_cow()
        }
    };
    let data_active = move || bool_attr(active.get());
    let hidden = hidden_style(!active.get_untracked());
    view! {
        <div id={id} class={class} role="region" data-p-active={data_active} aria-labelledby={labelled_by} {..pt.content_container.attrs()}>
            <div {..region} class="p-motion" style={hidden}>
                <div class=pt.content_wrapper.merge_class("p-accordioncontent-wrapper") {..pt.content_wrapper.attrs()}>
                    <div class=pt.content.merge_class("p-accordioncontent-content") {..pt.content.attrs()}>{children()}</div>
                </div>
            </div>
        </div>
    }
}

/// Moves focus to the header of the neighbouring non-disabled panel (browser only).
fn focus_neighbor(e: &web_sys::KeyboardEvent, nav: Nav) {
    use leptos::wasm_bindgen::JsCast;
    let Some(header) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
    let Some(panel) = header.closest(".p-accordionpanel").ok().flatten() else { return };
    let Some(parent) = panel.parent_element() else { return };
    let children = parent.children();
    let panels: Vec<web_sys::Element> =
        (0..children.length()).filter_map(|i| children.item(i)).filter(|c| c.class_list().contains("p-accordionpanel")).collect();
    let Some(from) = panels.iter().position(|p| *p == panel) else { return };
    let disabled: Vec<bool> = panels.iter().map(|p| p.get_attribute("data-p-disabled").as_deref() == Some("true")).collect();
    let Some(to) = nav_target(&disabled, from, nav) else { return };
    if let Ok(Some(target)) = panels[to].query_selector(".p-accordionheader")
        && let Ok(target) = target.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = target.focus();
    }
}
