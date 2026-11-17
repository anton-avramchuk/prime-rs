//! Unique numbers for identifiers and names (`pn_id_<n>`).

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Next unique number: from the shared hydration context (the same on the server and in the browser), and outside it from the global counter.
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

/// Next unique number from the global counter.
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
