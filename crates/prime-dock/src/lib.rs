//! Dock — port of `p-dock` from PrimeNG 21.1.10.
//!
//! An icon bar at the edge of a container: root `div.p-dock` (position `p-dock-bottom|top|left|right`, on a narrow window
//! `p-dock-mobile`), list `ul.p-dock-list[role=menu]` and items `li.p-dock-item` with a link, an icon and an optional badge.
//! The list gets focus as a whole; arrow keys (depending on orientation), `Home`, `End`, `Enter` and `Space` control the items.
//! PrimeNG 21 has no item magnification on hover, and neither does this port. The root in the theme is `position: absolute`: the container needs `position: relative`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_dock::{Dock, DockItem, DockPosition};
//!
//! let model = vec![DockItem::new().label("Finder"), DockItem::new().label("Terminal")];
//! let html = Owner::new().with(|| view! { <Dock model=model position=DockPosition::Left /> }.to_html());
//! assert!(html.contains("p-dock-left") && html.contains("aria-orientation=\"vertical\"") && html.contains("Terminal"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_badge::Badge;
use prime_core::{OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{Action, DockPosition, item_classes, item_id, key_action, orientation, root_classes, step};

/// Item command event.
#[derive(Clone)]
pub struct DockCommand {
    /// Original click event.
    pub event: web_sys::MouseEvent,
    /// The item.
    pub item: DockItem,
}

/// Dock item (instead of PrimeNG `MenuItem`, without router fields).
#[derive(Clone, Default)]
pub struct DockItem {
    /// Identifier (default `<dock id>_<index>`).
    pub id: Option<String>,
    /// Label (`aria-label`).
    pub label: Option<String>,
    /// Icon.
    pub icon: Option<ViewFn>,
    /// Link address.
    pub url: Option<String>,
    /// Link `target`.
    pub target: Option<String>,
    /// Link `title`.
    pub title: Option<String>,
    /// Disabled.
    pub disabled: bool,
    /// Badge value.
    pub badge: Option<String>,
    /// Command on click.
    pub command: Option<Callback<DockCommand>>,
    /// Extra class of the `li`.
    pub style_class: Option<String>,
    /// Inline style of the `li`.
    pub style: Option<String>,
    /// Extra class of the link.
    pub link_class: Option<String>,
    /// Extra class of the icon.
    pub icon_class: Option<String>,
}

impl DockItem {
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

    /// Link address.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Link `target`.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Link `title`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Disables the item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Badge value.
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Command on click.
    pub fn command(mut self, command: impl Into<Callback<DockCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Extra class of the `li`.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Inline style of the `li`.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Extra class of the link.
    pub fn link_class(mut self, class: impl Into<String>) -> Self {
        self.link_class = Some(class.into());
        self
    }

    /// Extra class of the icon.
    pub fn icon_class(mut self, class: impl Into<String>) -> Self {
        self.icon_class = Some(class.into());
        self
    }
}

/// Pass-through for the parts of Dock.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DockPt {
    /// Root.
    pub root: Pt,
    /// List container.
    pub list_container: Pt,
    /// List.
    pub list: Pt,
    /// Item.
    pub item: Pt,
    /// Item content.
    pub item_content: Pt,
    /// Item link.
    pub item_link: Pt,
    /// Item icon.
    pub item_icon: Pt,
}

/// Icon bar at the edge of a container.
///
/// Each property accepts a value or a signal.
#[component]
pub fn Dock(
    /// Items.
    #[prop(optional, into)]
    model: Prop<Vec<DockItem>>,
    /// Position (default: bottom).
    #[prop(optional, into)]
    position: Prop<DockPosition>,
    /// Window width up to which the `p-dock-mobile` mode is enabled (default `960px`).
    #[prop(optional, into)]
    breakpoint: OptProp<String>,
    /// `id` attribute of the list; item identifiers are built from it.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `aria-label` attribute of the list.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the list.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Item content instead of the icon; receives the item.
    #[prop(optional, into)]
    item: Option<Callback<DockItem, AnyView>>,
    /// The list received focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// The list lost focus.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<DockPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let list_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let mobile = RwSignal::new(false);
    #[cfg(target_arch = "wasm32")]
    client::bind_media(breakpoint.get_untracked().unwrap_or_else(|| "960px".to_owned()), mobile);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = breakpoint;

    // Position of the focus among the available (not disabled) items.
    let focused = RwSignal::new(None::<usize>);
    let has_focus = RwSignal::new(false);
    let focused_id = {
        let (model, list_id) = (model.clone(), list_id.clone());
        move || -> Option<String> {
            let item_index = *enabled_of(&model).get(focused.get()?)?;
            Some(model.with(|m| item_id(&list_id, m[item_index].id.as_deref(), item_index)))
        }
    };

    let root_class = {
        let (pt, style_class, position) = (pt.clone(), style_class.clone(), position.clone());
        move || {
            let extra = style_class.with(|c| c.clone());
            let merged = root_classes(position.get(), mobile.get(), extra.as_deref());
            pt.root.merge_class(&merged).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };

    let list_focus = {
        let model = model.clone();
        move |e: web_sys::FocusEvent| {
            has_focus.set(true);
            focused.set(step(enabled_of(&model).len(), None, Action::First));
            if let Some(cb) = on_focus {
                cb.run(e);
            }
        }
    };
    let list_blur = move |e: web_sys::FocusEvent| {
        has_focus.set(false);
        focused.set(None);
        if let Some(cb) = on_blur {
            cb.run(e);
        }
    };
    let key_down = {
        let (model, list_id, position) = (model.clone(), list_id.clone(), position.clone());
        move |e: web_sys::KeyboardEvent| {
            let Some(action) = key_action(position.get_untracked(), &e.code()) else { return };
            let count = enabled_of(&model).len();
            match action {
                Action::Activate => click_focused(&model, &list_id, focused.get_untracked()),
                _ => focused.set(step(count, focused.get_untracked(), action)),
            }
            e.prevent_default();
        }
    };

    let items = {
        let (model, pt, list_id) = (model.clone(), pt.clone(), list_id.clone());
        move || -> Vec<AnyView> {
            let all = model.get();
            let enabled_positions: Vec<usize> = all.iter().enumerate().filter(|(_, it)| !it.disabled).map(|(i, _)| i).collect();
            all.iter()
                .enumerate()
                .map(|(index, entry)| {
                    let own_position = enabled_positions.iter().position(|i| *i == index);
                    item_view(entry, index, own_position, &list_id, focused, &pt, item)
                })
                .collect()
        }
    };

    let list_attrs: Vec<AnyAttribute> = vec![
        custom_attribute("aria-orientation", move || Some(orientation(position.get()).to_owned())).into_any_attr(),
        custom_attribute("aria-label", move || aria_label.get()).into_any_attr(),
        custom_attribute("aria-labelledby", move || aria_labelledby.get()).into_any_attr(),
        custom_attribute("aria-activedescendant", move || if has_focus.get() { focused_id() } else { None }).into_any_attr(),
    ];
    let (root_attrs, container_attrs, pt_list_attrs) = (pt.root.attrs(), pt.list_container.attrs(), pt.list.attrs());
    let (container_class, list_class) = (pt.list_container.merge_class("p-dock-list-container"), pt.list.merge_class("p-dock-list"));

    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            <div class={container_class} {..container_attrs}>
                <ul
                    id={list_id}
                    class={list_class}
                    role="menu"
                    tabindex="0"
                    on:focus={list_focus}
                    on:blur={list_blur}
                    on:keydown={key_down}
                    {..list_attrs}
                    {..pt_list_attrs}
                >
                    {items}
                </ul>
            </div>
        </div>
    }
}

/// Indices of the available items of the model (without subscription).
fn enabled_of(model: &Prop<Vec<DockItem>>) -> Vec<usize> {
    model.get_untracked().iter().enumerate().filter(|(_, it)| !it.disabled).map(|(i, _)| i).collect()
}

/// `Enter`/`Space`: click on the link of the focused item (otherwise on the item itself), like `onSpaceKey` in PrimeNG.
fn click_focused(model: &Prop<Vec<DockItem>>, list_id: &str, position: Option<usize>) {
    use leptos::wasm_bindgen::JsCast;
    let Some(index) = position.and_then(|p| enabled_of(model).get(p).copied()) else { return };
    let id = {
        let items = model.get_untracked();
        item_id(list_id, items[index].id.as_deref(), index)
    };
    let Some(li) = leptos::prelude::document().get_element_by_id(&id) else { return };
    let target = li.query_selector("a, button").ok().flatten().unwrap_or(li);
    if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
        el.click();
    }
}

