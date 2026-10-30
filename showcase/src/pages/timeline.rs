//! Страница Timeline: демонстрации `basic`, `alignment`, `horizontal`, `opposite` по `apps/showcase/doc/timeline` PrimeNG.
//!
//! Демонстрации `template` нет: она опирается на иконки и карточки, которых нет в `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_timeline::{Timeline, TimelineAlign, TimelineLayout};

#[derive(Clone)]
struct EventItem {
    status: &'static str,
    date: &'static str,
}

fn events() -> Vec<EventItem> {
    vec![
        EventItem { status: "Ordered", date: "15/10/2020 10:30" },
        EventItem { status: "Processing", date: "15/10/2020 14:00" },
        EventItem { status: "Shipped", date: "15/10/2020 16:15" },
        EventItem { status: "Delivered", date: "16/10/2020 10:00" },
    ]
}

fn status() -> Callback<EventItem, AnyView> {
    Callback::new(|e: EventItem| e.status.into_any())
}

fn years() -> Vec<&'static str> {
    vec!["2020", "2021", "2022", "2023"]
}

fn year() -> Callback<&'static str, AnyView> {
    Callback::new(|e: &'static str| e.into_any())
}

/// Страница `/timeline`.
#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <h2>"Timeline"</h2>
        <Demo name="basic" title="Basic">
            <Timeline value={events()} content=status() />
        </Demo>
        <Demo name="alignment" title="Alignment">
            <Timeline value={events()} content=status() style="width: 20rem" />
            <Timeline value={events()} content=status() align=TimelineAlign::Right style="width: 20rem" />
            <Timeline value={events()} content=status() align=TimelineAlign::Alternate style="width: 20rem" />
        </Demo>
        <Demo name="horizontal" title="Horizontal">
            <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%">
                <Timeline value={years()} content=year() layout=TimelineLayout::Horizontal align=TimelineAlign::Top />
                <Timeline value={years()} content=year() layout=TimelineLayout::Horizontal align=TimelineAlign::Bottom />
                <Timeline
                    value={years()}
                    content=year()
                    opposite=Callback::new(|_: &'static str| view! { <span>"\u{a0}"</span> }.into_any())
                    layout=TimelineLayout::Horizontal
                    align=TimelineAlign::Alternate
                />
            </div>
        </Demo>
        <Demo name="opposite" title="Opposite">
            <Timeline
                value={events()}
                content=status()
                opposite=Callback::new(|e: EventItem| view! { <small style="color: var(--p-text-muted-color)">{e.date}</small> }.into_any())
            />
        </Demo>
    }
}
