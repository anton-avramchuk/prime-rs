//! Fieldset — port of `p-fieldset` from PrimeNG 21.1.10.
//!
//! Structure and classes match PrimeNG: root `<fieldset class="p-fieldset …">` with `<legend class="p-fieldset-legend">`
//! and content container `p-fieldset-content-container` (wrapper `p-fieldset-content-wrapper`, content `p-fieldset-content`).
//! With `toggleable`, a toggle button `p-fieldset-toggle-button` with a plus/minus icon and a label sits inside the legend.
//!
//! Collapsing is built on [`prime_rs_core::collapsible`], the same as in Panel: the state is a two-way `collapsed` signal
//! (without it, an own state with the initial value `default_collapsed`), the button gets `aria-controls` and `aria-expanded`,
//! the region gets `role="region"`, `aria-labelledby`, `aria-hidden`; expanding and collapsing animate the classes
//! `p-collapsible-enter-active`/`p-collapsible-leave-active`, and `on_after_toggle` is called when the animation ends.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_fieldset::Fieldset;
//!
//! let collapsed = RwSignal::new(true);
//! let html = view! { <Fieldset legend="Header" toggleable=true collapsed=collapsed><p>"Content"</p></Fieldset> }.to_html();
//! assert!(html.contains("p-fieldset-collapsed") && html.contains(r#"aria-expanded="false""#) && html.contains("Header"));
//! ```

mod state;

use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use prime_icons::{MinusIcon, PlusIcon};
use prime_rs_core::collapsible::{CollapseIds, CollapseState, aria_expanded, aria_hidden, region_tabindex};
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{data_p, write_root_classes};

/// `on_before_toggle` event: the original event and the state before toggling.
#[derive(Clone, Debug)]
pub struct FieldsetBeforeToggle {
    /// Original `click` event.
    pub original_event: web_sys::MouseEvent,
    /// Value of `collapsed` before toggling.
    pub collapsed: bool,
}

/// `on_after_toggle` event: the new state after the animation ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetAfterToggle {
    /// Value of `collapsed` after toggling.
    pub collapsed: bool,
}

/// Pass-through for the parts of Fieldset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldsetPt {
    /// Root element (`fieldset.p-fieldset`).
    pub root: Pt,
    /// Legend (`legend.p-fieldset-legend`).
    pub legend: Pt,
    /// Legend label (`span.p-fieldset-legend-label`).
    pub legend_label: Pt,
    /// Toggle button (`button.p-fieldset-toggle-button`).
    pub toggle_button: Pt,
    /// Toggle icon: slot wrapper (`span.p-fieldset-toggle-icon`); for standard icons only the class is applied.
    pub toggle_icon: Pt,
    /// Content container (`div.p-fieldset-content-container`).
    pub content_container: Pt,
    /// Content wrapper (`div.p-fieldset-content-wrapper`).
    pub content_wrapper: Pt,
    /// Content (`div.p-fieldset-content`).
    pub content: Pt,
}

