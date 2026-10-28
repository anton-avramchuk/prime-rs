//! SVG-иконки PrimeNG: по компоненту Leptos на иконку, каждая за своим feature-флагом.
//!
//! Данные контуров (`d`) и `viewBox` совпадают с `packages/primeng/src/icons`; код создаётся командой
//! `cargo xtask gen-icons`. Не включённые иконки не попадают в сборку.
//!
//! Общие свойства: `class`, `spin` (класс `p-icon-spin`), `size` (по умолчанию 14), `aria_label`.
//! Без `aria_label` иконка декоративная (`aria-hidden="true"`), с ним — `role="img"` и `aria-label`.
//!
//! Одно отличие от PrimeNG: иконки с обёрткой `<g clip-path>` и `clipPath` на весь `viewBox` рисуются
//! без неё, так как она ничего не обрезает; это сокращает DOM иконки примерно втрое.

#[macro_use]
mod icon;
pub use icon::{VIEW_BOX, icon_view};

#[cfg(feature = "blank")]
pub use icon::BlankIcon;

#[allow(missing_docs)]
#[rustfmt::skip]
mod generated;
pub use generated::*;
