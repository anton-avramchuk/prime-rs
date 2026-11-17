//! Timeline — port of `p-timeline` from PrimeNG 21.1.10.
//!
//! Root `div.p-timeline` (instead of the `<p-timeline>` host); for each event a `div.p-timeline-event` with three parts
//! (an opposite label, a separator with a marker and a connector, the content). The `content`, `opposite` and `marker` templates are
//! callbacks that receive the event. `align` is an enum of five values known to the theme CSS.
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

/// Pass-through of the Timeline parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimelinePt {
    /// Root element.
    pub root: Pt,
    /// Event.
    pub event: Pt,
    /// Part opposite the line.
    pub event_opposite: Pt,
    /// Separator (marker and connector).
    pub event_separator: Pt,
    /// Standard marker.
    pub event_marker: Pt,
    /// Connector.
    pub event_connector: Pt,
    /// Event content.
    pub event_content: Pt,
}

/// Event part template: receives the event, returns markup.
pub type TimelineSlot<T> = Callback<T, AnyView>;

/// A chain of events.
///
/// Every property accepts a value or a signal; the `value` signal rebuilds the event list.
#[component]
pub fn Timeline<T>(
    /// Events.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// Line position.
    #[prop(optional, into)]
    align: Prop<TimelineAlign>,
    /// Orientation.
    #[prop(optional, into)]
    layout: Prop<TimelineLayout>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Event content template.
    #[prop(optional, into)]
    content: Option<TimelineSlot<T>>,
    /// Template of the part opposite the line.
    #[prop(optional, into)]
    opposite: Option<TimelineSlot<T>>,
    /// Marker template (replaces the standard one).
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

/// Markup of a single event.
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
