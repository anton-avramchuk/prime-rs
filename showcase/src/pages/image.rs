//! Страница Image: демонстрации `basic`, `preview`, `previewimagesource` и `template` из showcase PrimeNG
//! (`apps/showcase/doc/image`). Фотографии с CDN заменены встроенными SVG-картинками (независимость от сети).

use super::Demo;
use leptos::prelude::*;
use prime_image::{Image, ImagePreviewContext};

/// SVG-картинка с градиентом и подписью как `data:`-ссылка; `size` — сторона viewBox.
fn picture(from: &str, to: &str, label: &str, size: u32) -> String {
    let half = size / 2;
    let font = size / 8;
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {size} {size}'%3E\
         %3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0' stop-color='%23{from}'/%3E\
         %3Cstop offset='1' stop-color='%23{to}'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='{size}' height='{size}' fill='url(%23g)'/%3E\
         %3Ctext x='{half}' y='{half}' font-size='{font}' font-family='sans-serif' fill='white' text-anchor='middle'%3E{label}%3C/text%3E%3C/svg%3E"
    )
}

/// Страница `/image`.
#[component]
pub fn ImagePage() -> impl IntoView {
    let small = picture("0ea5e9", "1e3a8a", "Photo", 400);
    let (basic, preview) = (small.clone(), small.clone());
    let (source_small, source_big) = (picture("f59e0b", "9a3412", "Small", 400), picture("f59e0b", "9a3412", "Large", 1200));
    let tpl = picture("10b981", "064e3b", "Custom", 400);
    let (tpl_image, tpl_preview) = (tpl.clone(), tpl.clone());
    view! {
        <Demo name="basic" title="Basic">
            <Image src={basic} alt="Image" width="250" />
        </Demo>
        <Demo name="preview" title="Preview">
            <Image src={preview} alt="Image" width="250" preview=true />
        </Demo>
        <Demo name="previewimagesource" title="Preview Image Source">
            <Image src={source_small} preview_image_src={source_big} alt="Image" width="250" preview=true />
        </Demo>
        <Demo name="template" title="Template">
            <Image
                src={tpl.clone()}
                alt="Image"
                width="250"
                preview=true
                indicator=ViewFn::from(|| view! { <span data-testid="custom-indicator">"Zoom"</span> })
                image=Callback::new(move |_err| view! { <img src={tpl_image.clone()} alt="image" width="250" /> }.into_any())
                preview_content=Callback::new(move |ctx: ImagePreviewContext| {
                    let cb = ctx.preview_callback;
                    view! { <img src={tpl_preview.clone()} alt="image" class={ctx.class.clone()} style={move || ctx.style.get()} on:click=move |_| cb.run(()) /> }
                        .into_any()
                })
            />
        </Demo>
    }
}
