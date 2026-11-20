//! Steps — port of PrimeNG 21.1.10 `p-steps`.
//!
//! Numbered wizard steps in a row: `nav.p-steps` > `ul.p-steps-list` > `li.p-steps-item` with a link, a number and a label. By
//! default the component only displays the current step (`readonly`); in interactive mode a click and the keys ←/→/`Home`/`End`/`Enter`/
//! `Space` switch steps. The current step is driven by the owner via `active_index` (a value or a signal).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_steps::{Steps, StepsItem};
//!
//! let model = vec![StepsItem::new().label("Personal"), StepsItem::new().label("Seat"), StepsItem::new().label("Payment")];
//! let html = Owner::new().with(|| view! { <Steps model=model active_index=1usize /> }.to_html());
//! assert!(html.contains("p-steps-item-active") && html.contains("aria-current=\"step\"") && html.contains(">3<"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{
    Click, aria_disabled, click_outcome, is_activation_key, is_item_disabled, item_classes, key_target, link_tabindex, root_classes,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Item command event.
#[derive(Clone)]
pub struct StepsCommand {
    /// Original event (click or key).
    pub event: web_sys::Event,
    /// Item.
    pub item: StepsItem,
    /// Item index.
    pub index: usize,
}

/// Steps item (instead of PrimeNG's `MenuItem`, without router fields).
#[derive(Clone, Default)]
pub struct StepsItem {
    /// `id` attribute of the item.
    pub id: Option<String>,
    /// Label.
    pub label: Option<String>,
    /// Link address; without it the link navigation is cancelled.
    pub url: Option<String>,
    /// `target` of the link.
    pub target: Option<String>,
    /// Disabled.
    pub disabled: bool,
    /// Custom `tabindex` of the link.
    pub tabindex: Option<i32>,
    /// Inline style of `li`.
    pub style: Option<String>,
    /// Command on item selection.
    pub command: Option<Callback<StepsCommand>>,
}

impl StepsItem {
    /// Empty item.
    pub fn new() -> Self {
        Self::default()
    }

    /// `id` attribute.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Link address.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// `target` of the link.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Whether the item is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `tabindex` of the link.
    pub fn tabindex(mut self, tabindex: i32) -> Self {
        self.tabindex = Some(tabindex);
        self
    }

    /// Inline style of `li`.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Command.
    pub fn command(mut self, command: impl Into<Callback<StepsCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }
}

/// Pass-through parts of [`Steps`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepsPt {
    /// `nav` root.
    pub root: Pt,
    /// List.
    pub list: Pt,
    /// Item `li`.
    pub item: Pt,
    /// Item link.
    pub item_link: Pt,
    /// Number.
    pub item_number: Pt,
    /// Label.
    pub item_label: Pt,
}

/// State shared by the handlers.
struct Core {
    model: Prop<Vec<StepsItem>>,
    readonly: Prop<bool>,
    active: Signal<usize>,
    active_signal: Option<RwSignal<usize>>,
    /// The link with keyboard focus (roving `tabindex`); `None` — the active item by default.
    focus_index: RwSignal<Option<usize>>,
    list: NodeRef<leptos::html::Ul>,
    on_change: Option<Callback<usize>>,
}

impl Core {
    fn item(&self, index: usize) -> Option<StepsItem> {
        self.model.get_untracked().get(index).cloned()
    }

    /// Click or `Enter`/`Space` on an item.
    fn activate(&self, event: web_sys::Event, index: usize) {
        let Some(item) = self.item(index) else { return };
        match click_outcome(item.disabled, self.readonly.get_untracked(), item.url.is_some()) {
            Click::Ignore => event.prevent_default(),
            Click::Activate { prevent_default } => {
                if let Some(sig) = self.active_signal {
                    sig.set(index);
                }
                if let Some(cb) = self.on_change {
                    cb.run(index);
                }
                if prevent_default {
                    event.prevent_default();
                }
                if let Some(cb) = item.command {
                    cb.run(StepsCommand { event, item, index });
                }
            }
        }
    }

    fn focus_link(&self, index: usize) {
        self.focus_index.set(Some(index));
        #[cfg(target_arch = "wasm32")]
        if let Some(list) = self.list.get_untracked()
            && let Some(link) = list
                .children()
                .item(index as u32)
                .and_then(|li| li.first_element_child())
                .and_then(|a| a.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = link.focus();
        }
    }

    fn key_down(&self, e: &web_sys::KeyboardEvent, index: usize) {
        let code = e.code();
        if let Some(target) = key_target(&code, index, self.model.get_untracked().len()) {
            self.focus_link(target);
            e.prevent_default();
        } else if code == "Tab" {
            self.focus_index.set(None);
        } else if is_activation_key(&code) {
            self.activate(e.clone().into(), index);
            e.prevent_default();
        }
    }
}

