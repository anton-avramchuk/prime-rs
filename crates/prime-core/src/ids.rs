//! Уникальные номера для идентификаторов и имён (`pn_id_<n>`).

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Следующий уникальный номер: из общего контекста гидратации (одинаковый на сервере и в браузере), а вне его — из глобального счётчика.
///
/// ```
/// use prime_core::ids::next_unique;
///
/// assert_ne!(next_unique(), next_unique());
/// ```
#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub fn next_unique() -> usize {
    use leptos::prelude::*;

    match Owner::current_shared_context() {
        Some(ctx) => ctx.next_id().into_inner(),
        None => NEXT_ID.fetch_add(1, Ordering::Relaxed),
    }
}

/// Следующий уникальный номер из глобального счётчика.
///
/// ```
/// use prime_core::ids::next_unique;
///
/// assert_ne!(next_unique(), next_unique());
/// ```
#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
pub fn next_unique() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
