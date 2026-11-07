//! Страница Steps: демонстрации `basic`, `controlled` и `interactive`. В локальной копии PrimeNG каталога `apps/showcase/doc/steps`
//! нет (демонстрации удалены апстримом), поэтому набор выведен из `pages/steps/index.ts` и кода компонента; `routing` не
//! переносится (нет маршрутизатора Angular).

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_steps::{Steps, StepsCommand, StepsItem};
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};

fn labels() -> Vec<&'static str> {
    vec!["Personal", "Seat", "Payment", "Confirmation"]
}

fn plain() -> Vec<StepsItem> {
    labels().into_iter().map(|l| StepsItem::new().label(l)).collect()
}

#[component]
fn ControlledDemo() -> impl IntoView {
    let active = RwSignal::new(0usize);
    let last = labels().len() - 1;
    view! {
        <Steps model=plain() active_index_signal=active />
        <div style="display: flex; gap: 0.5rem; margin-top: 1rem">
            <Button
                label="Back"
                severity=ButtonSeverity::Secondary
                disabled=Signal::derive(move || active.get() == 0)
                on_click=Callback::new(move |_| active.update(|a| *a = a.saturating_sub(1)))
            />
            <Button
                label="Next"
                disabled=Signal::derive(move || active.get() == last)
                on_click=Callback::new(move |_| active.update(|a| *a = (*a + 1).min(last)))
            />
        </div>
    }
}

#[component]
fn InteractiveDemo() -> impl IntoView {
    let toast = provide_message_service();
    let active = RwSignal::new(0usize);
    let items = labels()
        .into_iter()
        .map(|label| {
            StepsItem::new().label(label).command(Callback::new(move |c: StepsCommand| {
                toast.add(ToastMessage::new(ToastSeverity::Info, "Step", format!("{} — step {}", label, c.index + 1)).life(3000));
            }))
        })
        .collect::<Vec<_>>();
    // Четвёртый шаг недоступен, пока не пройден третий.
    let items = items.into_iter().enumerate().map(|(i, it)| if i == 3 { it.disabled(true) } else { it }).collect::<Vec<_>>();
    view! {
        <Toast />
        <Steps model=items active_index_signal=active readonly=false />
    }
}

/// Страница `/steps`.
#[component]
pub fn StepsPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <div style="width: 100%" data-testid="basic">
                <Steps model=plain() active_index=1usize />
            </div>
        </Demo>
        <Demo name="controlled" title="Controlled">
            <div style="width: 100%" data-testid="controlled">
                <ControlledDemo />
            </div>
        </Demo>
        <Demo name="interactive" title="Interactive">
            <div style="width: 100%" data-testid="interactive">
                <InteractiveDemo />
            </div>
        </Demo>
    }
}
