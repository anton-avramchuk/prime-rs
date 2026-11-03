//! `ConfirmationService` — общая шина подтверждений между приложением и компонентами (порт `ConfirmationService` PrimeNG).

use crate::confirmation::Confirmation;
use leptos::prelude::*;

/// Событие шины.
#[derive(Clone, Debug)]
pub enum ConfirmEvent {
    /// Показать подтверждение.
    Confirm(Box<Confirmation>),
    /// Закрыть показанное подтверждение.
    Close,
}

/// Сервис подтверждений: `Copy`-хэндл, который кладётся в контекст и доступен из любого обработчика.
///
/// ```
/// use leptos::prelude::*;
/// use prime_confirmation::{Confirmation, provide_confirmation_service};
///
/// Owner::new().with(|| {
///     let service = provide_confirmation_service();
///     // подписчиков ещё нет — подтверждение просто никому не доставляется
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
    /// Создаёт сервис без подписчиков. Вызывать внутри компонента (владелец хранилищ).
    pub fn new() -> Self {
        Self { subscribers: StoredValue::new(Vec::new()), next: StoredValue::new(0) }
    }

    /// Просит показать подтверждение.
    pub fn confirm(&self, confirmation: Confirmation) {
        self.emit(ConfirmEvent::Confirm(Box::new(confirmation)));
    }

    /// Просит закрыть показанное подтверждение.
    pub fn close(&self) {
        self.emit(ConfirmEvent::Close);
    }

    /// Подписывает обработчик; возвращает номер для [`ConfirmationService::unsubscribe`].
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

    /// Снимает подписку.
    pub fn unsubscribe(&self, id: usize) {
        self.subscribers.try_update_value(|s| s.retain(|(i, _)| *i != id));
    }

    fn emit(&self, event: ConfirmEvent) {
        // Копия списка: обработчик может отписаться во время доставки.
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

/// Создаёт сервис и кладёт его в контекст текущего владельца.
pub fn provide_confirmation_service() -> ConfirmationService {
    let service = ConfirmationService::new();
    provide_context(service);
    service
}

/// Берёт сервис из контекста; паникует с понятным сообщением, если `provide_confirmation_service()` не вызывался.
pub fn use_confirmation_service() -> ConfirmationService {
    use_context::<ConfirmationService>()
        .expect("ConfirmationService не найден: вызовите provide_confirmation_service() в корне приложения (или выше компонента)")
}
