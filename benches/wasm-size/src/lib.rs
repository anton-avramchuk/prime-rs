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
}

/// Точка входа wasm.
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount::mount_to_body(App);
}
