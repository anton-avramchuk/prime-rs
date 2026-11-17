//! MeterGroup: a port of `p-metergroup` from PrimeNG 21.1.10.
//!
//! Root `div.p-metergroup[role=meter]` (instead of the host `<p-metergroup>`), segments in `div.p-metergroup-meters` and a list
//! of labels before or after them. The `label`, `start`, `end`, `meter`, `icon` templates are callbacks. An item icon is a `ViewFn`
//! (for example, an icon from `prime-icons`), not a font class name.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_metergroup::{MeterGroup, MeterItem};
//!
//! let items = vec![MeterItem::new("Apps", "#34d399", 16.0)];
//! let html = view! { <MeterGroup value=items /> }.to_html();
//! assert!(html.contains("width: 16%") && html.contains("Apps"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{LabelPosition, MeterOrientation, cumulative, label_list_classes, percent, total_percent, write_classes};

/// Meter item.
#[derive(Clone)]
pub struct MeterItem {
    /// Label.
    pub label: String,
    /// Segment and marker color (any CSS value).
    pub color: String,
    /// Value.
    pub value: f64,
    /// Icon instead of the marker in the label.
    pub icon: Option<ViewFn>,
}

impl MeterItem {
    /// Item without an icon.
    pub fn new(label: impl Into<String>, color: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), color: color.into(), value, icon: None }
    }

    /// Sets the label icon.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Context of the `label`, `start` and `end` templates.
#[derive(Clone)]
pub struct MeterGroupContext {
    /// Items.
    pub items: Vec<MeterItem>,
    /// Total percentage.
    pub total_percent: u32,
    /// Cumulative sums of the values.
    pub percentages: Vec<f64>,
}

/// Pass-through parts of MeterGroup.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MeterGroupPt {
    /// Root element.
    pub root: Pt,
    /// Segments container.
    pub meters: Pt,
    /// Segment.
    pub meter: Pt,
    /// Labels list.
    pub label_list: Pt,
    /// Label.
    pub label: Pt,
    /// Label icon.
    pub label_icon: Pt,
    /// Label marker.
    pub label_marker: Pt,
    /// Label text.
    pub label_text: Pt,
}

