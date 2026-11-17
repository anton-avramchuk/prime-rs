//! Stepper - port of `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item` and `p-stepper-separator`
//! from PrimeNG 21.1.10.
//!
//! A compound component, like Tabs: [`Stepper`] stores the active value, [`StepList`] and [`Step`] are the list of steps with separators,
//! [`StepPanels`] and [`StepPanel`] are the panels, [`StepItem`] wraps a step and a panel in the vertical layout. A step and a panel are linked
//! by the same `value` (a number); the parts communicate through the Leptos context. The active value is the two-way signal `value`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_stepper::{Step, StepList, StepPanel, StepPanels, Stepper};
//!
//! let html = Owner::new().with(|| view! {
//!     <Stepper default_value=2u32>
//!         <StepList>
//!             <Step value=1u32>"Header I"</Step>
//!             <Step value=2u32>"Header II"</Step>
//!         </StepList>
//!         <StepPanels>
//!             <StepPanel value=1u32>"Content I"</StepPanel>
//!             <StepPanel value=2u32>"Content II"</StepPanel>
//!         </StepPanels>
//!     </Stepper>
//! }
//! .to_html());
//! assert!(html.contains("p-step-active") && html.contains("Content II") && !html.contains("Content I<"));
//! ```

mod state;

use leptos::prelude::*;
use leptos::tachys::reactive_graph::OwnedView;
use prime_core::collapsible::hidden_style;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{is_active, item_classes, panel_classes, panel_id, root_classes, separator_after, step_classes, step_disabled, step_id};

/// State of a step or panel passed to `content` (like PrimeNG's `content` template context).
#[derive(Clone, Copy)]
pub struct StepContext {
    /// Step or panel value.
    pub value: u32,
    /// Whether the step (panel) is active.
    pub active: Signal<bool>,
    /// Activates the step with the given value (`activateCallback`); works in linear mode too.
    pub activate: Callback<u32>,
}

/// Pass-through of the [`Step`] parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepPt {
    /// Step root.
    pub root: Pt,
    /// Header button.
    pub header: Pt,
    /// Number.
    pub number: Pt,
    /// Title.
    pub title: Pt,
    /// Separator.
    pub separator: Pt,
}

/// Pass-through of the [`StepPanel`] parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepPanelPt {
    /// Panel root.
    pub root: Pt,
    /// Content wrapper.
    pub content_wrapper: Pt,
    /// Content.
    pub content: Pt,
    /// Separator (vertical layout).
    pub separator: Pt,
}

/// Shared state of the group, available to the parts.
#[derive(Clone)]
struct StepperCtx {
    id: String,
    value: RwSignal<Option<u32>>,
    linear: Prop<bool>,
    /// `StepList` step values in creation order (for separators).
    steps: RwSignal<Vec<u32>>,
    /// `StepItem` values in creation order (for panel separators).
    items: RwSignal<Vec<u32>>,
}

/// Value of the nearest `StepItem`.
#[derive(Clone, Copy)]
struct ItemCtx {
    value: u32,
}

const NO_STEPPER: &str = "StepList, Step, StepPanels, StepPanel and StepItem must be inside Stepper";
const NO_VALUE: &str = "Step and StepPanel without value must be inside a StepItem with value";

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// Builds `f` in a child owner so that a context placed in `f` does not leak to siblings that come after the component.
///
/// The returned view remembers the owner and is built, updated and hydrated inside it, so reactive descendants
/// (for example, steps added by a condition) find the context too. The owner is kept until the parent is cleaned up: on its own it
/// would be released immediately and take the descendants' reactive values with it.
fn scoped(f: impl FnOnce() -> AnyView) -> OwnedView<AnyView> {
    match Owner::current() {
        Some(parent) => {
            let owner = parent.child();
            let out = owner.with(f);
            let keep = owner.clone();
            on_cleanup(move || drop(keep));
            OwnedView::new_with_owner(out, owner)
        }
        None => OwnedView::new_with_owner(f(), Owner::new()),
    }
}

/// Step/panel value: its own or the value of the nearest `StepItem`.
fn resolve_value(own: Option<u32>) -> (u32, bool) {
    let item = use_context::<ItemCtx>();
    (own.or(item.map(|i| i.value)).expect(NO_VALUE), item.is_some())
}

