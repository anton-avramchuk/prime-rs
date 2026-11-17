//! PrimeNG SVG icons: one Leptos component per icon, each behind its own feature flag.
//!
//! Path data (`d`) and `viewBox` match `packages/primeng/src/icons`; the code is produced by the command
//! `cargo xtask gen-icons`. Icons that are not enabled do not end up in the build.
//!
//! Common properties: `class`, `spin` (class `p-icon-spin`), `size` (14 by default), `aria_label`.
//! Without `aria_label` the icon is decorative (`aria-hidden="true"`); with it, `role="img"` and `aria-label` are set.
//!
//! One difference from PrimeNG: icons with a `<g clip-path>` wrapper and a `clipPath` over the whole `viewBox` are drawn
//! without it, since it clips nothing; this cuts the icon DOM by about a third.

#[macro_use]
mod icon;
pub use icon::{VIEW_BOX, icon_view};

#[cfg(feature = "blank")]
pub use icon::BlankIcon;

#[allow(missing_docs)]
#[rustfmt::skip]
mod generated;
#[allow(unused_imports)]
pub use generated::*;
