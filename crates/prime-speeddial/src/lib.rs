//! SpeedDial — port of `p-speeddial` from PrimeNG 21.1.10.
//!
//! A floating button that reveals a set of quick actions on press: in a straight line (`Linear`) or along an arc around the button
//! (`Circle`, `SemiCircle`, `QuarterCircle`) in one of eight directions. Items are `p-speeddial-action` buttons; the list is
//! controlled by mouse and keyboard (arrows along the layout direction, `Home`, `End`, `Enter`, `Space`, `Escape`). In a theme the root is
//! usually positioned manually (`position: absolute` in `style`).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_icons::PlusIcon;
//! use prime_speeddial::{SpeedDial, SpeedDialDirection, SpeedDialItem};
//!
//! let model = vec![SpeedDialItem::new().label("Add").icon(ViewFn::from(|| view! { <PlusIcon /> }))];
//! let html = Owner::new().with(|| view! { <SpeedDial model=model direction=SpeedDialDirection::Left /> }.to_html());
//! assert!(html.contains("p-speeddial-direction-left") && html.contains("p-speeddial-list") && html.contains("aria-label=\"Add\""));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_button::{ButtonSeverity, ButtonState, root_classes as button_classes, root_data_p as button_data_p};
use prime_icons::PlusIcon;
use prime_rs_core::props::Size;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt};
use prime_tooltip::{TooltipContent, TooltipOptions, tooltip};
use std::sync::Arc;

pub use state::{
    Action, IconChoice, SpeedDialDirection, SpeedDialType, TogglerAction, button_extra_classes, effective_radius, icon_choice,
    item_classes, item_style, list_action, list_inline_style, point_style, root_classes, root_inline_style, step, toggler_action,
    transition_delay,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Item command event.
#[derive(Clone)]
pub struct SpeedDialCommand {
    /// Original event.
    pub event: web_sys::Event,
    /// Item.
    pub item: SpeedDialItem,
}

/// SpeedDial item (instead of PrimeNG's `MenuItem`, without router fields).
#[derive(Clone)]
pub struct SpeedDialItem {
    /// Identifier (defaults to `<list id>_<index>`).
    pub id: Option<String>,
    /// Label (`aria-label` and tooltip text).
    pub label: Option<String>,
    /// Icon.
    pub icon: Option<ViewFn>,
    /// Disabled.
    pub disabled: bool,
    /// Whether the item is shown (`false` — class `p-hidden`).
    pub visible: bool,
    /// Custom `tabindex` of the item button.
    pub tabindex: Option<i32>,
    /// Command on activation.
    pub command: Option<Callback<SpeedDialCommand>>,
    /// Extra `li` class.
    pub style_class: Option<String>,
}

impl Default for SpeedDialItem {
    fn default() -> Self {
        Self { id: None, label: None, icon: None, disabled: false, visible: true, tabindex: None, command: None, style_class: None }
    }
}

impl SpeedDialItem {
    /// Empty item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Identifier.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Icon.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Whether the item is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Whether the item is shown.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// `tabindex` of the item button.
    pub fn tabindex(mut self, tabindex: i32) -> Self {
        self.tabindex = Some(tabindex);
        self
    }

    /// Command.
    pub fn command(mut self, command: impl Into<Callback<SpeedDialCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Extra class.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }
}

/// Context of the `item_content` view.
#[derive(Clone)]
pub struct SpeedDialItemContext {
    /// Item.
    pub item: SpeedDialItem,
    /// Item index.
    pub index: usize,
    /// Activates the item: runs the command and hides the list (like `toggleCallback($event, item)`).
    pub click: Callback<web_sys::Event>,
}

/// Pass-through of [`SpeedDial`] parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpeedDialPt {
    /// Root.
    pub root: Pt,
    /// Toggle button.
    pub button: Pt,
    /// List.
    pub list: Pt,
    /// Item.
    pub item: Pt,
    /// Item button.
    pub action: Pt,
    /// Mask.
    pub mask: Pt,
}

