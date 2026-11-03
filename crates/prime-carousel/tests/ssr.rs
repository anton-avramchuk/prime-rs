//! Разметка Carousel (SSR): корень, навигаторы, элементы, клоны, индикаторы, ориентация, шапка и подвал, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_carousel::{Carousel, CarouselOrientation, CarouselPt, Responsive};
use prime_core::Pt;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn show(n: i32) -> AnyView {
    view! { <b class="n">{n}</b> }.into_any()
}

fn five() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[test]
fn markup_has_all_parts() {
    let html = render(|| view! { <Carousel id="c" value=five() num_visible=2 item=Callback::new(show) /> }.to_html());
    assert!(html.contains("id=\"c\"") && html.contains("role=\"region\""), "{html}");
    assert!(html.contains("p-carousel p-component p-carousel-horizontal"), "{html}");
    for part in
        ["p-carousel-content-container", "p-carousel-content", "p-carousel-viewport", "p-carousel-item-list", "p-carousel-indicator-list"]
    {
        assert!(html.contains(part), "нет {part}: {html}");
    }
    assert!(html.contains("p-carousel-prev-button") && html.contains("p-carousel-next-button"), "{html}");
    assert!(html.contains("aria-label=\"Previous Page\"") && html.contains("aria-label=\"Next Page\""), "{html}");
    assert_eq!(html.matches("class=\"n\"").count(), 5, "ровно пять элементов, клонов нет: {html}");
    assert_eq!(html.matches("p-carousel-indicator-button").count(), 4, "четыре страницы: {html}");
    assert!(html.contains("aria-live=\"off\""), "{html}");
}

#[test]
fn items_carry_roles_labels_and_ranges() {
    let html = render(|| view! { <Carousel value=five() num_visible=2 item=Callback::new(show) /> }.to_html());
    assert_eq!(html.matches("role=\"group\"").count(), 5, "{html}");
    assert!(
        html.contains("aria-roledescription=\"Slide\"") && html.contains("aria-label=\"1\"") && html.contains("aria-label=\"5\""),
        "{html}"
    );
    assert_eq!(html.matches("class=\"p-carousel-item p-carousel-item-active").count(), 2, "активны первые два: {html}");
    assert!(html.contains("p-carousel-item p-carousel-item-active p-carousel-item-start"), "{html}");
    assert!(html.contains("flex: 1 0 50%"), "ширина элемента по числу видимых: {html}");
    assert!(html.contains("data-p-carousel-item-start=\"true\"") && html.contains("data-p-carousel-item-end=\"true\""), "{html}");
    assert_eq!(html.matches("role=\"group\" aria-hidden=\"false\"").count(), 2, "{html}");
    assert_eq!(html.matches("role=\"group\" aria-hidden=\"true\"").count(), 3, "{html}");
}

#[test]
fn indicators_mark_the_active_page() {
    let html = render(|| view! { <Carousel value=five() num_visible=2 item=Callback::new(show) /> }.to_html());
    assert!(html.contains("p-carousel-indicator p-carousel-indicator-active"), "{html}");
    assert_eq!(html.matches("p-carousel-indicator-active").count(), 1, "{html}");
    assert!(html.contains("aria-current=\"page\"") && html.contains("aria-label=\"1\""), "{html}");
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    assert_eq!(html.matches("tabindex=\"-1\"").count(), 3, "{html}");
}

#[test]
fn navigators_and_indicators_can_be_hidden() {
    let html = render(|| {
        view! { <Carousel value=five() num_visible=2 show_navigators=false show_indicators=false item=Callback::new(show) /> }.to_html()
    });
    assert!(
        !html.contains("p-carousel-prev-button") && !html.contains("p-carousel-next-button") && !html.contains("p-carousel-indicator-list"),
        "{html}"
    );
}

#[test]
fn edge_buttons_are_disabled_without_circular() {
    let html = render(|| view! { <Carousel value=five() num_visible=2 item=Callback::new(show) /> }.to_html());
    assert!(html.contains("p-carousel-prev-button p-disabled") && !html.contains("p-carousel-next-button p-disabled"), "{html}");
    let one_page = render(|| view! { <Carousel value=vec![1, 2] num_visible=2 item=Callback::new(show) /> }.to_html());
    assert!(one_page.contains("p-carousel-prev-button p-disabled") && one_page.contains("p-carousel-next-button p-disabled"), "{one_page}");
    let none: Vec<i32> = Vec::new();
    let empty = render(|| view! { <Carousel value=none item=Callback::new(show) /> }.to_html());
    assert!(empty.contains("p-carousel-prev-button p-disabled") && !empty.contains("p-carousel-indicator-button"), "{empty}");
}

