//! prime-rs core: configuration, class builder, shared props, pass-through and z-index.
//!
//! Components depend on this crate and do not duplicate its code.

pub mod class_names;
pub mod collapsible;
pub mod config;
pub mod data_p;
pub mod fluid;
pub mod ids;
pub mod prop;
pub mod props;
pub mod pt;
mod translation_defaults;
pub mod wrapper;
pub mod zindex;

pub use class_names::{ClassNames, class_memo};
pub use config::{Locale, PrimeConfig, ZIndexBases, provide_prime_config, use_prime_config};
pub use data_p::DataP;
pub use prop::{OptProp, Prop};
pub use props::{Size, Variant};
pub use pt::Pt;
pub use zindex::ZGroup;

/// `"true"`/`"false"` for ARIA and `data-p-*` state attributes, without allocating like `bool::to_string`.
///
/// ```
/// assert_eq!(prime_rs_core::bool_attr(true), "true");
/// assert_eq!(prime_rs_core::bool_attr(false), "false");
/// ```
pub fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}
