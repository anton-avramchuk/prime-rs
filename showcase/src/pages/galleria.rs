//! Страница Galleria: демонстрации `basic`, `autoplay`, `caption`, `controlled`, `thumbnail`, `indicator` и `fullscreen`
//! из showcase PrimeNG (`apps/showcase/doc/galleria`; формы Angular не переносятся).
//! Картинки — `data:`-ссылки с градиентом, чтобы страница не зависела от сети.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_galleria::{Galleria, GalleriaImage, GalleriaPosition, GalleriaResponsive};

const COLORS: [(&str, &str); 8] = [
    ("0ea5e9", "1e3a8a"),
    ("f59e0b", "9a3412"),
    ("10b981", "064e3b"),
    ("ec4899", "831843"),
    ("8b5cf6", "4c1d95"),
    ("ef4444", "7f1d1d"),
    ("14b8a6", "134e4a"),
    ("84cc16", "365314"),
];

/// SVG-картинка с градиентом и номером как `data:`-ссылка.
fn picture(index: usize, width: u32, height: u32) -> String {
    let (from, to) = COLORS[index % COLORS.len()];
    let (cx, cy, font) = (width / 2, height / 2, height / 4);
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'%3E\
         %3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0' stop-color='%23{from}'/%3E\
         %3Cstop offset='1' stop-color='%23{to}'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='{width}' height='{height}' fill='url(%23g)'/%3E\
         %3Ctext x='{cx}' y='{cy}' font-size='{font}' font-family='sans-serif' fill='white' text-anchor='middle' dominant-baseline='middle'%3E{}%3C/text%3E%3C/svg%3E",
        index + 1
    )
}

fn images(n: usize) -> Vec<GalleriaImage> {
    (0..n)
        .map(|i| {
            GalleriaImage::new(picture(i, 640, 360))
                .thumbnail(picture(i, 100, 60))
                .alt(format!("Description for Image {}", i + 1))
                .title(format!("Title {}", i + 1))
        })
        .collect()
}

const BOX: &str = "width: 100%; max-width: 640px";

#[component]
fn Basic() -> impl IntoView {
    let responsive = vec![
        GalleriaResponsive { breakpoint: 1024, num_visible: 5 },
        GalleriaResponsive { breakpoint: 768, num_visible: 3 },
        GalleriaResponsive { breakpoint: 560, num_visible: 1 },
    ];
    view! { <Galleria id="basic_galleria" value=images(8) num_visible=5 responsive_options=responsive style=BOX /> }
}

#[component]
fn AutoPlay() -> impl IntoView {
    view! { <Galleria id="autoplay_galleria" value=images(5) num_visible=5 circular=true auto_play=true transition_interval=2000 style=BOX /> }
}

#[component]
fn Caption() -> impl IntoView {
    let caption = Callback::new(|image: GalleriaImage| {
        view! {
            <div style="font-weight: bold">{image.title.clone()}</div>
            <p style="margin: 0">{image.alt.clone()}</p>
        }
        .into_any()
    });
    view! {
        <Galleria
            id="caption_galleria"
            value=images(5)
            num_visible=5
            circular=true
            show_item_navigators=true
            show_thumbnails=false
            show_indicators=true
            show_indicators_on_item=true
            caption=caption
            style=BOX
        />
    }
}

#[component]
fn Controlled() -> impl IntoView {
    let active = RwSignal::new(2usize);
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem; align-items: center">
            <Button label="Prev" on_click=Callback::new(move |_| active.update(|i| *i = i.saturating_sub(1))) />
            <Galleria id="controlled_galleria" value=images(5) active_index=active num_visible=5 style=BOX />
            <span data-testid="controlled-index">{move || active.get()}</span>
        </div>
    }
}

#[component]
fn Thumbnail() -> impl IntoView {
    let position = RwSignal::new(GalleriaPosition::Bottom);
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem">
            <div style="display: flex; gap: 0.5rem">
                <Button label="Bottom" on_click=Callback::new(move |_| position.set(GalleriaPosition::Bottom)) />
                <Button label="Top" on_click=Callback::new(move |_| position.set(GalleriaPosition::Top)) />
                <Button label="Left" on_click=Callback::new(move |_| position.set(GalleriaPosition::Left)) />
                <Button label="Right" on_click=Callback::new(move |_| position.set(GalleriaPosition::Right)) />
            </div>
            <Galleria
                id="thumbnail_galleria"
                value=images(6)
                num_visible=4
                thumbnails_position=position
                vertical_thumbnail_view_port_height="250px"
                style="width: 100%; max-width: 800px"
            />
        </div>
    }
}

#[component]
fn Indicator() -> impl IntoView {
    view! {
        <Galleria
            id="indicator_galleria"
            value=images(5)
            show_thumbnails=false
            show_indicators=true
            change_item_on_indicator_hover=true
            style=BOX
        />
    }
}

#[component]
fn FullScreen() -> impl IntoView {
    let visible = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let value = images(6);
    let thumbs = value.clone();
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 0.5rem">
            {thumbs
                .into_iter()
                .enumerate()
                .map(|(i, image)| {
                    view! {
                        <img
                            class="fs-thumb"
                            src={image.thumbnail_image_src.clone().unwrap_or_default()}
                            alt={image.alt.clone()}
                            style="cursor: pointer; display: block"
                            on:click=move |_| {
                                active.set(i);
                                visible.set(true);
                            }
                        />
                    }
                })
                .collect_view()}
        </div>
        <Galleria
            id="fullscreen_galleria"
            value=value
            active_index=active
            visible=visible
            full_screen=true
            circular=true
            num_visible=5
            show_item_navigators=true
            container_style="width: 100%; max-width: 850px"
        />
    }
}

/// Страница `/galleria`.
#[component]
pub fn GalleriaPage() -> impl IntoView {
    view! {
        <h2>"Galleria"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="autoplay" title="AutoPlay">
            <AutoPlay />
        </Demo>
        <Demo name="caption" title="Caption">
            <Caption />
        </Demo>
        <Demo name="controlled" title="Controlled">
            <Controlled />
        </Demo>
        <Demo name="thumbnail" title="Thumbnail">
            <Thumbnail />
        </Demo>
        <Demo name="indicator" title="Indicator">
            <Indicator />
        </Demo>
        <Demo name="fullscreen" title="FullScreen">
            <FullScreen />
        </Demo>
    }
}
