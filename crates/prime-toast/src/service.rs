//! `MessageService` — shared message bus between the app and `Toast` components (port of PrimeNG `MessageService`).

use crate::message::ToastMessage;
use leptos::prelude::*;

/// Bus event.
#[derive(Clone, Debug)]
pub enum ServiceEvent {
    /// Add messages.
    Add(Vec<ToastMessage>),
    /// Close messages: all (`None`) or only `Toast`s with the key.
    Clear(Option<String>),
}

/// Message service: a `Copy` handle put into context and available from any handler.
///
/// ```
/// use leptos::prelude::*;
/// use prime_toast::{ToastMessage, ToastSeverity, provide_message_service};
///
/// Owner::new().with(|| {
///     let service = provide_message_service();
///     // no subscribers yet — the message is simply delivered to nobody
///     service.add(ToastMessage::new(ToastSeverity::Success, "Saved", "Done"));
///     service.clear();
/// });
/// ```
#[derive(Clone, Copy)]
pub struct MessageService {
    subscribers: StoredValue<Vec<(usize, Callback<ServiceEvent>)>>,
    next: StoredValue<usize>,
}

impl MessageService {
    /// Creates a service with no subscribers. Call inside a component (storage owner).
    pub fn new() -> Self {
        Self { subscribers: StoredValue::new(Vec::new()), next: StoredValue::new(0) }
    }

    /// Adds a message.
    pub fn add(&self, message: ToastMessage) {
        self.emit(ServiceEvent::Add(vec![message]));
    }

    /// Adds several messages.
    pub fn add_all(&self, messages: Vec<ToastMessage>) {
        if !messages.is_empty() {
            self.emit(ServiceEvent::Add(messages));
        }
    }

    /// Closes messages in all `Toast`s.
    pub fn clear(&self) {
        self.emit(ServiceEvent::Clear(None));
    }

    /// Closes messages in `Toast`s with this key.
    pub fn clear_key(&self, key: impl Into<String>) {
        self.emit(ServiceEvent::Clear(Some(key.into())));
    }

    /// Subscribes a handler; returns an id for [`MessageService::unsubscribe`].
    pub fn subscribe(&self, callback: Callback<ServiceEvent>) -> usize {
        let id = self.next.try_update_value(|n| {
            let id = *n;
            *n += 1;
            id
        });
        let id = id.unwrap_or_default();
        self.subscribers.update_value(|s| s.push((id, callback)));
        id
    }

    /// Removes a subscription.
    pub fn unsubscribe(&self, id: usize) {
        self.subscribers.try_update_value(|s| s.retain(|(i, _)| *i != id));
    }

    fn emit(&self, event: ServiceEvent) {
        // Copy of the list: a handler may unsubscribe during delivery.
        let subscribers = self.subscribers.try_get_value().unwrap_or_default();
        for (_, callback) in subscribers {
            callback.run(event.clone());
        }
    }
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates the service and puts it into the current owner context.
pub fn provide_message_service() -> MessageService {
    let service = MessageService::new();
    provide_context(service);
    service
}

/// Takes the service from context; panics with a clear message if `provide_message_service()` was not called.
pub fn use_message_service() -> MessageService {
    use_context::<MessageService>()
        .expect("MessageService not found: call provide_message_service() at the app root (or above the Toast component)")
}