/// A wizard of steps: stores the active value.
///
/// `value` is a two-way signal; without it the state is internal, initially `default_value`.
#[component]
pub fn Stepper(
    /// Base for the `id`s of steps and panels; unique by default.
    #[prop(optional, into)]
    id: Option<String>,
    /// Two-way signal of the active value.
    #[prop(optional)]
    value: Option<RwSignal<Option<u32>>>,
    /// Initial value when `value` is not passed.
    #[prop(optional, into)]
    default_value: Option<u32>,
    /// Linear mode: clicks on inactive steps are unavailable, transitions are programmatic.
    #[prop(optional, into)]
    linear: Prop<bool>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through of the root.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Step list, panels or items.
    children: Children,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let id = id.unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let ctx = StepperCtx {
        id: id.clone(),
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        linear: linear.clone(),
        steps: RwSignal::new(Vec::new()),
        items: RwSignal::new(Vec::new()),
    };
    let content = scoped(|| {
        provide_context(ctx);
        children().into_any()
    });
    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = ClassNames::new();
            cn.add(root_classes(linear.get()));
            style_class.with(|c| cn.add_user(c.as_deref()));
            pt.merge_class(&cn.finish())
        })
    };
    let style = {
        let pt = pt.clone();
        Signal::derive(move || {
            let merged = pt.merge_style_parts(&[&style.get().unwrap_or_default()]);
            (!merged.is_empty()).then_some(merged)
        })
    };
    view! {
        <div class={class} style={style} id={id} role="tablist" {..pt.attrs()}>
            {content}
        </div>
    }
}