/// Grouping container with a legend and optional collapsing.
///
/// Text and boolean properties accept a value or a signal; `collapsed` is a two-way signal.
#[component]
pub fn Fieldset(
    /// `id` attribute of the root; the button and content identifiers are built from it. Unique by default.
    #[prop(optional, into)]
    id: Option<String>,
    /// Legend text (`p-fieldset-legend-label`).
    #[prop(optional, into)]
    legend: OptProp<String>,
    /// Whether the content can be collapsed.
    #[prop(optional, into)]
    toggleable: Prop<bool>,
    /// Two-way "collapsed" signal.
    #[prop(optional)]
    collapsed: Option<RwSignal<bool>>,
    /// Initial state when `collapsed` is not passed.
    #[prop(optional)]
    default_collapsed: bool,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Legend slot: rendered after the label.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Expand icon slot (shown for a collapsed Fieldset).
    #[prop(optional, into)]
    expand_icon_template: Option<ViewFn>,
    /// Collapse icon slot (shown for an expanded Fieldset).
    #[prop(optional, into)]
    collapse_icon_template: Option<ViewFn>,
    /// Called before toggling.
    #[prop(optional, into)]
    on_before_toggle: Option<Callback<FieldsetBeforeToggle>>,
    /// Called after the toggle animation ends.
    #[prop(optional, into)]
    on_after_toggle: Option<Callback<FieldsetAfterToggle>>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<FieldsetPt>,
    /// Content (`p-fieldset-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(legend, toggleable, style, style_class) && !toggleable.get() && collapsed.is_none();
    let state = CollapseState::new(collapsed, default_collapsed);
    let props = Arc::new(Props {
        ids: CollapseIds::new(id.as_deref()),
        state,
        legend,
        toggleable,
        style,
        style_class,
        header_template,
        expand_icon_template,
        collapse_icon_template,
        on_before_toggle,
    });

    // `NodeRef` is needed only by the client (animation and `tabindex`): there is none during SSR, and `()` is an empty attribute instead of `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        if props.toggleable.get_untracked() {
            let after = on_after_toggle.map(|cb| Callback::new(move |collapsed| cb.run(FieldsetAfterToggle { collapsed })));
            prime_rs_core::collapsible::bind_region(region, state, after);
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {
        let _ = on_after_toggle;
    };

    let (root_id, content_id) = (props.ids.root.clone(), props.ids.content.clone());
    let root_attrs = pt.root.attrs();
    let container_class = pt.content_container.merge_class("p-fieldset-content-container");
    let container_style = container_style_of(&props, &pt);
    let container_attrs = pt.content_container.attrs();
    let (wrapper_class, wrapper_attrs) = (pt.content_wrapper.merge_class("p-fieldset-content-wrapper"), pt.content_wrapper.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-fieldset-content"), pt.content.attrs());
    let content = children.map(|c| c());

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let legend = legend_of(&props, &pt);
        Either::Left(fieldset_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            legend,
            region,
            container_class,
            container_style,
            content_id,
            None::<String>,
            None::<&'static str>,
            None::<&'static str>,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content
        ))
    } else {
        let pt = Arc::new(pt);
        let (p, t) = (&props, &pt);
        let (class, style, data) = (rx(p, t, class_of), rx(p, t, style_of), rx(p, t, |p, _| data_of(p)));
        let legend = rx(p, t, legend_of);
        let labelled_by = rx(p, t, |p, _| p.toggleable.get().then(|| p.ids.header.clone()));
        let hidden = rx(p, t, |p, _| p.toggleable.get().then(|| aria_hidden(p.state.get())));
        let tabindex = rx(p, t, |p, _| p.toggleable.get().then(|| region_tabindex(p.state.get())).flatten());
        Either::Right(fieldset_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            legend,
            region,
            container_class,
            container_style,
            content_id,
            labelled_by,
            hidden,
            tabindex,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content
        ))
    }
}

/// Fieldset markup: one for the static and reactive branches (only the attribute and legend expressions differ).
macro_rules! fieldset_markup {
    (
        $root_id:expr, $class:expr, $style:expr, $data:expr, $root_attrs:expr, $legend:expr, $region:expr, $container_class:expr,
        $container_style:expr, $content_id:expr, $labelled_by:expr, $hidden:expr, $tabindex:expr, $container_attrs:expr,
        $wrapper_class:expr, $wrapper_attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr
    ) => {
        view! {
            <fieldset id={$root_id} class={$class} style={$style} data-p={$data} {..$root_attrs}>
                {$legend}
                <div
                    {..$region}
                    id={$content_id}
                    class={$container_class}
                    style={$container_style}
                    role="region"
                    aria-labelledby={$labelled_by}
                    aria-hidden={$hidden}
                    tabindex={$tabindex}
                    {..$container_attrs}
                >
                    <div class={$wrapper_class} {..$wrapper_attrs}>
                        <div class={$content_class} {..$content_attrs}>
                            {$content}
                        </div>
                    </div>
                </div>
            </fieldset>
        }
    };
}
use fieldset_markup;

/// Closure over the instance properties: `Arc` clones and a function pointer without capture.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<FieldsetPt>,
    f: fn(&Arc<Props>, &FieldsetPt) -> T,
) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn data_of(props: &Props) -> Option<Cow<'static, str>> {
    let data = data_p(props.toggleable.get());
    (!data.is_empty()).then_some(data)
}

