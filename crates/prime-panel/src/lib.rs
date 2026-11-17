//! Panel — port of `p-panel` from PrimeNG 21.1.10.
//!
//! Structure and classes match PrimeNG: header `p-panel-header` (title `p-panel-title`, actions block
//! `p-panel-header-actions` with the toggle button `p-panel-toggle-button`), content container `p-panel-content-container`
//! with wrapper `p-panel-content-wrapper`, content `p-panel-content` and footer `p-panel-footer`. Instead of the host `<p-panel>`
//! a `<div class="p-panel …">` is rendered.
//!
//! Collapsing (`toggleable`) is built on [`prime_rs_core::collapsible`]: the state is a two-way `collapsed` signal
//! (without it, an own state with the initial value `default_collapsed`), the button gets `aria-controls` and `aria-expanded`,
//! the region gets `role="region"`, `aria-labelledby`, `aria-hidden`; expanding and collapsing animate the classes
//! `p-collapsible-enter-active`/`p-collapsible-leave-active`, and `on_after_toggle` is called when the animation ends.
//!
//! Slots are `ViewFn` (analogues of the `#header`, `#icons`, `#footer` templates), `header_icons_template` receives `collapsed`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_panel::Panel;
//!
//! let collapsed = RwSignal::new(true);
//! let html = view! { <Panel header="Header" toggleable=true collapsed=collapsed><p>"Content"</p></Panel> }.to_html();
//! assert!(html.contains("p-panel-collapsed") && html.contains(r#"aria-expanded="false""#) && html.contains("Header"));
//! ```

mod state;

use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use prime_icons::{MinusIcon, PlusIcon};
use prime_rs_core::collapsible::{CollapseIds, CollapseState, aria_expanded, aria_hidden, region_tabindex};
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{IconPos, Toggler, actions_class, data_p, write_root_classes};

/// `on_before_toggle` event: the original event and the state before toggling.
#[derive(Clone, Debug)]
pub struct PanelBeforeToggle {
    /// Original `click` event.
    pub original_event: web_sys::MouseEvent,
    /// Value of `collapsed` before toggling.
    pub collapsed: bool,
}

/// `on_after_toggle` event: the new state after the animation ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PanelAfterToggle {
    /// Value of `collapsed` after toggling.
    pub collapsed: bool,
}

/// Pass-through for the parts of Panel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PanelPt {
    /// Root element.
    pub root: Pt,
    /// Header (`div.p-panel-header`).
    pub header: Pt,
    /// Title (`span.p-panel-title`).
    pub title: Pt,
    /// Actions block (`div.p-panel-header-actions`).
    pub header_actions: Pt,
    /// Toggle button (`button.p-panel-toggle-button`).
    pub toggle_button: Pt,
    /// Content container (`div.p-panel-content-container`).
    pub content_container: Pt,
    /// Content wrapper (`div.p-panel-content-wrapper`).
    pub content_wrapper: Pt,
    /// Content (`div.p-panel-content`).
    pub content: Pt,
    /// Footer (`div.p-panel-footer`).
    pub footer: Pt,
}

