//! Демо-приложение prime-rs: страница на компонент, общая тема и переключатель светлой и тёмной схемы.

mod app;
mod pages;

pub use app::{App, shell};

/// Точка входа гидрации в браузере.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