#[test]
fn circular_renders_clones_on_both_sides() {
    let html = render(|| view! { <Carousel value=five() num_visible=2 circular=true item=Callback::new(show) /> }.to_html());
    assert_eq!(html.matches("p-carousel-item-clone").count(), 4, "по два клона: {html}");
    assert_eq!(html.matches("class=\"n\"").count(), 9, "5 + 2 + 2: {html}");
    assert!(html.contains("translate3d(-100%, 0, 0)"), "список стоит за клонами начала: {html}");
    assert!(!html.contains("p-disabled"), "в бесконечном режиме кнопки не отключаются: {html}");
    let short = render(|| view! { <Carousel value=vec![1] num_visible=2 circular=true item=Callback::new(show) /> }.to_html());
    assert!(!short.contains("p-carousel-item-clone"), "элементов меньше видимых: бесконечный режим не включается: {short}");
}

#[test]
fn page_and_scroll_set_the_initial_shift() {
    let html = render(|| {
        let page = RwSignal::new(2usize);
        view! { <Carousel value=five() num_visible=2 page=page item=Callback::new(show) /> }.to_html()
    });
    assert!(html.contains("translate3d(-100%, 0, 0)"), "две страницы по шагу 1 = −2 элемента по 50%: {html}");
    assert!(html.contains("p-carousel-indicator-active"), "{html}");
}

#[test]
fn vertical_orientation_and_viewport_height() {
    let html = render(|| {
        view! { <Carousel value=five() orientation=CarouselOrientation::Vertical vertical_view_port_height="330px" item=Callback::new(show) /> }.to_html()
    });
    assert!(html.contains("p-carousel-vertical") && !html.contains("p-carousel-horizontal"), "{html}");
    assert!(
        html.contains("height: 330px") && html.contains("translate3d(0, 0%, 0)")
            || html.contains("translate3d(0, 0, 0)")
            || html.contains("translate3d(0, 0%"),
        "{html}"
    );
    let horizontal = render(|| view! { <Carousel value=five() item=Callback::new(show) /> }.to_html());
    assert!(horizontal.contains("height: auto"), "{horizontal}");
}

#[test]
fn header_footer_icons_and_classes() {
    let html = render(|| {
        view! {
            <Carousel
                value=five()
                header=ViewFn::from(|| view! { <i class="hd">"H"</i> })
                footer=ViewFn::from(|| view! { <i class="ft">"F"</i> })
                previous_icon=ViewFn::from(|| view! { <i class="pv">"<"</i> })
                next_icon=ViewFn::from(|| view! { <i class="nx">">"</i> })
                content_class="cc"
                indicators_content_class="ic"
                indicators_content_style="gap: 1px"
                indicator_style_class="isc"
                indicator_style="width: 2px"
                style_class="mine"
                style="width: 10rem"
                item=Callback::new(show)
            />
        }
        .to_html()
    });
    for needle in [
        "p-carousel-header",
        "hd",
        "p-carousel-footer",
        "ft",
        "pv",
        "nx",
        "p-carousel-content-container cc",
        "p-carousel-indicator-list ic",
        "gap: 1px",
        "p-carousel-indicator-button isc",
        "width: 2px",
        "mine",
        "width: 10rem",
    ] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
}

#[test]
fn autoplay_makes_the_content_live() {
    let html = render(|| view! { <Carousel value=five() autoplay_interval=3000 item=Callback::new(show) /> }.to_html());
    assert!(html.contains("aria-live=\"polite\""), "{html}");
}

#[test]
fn pt_adds_classes_and_attributes() {
    let pt = CarouselPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        viewport: Pt::new().class("v"),
        item_list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        indicator: Pt::new().class("d"),
        indicator_button: Pt::new().class("b"),
        ..CarouselPt::default()
    };
    let html = render(|| view! { <Carousel value=five() pt=pt item=Callback::new(show) /> }.to_html());
    assert!(html.contains("data-x=\"1\"") && html.contains("p-carousel-viewport v") && html.contains("p-carousel-item-list l"), "{html}");
    assert!(
        html.contains("p-carousel-item p-carousel-item-active p-carousel-item-start i") || html.contains("p-carousel-item i"),
        "{html}"
    );
    assert!(
        html.contains("p-carousel-indicator p-carousel-indicator-active d") && html.contains("p-carousel-indicator-button b"),
        "{html}"
    );
}

#[test]
fn responsive_options_do_not_change_the_server_markup() {
    let opts = vec![Responsive { breakpoint: "767px".into(), num_visible: 1, num_scroll: 1 }];
    let html = render(|| view! { <Carousel value=five() num_visible=3 responsive_options=opts item=Callback::new(show) /> }.to_html());
    assert!(html.contains("flex: 1 0 33.333333333333336%"), "на сервере действуют значения по умолчанию: {html}");
}
