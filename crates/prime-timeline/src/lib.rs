//! Timeline — порт `p-timeline` из PrimeNG 21.1.10.
//!
//! Корень `div.p-timeline` (вместо хоста `<p-timeline>`); на каждое событие — `div.p-timeline-event` с тремя частями
//! (подпись напротив, разделитель с маркером и соединителем, содержимое). Шаблоны `content`, `opposite` и `marker` —
//! колбэки, получающие событие. `align` — перечисление из пяти значений, которые знает CSS темы.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_timeline::Timeline;
//!
//! let content = Callback::new(|e: &'static str| view! { <b>{e}</b> }.into_any());
//! let html = view! { <Timeline value={vec!["a", "b"]} content=content /> }.to_html();
//! assert!(html.contains("p-timeline-vertical") && html.contains("p-timeline-event-connector"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{TimelineAlign, TimelineLayout, data_p, write_classes};

/// Pass-through частей Timeline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimelinePt {
    /// Корневой элемент.
    pub root: Pt,
    /// Событие.
    pub event: Pt,
    /// Часть напротив линии.
    pub event_opposite: Pt,
    /// Разделитель (маркер и соединитель).
    pub event_separator: Pt,
    /// Стандартный маркер.
    pub event_marker: Pt,
    /// Соединитель.
    pub event_connector: Pt,
    /// Содержимое события.
    pub event_content: Pt,
}

/// Шаблон части события: получает событие, возвращает разметку.
pub type TimelineSlot<T> = Callback<T, AnyView>;

/// Цепочка событий.
///
/// Каждое свойство принимает значение или сигнал; сигнал `value` перестраивает список событий.
#[component]
pub fn Timeline<T>(
    /// События.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// Положение линии.
    #[prop(optional, into)]
    align: Prop<TimelineAlign>,
    /// Ориентация.
    #[prop(optional, into)]
    layout: Prop<TimelineLayout>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Шаблон содержимого события.
    #[prop(optional, into)]
    content: Option<TimelineSlot<T>>,
    /// Шаблон части напротив линии.
    #[prop(optional, into)]
    opposite: Option<TimelineSlot<T>>,
    /// Шаблон маркера (заменяет стандартный).
    #[prop(optional, into)]
    marker: Option<TimelineSlot<T>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<TimelinePt>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let pt = Arc::new(pt.unwrap_or_default());
    let attrs = pt.root.attrs();
    let class = {
        let (pt, style_class, align, layout) = (pt.clone(), style_class, align.clone(), layout.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, align.get(), layout.get());
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        move || style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
    };
    let data = {
        let (align, layout) = (align.clone(), layout.clone());
        move || data_p(align.get(), layout.get())
    };
    let events = {
        let (pt, data) = (pt.clone(), data.clone());
        move || {
            let data = data().into_owned();
            let items = value.get();
            let last = items.len().saturating_sub(1);
            items.into_iter().enumerate().map(|(i, item)| event_view(&pt, &data, &item, i < last, content, opposite, marker)).collect_view()
        }
    };
    view! {
        <div class=class style=style data-p={data} {..attrs}>
            {events}
        </div>
    }
}

/// Разметка одного события.
fn event_view<T: Clone + Send + Sync + 'static>(
    pt: &TimelinePt,
    data: &str,
    item: &T,
    connector: bool,
    content: Option<TimelineSlot<T>>,
    opposite: Option<TimelineSlot<T>>,
    marker: Option<TimelineSlot<T>>,
) -> impl IntoView + use<T> {
    let part = |p: &Pt, base: &'static str| (p.merge_class(base), p.has_style().then(|| p.merge_style("")), p.attrs());
    let (ec, es, ea) = part(&pt.event, "p-timeline-event");
    let (oc, os, oa) = part(&pt.event_opposite, "p-timeline-event-opposite");
    let (sc, ss, sa) = part(&pt.event_separator, "p-timeline-event-separator");
    let (cc, cs, ca) = part(&pt.event_content, "p-timeline-event-content");
    let marker_view = match marker {
        Some(m) => m.run(item.clone()),
        None => {
            let (mc, ms, ma) = part(&pt.event_marker, "p-timeline-event-marker");
            view! { <div class=mc style=ms data-p=data.to_owned() {..ma}></div> }.into_any()
        }
    };
    let connector_view = connector.then(|| {
        let (kc, ks, ka) = part(&pt.event_connector, "p-timeline-event-connector");
        view! { <div class=kc style=ks data-p=data.to_owned() {..ka}></div> }
    });
    let opposite_view = opposite.map(|o| o.run(item.clone()));
    let content_view = content.map(|c| c.run(item.clone()));
    let (d1, d2, d3, d4) = (data.to_owned(), data.to_owned(), data.to_owned(), data.to_owned());
    view! {
        <div class=ec style=es data-p={d1} {..ea}>
            <div class=oc style=os data-p={d2} {..oa}>{opposite_view}</div>
            <div class=sc style=ss data-p={d3} {..sa}>
                {marker_view}
                {connector_view}
            </div>
            <div class=cc style=cs data-p={d4} {..ca}>{content_view}</div>
        </div>
    }
}
