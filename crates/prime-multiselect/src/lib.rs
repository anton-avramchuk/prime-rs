//! MultiSelect — port of `p-multiselect` from PrimeNG 21.1.10.
//!
//! Markup and classes as in PrimeNG: root `div.p-multiselect`, hidden `combobox` field, `p-multiselect-label` label (comma-separated
//! values or chips), clear icon, `p-multiselect-dropdown` button and `p-multiselect-overlay` panel with a header
//! ("select all" and search), an option list with checkboxes (`ul.p-multiselect-list`, groups, empty message) and hidden
//! focusable boundaries. The panel lives inside the root and is positioned through `prime-dom`.
//!
//! Values are a two-way signal `values: RwSignal<Vec<String>>`, panel openness is `open: RwSignal<bool>`; without them
//! the component keeps internal state. Options are given as a list `options` or as groups `groups`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_multiselect::{MultiSelect, MultiSelectOption};
//!
//! let values = RwSignal::new(vec!["NY".to_owned(), "LDN".to_owned()]);
//! let options = vec![MultiSelectOption::new("New York", "NY"), MultiSelectOption::new("London", "LDN")];
//! let html = view! { <MultiSelect options=options values=values placeholder="Select Cities" /> }.to_html();
//! assert!(html.contains("New York, London") && !html.contains("p-placeholder"));
//! ```

mod logic;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::either::Either;
use leptos::ev;
use leptos::html;
use leptos::leptos_dom::helpers::{TimeoutHandle, WindowListenerHandle};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_checkbox::CheckboxState;
use prime_chip::Chip;
use prime_core::ids::next_unique;
use prime_core::{ClassNames, DataP, OptProp, Prop, Pt, Size, Variant, ZGroup, use_prime_config};
use prime_dom::dom::{AlignMode, align_overlay};
use prime_icons::{CheckIcon, ChevronDownIcon, SearchIcon, SpinnerIcon, TimesIcon};
use std::sync::Arc;
use std::time::Duration;

pub use logic::{
    ChipContent, Entry, LabelContent, MultiSelectGroup, MultiSelectOption, RootState, all_options, all_selected, apply_limit, chip_content,
    comma_label, data_p, find_option, first_focused, first_valid, last_focused, last_valid, limit_reached, matches_filter,
    nearest_selected, next_valid, option_disabled, pos_in_set, prev_valid, search_match_from, select_all, select_range, selected_index,
    selected_labels, set_size, summary, toggle_all, toggle_value, visible_entries, write_label_classes, write_option_classes,
    write_root_classes,
};

/// How many milliseconds typed search characters accumulate.
const SEARCH_RESET_MS: u64 = 500;

/// How selected values are shown in the label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MultiSelectDisplay {
    /// Comma-separated labels.
    #[default]
    Comma,
    /// Chips with a remove button.
    Chip,
}

/// Selection change event: new values and the original event.
#[derive(Clone, Debug)]
pub struct MultiSelectChange {
    /// New values.
    pub values: Vec<String>,
    /// Original event.
    pub original_event: web_sys::Event,
}

/// Search event: the string and the original event.
#[derive(Clone, Debug)]
pub struct MultiSelectFilter {
    /// Search string.
    pub filter: String,
    /// Original `input` event.
    pub original_event: web_sys::Event,
}

/// Event for removing a value via a chip.
#[derive(Clone, Debug)]
pub struct MultiSelectRemove {
    /// Removed value.
    pub removed: String,
    /// Values after removal.
    pub values: Vec<String>,
}

/// "Select all" checkbox event.
#[derive(Clone, Debug)]
pub struct MultiSelectSelectAll {
    /// Whether all options are selected after the click.
    pub checked: bool,
    /// Original event.
    pub original_event: web_sys::Event,
}

/// Pass-through of MultiSelect parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MultiSelectPt {
    /// Root element.
    pub root: Pt,
    /// Hidden field container.
    pub hidden_input_container: Pt,
    /// Hidden field.
    pub hidden_input: Pt,
    /// Label container.
    pub label_container: Pt,
    /// Label.
    pub label: Pt,
    /// Chip item.
    pub chip_item: Pt,
    /// Chip.
    pub chip: Pt,
    /// Clear icon.
    pub clear_icon: Pt,
    /// Dropdown button.
    pub dropdown: Pt,
    /// Dropdown button icon.
    pub dropdown_icon: Pt,
    /// Loading icon.
    pub loading_icon: Pt,
    /// Panel.
    pub overlay: Pt,
    /// Panel header.
    pub header: Pt,
    /// Filter field container.
    pub filter_container: Pt,
    /// Filter field.
    pub filter: Pt,
    /// List container.
    pub list_container: Pt,
    /// List.
    pub list: Pt,
    /// Group header.
    pub option_group: Pt,
    /// Option.
    pub option: Pt,
    /// Empty list message.
    pub empty_message: Pt,
}