fn class_of(props: &Arc<Props>, pt: &FieldsetPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    let toggleable = props.toggleable.get();
    write_root_classes(&mut cn, toggleable, toggleable && props.state.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Arc<Props>, pt: &FieldsetPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Style of the content container: a Fieldset collapsed at the first render is hidden right away; after that the animation controls it.
fn container_style_of(props: &Props, pt: &FieldsetPt) -> Option<String> {
    let hidden = if props.toggleable.get_untracked() { props.state.initial_style() } else { None };
    (hidden.is_some() || pt.content_container.has_style()).then(|| pt.content_container.merge_style(hidden.unwrap_or("")))
}

/// Legend: label and slot, and with `toggleable` — inside the button; reads `collapsed` only inside nested closures.
fn legend_of(props: &Arc<Props>, pt: &FieldsetPt) -> AnyView {
    let toggleable = props.toggleable.get();
    let text = props.legend.get();
    let label = view! {
        <span class=pt.legend_label.merge_class("p-fieldset-legend-label") {..pt.legend_label.attrs()}>{text.clone()}</span>
    };
    let slot = props.header_template.clone().map(|slot| slot.run());
    let (class, data) = (pt.legend.merge_class("p-fieldset-legend"), data_of(props));
    if toggleable {
        let button = toggle_button(props, pt, text, label, slot);
        view! { <legend class=class data-p={data} {..pt.legend.attrs()}>{button}</legend> }.into_any()
    } else {
        view! { <legend class=class data-p={data} {..pt.legend.attrs()}>{label}{slot}</legend> }.into_any()
    }
}

/// Toggle button in the legend with `aria-controls` and `aria-expanded`; the icon depends on `collapsed`.
fn toggle_button(
    props: &Arc<Props>,
    pt: &FieldsetPt,
    aria_label: Option<String>,
    label: impl IntoView,
    slot: Option<AnyView>,
) -> impl IntoView {
    let state = props.state;
    let (expand, collapse) = (props.expand_icon_template.clone(), props.collapse_icon_template.clone());
    let icon_class = pt.toggle_icon.merge_class("p-fieldset-toggle-icon").into_owned();
    let (icon_span_class, icon_pt) = (icon_class.clone(), pt.toggle_icon.clone());
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| toggle(&props, e)
    };
    let icon = move || {
        let collapsed = state.get();
        match if collapsed { &expand } else { &collapse } {
            Some(slot) => EitherOf3::A(view! { <span class=icon_span_class.clone() {..icon_pt.attrs()}>{slot.run()}</span> }),
            None if collapsed => EitherOf3::B(view! { <PlusIcon class=icon_class.clone() /> }),
            None => EitherOf3::C(view! { <MinusIcon class=icon_class.clone() /> }),
        }
    };
    view! {
        <button
            id=props.ids.header.clone()
            type="button"
            class=pt.toggle_button.merge_class("p-fieldset-toggle-button")
            aria-controls=props.ids.content.clone()
            aria-expanded=move || aria_expanded(state.get())
            aria-label=aria_label
            on:click={click}
            {..pt.toggle_button.attrs()}
        >
            {icon}
            {label}
            {slot}
        </button>
    }
}

/// Toggles the Fieldset: `on_before_toggle` with the previous state, then changes `collapsed`.
fn toggle(props: &Props, e: web_sys::MouseEvent) {
    if !props.toggleable.get_untracked() {
        return;
    }
    e.prevent_default();
    if let Some(cb) = props.on_before_toggle {
        cb.run(FieldsetBeforeToggle { original_event: e, collapsed: props.state.get_untracked() });
    }
    props.state.toggle();
}

/// Instance properties.
struct Props {
    ids: CollapseIds,
    state: CollapseState,
    legend: OptProp<String>,
    toggleable: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    header_template: Option<ViewFn>,
    expand_icon_template: Option<ViewFn>,
    collapse_icon_template: Option<ViewFn>,
    on_before_toggle: Option<Callback<FieldsetBeforeToggle>>,
}