/// Item: `li` with content, link, icon (or view) and badge.
fn item_view(
    entry: &DockItem,
    index: usize,
    enabled_position: Option<usize>,
    list_id: &str,
    focused: RwSignal<Option<usize>>,
    pt: &Arc<DockPt>,
    template: Option<Callback<DockItem, AnyView>>,
) -> AnyView {
    let id = item_id(list_id, entry.id.as_deref(), index);
    let disabled = entry.disabled;
    let is_focused = move || enabled_position.is_some() && focused.get() == enabled_position;
    let class = {
        let (pt, extra) = (pt.clone(), entry.style_class.clone());
        move || {
            let merged = item_classes(is_focused(), disabled, extra.as_deref());
            pt.item.merge_class(&merged).into_owned()
        }
    };
    let style = entry.style.clone().filter(|s| !s.is_empty());
    let click = {
        let entry = entry.clone();
        move |e: web_sys::MouseEvent| {
            if let (Some(cb), false) = (entry.command, entry.disabled) {
                cb.run(DockCommand { event: e, item: entry.clone() });
            }
        }
    };
    let link_class = {
        let mut cn = prime_core::ClassNames::new();
        cn.add("p-dock-item-link").add_user(entry.link_class.as_deref()).add_user(pt.item_link.extra_class());
        cn.finish_cow().into_owned()
    };
    let icon = match (&template, &entry.icon) {
        (Some(template), _) => Some(template.run(entry.clone())),
        (None, Some(icon)) => {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-dock-item-icon").add_user(entry.icon_class.as_deref()).add_user(pt.item_icon.extra_class());
            let (class, icon_attrs) = (cn.finish_cow().into_owned(), pt.item_icon.attrs());
            let inner = icon.run();
            Some(view! { <span class={class} {..icon_attrs}>{inner}</span> }.into_any())
        }
        (None, None) => None,
    };
    let badge = entry.badge.clone().map(|value| view! { <Badge value=value /> });
    let (label, url, target, title) = (entry.label.clone(), entry.url.clone(), entry.target.clone(), entry.title.clone());
    let (content_class, content_attrs) = (pt.item_content.merge_class("p-dock-item-content"), pt.item_content.attrs());
    let (item_attrs, link_attrs) = (pt.item.attrs(), pt.item_link.attrs());
    view! {
        <li
            id={id}
            class={class}
            style={style}
            role="menuitem"
            aria-label={label}
            aria-disabled={disabled.to_string()}
            data-p-focused={move || is_focused().to_string()}
            data-p-disabled={disabled.to_string()}
            on:click={click}
            {..item_attrs}
        >
            <div class={content_class} {..content_attrs}>
                <a class={link_class} href={url} target={target} title={title} aria-hidden="true" {..link_attrs}>
                    {icon}
                    {badge}
                </a>
            </div>
        </li>
    }
    .into_any()
}

#[cfg(target_arch = "wasm32")]
mod client {
    //! Mobile mode: `matchMedia("(max-width: <breakpoint>)")`.

    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    /// Drives `mobile` from the media query and removes the listener on unmount.
    pub fn bind_media(breakpoint: String, mobile: RwSignal<bool>) {
        let Some(window) = web_sys::window() else { return };
        let Some(query) = window.match_media(&format!("(max-width: {breakpoint})")).ok().flatten() else { return };
        mobile.set(query.matches());
        let closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(move |e: web_sys::MediaQueryListEvent| {
            mobile.try_set(e.matches());
        });
        let _ = query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        let held = StoredValue::new_local((query, closure));
        on_cleanup(move || {
            held.try_with_value(|(query, closure)| {
                let _ = query.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            });
        });
    }
}
