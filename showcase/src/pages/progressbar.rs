//! Страница ProgressBar: демонстрации `basic`, `dynamic`, `indeterminate`, `template` из showcase PrimeNG
//! (`apps/showcase/doc/progressbar`). В `dynamic` значение растёт детерминированно (+10 каждые 500 мс), а не случайно.

use super::Demo;
use leptos::prelude::*;
use prime_progressbar::{ProgressBar, ProgressBarMode};
use std::time::Duration;

/// Страница `/progressbar`.
#[component]
pub fn ProgressBarPage() -> impl IntoView {
    let value = RwSignal::new(Some(0.0_f64));
    Effect::new(move |_| {
        // Интервал только в браузере: Effect не выполняется при SSR.
        let handle = set_interval_with_handle(
            move || {
                if value.get_untracked().unwrap_or_default() < 100.0 {
                    value.update(|v| *v = Some((v.unwrap_or_default() + 10.0).min(100.0)));
                }
            },
            Duration::from_millis(500),
        );
        if let Ok(handle) = handle {
            on_cleanup(move || handle.clear());
        }
    });
    let content = Callback::new(|v: Option<f64>| view! { <span>{format!("{}/100", v.unwrap_or_default())}</span> }.into_any());
    view! {
        <h2>"ProgressBar"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <ProgressBar value=50.0 />
            </div>
        </Demo>
        <Demo name="dynamic" title="Dynamic">
            <div style="width: 100%">
                <ProgressBar value=value />
            </div>
        </Demo>
        <Demo name="indeterminate" title="Indeterminate">
            <div style="width: 100%">
                <ProgressBar mode=ProgressBarMode::Indeterminate style="height: 6px" />
            </div>
        </Demo>
        <Demo name="template" title="Template">
            <div style="width: 100%">
                <ProgressBar value=50.0 content_template=content />
            </div>
        </Demo>
    }
}
