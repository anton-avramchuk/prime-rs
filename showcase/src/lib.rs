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

/// Exposes the wasm linear memory size as `window.wasmMemoryBytes()`: the JS heap does not include Rust memory,
/// so e2e measurements (`e2e/tests/memory.spec.ts`) read it from here.
#[cfg(feature = "hydrate")]
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
