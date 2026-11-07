//! Страница ImageCompare: демонстрации `basic` и `responsive` из showcase PrimeNG (`apps/showcase/doc/imagecompare`).
//! Фотографии с CDN заменены двумя встроенными SVG-картинками (независимость от сети).

use super::Demo;
use leptos::prelude::*;
use prime_imagecompare::ImageCompare;

/// SVG-картинка 16:9 с градиентом и подписью как `data:`-ссылка.
fn picture(from: &str, to: &str, label: &str) -> String {
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1600 900' preserveAspectRatio='none'%3E\
         %3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0' stop-color='%23{from}'/%3E\
         %3Cstop offset='1' stop-color='%23{to}'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='1600' height='900' fill='url(%23g)'/%3E\
         %3Ctext x='800' y='480' font-size='120' font-family='sans-serif' fill='white' text-anchor='middle'%3E{label}%3C/text%3E%3C/svg%3E"
    )
}

/// Страница `/imagecompare`.
#[component]
pub fn ImageComparePage() -> impl IntoView {
    let (left, right) = (picture("0ea5e9", "1e3a8a", "Before"), picture("f59e0b", "9a3412", "After"));
    let (left2, right2) = (left.clone(), right.clone());
    view! {
        <Demo name="basic" title="Basic">
            <ImageCompare
                left=move || view! { <img src={left.clone()} alt="Before" /> }
                right=move || view! { <img src={right.clone()} alt="After" /> }
                style="width: 36rem; max-width: 100%; border-radius: 1rem; box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.2)"
            />
        </Demo>
        <Demo name="responsive" title="Responsive">
            <ImageCompare
                left=move || view! { <img src={left2.clone()} alt="Before" /> }
                right=move || view! { <img src={right2.clone()} alt="After" /> }
                style="width: 24rem; max-width: 100%; border-radius: 1rem; box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.2)"
            />
        </Demo>
    }
}
