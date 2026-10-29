//! Ядро prime-rs: конфигурация, построитель классов, общие пропсы, pass-through и z-index.
//!
//! Компоненты зависят от этого крейта и не дублируют его код.

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