/// Selecting several values from a list in a dropdown panel.
///
/// Every property accepts a value or a signal; `values` and `open` are two-way signals.
#[component]
pub fn MultiSelect(
    /// Options (no groups).
    #[prop(optional, into)]
    options: Prop<Vec<MultiSelectOption>>,
    /// Option groups; if set, `options` is not used.
    #[prop(optional, into)]
    groups: Prop<Vec<MultiSelectGroup>>,
    /// Two-way signal of the selected values.
    #[prop(optional)]
    values: Option<RwSignal<Vec<String>>>,
    /// Initial values if `values` is not passed.
    #[prop(optional)]
    default_values: Vec<String>,
    /// Two-way signal of the panel openness.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    /// Hint shown when nothing is selected.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Label mode: comma-separated values or chips.
    #[prop(optional, into)]
    display: Prop<MultiSelectDisplay>,
    /// How many labels to show before switching to a summary (default 3).
    #[prop(optional, into)]
    max_selected_labels: OptProp<usize>,
    /// Summary when `max_selected_labels` is exceeded; `{0}` is replaced by the selected count (default from the translation).
    #[prop(optional, into)]
    selected_items_label: OptProp<String>,
    /// Show labels of the selected values (default `true`); otherwise the hint.
    #[prop(optional, into)]
    display_selected_label: OptProp<bool>,
    /// Maximum number of selected values.
    #[prop(optional, into)]
    selection_limit: OptProp<usize>,
    /// "Select all" checkbox in the header (default `true`).
    #[prop(optional, into)]
    show_toggle_all: OptProp<bool>,
    /// Panel header (default `true`).
    #[prop(optional, into)]
    show_header: OptProp<bool>,
    /// Search field in the header (default `true`).
    #[prop(optional, into)]
    filter: OptProp<bool>,
    /// Search field hint.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Reset the search string on close.
    #[prop(optional, into)]
    reset_filter_on_hide: Prop<bool>,
    /// Focus the search field on open.
    #[prop(optional, into)]
    autofocus_filter: Prop<bool>,
    /// Show the clear icon.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Loading in progress: a loading icon instead of the arrow, input is blocked.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Highlight the selected options (default `true`).
    #[prop(optional, into)]
    highlight_on_select: OptProp<bool>,
    /// Move focus to the option under the pointer (default `true`).
    #[prop(optional, into)]
    focus_on_hover: OptProp<bool>,
    /// Select an option when focus is moved to it with the keys.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Focus the first option when the panel opens.
    #[prop(optional, into)]
    auto_option_focus: Prop<bool>,
    /// List height above which scrolling appears (default `200px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Empty list message.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Empty search result message.
    #[prop(optional, into)]
    empty_filter_message: OptProp<String>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
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
    /// `id` attribute of the root; panel and option ids are built from it.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `id` attribute of the hidden field.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// `tabindex` attribute of the hidden field (default 0).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// `aria-label` attribute of the field.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the field.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// `aria-label` attribute of the search field.
    #[prop(optional, into)]
    aria_filter_label: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Inline style of the panel.
    #[prop(optional, into)]
    panel_style: OptProp<String>,
    /// Extra class of the panel.
    #[prop(optional, into)]
    panel_style_class: OptProp<String>,
    /// Dropdown button icon instead of the arrow.
    #[prop(optional, into)]
    dropdown_icon: Option<ViewFn>,
    /// Clear icon instead of the cross.
    #[prop(optional, into)]
    clear_icon: Option<ViewFn>,
    /// Chip remove icon.
    #[prop(optional, into)]
    chip_icon: Option<ViewFn>,
    /// Loading icon instead of the spinner.
    #[prop(optional, into)]
    loading_icon: Option<ViewFn>,
    /// Option content; receives the option and the selected flag.
    #[prop(optional, into)]
    item: Option<Callback<(MultiSelectOption, bool), AnyView>>,
    /// Group header content.
    #[prop(optional, into)]
    group: Option<Callback<MultiSelectGroup, AnyView>>,
    /// Label content; receives the selected options.
    #[prop(optional, into)]
    selected_items: Option<Callback<Vec<MultiSelectOption>, AnyView>>,
    /// Called after the user changes the selection.
    #[prop(optional, into)]
    on_change: Option<Callback<MultiSelectChange>>,
    /// Called on input in the search field.
    #[prop(optional, into)]
    on_filter: Option<Callback<MultiSelectFilter>>,
    /// Called when the field gains focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when the field loses focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Called on click on the field.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Called on clear.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Called after the panel opens.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Called after the panel closes.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Called when a value is removed via a chip.
    #[prop(optional, into)]
    on_remove: Option<Callback<MultiSelectRemove>>,
    /// Called on click on the "select all" checkbox.
    #[prop(optional, into)]
    on_select_all_change: Option<Callback<MultiSelectSelectAll>>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<MultiSelectPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let dom_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", next_unique()));
    let props = Arc::new(Props {
        options,
        groups,
        values: values.unwrap_or_else(|| RwSignal::new(default_values)),
        open: open.unwrap_or_else(|| RwSignal::new(false)),
        filter_text: RwSignal::new(String::new()),
        focused_index: RwSignal::new(None),
        start_range: RwSignal::new(None),
        focused: RwSignal::new(false),
        clicked: RwSignal::new(false),
        id: dom_id,
        placeholder,
        display,
        max_selected_labels,
        selected_items_label,
        display_selected_label,
        selection_limit,
        show_toggle_all,
        show_header,
        filter,
        filter_placeholder,
        reset_filter_on_hide,
        autofocus_filter,
        show_clear,
        loading,
        highlight_on_select,
        focus_on_hover,
        select_on_focus,
        auto_option_focus,
        scroll_height,
        empty_message,
        empty_filter_message,
        disabled,
        readonly,
        invalid,
        variant,
        config_variant: use_prime_config().input_variant,
        size,
        fluid,
        input_id,
        tabindex,
        aria_label,
        aria_labelledby,
        aria_filter_label,
        style,
        style_class,
        panel_style,
        panel_style_class,
        dropdown_icon,
        clear_icon,
        chip_icon,
        loading_icon,
        item,
        group,
        selected_items,
        on_change,
        on_filter,
        on_focus,
        on_blur,
        on_click,
        on_clear,
        on_show,
        on_hide,
        on_remove,
        on_select_all_change,
        default_empty: locale.text("emptyMessage").to_owned(),
        default_empty_filter: locale.text("emptyFilterMessage").to_owned(),
        default_summary: locale.text("selectionMessage").to_owned(),
        list_label: locale.text("aria.listLabel").to_owned(),
        select_all_label: locale.text("aria.selectAll").to_owned(),
        unselect_all_label: locale.text("aria.unselectAll").to_owned(),
        root_ref: NodeRef::new(),
        input_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        list_ref: NodeRef::new(),
        filter_ref: NodeRef::new(),
        search: StoredValue::new(String::new()),
        search_timer: StoredValue::new_local(None),
        listeners: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
    });
    // `window` subscriptions, the search timer and z-index live only while the panel is open; they are released on unmount.
    {
        let props = props.clone();
        on_cleanup(move || props.release());
    }
    // Side effects of opening and closing the panel (position, z-index, listeners, scrolling, focus) — browser only.
    #[cfg(target_arch = "wasm32")]
    {
        let props = props.clone();
        Effect::new(move |_| {
            let is_open = props.open.get();
            if is_open != props.was_open.get_value() {
                props.was_open.set_value(is_open);
                if is_open {
                    props.opened();
                } else {
                    props.closed();
                }
            }
        });
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_root_classes(&mut cn, &props.root_state());
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
    let data = {
        let props = props.clone();
        move || data_p(&props.root_state())
    };
    macro_rules! attr {
        ($name:literal, |$p:ident| $body:expr) => {{
            let $p = props.clone();
            reactive_attr($name, move || $body)
        }};
    }
    let input_attrs: Vec<AnyAttribute> = vec![
        attr!("id", |p| p.input_id.get()),
        attr!("aria-label", |p| p.aria_label.get()),
        attr!("aria-labelledby", |p| p.aria_labelledby.get()),
        attr!("aria-disabled", |p| Some(p.disabled.get().to_string())),
        attr!("aria-expanded", |p| Some(p.open.get().to_string())),
        attr!("aria-controls", |p| p.open.get().then(|| format!("{}_list", p.id))),
        attr!("disabled", |p| p.disabled.get().then(String::new)),
        attr!("tabindex", |p| Some(if p.disabled.get() { -1 } else { p.tabindex.get().unwrap_or(0) }.to_string())),
        attr!("aria-activedescendant", |p| p.active_descendant(p.focused.get())),
    ];
    let (focus, blur, key_down, container_click) = {
        let (a, b, c, d) = (props.clone(), props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::FocusEvent| a.input_focus(e),
            move |e: web_sys::FocusEvent| b.input_blur(e),
            move |e: web_sys::KeyboardEvent| c.key_down(e, false),
            move |e: web_sys::MouseEvent| d.container_click(e),
        )
    };
    let label_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let (shown, empty) = props.label_flags();
            let mut cn = ClassNames::new();
            write_label_classes(&mut cn, shown, empty);
            cn.add_user(pt.label.extra_class());
            cn.finish_cow()
        }
    };
    let label_data = {
        let props = props.clone();
        move || props.label_data()
    };
    let label_content = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.label_content(&pt)
    };
    let clear = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.clear_visible().then(|| clear_view(&props, &pt))
    };
    let dropdown_class = pt.dropdown.merge_class("p-multiselect-dropdown");
    let dropdown_content = {
        let (props, pt) = (props.clone(), pt.clone());
        move || dropdown_view(&props, &pt)
    };
    let panel = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.open.get().then(|| panel_view(&props, &pt))
    };
    let (root_attrs, hidden_container_attrs, hidden_attrs) = (pt.root.attrs(), pt.hidden_input_container.attrs(), pt.hidden_input.attrs());
    let (label_container_class, label_container_attrs, label_attrs) =
        (pt.label_container.merge_class("p-multiselect-label-container"), pt.label_container.attrs(), pt.label.attrs());
    let dropdown_attrs = pt.dropdown.attrs();
    let (root_id, root_ref, input_ref) = (props.id.clone(), props.root_ref, props.input_ref);

    view! {
        <div node_ref={root_ref} id={root_id} class={root_class} style={root_style} data-p={data} on:click={container_click} {..root_attrs}>
            <div class="p-hidden-accessible" data-p-hidden-accessible="true" {..hidden_container_attrs}>
                <input
                    node_ref={input_ref}
                    type="text"
                    readonly
                    role="combobox"
                    aria-haspopup="listbox"
                    on:focus={focus}
                    on:blur={blur}
                    on:keydown={key_down}
                    {..input_attrs}
                    {..hidden_attrs}
                />
            </div>
            <div class={label_container_class} {..label_container_attrs}>
                <div class={label_class} data-p={label_data} {..label_attrs}>
                    {label_content}
                </div>
            </div>
            {clear}
            <div class={dropdown_class} {..dropdown_attrs}>
                {dropdown_content}
            </div>
            {panel}
        </div>
    }
}

