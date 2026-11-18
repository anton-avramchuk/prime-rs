//! prime-rs demo app: one page per component, a shared theme and a light/dark scheme switch.
#![recursion_limit = "512"]

mod app;
mod doc;
#[rustfmt::skip]
mod generated;
mod pages;

pub use app::{App, shell};

/// Hydration entry point in the browser.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
    expose_wasm_memory();
}

/// Client-only entry for the static (GitHub Pages) build: injects the theme styles into `<head>`, then mounts the app.
#[cfg(feature = "csr")]
pub fn mount_csr() {
    console_error_panic_hook::set_once();
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(head) = document.head() {
            for css in [app::theme_css(), doc::DOC_CSS.to_string(), doc::SITE_CSS.to_string()] {
                if let Ok(style) = document.create_element("style") {
                    style.set_text_content(Some(&css));
                    let _ = head.append_child(&style);
                }
            }
        }
    }
    leptos::mount::mount_to_body(App);
    expose_wasm_memory();
}

/// Exposes the wasm linear memory size as `window.wasmMemoryBytes()`: the JS heap does not include Rust memory,
/// so e2e measurements (`e2e/tests/memory.spec.ts`) read it from here.
#[cfg(any(feature = "hydrate", feature = "csr"))]
fn expose_wasm_memory() {
    use wasm_bindgen::JsCast;
    let measure = wasm_bindgen::closure::Closure::<dyn Fn() -> f64>::new(|| {
        wasm_bindgen::memory()
            .unchecked_into::<js_sys::WebAssembly::Memory>()
            .buffer()
            .unchecked_into::<js_sys::ArrayBuffer>()
            .byte_length() as f64
    });
    if let Some(window) = web_sys::window() {
        let _ = js_sys::Reflect::set(&window, &"wasmMemoryBytes".into(), measure.as_ref());
    }
    measure.forget();
}
