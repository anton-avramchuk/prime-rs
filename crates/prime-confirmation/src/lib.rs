//! Confirmation и ConfirmationService — порт `api/confirmation.ts` и `api/confirmationservice.ts` из PrimeNG 21.1.10.
//!
//! Описание подтверждения ([`Confirmation`]) и общий сервис ([`ConfirmationService`]): приложение вызывает `confirm`, а
//! компоненты `ConfirmPopup` и `ConfirmDialog` показывают те подтверждения, чей `key` совпадает с их собственным. Крейт без
//! интерфейса и стилей.
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

pub use confirmation::{ConfirmButtonProps, ConfirmFocus, Confirmation};
pub use service::{ConfirmEvent, ConfirmationService, provide_confirmation_service, use_confirmation_service};