/// Boolean `data-p-*` attribute: `"true"` when true, otherwise no attribute.
fn flag(f: impl Fn() -> bool + Send + Sync + 'static) -> impl Fn() -> Option<&'static str> + Send + Sync + 'static {
    move || f().then_some("true")
}

/// Reactive attribute with an arbitrary name; `None` means no attribute.
fn reactive_attr(name: &'static str, f: impl Fn() -> Option<String> + Send + Sync + 'static) -> AnyAttribute {
    custom_attribute(name, f).into_any_attr()
}

/// Clear icon.
fn clear_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> impl IntoView + use<> {
    let class = pt.clear_icon.merge_class("p-multiselect-clear-icon");
    let attrs = pt.clear_icon.clone().attr("data-pc-section", "clearicon").attrs();
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.clear(e.into())
    };
    match props.clear_icon.clone() {
        Some(icon) => Either::Left(view! { <span class={class} aria-hidden="true" on:click={click} {..attrs}>{icon.run()}</span> }),
        None => Either::Right(view! { <TimesIcon class={class.into_owned()} on:click={click} {..attrs} /> }),
    }
}

/// Dropdown button content: loading or the arrow.
fn dropdown_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> AnyView {
    if props.loading.get() {
        let (class, attrs) = (pt.loading_icon.merge_class("p-multiselect-loading-icon"), pt.loading_icon.attrs());
        return match props.loading_icon.clone() {
            Some(icon) => view! { <span class={class} aria-hidden="true" {..attrs}>{icon.run()}</span> }.into_any(),
            None => view! { <SpinnerIcon spin=true class={class.into_owned()} {..attrs} /> }.into_any(),
        };
    }
    let class = pt.dropdown_icon.merge_class("p-multiselect-dropdown-icon");
    let icon_pt = match props.size.get() {
        Some(size) => pt.dropdown_icon.clone().attr("data-p", size.as_str()),
        None => pt.dropdown_icon.clone(),
    };
    let attrs = icon_pt.attrs();
    match props.dropdown_icon.clone() {
        Some(icon) => view! { <span class={class} aria-hidden="true" {..attrs}>{icon.run()}</span> }.into_any(),
        None => view! { <ChevronDownIcon class={class.into_owned()} {..attrs} /> }.into_any(),
    }
}

/// Panel: hidden boundaries, header, option list.
fn panel_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> impl IntoView + use<> {
    let class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-multiselect-overlay p-component-overlay p-component");
            props.panel_style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.overlay.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.panel_style.with(|st| pt.overlay.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let header = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.show_header.get().unwrap_or(true).then(|| header_view(&props, &pt))
    };
    let container_style = {
        let props = props.clone();
        move || format!("max-height: {}", props.scroll_height.get().unwrap_or_else(|| "200px".to_owned()))
    };
    let (container_class, list_class) =
        (pt.list_container.merge_class("p-multiselect-list-container"), pt.list.merge_class("p-multiselect-list"));
    let (list_attrs, container_attrs, overlay_attrs) = (pt.list.attrs(), pt.list_container.attrs(), pt.overlay.attrs());
    let list_label = props.list_label.clone();
    let items = {
        let (props, pt) = (props.clone(), pt.clone());
        move || items_view(&props, &pt)
    };
    let (first, last) = {
        let (a, b) = (props.clone(), props.clone());
        (move |e: web_sys::FocusEvent| a.first_hidden_focus(e), move |_: web_sys::FocusEvent| b.last_hidden_focus())
    };
    // A mouse press inside the panel must not move focus away from the hidden field, except a press in the search field.
    let mouse_down = move |e: web_sys::MouseEvent| {
        let on_input = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|el| el.tag_name().eq_ignore_ascii_case("INPUT") && el.class_list().contains("p-multiselect-filter"));
        if !on_input {
            e.prevent_default();
        }
    };
    let (overlay_ref, list_ref) = (props.overlay_ref, props.list_ref);
    let overlay_id = format!("{}_list", props.id);

    view! {
        <div node_ref={overlay_ref} id={overlay_id} class={class} style={style} data-p="overlay-self" on:mousedown={mouse_down} {..overlay_attrs}>
            <span
                role="presentation"
                class="p-hidden-accessible p-hidden-focusable"
                tabindex="0"
                data-p-hidden-accessible="true"
                data-p-hidden-focusable="true"
                on:focus={first}
            ></span>
            {header}
            <div class={container_class} style={container_style} {..container_attrs}>
                <ul node_ref={list_ref} class={list_class} role="listbox" aria-multiselectable="true" aria-label={list_label} {..list_attrs}>
                    {items}
                </ul>
            </div>
            <span
                role="presentation"
                class="p-hidden-accessible p-hidden-focusable"
                tabindex="0"
                data-p-hidden-accessible="true"
                data-p-hidden-focusable="true"
                on:focus={last}
            ></span>
        </div>
    }
}

/// Panel header: "select all" checkbox and search field.
fn header_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> impl IntoView + use<> {
    let (class, attrs) = (pt.header.merge_class("p-multiselect-header"), pt.header.attrs());
    let toggle = {
        let props = props.clone();
        move || {
            props.toggle_all_visible().then(|| {
                let (checked, disabled, click, label) = (props.clone(), props.clone(), props.clone(), props.clone());
                let text: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(move || label.toggle_all_text());
                checkbox_view(
                    props.variant_filled(),
                    Arc::new(move || checked.all_selected_now()),
                    Arc::new(move || disabled.disabled.get()),
                    Some(Callback::new(move |e: web_sys::MouseEvent| click.toggle_all_event(e.into()))),
                    Some(text),
                )
            })
        }
    };
    let search = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.filter.get().unwrap_or(true).then(|| filter_view(&props, &pt))
    };
    view! { <div class={class} {..attrs}>{toggle}{search}</div> }
}