/// Container with a header, optional collapsing and a footer.
///
/// Text and boolean properties accept a value or a signal; `collapsed` is a two-way signal.
#[component]
pub fn Panel(
    /// `id` attribute of the root; the header and content identifiers are built from it. Unique by default.
    #[prop(optional, into)]
    id: Option<String>,
    /// Header text (`p-panel-title`).
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Whether the content can be collapsed.
    #[prop(optional, into)]
    toggleable: Prop<bool>,
    /// Two-way "collapsed" signal.
    #[prop(optional)]
    collapsed: Option<RwSignal<bool>>,
    /// Initial state when `collapsed` is not passed.
    #[prop(optional)]
    default_collapsed: bool,
    /// Position of the icons in the header.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Whether to show the header.
    #[prop(default = Prop::Static(true), into)]
    show_header: Prop<bool>,
    /// What toggles the panel.
    #[prop(optional, into)]
    toggler: Prop<Toggler>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Header slot: rendered after the title.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Actions slot: rendered in the actions block before the toggle.
    #[prop(optional, into)]
    icons_template: Option<ViewFn>,
    /// Toggle icon slot; receives `collapsed` and replaces the minus/plus icon.
    #[prop(optional, into)]
    header_icons_template: Option<Callback<bool, AnyView>>,
    /// Footer slot (`p-panel-footer`).
    #[prop(optional, into)]
    footer_template: Option<ViewFn>,
    /// Called before toggling.
    #[prop(optional, into)]
    on_before_toggle: Option<Callback<PanelBeforeToggle>>,
    /// Called after the toggle animation ends.
    #[prop(optional, into)]
    on_after_toggle: Option<Callback<PanelAfterToggle>>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<PanelPt>,
    /// Content (`p-panel-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static =
        all_static!(header, toggleable, show_header, icon_pos, toggler, style, style_class) && !toggleable.get() && collapsed.is_none();
    let state = CollapseState::new(collapsed, default_collapsed);
    let props = Arc::new(Props {
        ids: CollapseIds::new(id.as_deref()),
        state,
        header,
        toggleable,
        show_header,
        icon_pos,
        toggler,
        style,
        style_class,
        header_template,
        icons_template,
        header_icons_template,
        on_before_toggle,
        footer_template,
    });

    // `NodeRef` is needed only by the client (animation and `tabindex`): there is none during SSR, and `()` is an empty attribute instead of `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        if props.toggleable.get_untracked() {
            let after = on_after_toggle.map(|cb| Callback::new(move |collapsed| cb.run(PanelAfterToggle { collapsed })));
            prime_rs_core::collapsible::bind_region(region, state, after);
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {
        let _ = on_after_toggle;
    };

    let (root_id, content_id, labelled_by) = (props.ids.root.clone(), props.ids.content.clone(), props.ids.header.clone());
    let root_attrs = pt.root.attrs();
    let container_class = pt.content_container.merge_class("p-panel-content-container");
    let container_style = container_style_of(&props, &pt);
    let container_attrs = pt.content_container.attrs();
    let (wrapper_class, wrapper_attrs) = (pt.content_wrapper.merge_class("p-panel-content-wrapper"), pt.content_wrapper.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-panel-content"), pt.content.attrs());
    let content = children.map(|c| c());
    let footer = footer_of(&props, &pt);

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let header = header_of(&props, &pt);
        Either::Left(panel_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            header,
            region,
            container_class,
            container_style,
            content_id,
            labelled_by,
            None::<&'static str>,
            None::<&'static str>,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content,
            footer
        ))
    } else {
        let pt = Arc::new(pt);
        let (p, t) = (&props, &pt);
        let (class, style, data) = (rx(p, t, class_of), rx(p, t, style_of), rx(p, t, |p, _| data_of(p)));
        let header = rx(p, t, header_of);
        let hidden = rx(p, t, |p, _| p.toggleable.get().then(|| aria_hidden(p.state.get())));
        let tabindex = rx(p, t, |p, _| p.toggleable.get().then(|| region_tabindex(p.state.get())).flatten());
        Either::Right(panel_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            header,
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
            content,
            footer
        ))
    }
}

/// Panel markup: one for the static and reactive branches (only the attribute and header expressions differ).
macro_rules! panel_markup {
    (
        $root_id:expr, $class:expr, $style:expr, $data:expr, $root_attrs:expr, $header:expr, $region:expr, $container_class:expr,
        $container_style:expr, $content_id:expr, $labelled_by:expr, $hidden:expr, $tabindex:expr, $container_attrs:expr,
        $wrapper_class:expr, $wrapper_attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr, $footer:expr
    ) => {
        view! {
            <div id={$root_id} class={$class} style={$style} data-p={$data} {..$root_attrs}>
                {$header}
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
                        {$footer}
                    </div>
                </div>
            </div>
        }
    };
}
use panel_markup;

/// Closure over the instance properties: `Arc` clones and a function pointer without capture.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<PanelPt>,
    f: fn(&Arc<Props>, &PanelPt) -> T,
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

