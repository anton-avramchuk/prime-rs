//! DOM helpers for overlays: positioning, focus trap, portal and ripple.
//!
//! All functions are SSR-safe: outside the browser they do nothing, and the pure positioning arithmetic
//! ([`position`]) works on any target.
//!
//! - [`position`] — port of `absolutePosition` and `relativePosition` from `DomHandler` with the same flipping rules;
//! - [`dom::align_overlay`] — measures the anchor and the overlay and writes `top`, `left`, `transform-origin`;
//! - [`FocusTrap`] — focus trap on the service `.p-hidden-focusable` elements, like `pFocusTrap`;
//! - [`Portal`] — moves the content into `body` (from Leptos);
//! - [`Ripple`] — `span.p-ink`, activated on `mousedown`;
//! - [`scroll`] — closing popup panels on scroll ([`watch_scroll`]).
//!
//! Browser tests: `cargo test -p prime-dom --target wasm32-unknown-unknown --test web`
//! (requires `wasm-bindgen-cli` and `chromedriver`, see `docs/build.md`).

pub mod dom;
pub mod focus;
pub mod position;
pub mod ripple;
pub mod scroll;

pub use dom::{AlignMode, align_overlay, block_body_scroll, unblock_body_scroll};
pub use focus::{FocusTrap, first_focusable, last_focusable};
pub use leptos::portal::Portal;
pub use position::{Origin, Placement, absolute_position, relative_position};
pub use ripple::Ripple;
pub use scroll::{focus_no_scroll, scroll_hides, target_moved, watch_scroll};