/// Search field with an icon.
fn filter_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> impl IntoView + use<> {
    let (container_class, container_attrs) =
        (pt.filter_container.merge_class("p-iconfield p-multiselect-filter-container"), pt.filter_container.attrs());
    let (class, attrs) = (pt.filter.merge_class("p-inputtext p-component p-multiselect-filter"), pt.filter.attrs());
    let value = {
        let props = props.clone();
        move || props.filter_text.get()
    };
    let value_attr = value.clone();
    let placeholder = {
        let props = props.clone();
        move || props.filter_placeholder.get()
    };
    let aria_label = {
        let props = props.clone();
        move || props.aria_filter_label.get()
    };
    let active = {
        let props = props.clone();
        move || props.active_descendant(true)
    };
    let owns = format!("{}_list", props.id);
    let (input, key_down, blur) = {
        let (a, b, c) = (props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::Event| a.filter_input(e),
            move |e: web_sys::KeyboardEvent| b.key_down(e, true),
            move |_: web_sys::FocusEvent| c.filter_blur(),
        )
    };
    let filter_ref = props.filter_ref;
    view! {
        <span class={container_class} {..container_attrs}>
            <input
                node_ref={filter_ref}
                type="text"
                role="searchbox"
                autocomplete="off"
                class={class}
                value={value_attr}
                prop:value={value}
                placeholder={placeholder}
                aria-owns={owns}
                aria-label={aria_label}
                aria-activedescendant={active}
                on:input={input}
                on:keydown={key_down}
                on:blur={blur}
                {..attrs}
            />
            <span class="p-inputicon"><SearchIcon /></span>
        </span>
    }
}

/// Checkbox as in `Checkbox`: same markup and classes, state comes from closures. A click on the field does not change it by itself.
fn checkbox_view(
    filled: bool,
    checked: Arc<dyn Fn() -> bool + Send + Sync>,
    disabled: Arc<dyn Fn() -> bool + Send + Sync>,
    click: Option<Callback<web_sys::MouseEvent>>,
    aria_label: Option<Arc<dyn Fn() -> String + Send + Sync>>,
) -> AnyView {
    let variant = filled.then_some(Variant::Filled);
    let state = {
        let (checked, disabled) = (checked.clone(), disabled.clone());
        move || CheckboxState { checked: checked(), disabled: disabled(), invalid: false, variant, size: None }
    };
    let root_class = {
        let state = state.clone();
        move || {
            let mut cn = ClassNames::new();
            prime_checkbox::write_classes(&mut cn, &state());
            cn.finish_cow()
        }
    };
    let data = {
        let state = state.clone();
        move || prime_checkbox::data_p(&state())
    };
    let data_box = data.clone();
    let (highlight, highlight_checked) = (
        flag({
            let c = checked.clone();
            move || c()
        }),
        flag({
            let c = checked.clone();
            move || c()
        }),
    );
    let disabled_attr = {
        let disabled = disabled.clone();
        move || disabled().then_some("true")
    };
    let is_checked = {
        let checked = checked.clone();
        move || checked()
    };
    let is_checked_attr = is_checked.clone();
    let is_disabled = {
        let disabled = disabled.clone();
        move || disabled().then_some("")
    };
    let icon = {
        let checked = checked.clone();
        move || checked().then(|| view! { <CheckIcon class="p-checkbox-icon".to_owned() /> })
    };
    let on_click = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if let Some(cb) = click {
            cb.run(e);
        }
    };
    view! {
        <div class={root_class} data-p-highlight={highlight} data-p-checked={highlight_checked} data-p-disabled={disabled_attr} data-p={data}>
            <input
                type="checkbox"
                class="p-checkbox-input"
                tabindex="-1"
                aria-label={aria_label.map(|f| move || f())}
                checked={is_checked_attr}
                prop:checked={is_checked}
                disabled={is_disabled}
                on:click={on_click}
            />
            <div class="p-checkbox-box" data-p={data_box}>{icon}</div>
        </div>
    }
    .into_any()
}

/// Item list: options, group headers, empty list message. Reads only the options, groups and the search string.
fn items_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>) -> impl IntoView + use<> {
    let entries = props.entries();
    if entries.is_empty() {
        let (class, attrs) = (pt.empty_message.merge_class("p-multiselect-empty-message"), pt.empty_message.attrs());
        let text = props.empty_text();
        return Either::Left(view! { <li class={class} role="option" {..attrs}>{text}</li> });
    }
    let size = set_size(&entries);
    let views: Vec<AnyView> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| match entry {
            Entry::Group { index: group, label } => group_view(props, pt, index, *group, label),
            Entry::Option(option) => option_view(props, pt, index, option, size, pos_in_set(&entries, index)),
        })
        .collect();
    Either::Right(views)
}

/// Group header.
fn group_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>, index: usize, group: usize, label: &str) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let (class, attrs) = (pt.option_group.merge_class("p-multiselect-option-group"), pt.option_group.attrs());
    let content = match (props.group.as_ref(), props.groups.get_untracked().get(group)) {
        (Some(template), Some(g)) => template.run(g.clone()),
        _ => view! { <span>{label.to_owned()}</span> }.into_any(),
    };
    view! { <li id={id} class={class} role="option" {..attrs}>{content}</li> }.into_any()
}

/// Option.
fn option_view(props: &Arc<Props>, pt: &Arc<MultiSelectPt>, index: usize, option: &MultiSelectOption, size: usize, pos: usize) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let selected = {
        let (props, value) = (props.clone(), option.value.clone());
        move || props.is_selected(&value)
    };
    let focused = {
        let props = props.clone();
        move || props.focused_index.get() == Some(index)
    };
    let disabled = {
        let (props, option) = (props.clone(), option.clone());
        move || props.option_disabled_now(&option)
    };
    let class = {
        let (props, pt, selected, focused, disabled) = (props.clone(), pt.clone(), selected.clone(), focused.clone(), disabled.clone());
        move || {
            let mut cn = ClassNames::new();
            write_option_classes(&mut cn, selected() && props.highlight_on_select.get().unwrap_or(true), focused(), disabled());
            cn.add_user(pt.option.extra_class());
            cn.finish_cow()
        }
    };
    let (selected_attr, highlight_attr) = (flag(selected.clone()), flag(selected.clone()));
    let aria_selected = {
        let selected = selected.clone();
        move || selected().to_string()
    };
    let aria_checked = aria_selected.clone();
    let focused_attr = flag(focused);
    let disabled_attr = flag(disabled.clone());
    let checkbox = {
        let (selected, disabled) = (selected.clone(), disabled.clone());
        let label = option.label.clone();
        let text: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(move || label.clone());
        checkbox_view(props.variant_filled(), Arc::new(selected), Arc::new(disabled), None, Some(text))
    };
    let content = {
        let (props, option, selected) = (props.clone(), option.clone(), selected.clone());
        move || match props.item {
            Some(template) => template.run((option.clone(), selected())),
            None => view! { <span>{option.label.clone()}</span> }.into_any(),
        }
    };
    let (click, enter) = {
        let (a, b, option) = (props.clone(), props.clone(), option.clone());
        (
            move |e: web_sys::MouseEvent| a.select_option(&option, e.into(), Some(index)),
            move |_: web_sys::MouseEvent| b.option_mouse_enter(index),
        )
    };
    let attrs = pt.option.attrs();
    let (label, size, pos) = (option.label.clone(), size.to_string(), pos.to_string());
    view! {
        <li
            id={id}
            class={class}
            role="option"
            aria-label={label}
            aria-setsize={size}
            aria-posinset={pos}
            aria-selected={aria_selected}
            aria-checked={aria_checked}
            data-p-selected={selected_attr}
            data-p-highlight={highlight_attr}
            data-p-focused={focused_attr}
            data-p-disabled={disabled_attr}
            on:click={click}
            on:mouseenter={enter}
            {..attrs}
        >
            {checkbox}
            {content}
        </li>
    }
    .into_any()
}