/// Wizard step indicator.
///
/// Properties accept a value or a signal. `active_index` is driven by the owner: a click in interactive mode reports the new index
/// via `on_active_index_change` and updates `active_index_signal` if it is set.
#[component]
pub fn Steps(
    /// Items.
    #[prop(optional, into)]
    model: Prop<Vec<StepsItem>>,
    /// Current step (zero-based).
    #[prop(optional, into)]
    active_index: Prop<usize>,
    /// Two-way signal of the current step: a click writes the index to it (instead of `active_index`, if set).
    #[prop(optional)]
    active_index_signal: Option<RwSignal<usize>>,
    /// Display only: a click on the steps does nothing.
    #[prop(optional, default = Prop::Static(true), into)]
    readonly: Prop<bool>,
    /// PrimeNG compatibility property (there it affects only router links).
    #[prop(optional, default = Prop::Static(true), into)]
    exact: Prop<bool>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<StepsPt>,
    /// The current step was chosen by the user.
    #[prop(optional)]
    on_active_index_change: Option<Callback<usize>>,
) -> impl IntoView {
    let _ = exact;
    let pt = Arc::new(pt.unwrap_or_default());
    let active: Signal<usize> = match active_index_signal {
        Some(sig) => sig.into(),
        None => Signal::derive({
            let active_index = active_index.clone();
            move || active_index.get()
        }),
    };
    let core = Arc::new(Core {
        model: model.clone(),
        readonly: readonly.clone(),
        active,
        active_signal: active_index_signal,
        focus_index: RwSignal::new(None),
        list: NodeRef::new(),
        on_change: on_active_index_change,
    });
    // A change of the current step returns the roving tabindex to it.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            core.active.track();
            core.focus_index.set(None);
        });
    }

    let root_class = {
        let (pt, readonly) = (pt.clone(), readonly.clone());
        move || {
            let extra = style_class.with(|c| c.clone());
            let merged = root_classes(readonly.get(), extra.as_deref());
            pt.root.merge_class(&merged)
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let items = {
        let (core, pt) = (core.clone(), pt.clone());
        move || core.model.get().into_iter().enumerate().map(|(index, item)| item_view(&core, &pt, index, item)).collect_view()
    };
    let (root_attrs, list_attrs) = (pt.root.attrs(), pt.list.attrs());
    let list_class = pt.list.merge_class("p-steps-list");
    view! {
        <nav class={root_class} style={root_style} data-pc-name="steps" {..root_attrs}>
            <ul node_ref=core.list class={list_class} data-pc-section="menu" {..list_attrs}>
                {items}
            </ul>
        </nav>
    }
}

fn item_view(core: &Arc<Core>, pt: &Arc<StepsPt>, index: usize, item: StepsItem) -> AnyView {
    let (active, readonly, focus_index) = (core.active, core.readonly.clone(), core.focus_index);
    let is_active = move || active.get() == index;
    let disabled = {
        let (item_disabled, readonly) = (item.disabled, readonly.clone());
        move || is_item_disabled(item_disabled, index, active.get(), readonly.get())
    };
    let class = {
        let (pt, disabled, is_active) = (pt.clone(), disabled.clone(), is_active);
        move || pt.item.merge_class(&item_classes(is_active(), disabled())).into_owned()
    };
    let tabindex = {
        let (item_disabled, item_tab) = (item.disabled, item.tabindex);
        move || {
            match focus_index.get() {
                Some(f) => {
                    if f == index {
                        0
                    } else {
                        -1
                    }
                }
                None => link_tabindex(item_disabled, item_tab, index, active.get()),
            }
            .to_string()
        }
    };
    let aria_dis = {
        let (item_disabled, readonly) = (item.disabled, readonly.clone());
        move || aria_disabled(item_disabled, index, active.get(), readonly.get()).to_string()
    };
    let (click_core, key_core) = (core.clone(), core.clone());
    let attrs: Vec<AnyAttribute> = vec![custom_attribute("aria-current", move || is_active().then_some("step")).into_any_attr()];
    let (item_attrs, link_attrs) = (pt.item.attrs(), pt.item_link.attrs());
    let (link_class, number_class, label_class) = (
        pt.item_link.merge_class("p-steps-item-link"),
        pt.item_number.merge_class("p-steps-item-number"),
        pt.item_label.merge_class("p-steps-item-label"),
    );
    let (number_attrs, label_attrs) = (pt.item_number.attrs(), pt.item_label.attrs());
    view! {
        <li class={class} style={item.style.clone()} id={item.id.clone()} data-pc-section="menuitem" {..attrs} {..item_attrs}>
            <a
                role="link"
                class={link_class}
                href={item.url.clone()}
                target={item.target.clone()}
                tabindex={tabindex}
                aria-expanded={move || is_active().to_string()}
                aria-disabled={aria_dis}
                on:click={move |e: web_sys::MouseEvent| click_core.activate(e.into(), index)}
                on:keydown={move |e: web_sys::KeyboardEvent| key_core.key_down(&e, index)}
                {..link_attrs}
            >
                <span class={number_class} {..number_attrs}>{index + 1}</span>
                <span class={label_class} {..label_attrs}>{item.label.clone()}</span>
            </a>
        </li>
    }
    .into_any()
}
