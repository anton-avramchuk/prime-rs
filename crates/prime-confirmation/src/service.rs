//! `ConfirmationService` — a shared bus of confirmations between the application and components (port of PrimeNG `ConfirmationService`).

use crate::confirmation::Confirmation;
use leptos::prelude::*;

/// Bus event.
#[derive(Clone, Debug)]
pub enum ConfirmEvent {
    /// Show a confirmation.
    Confirm(Box<Confirmation>),
    /// Close the shown confirmation.
    Close,
}

/// Confirmation service: a `Copy` handle that is put into the context and is accessible from any handler.
///
/// ```
/// use leptos::prelude::*;
/// use prime_confirmation::{Confirmation, provide_confirmation_service};
///
/// Owner::new().with(|| {
///     let service = provide_confirmation_service();
///     // there are no subscribers yet — the confirmation is simply not delivered to anyone
///     service.confirm(Confirmation::new().message("Sure?"));
///     service.close();
/// });
/// ```
#[derive(Clone, Copy)]
pub struct ConfirmationService {
    subscribers: StoredValue<Vec<(usize, Callback<ConfirmEvent>)>>,
    next: StoredValue<usize>,
}

impl ConfirmationService {
    /// Creates a service without subscribers. Call inside a component (the owner of the storage).
    pub fn new() -> Self {
        Self { subscribers: StoredValue::new(Vec::new()), next: StoredValue::new(0) }
    }

    /// Asks to show a confirmation.
    pub fn confirm(&self, confirmation: Confirmation) {
        self.emit(ConfirmEvent::Confirm(Box::new(confirmation)));
    }

    /// Asks to close the shown confirmation.
    pub fn close(&self) {
        self.emit(ConfirmEvent::Close);
    }

    /// Subscribes a handler; returns the number for [`ConfirmationService::unsubscribe`].
    pub fn subscribe(&self, callback: Callback<ConfirmEvent>) -> usize {
        let id = self
            .next
            .try_update_value(|n| {
                let id = *n;
                *n += 1;
                id
            })
            .unwrap_or_default();
        self.subscribers.update_value(|s| s.push((id, callback)));
        id
    }

    /// Removes the subscription.
    pub fn unsubscribe(&self, id: usize) {
        self.subscribers.try_update_value(|s| s.retain(|(i, _)| *i != id));
    }

    fn emit(&self, event: ConfirmEvent) {
        // A copy of the list: a handler may unsubscribe during delivery.
        let subscribers = self.subscribers.try_get_value().unwrap_or_default();
        for (_, callback) in subscribers {
            callback.run(event.clone());
        }
    }
}

impl Default for ConfirmationService {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates the service and puts it into the context of the current owner.
pub fn provide_confirmation_service() -> ConfirmationService {
    let service = ConfirmationService::new();
    provide_context(service);
    service
}

/// Gets the service from the context; panics with a clear message if `provide_confirmation_service()` was not called.
pub fn use_confirmation_service() -> ConfirmationService {
    use_context::<ConfirmationService>()
        .expect("ConfirmationService not found: call provide_confirmation_service() at the application root (or above the component)")
}
