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

    #[cfg(feature = "case-tag")]
    return view! { <prime_tag::Tag value="New" severity=prime_tag::TagSeverity::Success rounded=true /> };

    #[cfg(feature = "case-chip")]
    return view! { <prime_chip::Chip label="Thriller" removable=true /> };

    #[cfg(feature = "case-avatar")]
    return view! { <prime_avatar::Avatar label="P" shape=prime_avatar::AvatarShape::Circle size=prime_avatar::AvatarSize::Large /> };

    #[cfg(feature = "case-divider")]
    return view! { <prime_divider::Divider align=prime_divider::DividerAlign::Center>"OR"</prime_divider::Divider> };

    #[cfg(feature = "case-button")]
    return view! { <prime_button::Button label="Сохранить" severity=prime_button::ButtonSeverity::Success /> };
}

/// Точка входа wasm.
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount::mount_to_body(App);
}
