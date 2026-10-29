//! Демо-приложение prime-rs: страница на компонент, общая тема и переключатель светлой и тёмной схемы.
#![recursion_limit = "256"]

mod app;
mod pages;

pub use app::{App, shell};

/// Точка входа гидрации в браузере.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
    expose_wasm_memory();
}

/// Выставляет в `window.wasmMemoryBytes()` размер линейной памяти wasm: JS-куча не включает память Rust,
/// поэтому e2e-замеры (`e2e/tests/memory.spec.ts`) читают её отсюда.
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
