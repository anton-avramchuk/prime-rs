//! Inplace — port of `p-inplace` from PrimeNG 21.1.10.
//!
//! An area that replaces the "display" view with the "content" on click (or `Enter`). The content receives a close
//! function (analogue of `closeCallback`). The state is an optional two-way signal `active`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inplace::Inplace;
//!
//! let html = view! {
//!     <Inplace
//!         display=ViewFn::from(|| view! { <span>"View"</span> })
//!         content=Callback::new(|_close| view! { <p>"Body"</p> }.into_any())
//!     />
//! }
//! .to_html();
//! assert!(html.contains("p-inplace-display") && html.contains("View") && !html.contains("Body"));
//! ```

mod state;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt};
use prime_icons::TimesIcon;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{Action, display_classes, next_state};

/// Pass-through for the parts of [`Inplace`].
#[derive(Clone, Debug, Default)]
pub struct InplacePt {
    /// Root.
    pub root: Pt,
    /// Display view.
    pub display: Pt,
    /// Content.
    pub content: Pt,
    /// Close button.
    pub close_button: Pt,
}

/// Area that switches between the view and the content.
///
/// Each property accepts a value or a signal; `active` is a two-way signal.
#[component]
pub fn Inplace(
    /// Two-way activity signal; without it the state is stored internally (inactive by default).
    #[prop(optional)]
    active: Option<RwSignal<bool>>,
    /// Show a close button in the content.
    #[prop(optional, into)]
    closable: Prop<bool>,
    /// Unavailable: cannot be activated or closed.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Disable the click on the view and closing (`Enter` remains, as in PrimeNG).
    #[prop(optional, into)]
    prevent_click: Prop<bool>,
    /// Icon of the close button instead of `TimesIcon`.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// `aria-label` attribute of the close button.
    #[prop(optional, into)]
    close_aria_label: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// View (inactive state).
    #[prop(into)]
    display: ViewFn,
    /// Content (active state); receives the close function.
    content: Callback<Callback<()>, AnyView>,
    /// Called after activation.
    #[prop(optional, into)]
    on_activate: Option<Callback<()>>,
    /// Called after returning to the view.
    #[prop(optional, into)]
    on_deactivate: Option<Callback<()>>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<InplacePt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let active = active.unwrap_or_default();
    let apply = {
        let (disabled, prevent_click) = (disabled.clone(), prevent_click.clone());
        Callback::new(move |action: Action| {
            let before = active.get_untracked();
            let after = next_state(before, action, disabled.get_untracked(), prevent_click.get_untracked());
            if after == before {
                return;
            }
            active.set(after);
            let callback = if after { on_activate } else { on_deactivate };
            if let Some(callback) = callback {
                callback.run(());
            }
        })
    };
    let close = Callback::new(move |()| apply.run(Action::Close));

    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = ClassNames::new();
            cn.add("p-inplace p-component");
            style_class.with(|c| cn.add_user(c.as_deref()));
            pt.root.merge_class(&cn.finish())
        })
    };
    let root_style = {
        let pt = pt.clone();
        Signal::derive(move || {
            let user = style.get().unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&user]);
            (!merged.is_empty()).then_some(merged)
        })
    };

    let body = {
        let pt = pt.clone();
        move || {
            if active.get() {
                let button = closable.get().then(|| {
                    let icon = close_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <TimesIcon /> }));
                    view! {
                        <Button
                            button_type="button"
                            icon=icon
                            aria_label=close_aria_label.clone()
                            on_click=Callback::new(move |_: web_sys::MouseEvent| close.run(()))
                            pt=ButtonPt { root: pt.close_button.clone(), ..Default::default() }
                        />
                    }
                });
                view! {
                    <div class={pt.content.merge_class("p-inplace-content")} {..pt.content.attrs()}>
                        {content.run(close)}
                        {button}
                    </div>
                }
                .into_any()
            } else {
                let class_disabled = disabled.clone();
                let data_disabled = disabled.clone();
                view! {
                    <div
                        class={
                            let pt = pt.clone();
                            move || pt.display.merge_class(display_classes(class_disabled.get()))
                        }
                        tabindex="0"
                        role="button"
                        data-p-disabled={move || data_disabled.get().then_some("true")}
                        on:click=move |ev: web_sys::MouseEvent| {
                            // The same click that closed the content also reaches the new view: its target is already outside the document.
                            if is_detached(&ev) {
                                return;
                            }
                            apply.run(Action::Click);
                        }
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                apply.run(Action::Enter);
                                ev.prevent_default();
                            }
                        }
                        {..pt.display.attrs()}
                    >
                        {display.run()}
                    </div>
                }
                .into_any()
            }
        }
    };

    view! {
        <div class={class} style={root_style} aria-live="polite" {..pt.root.attrs()}>
            {body}
        </div>
    }
}

/// The event target has already been removed from the document (the click that closed the content keeps bubbling).
fn is_detached(ev: &web_sys::MouseEvent) -> bool {
    use leptos::wasm_bindgen::JsCast;
    ev.target().is_some_and(|target| target.dyn_ref::<web_sys::Node>().is_some_and(|node| !node.is_connected()))
}