/// Instance props and state.
struct Props {
    options: Prop<Vec<MultiSelectOption>>,
    groups: Prop<Vec<MultiSelectGroup>>,
    values: RwSignal<Vec<String>>,
    open: RwSignal<bool>,
    /// Search string.
    filter_text: RwSignal<String>,
    /// Index of the focused item in the flat list.
    focused_index: RwSignal<Option<usize>>,
    /// Range anchor index (`startRangeIndex`).
    start_range: RwSignal<Option<usize>>,
    /// Hidden field is focused.
    focused: RwSignal<bool>,
    /// Panel opened by the mouse (not used for navigation, but reset as in Select).
    clicked: RwSignal<bool>,
    id: String,
    placeholder: OptProp<String>,
    display: Prop<MultiSelectDisplay>,
    max_selected_labels: OptProp<usize>,
    selected_items_label: OptProp<String>,
    display_selected_label: OptProp<bool>,
    selection_limit: OptProp<usize>,
    show_toggle_all: OptProp<bool>,
    show_header: OptProp<bool>,
    filter: OptProp<bool>,
    filter_placeholder: OptProp<String>,
    reset_filter_on_hide: Prop<bool>,
    autofocus_filter: Prop<bool>,
    show_clear: Prop<bool>,
    loading: Prop<bool>,
    highlight_on_select: OptProp<bool>,
    focus_on_hover: OptProp<bool>,
    select_on_focus: Prop<bool>,
    auto_option_focus: Prop<bool>,
    scroll_height: OptProp<String>,
    empty_message: OptProp<String>,
    empty_filter_message: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    invalid: Prop<bool>,
    variant: OptProp<Variant>,
    config_variant: Option<Variant>,
    size: OptProp<Size>,
    fluid: Prop<bool>,
    input_id: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    aria_filter_label: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    panel_style: OptProp<String>,
    panel_style_class: OptProp<String>,
    dropdown_icon: Option<ViewFn>,
    clear_icon: Option<ViewFn>,
    chip_icon: Option<ViewFn>,
    loading_icon: Option<ViewFn>,
    item: Option<Callback<(MultiSelectOption, bool), AnyView>>,
    group: Option<Callback<MultiSelectGroup, AnyView>>,
    selected_items: Option<Callback<Vec<MultiSelectOption>, AnyView>>,
    on_change: Option<Callback<MultiSelectChange>>,
    on_filter: Option<Callback<MultiSelectFilter>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    on_click: Option<Callback<web_sys::MouseEvent>>,
    on_clear: Option<Callback<()>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_remove: Option<Callback<MultiSelectRemove>>,
    on_select_all_change: Option<Callback<MultiSelectSelectAll>>,
    default_empty: String,
    default_empty_filter: String,
    default_summary: String,
    list_label: String,
    select_all_label: String,
    unselect_all_label: String,
    root_ref: NodeRef<html::Div>,
    input_ref: NodeRef<html::Input>,
    overlay_ref: NodeRef<html::Div>,
    list_ref: NodeRef<html::Ul>,
    filter_ref: NodeRef<html::Input>,
    /// Accumulated type-ahead search characters.
    search: StoredValue<String>,
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    /// `window` subscriptions while the panel is open.
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    /// z-index assigned to the panel.
    z_index: StoredValue<Option<i32>>,
    /// Previous openness state (for side effects only on change).
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    was_open: StoredValue<bool>,
}

impl Props {
    // ---- data ----

    /// Flat list of visible items ignoring the selection limit (subscribes to options, groups and the search string).
    fn entries(&self) -> Vec<Entry> {
        visible_entries(&self.options.get(), &self.groups.get(), &self.filter_text.get())
    }

    fn entries_untracked(&self) -> Vec<Entry> {
        untrack(|| self.entries())
    }

    /// List for navigation and selection: unselected items are disabled when the limit is reached.
    fn nav_entries(&self) -> Vec<Entry> {
        apply_limit(self.entries_untracked(), &self.values.get_untracked(), self.limit_untracked())
    }

    fn all(&self) -> Vec<MultiSelectOption> {
        all_options(&self.options.get(), &self.groups.get())
    }

    fn all_untracked(&self) -> Vec<MultiSelectOption> {
        untrack(|| self.all())
    }

    /// Selected values that exist among the options, in selection order (subscribes).
    fn known_values(&self) -> Vec<String> {
        let all = self.all();
        self.values.get().into_iter().filter(|v| all.iter().any(|o| &o.value == v)).collect()
    }

    fn limit_untracked(&self) -> Option<usize> {
        self.selection_limit.get_untracked()
    }

    fn is_selected(&self, value: &str) -> bool {
        self.values.with(|vs| vs.iter().any(|v| v == value))
    }

    /// Whether an option is disabled (its own `disabled` or the selection limit), subscribes.
    fn option_disabled_now(&self, option: &MultiSelectOption) -> bool {
        self.values.with(|vs| option_disabled(option, vs, self.selection_limit.get()))
    }

    fn all_selected_now(&self) -> bool {
        all_selected(&self.entries(), &self.values.get())
    }

    fn variant_filled(&self) -> bool {
        self.variant.get().or(self.config_variant) == Some(Variant::Filled)
    }

    fn root_state(&self) -> RootState {
        RootState {
            chip: self.display.get() == MultiSelectDisplay::Chip,
            disabled: self.disabled.get(),
            invalid: self.invalid.get(),
            variant_filled: self.variant_filled(),
            focused: self.focused.get(),
            filled: self.values.with(|v| !v.is_empty()),
            open: self.open.get(),
            fluid: self.fluid.get(),
            size: self.size.get(),
        }
    }

    fn summary_template(&self) -> String {
        self.selected_items_label.get().filter(|l| !l.is_empty()).unwrap_or_else(|| self.default_summary.clone())
    }

    fn max_labels(&self) -> Option<usize> {
        Some(self.max_selected_labels.get().unwrap_or(3))
    }

    fn placeholder_text(&self) -> Option<String> {
        self.placeholder.get().filter(|p| !p.is_empty())
    }

    /// Label content by mode.
    fn content(&self) -> Content {
        let known = self.known_values();
        let placeholder = self.placeholder_text();
        let template = self.summary_template();
        match self.display.get() {
            MultiSelectDisplay::Comma => {
                let labels = selected_labels(&self.all(), &known);
                Content::Comma(comma_label(
                    &labels,
                    self.max_labels(),
                    &template,
                    placeholder.as_deref(),
                    self.display_selected_label.get().unwrap_or(true),
                ))
            }
            MultiSelectDisplay::Chip => Content::Chip(chip_content(&known, self.max_labels(), &template, placeholder.as_deref())),
        }
    }

