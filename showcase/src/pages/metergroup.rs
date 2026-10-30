//! Страница MeterGroup: демонстрации `basic`, `multiple`, `minmax`, `vertical`, `label`, `icon` по `apps/showcase/doc/metergroup` PrimeNG.
//!
//! В `prime-icons` нет иконок `table`, `inbox`, `image`, `cog`, поэтому во `icon` взяты `th-large`, `info-circle`, `eye`, `refresh`.

use super::Demo;
use leptos::prelude::*;
use prime_icons::{EyeIcon, InfoCircleIcon, RefreshIcon, ThLargeIcon};
use prime_metergroup::{LabelPosition, MeterGroup, MeterItem, MeterOrientation};

fn multiple() -> Vec<MeterItem> {
    vec![
        MeterItem::new("Apps", "#34d399", 16.0),
        MeterItem::new("Messages", "#fbbf24", 8.0),
        MeterItem::new("Media", "#60a5fa", 24.0),
        MeterItem::new("System", "#c084fc", 10.0),
    ]
}

fn with_icons() -> Vec<MeterItem> {
    vec![
        MeterItem::new("Apps", "#34d399", 16.0).icon(ViewFn::from(|| view! { <ThLargeIcon /> })),
        MeterItem::new("Messages", "#fbbf24", 8.0).icon(ViewFn::from(|| view! { <InfoCircleIcon /> })),
        MeterItem::new("Media", "#60a5fa", 24.0).icon(ViewFn::from(|| view! { <EyeIcon /> })),
        MeterItem::new("System", "#c084fc", 10.0).icon(ViewFn::from(|| view! { <RefreshIcon /> })),
    ]
}

/// Страница `/metergroup`.
#[component]
pub fn MeterGroupPage() -> impl IntoView {
    view! {
        <h2>"MeterGroup"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <MeterGroup value={vec![MeterItem::new("Space used", "var(--p-primary-color)", 15.0)]} />
            </div>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <div style="width: 100%">
                <MeterGroup value={multiple()} />
            </div>
        </Demo>
        <Demo name="minmax" title="MinMax">
            <div style="width: 100%">
                <MeterGroup value={multiple()} max=200.0 />
            </div>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <MeterGroup
                value={multiple()}
                orientation=MeterOrientation::Vertical
                label_orientation=MeterOrientation::Vertical
                style="height: 300px"
            />
        </Demo>
        <Demo name="label" title="Label">
            <div style="width: 100%">
                <MeterGroup value={multiple()} label_position=LabelPosition::Start label_orientation=MeterOrientation::Vertical />
            </div>
        </Demo>
        <Demo name="icon" title="Icon">
            <div style="width: 100%">
                <MeterGroup value={with_icons()} />
            </div>
        </Demo>
    }
}
