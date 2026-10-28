//! DOM-хелперы для оверлеев: позиционирование, ловушка фокуса, портал и ripple.
//!
//! Все функции безопасны для SSR: вне браузера они ничего не делают, а чистая арифметика
//! позиционирования ([`position`]) работает на любой цели.
//!
//! - [`position`] — порт `absolutePosition` и `relativePosition` из `DomHandler` с теми же правилами переворота;
//! - [`dom::align_overlay`] — измеряет якорь и оверлей и записывает `top`, `left`, `transform-origin`;
//! - [`FocusTrap`] — ловушка фокуса на служебных элементах `.p-hidden-focusable`, как `pFocusTrap`;
//! - [`Portal`] — вынос содержимого в `body` (из Leptos);
//! - [`Ripple`] — `span.p-ink`, активируемый на `mousedown`.
//!
//! Браузерные тесты: `cargo test -p prime-dom --target wasm32-unknown-unknown --test web`
//! (нужны `wasm-bindgen-cli` и `chromedriver`, см. `docs/build.md`).

pub mod dom;
pub mod focus;
pub mod position;
pub mod ripple;

pub use dom::{AlignMode, align_overlay};
pub use focus::{FocusTrap, first_focusable, last_focusable};
pub use leptos::portal::Portal;
pub use position::{Origin, Placement, absolute_position, relative_position};
pub use ripple::Ripple;
