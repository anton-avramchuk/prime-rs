//! Приложение для измерения размера wasm: набор отрисованного определяется feature-флагом `case-*`.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    #[cfg(feature = "case-baseline")]
    return view! { <div>"baseline"</div> };

    #[cfg(feature = "case-one-icon")]
    return view! { <div><prime_icons::CheckIcon /></div> };

    #[cfg(feature = "case-all-icons")]
    return view! { <div><prime_icons::AllIcons /></div> };

    #[cfg(feature = "case-badge")]
    return view! { <prime_badge::Badge value="5" severity=prime_badge::BadgeSeverity::Danger /> };

    #[cfg(feature = "case-button")]
    return view! { <prime_button::Button label="Сохранить" severity=prime_button::ButtonSeverity::Success /> };
}

/// Точка входа wasm.
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount::mount_to_body(App);
}
