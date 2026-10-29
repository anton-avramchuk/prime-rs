//! Страница Rating: демонстрации `basic`, `disabled`, `numberofstars`, `readonly`, `template` из showcase PrimeNG
//! (`apps/showcase/doc/rating`). Демонстрации с Angular Forms и `withoutcancel` (в 21.1.10 кнопки отмены нет) не переносятся;
//! в `template` вместо внешних картинок пользовательские иконки — символы `♥` и `♡`.

use super::Demo;
use leptos::prelude::*;
use prime_rating::{Rating, RatingIcon};

/// Страница `/rating`.
#[component]
pub fn RatingPage() -> impl IntoView {
    let basic = RwSignal::new(None::<u32>);
    let numbered = RwSignal::new(None::<u32>);
    let templated = RwSignal::new(Some(3u32));
    let on_icon = Callback::new(|i: RatingIcon| view! { <span class={i.class} style="color: var(--p-red-500, red)">"♥"</span> }.into_any());
    let off_icon = Callback::new(|i: RatingIcon| view! { <span class={i.class}>"♡"</span> }.into_any());
    view! {
        <h2>"Rating"</h2>
        <Demo name="basic" title="Basic">
            <Rating value=basic />
            <span data-testid="echo">{move || basic.get().map(|v| v.to_string()).unwrap_or_default()}</span>
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Rating default_value=3 disabled=true />
        </Demo>
        <Demo name="numberofstars" title="Number of Stars">
            <Rating value=numbered stars=10 />
        </Demo>
        <Demo name="readonly" title="Readonly">
            <Rating default_value=5 readonly=true />
        </Demo>
        <Demo name="template" title="Template">
            <Rating value=templated on_icon_template=on_icon off_icon_template=off_icon />
        </Demo>
    }
}