/// Step list.
#[component]
pub fn StepList(
    /// Pass-through of the list.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Steps.
    children: Children,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    view! {
        <div class={pt.merge_class("p-steplist")} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Step: number, title and separator; inside a `StepItem` - without a separator.
#[component]
pub fn Step(
    /// Value linking the step to a panel; may be omitted inside a `StepItem`.
    #[prop(optional)]
    value: Option<u32>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Custom step view instead of the header button.
    #[prop(optional, into)]
    content: Option<Callback<StepContext, AnyView>>,
    /// Pass-through of the step parts.
    #[prop(optional)]
    pt: Option<StepPt>,
    /// Title.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = Arc::new(pt.unwrap_or_default());
    let (value, in_item) = resolve_value(value);
    if !in_item {
        ctx.steps.update(|s| s.push(value));
        let steps = ctx.steps;
        on_cleanup(move || {
            steps.try_update(|s| s.retain(|v| *v != value));
        });
    }
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let is_disabled = {
        let (linear, disabled) = (ctx.linear.clone(), disabled.clone());
        Signal::derive(move || step_disabled(active.get(), linear.get(), disabled.get()))
    };
    let activate = {
        let current = ctx.value;
        Callback::new(move |v: u32| current.set(Some(v)))
    };
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add(&step_classes(active.get(), is_disabled.get()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let separator = {
        let (steps, pt) = (ctx.steps, pt.clone());
        move || {
            (!in_item && steps.with(|s| separator_after(s, value))).then(|| {
                view! { <div class={pt.separator.merge_class("p-stepper-separator")} {..pt.separator.attrs()}></div> }
            })
        }
    };
    let body = match content {
        Some(template) => template.run(StepContext { value, active, activate }),
        None => {
            let (id, controls) = (step_id(&ctx.id, value), panel_id(&ctx.id, value));
            let click = move |_| activate.run(value);
            let tabindex = move || is_disabled.get().then_some("-1");
            let disabled_attr = move || is_disabled.get().then_some("");
            let (header_class, number_class, title_class) =
                (pt.header.merge_class("p-step-header"), pt.number.merge_class("p-step-number"), pt.title.merge_class("p-step-title"));
            let (header_attrs, number_attrs, title_attrs) = (pt.header.attrs(), pt.number.attrs(), pt.title.attrs());
            view! {
                <button
                    type="button"
                    id={id}
                    class={header_class}
                    role="tab"
                    aria-controls={controls}
                    tabindex={tabindex}
                    disabled={disabled_attr}
                    on:click={click}
                    {..header_attrs}
                >
                    <span class={number_class} {..number_attrs}>{value.to_string()}</span>
                    <span class={title_class} {..title_attrs}>{children.map(|c| c())}</span>
                </button>
            }
            .into_any()
        }
    };
    view! {
        <div
            class={class}
            role="presentation"
            aria-current={move || active.get().then_some("step")}
            data-p-active={move || bool_attr(active.get())}
            data-p-disabled={move || bool_attr(is_disabled.get())}
            {..pt.root.attrs()}
        >
            {body}
            {separator}
        </div>
    }
}

/// Panel container.
#[component]
pub fn StepPanels(
    /// Pass-through of the container.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Panels.
    children: Children,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    view! {
        <div class={pt.merge_class("p-steppanels")} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Step panel: in the horizontal layout only the active one has content, in the vertical one (inside `StepItem`) all of them do, and
/// inactive ones are hidden and expand with an animation.
#[component]
pub fn StepPanel(
    /// Value linking the panel to a step; may be omitted inside a `StepItem`.
    #[prop(optional)]
    value: Option<u32>,
    /// Custom content view; receives [`StepContext`].
    #[prop(optional, into)]
    content: Option<Callback<StepContext, AnyView>>,
    /// Pass-through of the panel parts.
    #[prop(optional)]
    pt: Option<StepPanelPt>,
    /// Content.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = Arc::new(pt.unwrap_or_default());
    let (value, vertical) = resolve_value(value);
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let activate = {
        let current = ctx.value;
        Callback::new(move |v: u32| current.set(Some(v)))
    };
    let context = StepContext { value, active, activate };
    let content_view = {
        let pt = pt.clone();
        move || {
            let inner = match (content, children.as_ref()) {
                (Some(template), _) => template.run(context),
                (None, Some(children)) => children().into_any(),
                (None, None) => ().into_any(),
            };
            let separator = (vertical).then(|| {
                let items = ctx.items;
                let pt = pt.clone();
                move || {
                    items
                        .with(|i| separator_after(i, value))
                        .then(|| view! { <div class={pt.separator.merge_class("p-stepper-separator")} {..pt.separator.attrs()}></div> })
                }
            });
            view! {
                <div class={pt.content_wrapper.merge_class("p-steppanel-content-wrapper")} {..pt.content_wrapper.attrs()}>
                    {separator}
                    <div class={pt.content.merge_class("p-steppanel-content")} {..pt.content.attrs()}>{inner}</div>
                </div>
            }
        }
    };
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add(panel_classes(vertical, active.get()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let (id, controls) = (panel_id(&ctx.id, value), step_id(&ctx.id, value));
    let data_active = move || bool_attr(active.get());
    if vertical {
        // `NodeRef` is needed only by the client (animation and `tabindex`): there is none during SSR, and `()` is an empty attribute instead of `node_ref`.
        #[cfg(target_arch = "wasm32")]
        let region = {
            let region = NodeRef::<leptos::html::Div>::new();
            prime_core::collapsible::bind_region_with(region, move || !active.get(), None);
            leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
        };
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(clippy::let_unit_value)]
        let region = {};
        let hidden = hidden_style(!active.get_untracked());
        view! {
            <div class={class} id={id} role="tabpanel" aria-controls={controls} data-p-active={data_active} data-pc-name="steppanel" {..pt.root.attrs()}>
                <div {..region} class="p-motion" style={hidden}>
                    {content_view()}
                </div>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class={class} id={id} role="tabpanel" aria-controls={controls} data-p-active={data_active} data-pc-name="steppanel" {..pt.root.attrs()}>
                {move || active.get().then(&content_view)}
            </div>
        }
        .into_any()
    }
}

/// Vertical layout item: wraps a [`Step`] and a [`StepPanel`] and hands them its value.
#[component]
pub fn StepItem(
    /// Item value.
    value: u32,
    /// Pass-through of the item.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Step and panel.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = pt.unwrap_or_default();
    ctx.items.update(|i| i.push(value));
    let items = ctx.items;
    on_cleanup(move || {
        items.try_update(|i| i.retain(|v| *v != value));
    });
    let content = scoped(|| {
        provide_context(ItemCtx { value });
        children().into_any()
    });
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let base = pt.clone();
    view! {
        <div
            class={move || base.merge_class(item_classes(active.get()))}
            data-p-active={move || bool_attr(active.get())}
            {..pt.attrs()}
        >
            {content}
        </div>
    }
}