/// State shared by the handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    model: Prop<Vec<SpeedDialItem>>,
    direction: Prop<SpeedDialDirection>,
    hide_on_click_outside: Prop<bool>,
    disabled: Prop<bool>,
    visible: RwSignal<bool>,
    /// Focus position among the available items.
    focused: RwSignal<Option<usize>>,
    has_focus: RwSignal<bool>,
    container: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    button: NodeRef<leptos::html::Button>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    on_visible_change: Option<Callback<bool>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
}

impl Core {
    fn item_dom_id(&self, index: usize) -> String {
        let own = self.model.with(|m| m.get(index).and_then(|it| it.id.clone()));
        own.filter(|id| !id.is_empty()).unwrap_or_else(|| format!("{}_{index}", self.id))
    }

    /// Indices of available items (visible and not disabled).
    fn enabled(&self) -> Vec<usize> {
        self.model.get_untracked().iter().enumerate().filter(|(_, it)| it.visible && !it.disabled).map(|(i, _)| i).collect()
    }

    fn show(&self) {
        if self.visible.get_untracked() {
            return;
        }
        self.visible.set(true);
        if let Some(cb) = self.on_visible_change {
            cb.run(true);
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    fn hide(&self) {
        if !self.visible.get_untracked() {
            return;
        }
        self.visible.set(false);
        if let Some(cb) = self.on_visible_change {
            cb.run(false);
        }
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
    }

    /// Item click: command (if not disabled) and hide.
    fn item_click(&self, event: web_sys::Event, index: usize) {
        let Some(item) = self.model.get_untracked().get(index).cloned() else { return };
        if let (Some(cb), false) = (item.command, item.disabled) {
            cb.run(SpeedDialCommand { event, item });
        }
        self.hide();
    }

    fn focus_button(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(button) = self.button.get_untracked() {
            let _ = button.focus();
        }
    }

    fn focus_list(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(list) = self.list.get_untracked() {
            let _ = list.focus();
        }
    }

    /// Key in the list.
    fn list_key(&self, e: &web_sys::KeyboardEvent) {
        let Some(action) = list_action(self.direction.get_untracked(), &e.code()) else { return };
        match action {
            Action::Activate => {
                // A press on the item button itself is handled by the button (native click).
                let on_list = e.target().zip(e.current_target()).is_some_and(|(t, c)| t == c);
                if !on_list {
                    return;
                }
                let enabled = self.enabled();
                if let Some(index) = self.focused.get_untracked().and_then(|p| enabled.get(p).copied()) {
                    self.item_click(e.clone().into(), index);
                }
                self.has_focus.set(false);
                self.focused.set(None);
                self.focus_button();
            }
            Action::Hide => {
                self.hide();
                self.focus_button();
            }
            _ => self.focused.set(step(self.enabled().len(), self.focused.get_untracked(), action)),
        }
        e.prevent_default();
    }

    /// Key on the toggle button.
    fn toggler_key(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        let Some(action) = toggler_action(&e.code()) else { return };
        match action {
            TogglerAction::Hide => {
                self.hide();
                self.focus_button();
            }
            TogglerAction::OpenNext | TogglerAction::OpenPrev => {
                self.has_focus.set(true);
                self.show();
                let nav = if action == TogglerAction::OpenNext { Action::Next } else { Action::Prev };
                self.focused.set(step(self.enabled().len(), self.focused.get_untracked(), nav));
                // The list becomes focusable after the show classes are applied.
                let this = self.clone();
                set_timeout(move || this.focus_list(), std::time::Duration::ZERO);
                e.prevent_default();
            }
        }
    }

    /// Click outside the component while shown.
    #[cfg(target_arch = "wasm32")]
    fn bind_outside_click(self: &Arc<Self>) {
        let this = self.clone();
        let handle = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            if !this.visible.get_untracked() || !this.hide_on_click_outside.get_untracked() {
                return;
            }
            let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let inside = this.container.get_untracked().is_some_and(|c| c.contains(node.as_ref()));
            if !inside {
                this.hide();
            }
        });
        self.cleanups.update_value(|c| c.push(Box::new(move || handle.remove())));
    }
}