/// Composite meter.
///
/// Every property accepts a value or a signal; the `value` signal rebuilds the segments and labels.
#[component]
pub fn MeterGroup(
    /// Items.
    #[prop(optional, into)]
    value: Prop<Vec<MeterItem>>,
    /// Lower bound of the range.
    #[prop(default = Prop::Static(0.0), into)]
    min: Prop<f64>,
    /// Upper bound of the range.
    #[prop(default = Prop::Static(100.0), into)]
    max: Prop<f64>,
    /// Meter orientation.
    #[prop(optional, into)]
    orientation: Prop<MeterOrientation>,
    /// Label position.
    #[prop(optional, into)]
    label_position: Prop<LabelPosition>,
    /// Label list orientation.
    #[prop(optional, into)]
    label_orientation: Prop<MeterOrientation>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Template of the whole label list.
    #[prop(optional, into)]
    label: Option<Callback<MeterGroupContext, AnyView>>,
    /// Segment template (item and its index).
    #[prop(optional, into)]
    meter: Option<Callback<(MeterItem, usize), AnyView>>,
    /// Template before the meter.
    #[prop(optional, into)]
    start: Option<Callback<MeterGroupContext, AnyView>>,
    /// Template after the meter.
    #[prop(optional, into)]
    end: Option<Callback<MeterGroupContext, AnyView>>,
    /// Label marker template.
    #[prop(optional, into)]
    icon: Option<Callback<MeterItem, AnyView>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<MeterGroupPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let attrs = pt.root.attrs();
    let ctx = {
        let (value, min, max) = (value.clone(), min.clone(), max.clone());
        move || {
            let items = value.get();
            let total = total_percent(items.iter().map(|i| i.value), min.get(), max.get());
            let percentages = cumulative(items.iter().map(|i| i.value));
            MeterGroupContext { items, total_percent: total, percentages }
        }
    };
    let class = {
        let (pt, orientation) = (pt.clone(), orientation.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, orientation.get());
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        move || style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
    };
    let valuenow = {
        let ctx = ctx.clone();
        move || ctx().total_percent.to_string()
    };
    let (vmin, vmax) = (min.clone(), max.clone());
    let data = {
        let orientation = orientation.clone();
        move || orientation.get().as_str()
    };

    let labels = {
        let (pt, ctx, min, max, label_orientation) = (pt.clone(), ctx.clone(), min.clone(), max.clone(), label_orientation);
        move |position: LabelPosition| {
            let pos = label_position.get();
            let ctx = ctx();
            if pos != position {
                return ().into_any();
            }
            match label {
                Some(t) => t.run(ctx),
                None => label_list(&pt, &ctx.items, min.get(), max.get(), label_orientation.get(), icon),
            }
        }
    };
    let (labels_start, labels_end) = (labels.clone(), labels);
    let meters = {
        let (pt, ctx, orientation) = (pt.clone(), ctx.clone(), orientation.clone());
        move || {
            let (min, max, orientation) = (min.get(), max.get(), orientation.get());
            let items = ctx().items;
            items
                .into_iter()
                .enumerate()
                .filter(|(_, item)| item.value > 0.0)
                .map(|(i, item)| match meter {
                    Some(t) => t.run((item, i)),
                    None => {
                        let p = percent(item.value, min, max);
                        let dim = match orientation {
                            MeterOrientation::Horizontal => "width",
                            MeterOrientation::Vertical => "height",
                        };
                        let style = pt.meter.merge_style(&format!("background-color: {}; {dim}: {p}%", item.color));
                        let class = pt.meter.merge_class("p-metergroup-meter");
                        view! { <span class=class style=style data-p=orientation.as_str() {..pt.meter.attrs()}></span> }.into_any()
                    }
                })
                .collect_view()
        }
    };
    let (start_ctx, end_ctx) = (ctx.clone(), ctx);
    let pt_meters = pt.clone();
    let meters_class = pt.meters.merge_class("p-metergroup-meters");
    let meters_style = pt.meters.has_style().then(|| pt.meters.merge_style(""));
    let meters_attrs = pt.meters.attrs();
    let data_meters = data.clone();
    let _ = pt_meters;
    view! {
        <div
            class=class
            style=style
            role="meter"
            aria-valuemin=move || vmin.get().to_string()
            aria-valuemax=move || vmax.get().to_string()
            aria-valuenow=valuenow
            data-p={data}
            {..attrs}
        >
            {move || labels_start(LabelPosition::Start)}
            {move || start.map(|t| t.run(start_ctx()))}
            <div class=meters_class style=meters_style data-p={data_meters} {..meters_attrs}>
                {meters}
            </div>
            {move || end.map(|t| t.run(end_ctx()))}
            {move || labels_end(LabelPosition::End)}
        </div>
    }
}

/// Default label list.
fn label_list(
    pt: &MeterGroupPt,
    items: &[MeterItem],
    min: f64,
    max: f64,
    orientation: MeterOrientation,
    icon: Option<Callback<MeterItem, AnyView>>,
) -> AnyView {
    let mut cn = ClassNames::new();
    label_list_classes(&mut cn, orientation);
    cn.add_user(pt.label_list.extra_class());
    let class = cn.finish();
    let style = pt.label_list.has_style().then(|| pt.label_list.merge_style(""));
    let rows = items
        .iter()
        .map(|item| {
            let marker = match (&icon, &item.icon) {
                (Some(t), _) => t.run(item.clone()),
                (None, Some(i)) => {
                    let style = pt.label_icon.merge_style(&format!("color: {}", item.color));
                    view! { <span class=pt.label_icon.merge_class("p-metergroup-label-icon") style={style} {..pt.label_icon.attrs()}>{i.run()}</span> }
                        .into_any()
                }
                (None, None) => {
                    let style = pt.label_marker.merge_style(&format!("background-color: {}", item.color));
                    view! { <span class=pt.label_marker.merge_class("p-metergroup-label-marker") style={style} {..pt.label_marker.attrs()}></span> }
                        .into_any()
                }
            };
            let text = format!("{} ({}%)", item.label, percent(item.value, min, max));
            view! {
                <li class=pt.label.merge_class("p-metergroup-label") style=pt.label.has_style().then(|| pt.label.merge_style("")) {..pt.label.attrs()}>
                    {marker}
                    <span class=pt.label_text.merge_class("p-metergroup-label-text") style=pt.label_text.has_style().then(|| pt.label_text.merge_style("")) {..pt.label_text.attrs()}>{text}</span>
                </li>
            }
        })
        .collect_view();
    view! { <ol class=class style=style data-p=orientation.as_str() {..pt.label_list.attrs()}>{rows}</ol> }.into_any()
}
