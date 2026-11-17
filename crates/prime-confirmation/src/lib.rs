//! Confirmation and ConfirmationService — port of `api/confirmation.ts` and `api/confirmationservice.ts` from PrimeNG 21.1.10.
//!
//! Confirmation description ([`Confirmation`]) and the shared service ([`ConfirmationService`]): the application calls `confirm`, and
//! the `ConfirmPopup` and `ConfirmDialog` components show the confirmations whose `key` matches their own. The crate without
//! any UI or styles.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_confirmation::{Confirmation, ConfirmEvent, provide_confirmation_service};
//!
//! Owner::new().with(|| {
//!     let service = provide_confirmation_service();
//!     let seen = StoredValue::new(Vec::<String>::new());
//!     service.subscribe(Callback::new(move |e: ConfirmEvent| {
//!         if let ConfirmEvent::Confirm(c) = e {
//!             seen.update_value(|s| s.push(c.message.clone().unwrap_or_default()));
//!         }
//!     }));
//!     service.confirm(Confirmation::new().message("Sure?"));
//!     assert_eq!(seen.get_value(), ["Sure?"]);
//! });
//! ```

mod confirmation;
mod service;

pub use confirmation::{ConfirmButtonProps, ConfirmEventType, ConfirmFocus, Confirmation};
pub use service::{ConfirmEvent, ConfirmationService, provide_confirmation_service, use_confirmation_service};