    /// (whether the hint is shown, label is empty).
    fn label_flags(&self) -> (bool, bool) {
        let empty = self.known_values().is_empty();
        let placeholder = match self.content() {
            Content::Comma(LabelContent::Placeholder(p)) | Content::Chip(ChipContent::Placeholder(p)) => Some(p),
            _ => None,
        };
        (placeholder.as_ref().is_some_and(Option::is_some), placeholder.as_ref().is_some_and(Option::is_none) && empty)
    }

    fn label_data(&self) -> String {
        let (shown, empty) = self.label_flags();
        let chips = matches!(self.content(), Content::Chip(ChipContent::Chips(_)));
        let mut d = DataP::new();
        d.add_if(shown, "placeholder")
            .add_if(self.show_clear.get(), "clearable")
            .add_if(self.disabled.get(), "disabled")
            .add_opt(self.size.get().map(Size::as_str))
            .add_if(chips, "has-chip")
            .add_if(empty, "empty");
        d.finish()
    }

    fn label_content(self: &Arc<Self>, pt: &Arc<MultiSelectPt>) -> AnyView {
        if let Some(template) = self.selected_items {
            let known = self.known_values();
            if !known.is_empty() {
                let all = self.all();
                let chosen: Vec<MultiSelectOption> = known.iter().filter_map(|v| all.iter().find(|o| &o.value == v).cloned()).collect();
                return template.run(chosen);
            }
        }
        match self.content() {
            Content::Comma(LabelContent::Joined(text) | LabelContent::Summary(text)) => text.into_any(),
            Content::Comma(LabelContent::Placeholder(p)) | Content::Chip(ChipContent::Placeholder(p)) => {
                p.unwrap_or_else(|| "empty".to_owned()).into_any()
            }
            Content::Chip(ChipContent::Summary(text)) => text.into_any(),
            Content::Chip(ChipContent::Chips(values)) => {
                let all = self.all();
                let removable = !self.disabled.get() && !self.readonly.get();
                let views: Vec<AnyView> = values
                    .iter()
                    .map(|value| {
                        let label = all.iter().find(|o| &o.value == value).map(|o| o.label.clone()).unwrap_or_default();
                        let (class, attrs) = (pt.chip_item.merge_class("p-multiselect-chip-item"), pt.chip_item.attrs());
                        let remove = {
                            let (props, value) = (self.clone(), value.clone());
                            Callback::new(move |e: web_sys::Event| props.remove_value(&value, e))
                        };
                        let chip = match self.chip_icon.clone() {
                            Some(icon) => view! {
                                <Chip label=label removable=removable remove_icon=icon style_class="p-multiselect-chip" on_remove=remove />
                            }
                            .into_any(),
                            None => view! { <Chip label=label removable=removable style_class="p-multiselect-chip" on_remove=remove /> }
                                .into_any(),
                        };
                        view! { <div class={class} {..attrs}>{chip}</div> }.into_any()
                    })
                    .collect();
                views.into_any()
            }
        }
    }

    fn clear_visible(&self) -> bool {
        self.show_clear.get() && !self.disabled.get() && !self.readonly.get() && self.values.with(|v| !v.is_empty())
    }

