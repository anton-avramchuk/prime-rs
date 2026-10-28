//! Страница Card: демонстрации `basic` и `advanced` из showcase PrimeNG (`apps/showcase/doc/card`).
//!
//! Изображение в шапке берётся с CDN PrimeNG, как в оригинале; кнопки подвала — наш `Button`.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity, ButtonVariant};
use prime_card::Card;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Inventore sed consequuntur error repudiandae numquam deserunt quisquam repellat libero asperiores earum nam nobis, culpa ratione quam perferendis esse, cupiditate neque quas!";

/// Страница `/card`.
#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <h2>"Card"</h2>
        <Demo name="basic" title="Basic">
            <Card header="Simple Card">
                <p style="margin: 0">{LOREM}</p>
            </Card>
        </Demo>
        <Demo name="advanced" title="Advanced">
            <Card
                style="width: 25rem; overflow: hidden"
                header_template=ViewFn::from(|| {
                    view! { <img alt="Card" style="width: 100%" src="https://primefaces.org/cdn/primeng/images/card-ng.jpg" /> }
                })
                title_template=ViewFn::from(|| view! { "Advanced Card" })
                subtitle_template=ViewFn::from(|| view! { "Card subtitle" })
                footer_template=ViewFn::from(|| {
                    view! {
                        <div style="display: flex; gap: 1rem; margin-top: 0.25rem">
                            <Button label="Cancel" severity=ButtonSeverity::Secondary variant=ButtonVariant::Outlined style="width: 100%" />
                            <Button label="Save" style="width: 100%" />
                        </div>
                    }
                })
            >
                <p>{LOREM}</p>
            </Card>
        </Demo>
    }
}
