//! `MessageService` — общая шина сообщений между приложением и компонентами `Toast` (порт `MessageService` PrimeNG).

use crate::message::ToastMessage;
use leptos::prelude::*;

/// Событие шины.
#[derive(Clone, Debug)]
pub enum ServiceEvent {
    /// Добавить сообщения.
    Add(Vec<ToastMessage>),
    /// Закрыть сообщения: все (`None`) или только `Toast` с ключом.
    Clear(Option<String>),
}

/// Сервис сообщений: `Copy`-хэндл, который кладётся в контекст и доступен из любого обработчика.
///
/// ```
/// use leptos::prelude::*;
/// use prime_toast::{ToastMessage, ToastSeverity, provide_message_service};
///
/// Owner::new().with(|| {
///     let service = provide_message_service();
///     // подписчиков ещё нет — сообщение просто никому не доставляется
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
    /// Создаёт сервис без подписчиков. Вызывать внутри компонента (владелец хранилищ).
    pub fn new() -> Self {
        Self { subscribers: StoredValue::new(Vec::new()), next: StoredValue::new(0) }
    }

    /// Добавляет сообщение.
    pub fn add(&self, message: ToastMessage) {
        self.emit(ServiceEvent::Add(vec![message]));
    }

    /// Добавляет несколько сообщений.
    pub fn add_all(&self, messages: Vec<ToastMessage>) {
        if !messages.is_empty() {
            self.emit(ServiceEvent::Add(messages));
        }
    }

    /// Закрывает сообщения во всех `Toast`.
    pub fn clear(&self) {
        self.emit(ServiceEvent::Clear(None));
    }

    /// Закрывает сообщения в `Toast` с этим ключом.
    pub fn clear_key(&self, key: impl Into<String>) {
        self.emit(ServiceEvent::Clear(Some(key.into())));
    }

    /// Подписывает обработчик; возвращает номер для [`MessageService::unsubscribe`].
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

    /// Снимает подписку.
    pub fn unsubscribe(&self, id: usize) {
        self.subscribers.try_update_value(|s| s.retain(|(i, _)| *i != id));
    }

    fn emit(&self, event: ServiceEvent) {
        // Копия списка: обработчик может отписаться во время доставки.
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

/// Создаёт сервис и кладёт его в контекст текущего владельца.
pub fn provide_message_service() -> MessageService {
    let service = MessageService::new();
    provide_context(service);
    service
}

/// Берёт сервис из контекста; паникует с понятным сообщением, если `provide_message_service()` не вызывался.
pub fn use_message_service() -> MessageService {
    use_context::<MessageService>()
        .expect("MessageService не найден: вызовите provide_message_service() в корне приложения (или выше компонента Toast)")
}