    fn empty_text(&self) -> String {
        if self.filter_text.get().trim().is_empty() {
            self.empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty.clone())
        } else {
            self.empty_filter_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty_filter.clone())
        }
    }

    /// Id of the focused option (for `aria-activedescendant`); `enabled` is the focused item.
    fn active_descendant(&self, enabled: bool) -> Option<String> {
        let index = self.focused_index.get()?;
        enabled.then(|| format!("{}_{}", self.id, index))
    }

    fn toggle_all_text(&self) -> String {
        if self.all_selected_now() { self.select_all_label.clone() } else { self.unselect_all_label.clone() }
    }

    /// Whether to show the "select all" checkbox: enabled and no selection limit.
    fn toggle_all_visible(&self) -> bool {
        self.show_toggle_all.get().unwrap_or(true) && self.selection_limit.get().is_none_or(|l| l == 0)
    }

    /// Disabled, read-only or loading: input is blocked.
    fn blocked(&self) -> bool {
        self.disabled.get_untracked() || self.readonly.get_untracked() || self.loading.get_untracked()
    }

    fn focus_input(&self) {
        if let Some(input) = self.input_ref.get_untracked() {
            let _ = input.focus();
        }
    }

    // ---- open and close ----

    /// `show()`: opens the panel; focused option stays the previous one, otherwise the first (`auto_option_focus`) or the selected one.
    fn show(&self, focus: bool) {
        self.open.set(true);
        if self.focused_index.get_untracked().is_none() {
            let entries = self.nav_entries();
            let values = self.values.get_untracked();
            let index =
                if self.auto_option_focus.get_untracked() { first_focused(&entries, &values) } else { selected_index(&entries, &values) };
            self.focused_index.set(index);
        }
        if focus {
            self.focus_input();
        }
    }

    /// `hide()`: closes the panel and resets the list state.
    fn hide(&self, focus: bool) {
        self.open.set(false);
        self.focused_index.set(None);
        self.start_range.set(None);
        self.clicked.set(false);
        self.search.set_value(String::new());
        if self.reset_filter_on_hide.get_untracked() {
            self.filter_text.set(String::new());
        }
        if focus {
            self.focus_input();
        }
    }

    /// The panel has just opened: z-index, position, subscriptions, scrolling and search field focus, `on_show`.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn opened(self: &Arc<Self>) {
        if let (Some(overlay), Some(root)) = (self.overlay_ref.get_untracked(), self.root_ref.get_untracked()) {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let overlay: web_sys::HtmlElement = overlay.unchecked_into();
            let _ = overlay.style().set_property("z-index", &z.to_string());
            align_overlay(&overlay, &root.unchecked_into::<web_sys::HtmlElement>(), AlignMode::Relative, true);
        }
        self.bind_window();
        if let Some(index) = selected_index(&self.entries_untracked(), &self.values.get_untracked()) {
            self.scroll_to(index);
        }
        if self.filter.get_untracked().unwrap_or(true)
            && self.autofocus_filter.get_untracked()
            && let Some(input) = self.filter_ref.get_untracked()
        {
            let _ = input.focus();
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// The panel has just closed: subscriptions and z-index are released, `on_hide`.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn closed(&self) {
        self.release();
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    /// Releases the `window` subscriptions, the search timer and z-index.
    fn release(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Closing on a click outside the component and on window resize.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let outside = window_event_listener(ev::click, move |e: web_sys::MouseEvent| {
            let inside = this.root_ref.get_untracked().is_some_and(|root| {
                e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|node| root.contains(Some(&node)))
            });
            if !inside && this.open.get_untracked() {
                this.hide(false);
            }
        });
        let this = self.clone();
        let resize = window_event_listener(ev::resize, move |_| {
            if this.open.get_untracked() {
                this.hide(false);
            }
        });
        self.listeners.update_value(|l| l.extend([outside, resize]));
    }

    /// Recomputes the panel position after the list or label is re-rendered.
    fn realign(self: &Arc<Self>) {
        let this = self.clone();
        set_timeout(
            move || {
                if this.open.is_disposed() || !this.open.get_untracked() {
                    return;
                }
                if let (Some(overlay), Some(root)) = (this.overlay_ref.get_untracked(), this.root_ref.get_untracked()) {
                    align_overlay(
                        &overlay.unchecked_into::<web_sys::HtmlElement>(),
                        &root.unchecked_into::<web_sys::HtmlElement>(),
                        AlignMode::Relative,
                        true,
                    );
                }
            },
            Duration::ZERO,
        );
    }

    fn scroll_to(&self, index: usize) {
        if let Some(el) = leptos::prelude::document().get_element_by_id(&format!("{}_{}", self.id, index)) {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            options.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    // ---- selection ----

    /// `updateModel`: writes the values into the model and calls `on_change`.
    fn update_model(self: &Arc<Self>, values: Vec<String>, event: web_sys::Event) {
        self.values.set(values.clone());
        if let Some(cb) = self.on_change {
            cb.run(MultiSelectChange { values, original_event: event });
        }
        if self.open.get_untracked() {
            self.realign();
        }
    }

    /// `onOptionSelect`: toggles the option value.
    fn select_option(self: &Arc<Self>, option: &MultiSelectOption, event: web_sys::Event, index: Option<usize>) {
        if self.disabled.get_untracked() || self.readonly.get_untracked() {
            return;
        }
        let current = self.values.get_untracked();
        if option_disabled(option, &current, self.limit_untracked()) {
            return;
        }
        self.update_model(toggle_value(&current, &option.value), event);
        if index.is_some() {
            self.focused_index.set(index);
        }
    }

    /// Removing a value via a chip (`removeOption`).
    fn remove_value(self: &Arc<Self>, value: &str, event: web_sys::Event) {
        event.stop_propagation();
        if self.disabled.get_untracked() || self.readonly.get_untracked() {
            return;
        }
        let new: Vec<String> = self.values.get_untracked().into_iter().filter(|v| v != value).collect();
        self.update_model(new.clone(), event);
        if let Some(cb) = self.on_remove {
            cb.run(MultiSelectRemove { removed: value.to_owned(), values: new });
        }
    }

    /// Clear: values are empty, `on_change`, `on_clear`, search string reset.
    fn clear(self: &Arc<Self>, event: web_sys::Event) {
        event.stop_propagation();
        self.filter_text.set(String::new());
        self.update_model(Vec::new(), event);
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }

    /// `onToggleAll`: click on the "select all" checkbox.
    fn toggle_all_event(self: &Arc<Self>, event: web_sys::Event) {
        if self.disabled.get_untracked() || self.readonly.get_untracked() {
            return;
        }
        let all = self.all_untracked();
        let entries = self.entries_untracked();
        let new = toggle_all(&all, &entries, &self.values.get_untracked(), self.filter.get_untracked().unwrap_or(true));
        let everything = !new.is_empty() && new.len() == all.len();
        self.update_model(new.clone(), event.clone());
        if (new.is_empty() || everything)
            && let Some(cb) = self.on_select_all_change
        {
            cb.run(MultiSelectSelectAll { checked: !new.is_empty(), original_event: event });
        }
    }

    // ---- focus ----

    /// Moves the highlight to an item without selecting and without scrolling (mouse).
    fn option_mouse_enter(&self, index: usize) {
        if self.focus_on_hover.get_untracked().unwrap_or(true) && !self.disabled.get_untracked() {
            self.focused_index.set(Some(index));
        }
    }

    /// `changeFocusedOptionIndex`: moves focus with the keys; with `select_on_focus` it also selects.
    fn change_focused(self: &Arc<Self>, event: &web_sys::Event, index: Option<usize>, shift: bool) {
        if self.focused_index.get_untracked() == index {
            return;
        }
        self.focused_index.set(index);
        if let Some(i) = index {
            self.scroll_to(i);
            if self.select_on_focus.get_untracked()
                && !shift
                && let Some(Entry::Option(option)) = self.entries_untracked().get(i).cloned()
            {
                self.select_option(&option, event.clone(), None);
            }
        }
    }

    fn input_focus(&self, e: web_sys::FocusEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        self.focused.set(true);
        if self.focused_index.get_untracked().is_none() && self.open.get_untracked() && self.auto_option_focus.get_untracked() {
            let entries = self.nav_entries();
            self.focused_index.set(first_focused(&entries, &self.values.get_untracked()));
        }
        if self.open.get_untracked()
            && let Some(i) = self.focused_index.get_untracked()
        {
            self.scroll_to(i);
        }
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn input_blur(&self, e: web_sys::FocusEvent) {
        self.focused.set(false);
        self.focused_index.set(None);
        self.search.set_value(String::new());
        if let Some(cb) = self.on_blur {
            cb.run(e);
        }
    }

    fn filter_blur(&self) {
        self.focused_index.set(None);
        self.start_range.set(None);
    }

    /// The first hidden boundary got focus: from the field to the search field, otherwise back to the field.
    fn first_hidden_focus(&self, e: web_sys::FocusEvent) {
        let from_input = e
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .is_some_and(|node| self.input_ref.get_untracked().is_some_and(|input| input.is_same_node(Some(&node))));
        match (from_input, self.filter_ref.get_untracked()) {
            (true, Some(filter)) => {
                let _ = filter.focus();
            }
            _ => self.focus_input(),
        }
    }

    /// The last hidden boundary got focus: the panel closes, focus goes to the field.
    fn last_hidden_focus(&self) {
        self.hide(true);
    }

    // ---- container events ----

    fn container_click(&self, e: web_sys::MouseEvent) {
        if self.blocked() {
            return;
        }
        let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        if target.tag_name().eq_ignore_ascii_case("INPUT")
            || target.closest("[data-pc-section=\"clearicon\"], .p-chip-remove-icon").ok().flatten().is_some()
        {
            return;
        }
        let in_overlay = self.overlay_ref.get_untracked().is_some_and(|overlay| overlay.contains(Some(&target)));
        if !in_overlay {
            if self.open.get_untracked() {
                self.hide(true);
            } else {
                self.show(true);
            }
            self.focus_input();
            if let Some(cb) = self.on_click {
                cb.run(e);
            }
        }
    }

    // ---- search ----

    fn filter_input(self: &Arc<Self>, e: web_sys::Event) {
        let Some(input) = self.filter_ref.get_untracked() else { return };
        let text = input.value();
        self.filter_text.set(text.clone());
        self.focused_index.set(None);
        self.start_range.set(None);
        if let Some(cb) = self.on_filter {
            cb.run(MultiSelectFilter { filter: text, original_event: e });
        }
        self.realign();
    }

    /// Type-ahead search (`searchOptions`): characters accumulate for 500 ms.
    fn search_by_typing(self: &Arc<Self>, event: &web_sys::Event, key: &str) {
        let typed = {
            let mut typed = self.search.get_value();
            typed.push_str(key);
            self.search.set_value(typed.clone());
            typed
        };
        let entries = self.nav_entries();
        let focused = self.focused_index.get_untracked();
        let mut index = search_match_from(&entries, &typed, focused);
        if index.is_none() && focused.is_none() {
            index = first_focused(&entries, &self.values.get_untracked());
        }
        if index.is_some() {
            self.change_focused(event, index, false);
        }
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if !this.open.is_disposed() {
                    this.search.set_value(String::new());
                }
            },
            Duration::from_millis(SEARCH_RESET_MS),
        )
        .ok();
        self.search_timer.set_value(handle);
    }

    // ---- keyboard ----

    /// `onKeyDown` of the hidden field and `onFilterKeyDown` of the search field (`in_filter`).
    fn key_down(self: &Arc<Self>, e: web_sys::KeyboardEvent, in_filter: bool) {
        if self.disabled.get_untracked() {
            e.prevent_default();
            return;
        }
        let event: web_sys::Event = e.clone().into();
        let meta = e.meta_key() || e.ctrl_key();
        match e.code().as_str() {
            "ArrowDown" => self.arrow_down(&e),
            "ArrowUp" => self.arrow_up(&e, in_filter),
            "ArrowLeft" | "ArrowRight" if in_filter => self.focused_index.set(None),
            "Home" => self.home_end(&e, in_filter, true),
            "End" => self.home_end(&e, in_filter, false),
            "PageDown" if !in_filter => self.page(&e, true),
            "PageUp" if !in_filter => self.page(&e, false),
            "Enter" | "NumpadEnter" => self.enter_key(&e),
            "Space" if !in_filter => self.enter_key(&e),
            "Escape" => {
                if self.open.get_untracked() {
                    self.hide(true);
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            "Tab" => self.tab_key(&e, in_filter),
            "ShiftLeft" | "ShiftRight" => self.start_range.set(self.focused_index.get_untracked()),
            _ if in_filter => {}
            "KeyA" if meta => {
                let all = select_all(&self.nav_entries());
                self.update_model(all, event);
                e.prevent_default();
            }
            _ => {
                let key = e.key();
                if !meta && key.chars().count() == 1 {
                    if !self.open.get_untracked() {
                        self.show(false);
                    }
                    self.search_by_typing(&event, &key);
                    e.prevent_default();
                }
            }
        }
    }

    /// `onOptionSelectRange`: selects the range between the anchor index and the focused one.
    fn select_range_event(self: &Arc<Self>, event: &web_sys::Event, start: Option<usize>, end: Option<usize>) {
        let entries = self.nav_entries();
        let values = self.values.get_untracked();
        let (start, end) = match (start, end) {
            (Some(s), Some(e)) => (s, e),
            (None, Some(e)) => (nearest_selected(&entries, &values, e, true), e),
            (Some(s), None) => (s, nearest_selected(&entries, &values, s, false)),
            (None, None) => return,
        };
        self.update_model(select_range(&entries, start, end), event.clone());
    }

    fn arrow_down(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        let entries = self.nav_entries();
        let index = match self.focused_index.get_untracked() {
            Some(i) => Some(next_valid(&entries, i)),
            None => first_focused(&entries, &self.values.get_untracked()),
        };
        let event: web_sys::Event = e.clone().into();
        if e.shift_key() {
            self.select_range_event(&event, self.start_range.get_untracked(), index);
        }
        self.change_focused(&event, index, e.shift_key());
        if !self.open.get_untracked() {
            self.show(false);
        }
        e.prevent_default();
        e.stop_propagation();
    }

    fn arrow_up(self: &Arc<Self>, e: &web_sys::KeyboardEvent, in_filter: bool) {
        let event: web_sys::Event = e.clone().into();
        if e.alt_key() && !in_filter {
            if let Some(Entry::Option(option)) = self.focused_index.get_untracked().and_then(|i| self.entries_untracked().get(i).cloned()) {
                self.select_option(&option, event, None);
            }
            if self.open.get_untracked() {
                self.hide(false);
            }
        } else {
            let entries = self.nav_entries();
            let index = match self.focused_index.get_untracked() {
                Some(i) => Some(prev_valid(&entries, i)),
                None => last_focused(&entries, &self.values.get_untracked()),
            };
            if e.shift_key() {
                self.select_range_event(&event, index, self.start_range.get_untracked());
            }
            self.change_focused(&event, index, e.shift_key());
            if !self.open.get_untracked() {
                self.show(false);
            }
        }
        e.prevent_default();
        e.stop_propagation();
    }

    /// `Home` and `End`: in the search field they move the cursor, otherwise focus goes to the first or last option.
    fn home_end(self: &Arc<Self>, e: &web_sys::KeyboardEvent, in_filter: bool, home: bool) {
        if in_filter {
            if let Some(input) = self.filter_ref.get_untracked() {
                let len = input.value().chars().count() as u32;
                if e.shift_key() {
                    let _ = input.set_selection_range(0, len);
                } else {
                    let caret = if home { 0 } else { len };
                    let _ = input.set_selection_range(caret, caret);
                }
            }
            self.focused_index.set(None);
        } else {
            let entries = self.nav_entries();
            let index = if home { first_valid(&entries) } else { last_valid(&entries) };
            let event: web_sys::Event = e.clone().into();
            if e.shift_key() && (e.ctrl_key() || e.meta_key()) {
                if home {
                    self.select_range_event(&event, index, self.start_range.get_untracked());
                } else {
                    self.select_range_event(&event, self.start_range.get_untracked(), index);
                }
            }
            self.change_focused(&event, index, e.shift_key());
            if !self.open.get_untracked() {
                self.show(false);
            }
        }
        e.prevent_default();
    }

    /// `PageDown`/`PageUp`: scrolls the list by the visible area height.
    fn page(&self, e: &web_sys::KeyboardEvent, down: bool) {
        if let Some(list) = self.list_ref.get_untracked()
            && let Some(container) = list.parent_element()
        {
            let step = container.client_height();
            container.set_scroll_top(container.scroll_top() + if down { step } else { -step });
        }
        e.prevent_default();
    }

    fn enter_key(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        if !self.open.get_untracked() {
            self.arrow_down(e);
        } else if let Some(i) = self.focused_index.get_untracked() {
            let event: web_sys::Event = e.clone().into();
            if e.shift_key() {
                self.select_range_event(&event, Some(i), None);
            } else if let Some(Entry::Option(option)) = self.entries_untracked().get(i).cloned() {
                self.select_option(&option, event, None);
            }
        }
        e.prevent_default();
    }

    fn tab_key(self: &Arc<Self>, e: &web_sys::KeyboardEvent, in_filter: bool) {
        if in_filter {
            return;
        }
        // with a search field, Tab from the hidden field leads into it through the first hidden panel boundary
        let goes_to_filter = self.open.get_untracked() && self.filter.get_untracked().unwrap_or(true);
        if goes_to_filter {
            return;
        }
        if self.open.get_untracked() {
            if let Some(Entry::Option(option)) = self.focused_index.get_untracked().and_then(|i| self.entries_untracked().get(i).cloned())
                && !self.is_selected_untracked(&option.value)
            {
                self.select_option(&option, e.clone().into(), None);
            }
            self.hide(false);
        }
    }

    fn is_selected_untracked(&self, value: &str) -> bool {
        self.values.with_untracked(|vs| vs.iter().any(|v| v == value))
    }
}

/// Label content by display mode.
enum Content {
    Comma(LabelContent),
    Chip(ChipContent),
}
