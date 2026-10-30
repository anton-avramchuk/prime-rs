//! Inplace — порт `p-inplace` из PrimeNG 21.1.10.
//!
//! Область, которая по клику (или `Enter`) заменяет «вид» (`display`) на «содержимое» (`content`). Содержимому передаётся
//! функция закрытия (аналог `closeCallback`). Состояние — необязательный двусторонний сигнал `active`.
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
use prime_core::{ClassNames, OptProp, Prop, Pt};
use prime_icons::TimesIcon;
use std::sync::Arc;

pub use state::{Action, display_classes, next_state};

/// Pass-through частей [`Inplace`].
#[derive(Clone, Debug, Default)]
pub struct InplacePt {
    /// Корень.
    pub root: Pt,
    /// Вид.
    pub display: Pt,
    /// Содержимое.
    pub content: Pt,
    /// Кнопка закрытия.
    pub close_button: Pt,
}

/// Область, переключаемая между видом и содержимым.
///
/// Каждое свойство принимает значение или сигнал; `active` — двусторонний сигнал.
#[component]
pub fn Inplace(
    /// Двусторонний сигнал активности; без него состояние хранится внутри (по умолчанию неактивно).
    #[prop(optional)]
    active: Option<RwSignal<bool>>,
    /// Показывать в содержимом кнопку закрытия.
    #[prop(optional, into)]
    closable: Prop<bool>,
    /// Недоступно: не активируется и не закрывается.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Отключить клик по виду и закрытие (остаётся `Enter`, как в PrimeNG).
    #[prop(optional, into)]
    prevent_click: Prop<bool>,
    /// Значок кнопки закрытия вместо `TimesIcon`.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// Атрибут `aria-label` кнопки закрытия.
    #[prop(optional, into)]
    close_aria_label: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вид (неактивное состояние).
    #[prop(into)]
    display: ViewFn,
    /// Содержимое (активное состояние); получает функцию закрытия.
    content: Callback<Callback<()>, AnyView>,
    /// Вызывается после активации.
    #[prop(optional, into)]
    on_activate: Option<Callback<()>>,
    /// Вызывается после возврата к виду.
    #[prop(optional, into)]
    on_deactivate: Option<Callback<()>>,
    /// Pass-through частей.
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
                            // Тот же клик, что закрыл содержимое, доходит и до нового вида: его цель уже вне документа.
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

/// Цель события уже убрана из документа (клик, закрывший содержимое, продолжает всплывать).
fn is_detached(ev: &web_sys::MouseEvent) -> bool {
    use leptos::wasm_bindgen::JsCast;
    ev.target().is_some_and(|target| target.dyn_ref::<web_sys::Node>().is_some_and(|node| !node.is_connected()))
}
