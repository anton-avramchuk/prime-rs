//! ScrollTop — порт компонента `p-scrolltop` из PrimeNG 21.1.10.
//!
//! Кнопка «наверх» (`Button` с классом `p-scrolltop`), которая показывается, когда прокрутка окна (`target = Window`) или
//! родительского элемента (`target = Parent`) строго больше `threshold` (по умолчанию 400). Клик прокручивает цель к началу.
//!
//! У окна кнопка монтируется только пока видна. Родителя до показа найти нельзя, поэтому при `Parent` кнопка всегда лежит в DOM и
//! в скрытом состоянии прячется стилем `display: none`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_scrolltop::{ScrollTop, ScrollTopTarget};
//!
//! // На сервере кнопка окна скрыта, у родителя — в разметке, но спрятана.
//! assert!(!view! { <ScrollTop /> }.to_html().contains("p-scrolltop"));
//! let html = view! { <ScrollTop target=ScrollTopTarget::Parent /> }.to_html();
//! assert!(html.contains("p-scrolltop-sticky") && html.contains("display: none"));
//! ```

#[cfg(target_arch = "wasm32")]
mod client;
mod state;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{ClassNames, OptProp, Prop, Pt, ZGroup, use_prime_config};
use prime_icons::ChevronUpIcon;

pub use state::{Phase, ScrollTopBehavior, ScrollTopTarget, enter_classes, is_visible, leave_classes, root_classes};

/// Порог прокрутки по умолчанию.
const DEFAULT_THRESHOLD: f64 = 400.0;

/// Pass-through частей ScrollTop.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrollTopPt {
    /// Корневая кнопка.
    pub root: Pt,
}

/// Кнопка возврата к началу страницы или прокручиваемого родителя.
#[component]
pub fn ScrollTop(
    /// Что прокручивать: окно (по умолчанию) или родителя.
    #[prop(optional)]
    target: ScrollTopTarget,
    /// Прокрутка, после которой кнопка показывается (строго больше; по умолчанию 400).
    #[prop(default = Prop::Static(DEFAULT_THRESHOLD), into)]
    threshold: Prop<f64>,
    /// Поведение прокрутки по клику (по умолчанию плавное).
    #[prop(optional)]
    behavior: ScrollTopBehavior,
    /// Свой значок вместо `ChevronUpIcon`; оборачивается в `span.p-scrolltop-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Атрибут `aria-label`; по умолчанию из локали (`aria.scrollTop`).
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Цвет кнопки.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// Кнопка с тенью.
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Скруглённая кнопка (по умолчанию да).
    #[prop(default = Prop::Static(true), into)]
    rounded: Prop<bool>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ScrollTopPt>,
) -> impl IntoView {
    let id = prime_core::ids::next_unique();
    let config = use_prime_config();
    let aria_label = match aria_label {
        Prop::Static(None) => Prop::Static(Some(config.locale.text("aria.scrollTop").to_owned())),
        other => other,
    };
    let overlay_base = config.z_index.base(ZGroup::Overlay);

    let phase = RwSignal::new(Phase::Hidden);
    let animation = RwSignal::new("");
    let z_index = RwSignal::new(None::<i32>);

    #[cfg(target_arch = "wasm32")]
    client::setup(client::Setup {
        target,
        threshold: StoredValue::new(threshold),
        phase,
        animation,
        z_index,
        overlay_base,
        epoch: StoredValue::new(0),
        id,
    });
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (threshold, overlay_base);

    let class = Signal::derive(move || {
        let mut cn = ClassNames::new();
        cn.add(root_classes(target));
        let animation = animation.get();
        cn.add_if(!animation.is_empty(), animation);
        style_class.with(|c| cn.add_user(c.as_deref()));
        Some(cn.finish_cow().into_owned())
    });
    let button_style = Signal::derive(move || {
        let mut parts = Vec::new();
        if let Some(z) = z_index.get() {
            parts.push(format!("z-index: {z}"));
        }
        if target == ScrollTopTarget::Parent && phase.get() == Phase::Hidden {
            parts.push("display: none".to_owned());
        }
        style.with(|s| parts.extend(s.clone()));
        (!parts.is_empty()).then(|| parts.join("; "))
    });
    let root = pt.unwrap_or_default().root.attr("data-scrolltop", id.to_string());
    let button_icon = ViewFn::from(move || match &icon {
        Some(custom) => view! { <span class="p-scrolltop-icon">{custom.run()}</span> }.into_any(),
        None => view! { <ChevronUpIcon class="p-scrolltop-icon" /> }.into_any(),
    });
    let click = Callback::new(move |_: web_sys::MouseEvent| scroll_to_top(target, behavior, id));
    let button = move || {
        view! {
            <Button
                button_type="button"
                icon=button_icon.clone()
                rounded=rounded.clone()
                raised=raised.clone()
                severity=severity.clone()
                aria_label=aria_label.clone()
                style=button_style
                style_class=class
                on_click=click
                pt=ButtonPt { root: root.clone(), ..Default::default() }
            />
        }
    };

    match target {
        ScrollTopTarget::Window => (move || (phase.get() != Phase::Hidden).then(&button)).into_any(),
        ScrollTopTarget::Parent => button().into_any(),
    }
}

/// Прокручивает цель к началу.
fn scroll_to_top(target: ScrollTopTarget, behavior: ScrollTopBehavior, id: usize) {
    let options = web_sys::ScrollToOptions::new();
    options.set_top(0.0);
    options.set_behavior(match behavior {
        ScrollTopBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        ScrollTopBehavior::Auto => web_sys::ScrollBehavior::Auto,
    });
    match target {
        ScrollTopTarget::Window => {
            if let Some(window) = web_sys::window() {
                window.scroll_with_scroll_to_options(&options);
            }
        }
        ScrollTopTarget::Parent => {
            if let Some(parent) = parent_of(id) {
                parent.scroll_with_scroll_to_options(&options);
            }
        }
    }
}

/// Родитель кнопки с номером `id`.
pub(crate) fn parent_of(id: usize) -> Option<web_sys::Element> {
    let document = web_sys::window()?.document()?;
    document.query_selector(&format!(r#"[data-scrolltop="{id}"]"#)).ok().flatten()?.parent_element()
}