fn class_of(props: &Arc<Props>, pt: &PanelPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    let toggleable = props.toggleable.get();
    write_root_classes(&mut cn, toggleable, toggleable && props.state.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Arc<Props>, pt: &PanelPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Style of the content container: a panel collapsed at the first render is hidden right away; after that the animation controls it.
fn container_style_of(props: &Props, pt: &PanelPt) -> Option<String> {
    let hidden = if props.toggleable.get_untracked() { props.state.initial_style() } else { None };
    (hidden.is_some() || pt.content_container.has_style()).then(|| pt.content_container.merge_style(hidden.unwrap_or("")))
}

/// Footer: only when the slot is set.
fn footer_of(props: &Props, pt: &PanelPt) -> Option<AnyView> {
    props
        .footer_template
        .clone()
        .map(|slot| view! { <div class=pt.footer.merge_class("p-panel-footer") {..pt.footer.attrs()}>{slot.run()}</div> }.into_any())
}

/// Header: title, slot and the actions block with the toggle; reads `collapsed` only inside nested closures.
fn header_of(props: &Arc<Props>, pt: &PanelPt) -> Option<AnyView> {
    if !props.show_header.get() {
        return None;
    }
    let toggleable = props.toggleable.get();
    let label = props.header.get();
    let title = label.clone().map(|text| {
        view! { <span id=props.ids.header.clone() class=pt.title.merge_class("p-panel-title") {..pt.title.attrs()}>{text}</span> }
    });
    let slot = props.header_template.clone().map(|slot| slot.run());
    let icons = props.icons_template.clone().map(|slot| slot.run());
    let button = toggleable.then(|| toggle_button(props, pt, label.clone()));
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            if props.toggler.get_untracked() == Toggler::Header {
                toggle(&props, e);
            }
        }
    };
    let actions_class = pt.header_actions.merge_class(actions_class(props.icon_pos.get()));
    Some(
        view! {
            <div
                id=props.ids.titlebar.clone()
                class=pt.header.merge_class("p-panel-header")
                data-p=data_of(props)
                on:click={click}
                {..pt.header.attrs()}
            >
                {title}
                {slot}
                <div class={actions_class} {..pt.header_actions.attrs()}>{icons}{button}</div>
            </div>
        }
        .into_any(),
    )
}

/// Toggle button with `aria-controls` and `aria-expanded`; without a title it takes the header identifier.
fn toggle_button(props: &Arc<Props>, pt: &PanelPt, label: Option<String>) -> impl IntoView + use<> {
    let state = props.state;
    let template = props.header_icons_template;
    let id = label.is_none().then(|| props.ids.header.clone());
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            if props.toggler.get_untracked() == Toggler::Icon {
                toggle(&props, e);
            }
        }
    };
    let icon = move || {
        let collapsed = state.get();
        match template {
            Some(template) => EitherOf3::A(template.run(collapsed)),
            None if collapsed => EitherOf3::B(view! { <PlusIcon /> }),
            None => EitherOf3::C(view! { <MinusIcon /> }),
        }
    };
    view! {
        <button
            id=id
            type="button"
            class=pt.toggle_button.merge_class(
                "p-button p-component p-button-icon-only p-button-secondary p-button-rounded p-button-text p-panel-toggle-button",
            )
            data-p="icon-only rounded text"
            aria-label=label
            aria-controls=props.ids.content.clone()
            aria-expanded=move || aria_expanded(state.get())
            on:click={click}
            {..pt.toggle_button.attrs()}
        >
            <span class="p-button-icon" data-p="left">
                {icon}
            </span>
        </button>
    }
}

/// Toggles the panel: `on_before_toggle` with the previous state, then changes `collapsed`.
fn toggle(props: &Props, e: web_sys::MouseEvent) {
    if !props.toggleable.get_untracked() {
        return;
    }
    e.prevent_default();
    if let Some(cb) = props.on_before_toggle {
        cb.run(PanelBeforeToggle { original_event: e, collapsed: props.state.get_untracked() });
    }
    props.state.toggle();
}

/// Instance properties.
struct Props {
    ids: CollapseIds,
    state: CollapseState,
    header: OptProp<String>,
    toggleable: Prop<bool>,
    show_header: Prop<bool>,
    icon_pos: Prop<IconPos>,
    toggler: Prop<Toggler>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    header_template: Option<ViewFn>,
    icons_template: Option<ViewFn>,
    header_icons_template: Option<Callback<bool, AnyView>>,
    on_before_toggle: Option<Callback<PanelBeforeToggle>>,
    footer_template: Option<ViewFn>,
}
