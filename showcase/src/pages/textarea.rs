//! Страница Textarea: демонстрации `basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid` из showcase PrimeNG
//! (`apps/showcase/doc/textarea`). Демонстрации `floatlabel`, `iftalabel`, `keyfilter` не перенесены (нет соответствующих компонентов).

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_textarea::Textarea;

/// Страница `/textarea`.
#[component]
pub fn TextareaPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let auto = RwSignal::new(String::new());
    view! {
        <h2>"Textarea"</h2>
        <Demo name="basic" title="Basic">
            <Textarea value=value rows=5u32 cols=30u32 />
            <span data-testid="echo">{move || value.get().len().to_string()}</span>
        </Demo>
        <Demo name="autoresize" title="Auto Resize">
            <Textarea value=auto auto_resize=true rows=5u32 cols=30u32 />
        </Demo>
        <Demo name="filled" title="Filled">
            <Textarea variant=Variant::Filled rows=5u32 cols=30u32 />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; gap: 1rem">
                <Textarea size=Size::Small rows=3u32 cols=30u32 placeholder="Small" />
                <Textarea rows=3u32 cols=30u32 placeholder="Normal" />
                <Textarea size=Size::Large rows=3u32 cols=30u32 placeholder="Large" />
            </div>
        </Demo>
        <Demo name="invalid" title="Invalid">
            <Textarea invalid=true rows=5u32 cols=30u32 placeholder="Invalid" />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Textarea disabled=true rows=5u32 cols=30u32 placeholder="Disabled" />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <Textarea fluid=true rows=5u32 cols=30u32 placeholder="Fluid" />
        </Demo>
    }
}
