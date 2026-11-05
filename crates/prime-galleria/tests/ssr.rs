//! Разметка Galleria (SSR): кадр, миниатюры, индикаторы, кнопки, подпись, позиции, полноэкранный режим, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_galleria::{Galleria, GalleriaImage, GalleriaPosition, GalleriaPt, GalleriaResponsive};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn images(n: usize) -> Vec<GalleriaImage> {
    (0..n)
        .map(|i| {
            GalleriaImage::new(format!("big{i}.jpg")).thumbnail(format!("small{i}.jpg")).alt(format!("Alt {i}")).title(format!("T{i}"))
        })
        .collect()
}

#[test]
fn basic_markup() {
    let html = render(|| view! { <Galleria value=images(3) /> }.to_html());
    assert!(html.contains("p-galleria p-component p-galleria-thumbnails-bottom") && html.contains("role=\"region\""), "{html}");
    assert!(
        html.contains("p-galleria-content") && html.contains("p-galleria-items-container") && html.contains("p-galleria-items"),
        "{html}"
    );
    assert!(html.contains("p-galleria-item") && html.contains("src=\"big0.jpg\"") && !html.contains("big1.jpg"), "{html}");
    assert_eq!(
        html.matches("p-galleria-thumbnail-item ").count() + html.matches("class=\"p-galleria-thumbnail-item\"").count(),
        3,
        "{html}"
    );
    assert!(html.contains("src=\"small2.jpg\""), "{html}");
    assert!(!html.contains("p-galleria-indicator-list") && !html.contains("p-galleria-prev-button"), "{html}");
    assert!(html.contains("p-galleria-thumbnail-prev-button") && html.contains("p-galleria-thumbnail-next-button"), "{html}");
}

#[test]
fn empty_value_renders_nothing_inside() {
    let html = render(|| view! { <Galleria value={Vec::<GalleriaImage>::new()} /> }.to_html());
    assert!(html.contains("p-galleria") && !html.contains("p-galleria-content"), "{html}");
}

#[test]
fn indicators_and_item_navigators() {
    let html = render(|| view! { <Galleria value=images(3) show_indicators=true show_item_navigators=true /> }.to_html());
    assert_eq!(html.matches("p-galleria-indicator ").count() + html.matches("class=\"p-galleria-indicator\"").count(), 3, "{html}");
    assert!(html.contains("p-galleria-indicator p-galleria-indicator-active"), "первый активен: {html}");
    assert!(html.contains("p-galleria-prev-button p-galleria-nav-button p-disabled"), "в начале «назад» недоступна: {html}");
    assert!(html.contains("class=\"p-galleria-next-button p-galleria-nav-button\""), "{html}");
    assert!(html.contains("p-galleria-indicators-bottom"), "{html}");
}

#[test]
fn circular_enables_backward_button() {
    let html = render(|| view! { <Galleria value=images(3) show_item_navigators=true circular=true /> }.to_html());
    assert!(!html.contains("p-galleria-prev-button p-galleria-nav-button p-disabled"), "{html}");
}

#[test]
fn active_index_selects_the_item_and_window() {
    let html = render(|| {
        let active = RwSignal::new(4);
        view! { <Galleria value=images(6) active_index=active /> }.to_html()
    });
    assert!(html.contains("src=\"big4.jpg\"") && !html.contains("src=\"big0.jpg\""), "{html}");
    assert!(html.contains("p-galleria-thumbnail-item p-galleria-thumbnail-item-current p-galleria-thumbnail-item-active"), "{html}");
    assert!(html.contains("translate3d(-100%, 0, 0)"), "активный 4 из 6 при трёх видимых: смещение −3 → {html}");
}

#[test]
fn caption_templates_and_header_footer() {
    let html = render(|| {
        let caption = Callback::new(|i: GalleriaImage| view! { <span>{i.title}</span> }.into_any());
        let indicator = Callback::new(|i: usize| view! { <b>{format!("#{i}")}</b> }.into_any());
        view! {
            <Galleria
                value=images(2)
                caption=caption
                indicator=indicator
                show_indicators=true
                header=ViewFn::from(|| view! { <h3>"Head"</h3> })
                footer=ViewFn::from(|| view! { <em>"Foot"</em> })
            />
        }
        .to_html()
    });
    assert!(html.contains("p-galleria-caption") && html.contains("<span>T0</span>"), "{html}");
    assert!(html.contains("<b>#1</b>") && !html.contains("p-galleria-indicator-button"), "{html}");
    assert!(
        html.contains("p-galleria-header") && html.contains("Head") && html.contains("p-galleria-footer") && html.contains("Foot"),
        "{html}"
    );
}

#[test]
fn positions_and_flags() {
    let html = render(|| {
        view! {
            <Galleria
                value=images(3)
                thumbnails_position=GalleriaPosition::Left
                show_indicators=true
                indicators_position=GalleriaPosition::Top
                show_indicators_on_item=true
                show_item_navigators_on_hover=true
            />
        }
        .to_html()
    });
    assert!(
        html.contains("p-galleria p-component p-galleria-inset-indicators p-galleria-hover-navigators p-galleria-thumbnails-left p-galleria-indicators-top"),
        "{html}"
    );
    assert!(html.contains("height: 300px"), "вертикальное окно миниатюр: {html}");
    assert!(html.contains("p-galleria-thumbnail-prev-button") && html.contains("<svg"), "{html}");
}

#[test]
fn thumbnails_can_be_hidden() {
    let html = render(|| view! { <Galleria value=images(3) show_thumbnails=false /> }.to_html());
    assert!(!html.contains("p-galleria-thumbnails") && !html.contains("p-galleria-thumbnails-bottom"), "{html}");
    let html = render(|| view! { <Galleria value=images(3) show_thumbnail_navigators=false /> }.to_html());
    assert!(html.contains("p-galleria-thumbnails-content") && !html.contains("p-galleria-thumbnail-prev-button"), "{html}");
}

#[test]
fn num_visible_sets_thumbnail_width_and_is_limited_by_count() {
    let html = render(|| view! { <Galleria value=images(6) num_visible=4 /> }.to_html());
    assert!(html.contains("flex: 1 0 25%"), "{html}");
    let html = render(|| view! { <Galleria value=images(2) num_visible=5 /> }.to_html());
    assert!(html.contains("flex: 1 0 50%"), "число видимых не больше числа изображений: {html}");
    let responsive = vec![GalleriaResponsive { breakpoint: 960, num_visible: 4 }];
    let html = render(|| view! { <Galleria value=images(6) responsive_options=responsive /> }.to_html());
    assert!(html.contains("flex: 1 0 33.33"), "на сервере ширины окна нет — базовое значение: {html}");
}

#[test]
fn full_screen_hidden_renders_nothing() {
    let html = render(|| view! { <Galleria value=images(3) full_screen=true /> }.to_html());
    assert!(!html.contains("p-galleria"), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = GalleriaPt { root: Pt::new().class("root-x"), item: Pt::new().class("item-x"), ..Default::default() };
    let html = render(|| view! { <Galleria value=images(2) id="g" style="max-width: 640px" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("id=\"g\"") && html.contains("max-width: 640px"), "{html}");
    assert!(html.contains("p-galleria-item item-x"), "{html}");
    assert!(html.contains("aria-label=\"1\"") && html.contains("aria-roledescription=\"Slide\""), "{html}");
}