/// Floating button with an expanding set of actions.
///
/// Properties accept a value or a signal. Items and views are built on mount and when the model changes.
#[component]
pub fn SpeedDial(
    /// Items.
    #[prop(optional, into)]
    model: Prop<Vec<SpeedDialItem>>,
    /// Whether the list is shown; two-way signal.
    #[prop(optional)]
    visible: Option<RwSignal<bool>>,
    /// `id` attribute (list and item identifiers are built from it).
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra root class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Expansion direction.
    #[prop(optional, into)]
    direction: Prop<SpeedDialDirection>,
    /// Transition delay step of the items, ms.
    #[prop(optional, default = Prop::Static(30), into)]
    transition_delay: Prop<u32>,
    /// Layout type.
    #[prop(optional, into)]
    r#type: Prop<SpeedDialType>,
    /// Radius of the arc layout; `0` — `20 × item count`.
    #[prop(optional, into)]
    radius: Prop<f64>,
    /// Render a mask under the list.
    #[prop(optional, into)]
    mask: Prop<bool>,
    /// Disable the component.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Hide the list on click outside the component.
    #[prop(optional, default = Prop::Static(true), into)]
    hide_on_click_outside: Prop<bool>,
    /// Inline style of the button.
    #[prop(optional, into)]
    button_style: OptProp<String>,
    /// Extra button class.
    #[prop(optional, into)]
    button_class: OptProp<String>,
    /// Button color.
    #[prop(optional, into)]
    button_severity: OptProp<ButtonSeverity>,
    /// Inline style of the mask.
    #[prop(optional, into)]
    mask_style: OptProp<String>,
    /// Extra mask class.
    #[prop(optional, into)]
    mask_class: OptProp<String>,
    /// Button icon in the closed state.
    #[prop(optional, into)]
    show_icon: Option<ViewFn>,
    /// Button icon in the open state.
    #[prop(optional, into)]
    hide_icon: Option<ViewFn>,
    /// Rotate the button icon on open.
    #[prop(optional, default = Prop::Static(true), into)]
    rotate_animation: Prop<bool>,
    /// `aria-label` attribute of the button.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the button.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Item tooltip template (the text comes from the item label).
    #[prop(optional)]
    tooltip_options: Option<TooltipOptions>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<SpeedDialPt>,
    /// Custom button instead of the default one; receives the toggle callback.
    #[prop(optional)]
    button_content: Option<Callback<Callback<web_sys::MouseEvent>, AnyView>>,
    /// Custom item instead of the action button.
    #[prop(optional)]
    item_content: Option<Callback<SpeedDialItemContext, AnyView>>,
    /// Custom button icon instead of the "plus".
    #[prop(optional, into)]
    icon_content: Option<ViewFn>,
    /// The list is shown or hidden.
    #[prop(optional)]
    on_visible_change: Option<Callback<bool>>,
    /// Button click.
    #[prop(optional)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// The list is shown.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// The list is hidden.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let list_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_rs_core::ids::next_unique()));
    let core = Arc::new(Core {
        id: list_id.clone(),
        model: model.clone(),
        direction: direction.clone(),
        hide_on_click_outside,
        disabled: disabled.clone(),
        visible: visible.unwrap_or_default(),
        focused: RwSignal::new(None),
        has_focus: RwSignal::new(false),
        container: NodeRef::new(),
        list: NodeRef::new(),
        button: NodeRef::new(),
        cleanups: StoredValue::new_local(Vec::new()),
        on_visible_change,
        on_show,
        on_hide,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // The outside-click listener is attached only while shown; layout offsets are computed after mount, in the browser only.
    #[cfg(target_arch = "wasm32")]
    {
        let core2 = core.clone();
        let was = StoredValue::new(false);
        Effect::new(move |_| {
            let shown = core2.visible.get();
            if shown && !was.get_value() {
                was.set_value(true);
                let this = core2.clone();
                set_timeout(
                    move || {
                        if !this.visible.is_disposed() && this.visible.get_untracked() {
                            this.bind_outside_click();
                        }
                    },
                    std::time::Duration::ZERO,
                );
            } else if !shown && was.get_value() {
                was.set_value(false);
                core2.release();
            }
        });
        let (core3, kind) = (core.clone(), r#type.clone());
        Effect::new(move |_| {
            let (Some(container), Some(list)) = (core3.container.get(), core3.list.get()) else { return };
            if kind.get_untracked() == SpeedDialType::Linear {
                return;
            }
            let container: web_sys::HtmlElement = container.unchecked_into();
            let list: web_sys::HtmlElement = list.unchecked_into();
            let button =
                container.query_selector(".p-speeddial-button").ok().flatten().and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
            let first = list.query_selector(".p-speeddial-item").ok().flatten().and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
            if let (Some(button), Some(first)) = (button, first) {
                let wdiff = (f64::from(button.offset_width()) - f64::from(first.offset_width())).abs();
                let hdiff = (f64::from(button.offset_height()) - f64::from(first.offset_height())).abs();
                let _ = list.style().set_property("--item-diff-x", &format!("{}px", wdiff / 2.0));
                let _ = list.style().set_property("--item-diff-y", &format!("{}px", hdiff / 2.0));
            }
        });
    }

    let visible_sig = core.visible;
    let root_class = {
        let (pt, kind, direction, disabled) = (pt.clone(), r#type.clone(), direction.clone(), disabled.clone());
        move || {
            let extra = style_class.with(|c| c.clone());
            let merged = root_classes(kind.get(), direction.get(), visible_sig.get(), disabled.get(), extra.as_deref());
            pt.root.merge_class(&merged).into_owned()
        }
    };
    let root_style = {
        let (pt, direction) = (pt.clone(), direction.clone());
        move || {
            let own = style.with(|s| s.clone()).unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&root_inline_style(direction.get()), &own]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let list_style = {
        let (pt, direction) = (pt.clone(), direction.clone());
        move || {
            let merged = pt.list.merge_style(&list_inline_style(direction.get()));
            (!merged.is_empty()).then_some(merged)
        }
    };

    let toggle = {
        let core = core.clone();
        Callback::new(move |e: web_sys::MouseEvent| {
            if core.disabled.get_untracked() {
                return;
            }
            if core.visible.get_untracked() {
                core.hide();
            } else {
                core.show();
            }
            if let Some(cb) = on_click {
                cb.run(e);
            }
        })
    };

    let toggler = match button_content {
        Some(custom) => custom.run(toggle).into_any(),
        None => {
            let icon = {
                let (show_icon, hide_icon, icon_content) = (show_icon.clone(), hide_icon.clone(), icon_content.clone());
                move || match (icon_choice(visible_sig.get(), show_icon.is_some(), hide_icon.is_some()), &icon_content) {
                    (IconChoice::Hide, _) => hide_icon.clone().map_or_else(|| ().into_any(), |i| i.run()),
                    (IconChoice::Show, _) => show_icon.clone().map_or_else(|| ().into_any(), |i| i.run()),
                    (IconChoice::Default, Some(custom)) => custom.run(),
                    (IconChoice::Default, None) => view! { <PlusIcon /> }.into_any(),
                }
            };
            let class = {
                let (pt, severity, rotate, extra, has_hide) =
                    (pt.clone(), button_severity.clone(), rotate_animation.clone(), button_class.clone(), hide_icon.is_some());
                move || {
                    let state = ButtonState { has_icon: true, rounded: true, severity: severity.get(), ..ButtonState::default() };
                    let extra = extra.with(|c| c.clone());
                    let mut cn = ClassNames::new();
                    cn.add(&button_classes(&state))
                        .add(&button_extra_classes(rotate.get(), has_hide, extra.as_deref()))
                        .add_user(pt.button.extra_class());
                    cn.finish_cow().into_owned()
                }
            };
            let data = {
                let severity = button_severity.clone();
                move || {
                    button_data_p(&ButtonState { has_icon: true, rounded: true, severity: severity.get(), ..ButtonState::default() })
                        .into_owned()
                }
            };
            let btn_style = {
                let pt = pt.clone();
                move || {
                    let own = button_style.with(|s| s.clone()).unwrap_or_default();
                    let merged = pt.button.merge_style(&own);
                    (!merged.is_empty()).then_some(merged)
                }
            };
            let attrs: Vec<AnyAttribute> = {
                let controls = format!("{list_id}_list");
                vec![
                    custom_attribute("aria-expanded", move || Some(visible_sig.get().to_string())).into_any_attr(),
                    custom_attribute("aria-controls", move || Some(controls.clone())).into_any_attr(),
                    custom_attribute("aria-label", move || aria_label.get()).into_any_attr(),
                    custom_attribute("aria-labelledby", move || aria_labelledby.get()).into_any_attr(),
                ]
            };
            let pt_attrs = pt.button.attrs();
            let disabled_attr = disabled.clone();
            let on_keydown = {
                let core = core.clone();
                move |e: web_sys::KeyboardEvent| core.toggler_key(&e)
            };
            view! {
                <button
                    node_ref=core.button
                    type="button"
                    class={class}
                    data-p={data}
                    style={btn_style}
                    aria-haspopup="true"
                    disabled={move || disabled_attr.get().then_some("")}
                    on:click={move |e| toggle.run(e)}
                    on:keydown={on_keydown}
                    {..attrs}
                    {..pt_attrs}
                >
                    {icon}
                </button>
            }
            .into_any()
        }
    };

    let items = {
        let (core, pt, kind, direction, radius, step_ms) =
            (core.clone(), pt.clone(), r#type.clone(), direction.clone(), radius.clone(), transition_delay.clone());
        let tooltip_options = tooltip_options.map(Arc::new);
        move || -> Vec<AnyView> {
            let all = core.model.get();
            let length = all.len();
            let enabled: Vec<usize> = all.iter().enumerate().filter(|(_, it)| it.visible && !it.disabled).map(|(i, _)| i).collect();
            all.iter()
                .enumerate()
                .map(|(index, entry)| {
                    let position = enabled.iter().position(|i| *i == index);
                    let layout =
                        Layout { kind: kind.clone(), direction: direction.clone(), radius: radius.clone(), step: step_ms.clone(), length };
                    item_view(&core, &pt, entry, index, position, layout, item_content, tooltip_options.clone())
                })
                .collect()
        }
    };

    let mask_view = {
        let pt = pt.clone();
        move || {
            (mask.get() && visible_sig.get()).then(|| {
                let class = {
                    let extra = mask_class.with(|c| c.clone());
                    let mut cn = ClassNames::new();
                    cn.add("p-speeddial-mask p-overlay-mask p-overlay-mask-enter-active")
                        .add_user(extra.as_deref())
                        .add_user(pt.mask.extra_class());
                    cn.finish_cow().into_owned()
                };
                let style = mask_style.with(|s| pt.mask.merge_style(s.as_deref().unwrap_or_default()));
                let style = (!style.is_empty()).then_some(style);
                view! { <div class={class} style={style} {..pt.mask.attrs()}></div> }
            })
        }
    };

    let list_attrs: Vec<AnyAttribute> = {
        let core = core.clone();
        vec![
            custom_attribute("aria-activedescendant", move || {
                if !core.has_focus.get() {
                    return None;
                }
                let index = *core.enabled().get(core.focused.get()?)?;
                Some(core.item_dom_id(index))
            })
            .into_any_attr(),
        ]
    };
    let (root_attrs, list_pt_attrs) = (pt.root.attrs(), pt.list.attrs());
    let list_class = pt.list.merge_class("p-speeddial-list").into_owned();
    let (list_focus, list_blur, list_key) = {
        let (a, b, c) = (core.clone(), core.clone(), core.clone());
        (
            move |_| a.has_focus.set(true),
            move |_| {
                b.has_focus.set(false);
                b.focused.set(None);
            },
            move |e: web_sys::KeyboardEvent| c.list_key(&e),
        )
    };
    view! {
        <div node_ref=core.container class={root_class} style={root_style} {..root_attrs}>
            {toggler}
            <ul
                node_ref=core.list
                id={format!("{}_list", core.id)}
                class={list_class}
                style={list_style}
                role="menu"
                tabindex="-1"
                on:focus={list_focus}
                on:focusout={list_blur}
                on:keydown={list_key}
                {..list_attrs}
                {..list_pt_attrs}
            >
                {items}
            </ul>
        </div>
        {mask_view}
    }
}

/// Item layout parameters.
struct Layout {
    kind: Prop<SpeedDialType>,
    direction: Prop<SpeedDialDirection>,
    radius: Prop<f64>,
    step: Prop<u32>,
    length: usize,
}

/// Item: `li` with an action button (or a custom view) and an optional tooltip.
#[allow(clippy::too_many_arguments)]
fn item_view(
    core: &Arc<Core>,
    pt: &Arc<SpeedDialPt>,
    entry: &SpeedDialItem,
    index: usize,
    position: Option<usize>,
    layout: Layout,
    template: Option<Callback<SpeedDialItemContext, AnyView>>,
    tooltip_template: Option<Arc<TooltipOptions>>,
) -> AnyView {
    let (visible, focused, has_focus) = (core.visible, core.focused, core.has_focus);
    let is_focused = move || has_focus.get() && position.is_some() && focused.get() == position;
    let dom_id = core.item_dom_id(index);
    let class = {
        let (pt, extra, hidden) = (pt.clone(), entry.style_class.clone(), !entry.visible);
        move || {
            let merged = item_classes(hidden, is_focused(), extra.as_deref());
            pt.item.merge_class(&merged).into_owned()
        }
    };
    let style = {
        let pt = pt.clone();
        move || {
            let base = item_style(
                visible.get(),
                layout.kind.get(),
                layout.direction.get(),
                index,
                layout.length,
                layout.radius.get(),
                layout.step.get(),
            );
            pt.item.merge_style(&base)
        }
    };
    let click = {
        let core = core.clone();
        Callback::new(move |e: web_sys::Event| core.item_click(e, index))
    };
    let inner = match template {
        Some(custom) => custom.run(SpeedDialItemContext { item: entry.clone(), index, click }),
        None => action_button(pt, entry, click, visible),
    };
    let item_attrs = pt.item.attrs();
    match (tooltip_template, entry.label.clone()) {
        (Some(template), Some(label)) => {
            let mut options = (*template).clone();
            options.content = TooltipContent::Text(Prop::Static(label));
            view! {
                <li
                    id={dom_id}
                    class={class}
                    style={style}
                    role="menuitem"
                    data-p-active={move || is_focused().to_string()}
                    use:tooltip={options}
                    {..item_attrs}
                >
                    {inner}
                </li>
            }
            .into_any()
        }
        _ => view! {
            <li id={dom_id} class={class} style={style} role="menuitem" data-p-active={move || is_focused().to_string()} {..item_attrs}>
                {inner}
            </li>
        }
        .into_any(),
    }
}

/// Item action button: secondary, round, small.
fn action_button(pt: &Arc<SpeedDialPt>, entry: &SpeedDialItem, click: Callback<web_sys::Event>, list_visible: RwSignal<bool>) -> AnyView {
    let state = ButtonState {
        has_icon: entry.icon.is_some(),
        severity: Some(ButtonSeverity::Secondary),
        rounded: true,
        size: Some(Size::Small),
        ..ButtonState::default()
    };
    let class = {
        let mut cn = ClassNames::new();
        cn.add(&button_classes(&state)).add("p-speeddial-action").add_user(pt.action.extra_class());
        cn.finish_cow().into_owned()
    };
    let data = button_data_p(&state).into_owned();
    let icon = entry.icon.clone().map(|icon| icon.run());
    let (label, disabled, tabindex) = (entry.label.clone(), entry.disabled, entry.tabindex);
    let tab = move || if disabled || !list_visible.get() { None } else { Some(tabindex.unwrap_or(0).to_string()) };
    let attrs = pt.action.attrs();
    view! {
        <button
            type="button"
            role="menuitem"
            class={class}
            data-p={data}
            data-p-severity="secondary"
            aria-label={label}
            disabled={disabled.then_some("")}
            tabindex={tab}
            on:click={move |e: web_sys::MouseEvent| click.run(e.into())}
            {..attrs}
        >
            {icon}
        </button>
    }
    .into_any()
}
