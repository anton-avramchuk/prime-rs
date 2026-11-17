//! Carousel behavior in a real browser (wasm-bindgen-test): paging with buttons and dots, boundaries, external page,
//! infinite mode with a jump after the transition, remainder, autoplay, indicator keyboard, responsiveness, swipe,
//! vertical orientation and value change. Theme styles are not loaded: transitions are set inline and run for real.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_carousel::{Carousel, CarouselOrientation, Responsive};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<usize>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("no {sel}")).dyn_into().unwrap()
}

fn all(host: &HtmlElement, sel: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(sel).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    sleep(60).await;
}

/// Waits for the `transform 500ms` transition to end.
async fn after_transition() {
    sleep(750).await;
}

/// Normalizes `transform` to the form `(vertical, percent to thousandths)`: the browser appends units to zeros
/// (`0` → `0px`) and rounds percentages in the computed style.
fn canon(s: &str) -> String {
    let vertical = s.starts_with("translate3d(0, ") || s.starts_with("translate3d(0px, ");
    let percent: f64 = s.split('%').next().unwrap().rsplit(['(', ' ']).next().unwrap().parse().unwrap_or_else(|_| panic!("percent in {s}"));
    format!("{}{:.3}", if vertical { "v" } else { "h" }, percent)
}

/// Expected `transform` value in the same form.
fn translate(shift: i32, num_visible: usize, vertical: bool) -> String {
    canon(&prime_carousel::translate(shift, num_visible, vertical))
}

fn show(n: i32) -> AnyView {
    view! { <b class="n">{n}</b> }.into_any()
}

fn page_of(host: &HtmlElement) -> usize {
    all(host, ".p-carousel-indicator").iter().position(|li| li.class_name().contains("p-carousel-indicator-active")).expect("active dot")
}

fn transform(host: &HtmlElement) -> String {
    canon(&q(host, ".p-carousel-item-list").style().get_property_value("transform").unwrap())
}

fn log_page(log: &Log) -> Callback<usize> {
    let log = log.clone();
    Callback::new(move |p| log.lock().unwrap().push(p))
}

async fn buttons_dots_borders_and_events() {
    let (host, log) = (host(), Log::default());
    let on_page = log_page(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=2 item=Callback::new(show) on_page=on_page /> }
    });
    settle().await;
    let (prev, next) = (q(&host, ".p-carousel-prev-button"), q(&host, ".p-carousel-next-button"));
    assert!(prev.class_name().contains("p-disabled") && !next.class_name().contains("p-disabled"));
    assert_eq!(transform(&host), translate(0, 2, false));
    next.click();
    settle().await;
    assert_eq!(page_of(&host), 1);
    assert_eq!(transform(&host), translate(-1, 2, false));
    assert!(
        q(&host, ".p-carousel-item-list").style().get_property_value("transition").unwrap().contains("transform"),
        "transition enabled"
    );
    assert!(!prev.class_name().contains("p-disabled"));
    after_transition().await;
    assert!(q(&host, ".p-carousel-item-list").class_name().contains("p-items-hidden"), "after the transition — p-items-hidden");
    assert!(q(&host, ".p-carousel-item-list").style().get_property_value("transition").unwrap().is_empty(), "transition removed");
    // active items
    let active: Vec<String> = all(&host, ".p-carousel-item-active").iter().map(|e| e.text_content().unwrap()).collect();
    assert_eq!(active, ["2", "3"]);
    assert_eq!(all(&host, ".p-carousel-item")[0].get_attribute("aria-hidden").as_deref(), Some("true"));
    all(&host, ".p-carousel-indicator-button")[3].click();
    settle().await;
    assert_eq!(page_of(&host), 3);
    assert!(next.class_name().contains("p-disabled"), "last page");
    next.click();
    settle().await;
    assert_eq!(page_of(&host), 3, "does not page past the boundary");
    prev.click();
    settle().await;
    assert_eq!(page_of(&host), 2);
    all(&host, ".p-carousel-indicator-button")[0].click();
    settle().await;
    assert_eq!(page_of(&host), 0);
    assert_eq!(*log.lock().unwrap(), [1, 3, 2, 0]);
    host.remove();
}

async fn external_page_signal_is_two_way() {
    let host = host();
    let page = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=2 page=page item=Callback::new(show) /> }
    });
    settle().await;
    page.set(2);
    settle().await;
    assert_eq!(page_of(&host), 2);
    assert_eq!(transform(&host), translate(-2, 2, false));
    q(&host, ".p-carousel-next-button").click();
    settle().await;
    assert_eq!(page.get_untracked(), 3, "the button updates the signal");
    page.set(0);
    settle().await;
    assert_eq!(page_of(&host), 0);
    page.set(99);
    settle().await;
    assert_eq!(page.get_untracked(), 0, "a nonexistent page is discarded");
    host.remove();
}

async fn remainder_page_shows_the_last_items() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5, 6, 7, 8] num_visible=3 num_scroll=2 item=Callback::new(show) /> }
    });
    settle().await;
    let next = q(&host, ".p-carousel-next-button");
    for _ in 0..3 {
        next.click();
        settle().await;
    }
    assert_eq!(page_of(&host), 3);
    assert_eq!(transform(&host), translate(-5, 3, false), "the last step is shorter by the remainder");
    let active: Vec<String> = all(&host, ".p-carousel-item-active").iter().map(|e| e.text_content().unwrap()).collect();
    assert_eq!(active, ["6", "7", "8"]);
    q(&host, ".p-carousel-prev-button").click();
    settle().await;
    assert_eq!(transform(&host), translate(-4, 3, false));
    host.remove();
}

async fn circular_wraps_and_jumps_after_the_transition() {
    let (host, log) = (host(), Log::default());
    let on_page = log_page(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=2 circular=true item=Callback::new(show) on_page=on_page /> }
    });
    settle().await;
    assert_eq!(all(&host, ".p-carousel-item-clone").len(), 4);
    assert_eq!(transform(&host), translate(-2, 2, false), "the list sits behind the start clones");
    let (prev, next) = (q(&host, ".p-carousel-prev-button"), q(&host, ".p-carousel-next-button"));
    assert!(!prev.class_name().contains("p-disabled") && !next.class_name().contains("p-disabled"));
    all(&host, ".p-carousel-indicator-button")[3].click();
    after_transition().await;
    assert_eq!(page_of(&host), 3);
    next.click();
    settle().await;
    assert_eq!(page_of(&host), 0, "from the last — to zero");
    assert_eq!(transform(&host), translate(-7, 2, false), "transition to the end clones");
    after_transition().await;
    assert_eq!(transform(&host), translate(-2, 2, false), "after the transition — a jump to the start without animation");
    assert!(q(&host, ".p-carousel-item-list").style().get_property_value("transition").unwrap().is_empty());
    prev.click();
    settle().await;
    assert_eq!(page_of(&host), 3, "from zero backward — to the last");
    assert_eq!(transform(&host), translate(0, 2, false), "transition to the start clones");
    after_transition().await;
    assert_eq!(transform(&host), translate(-5, 2, false), "a jump to the real end");
    assert_eq!(*log.lock().unwrap(), [3, 0, 3]);
    host.remove();
}

async fn autoplay_runs_and_stops_on_manual_navigation() {
    let (host, log) = (host(), Log::default());
    let on_page = log_page(&log);
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3] autoplay_interval=120 circular=true item=Callback::new(show) on_page=on_page /> }
    });
    settle().await;
    assert_eq!(q(&host, ".p-carousel-content").get_attribute("aria-live").as_deref(), Some("polite"));
    sleep(480).await;
    let seen = log.lock().unwrap().clone();
    assert!(seen.len() >= 3 && seen[..3] == [1, 2, 0], "pages go in a circle: {seen:?}");
    q(&host, ".p-carousel-next-button").click();
    settle().await;
    assert_eq!(q(&host, ".p-carousel-content").get_attribute("aria-live").as_deref(), Some("off"));
    let count = log.lock().unwrap().len();
    sleep(400).await;
    assert_eq!(log.lock().unwrap().len(), count, "after manual paging autoplay stays stopped");
    drop(handle);
    host.remove();

    let (host, log) = (self::host(), Log::default());
    let on_page = log_page(&log);
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3] autoplay_interval=100 item=Callback::new(show) on_page=on_page /> }
    });
    sleep(250).await;
    drop(handle);
    settle().await;
    let count = log.lock().unwrap().len();
    sleep(300).await;
    assert_eq!(log.lock().unwrap().len(), count, "unmounting stops the timer");
    host.remove();
}

async fn indicator_arrow_keys_move_focus() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Carousel value=vec![1, 2, 3, 4] item=Callback::new(show) /> });
    settle().await;
    let (list, buttons) = (q(&host, ".p-carousel-indicator-list"), all(&host, ".p-carousel-indicator-button"));
    assert_eq!(buttons[0].get_attribute("tabindex").as_deref(), Some("0"));
    let press = |code: &str| {
        let init = KeyboardEventInit::new();
        init.set_code(code);
        init.set_bubbles(true);
        list.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    };
    press("ArrowRight");
    settle().await;
    assert_eq!(buttons[1].get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(buttons[0].get_attribute("tabindex").as_deref(), Some("-1"));
    assert!(doc().active_element().unwrap().is_same_node(Some(&buttons[1])), "focus on the second dot");
    press("ArrowLeft");
    press("ArrowLeft");
    settle().await;
    assert!(doc().active_element().unwrap().is_same_node(Some(&buttons[0])), "does not go left of the first");
    for _ in 0..6 {
        press("ArrowRight");
    }
    settle().await;
    assert!(doc().active_element().unwrap().is_same_node(Some(&buttons[3])), "does not go right of the last");
    press("KeyA");
    host.remove();
}

async fn responsive_options_switch_the_page_size() {
    let host = host();
    let opts = vec![Responsive { breakpoint: "100000px".into(), num_visible: 1, num_scroll: 1 }];
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=3 num_scroll=3 responsive_options=opts item=Callback::new(show) /> }
    });
    settle().await;
    assert_eq!(all(&host, ".p-carousel-indicator").len(), 5, "window narrower than the limit: one item per page");
    assert!(all(&host, ".p-carousel-item")[0].style().get_property_value("flex").unwrap().contains("100%"));
    host.remove();

    let host = self::host();
    let opts = vec![Responsive { breakpoint: "1px".into(), num_visible: 1, num_scroll: 1 }];
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=3 num_scroll=3 responsive_options=opts item=Callback::new(show) /> }
    });
    settle().await;
    assert_eq!(all(&host, ".p-carousel-indicator").len(), 2, "window wider than the limit: defaults");
    host.remove();
}

async fn swipe_pages_by_touch_events() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Carousel value=vec![1, 2, 3, 4] item=Callback::new(show) /> });
    settle().await;
    let viewport = q(&host, ".p-carousel-viewport");
    let touch = |kind: &str, x: i32| {
        let init = web_sys::TouchInit::new(1, &viewport);
        init.set_page_x(x);
        init.set_page_y(10);
        let t = web_sys::Touch::new(&init).unwrap();
        let list = js_sys::Array::of1(&t);
        let ev = web_sys::TouchEventInit::new();
        ev.set_changed_touches(&list);
        ev.set_bubbles(true);
        ev.set_cancelable(true);
        viewport.dispatch_event(&web_sys::TouchEvent::new_with_event_init_dict(kind, &ev).unwrap()).unwrap()
    };
    touch("touchstart", 200);
    touch("touchend", 140);
    settle().await;
    assert_eq!(page_of(&host), 1, "left by 60 px — forward");
    touch("touchstart", 100);
    touch("touchend", 110);
    settle().await;
    assert_eq!(page_of(&host), 1, "10 px — below the threshold");
    touch("touchstart", 100);
    touch("touchend", 180);
    settle().await;
    assert_eq!(page_of(&host), 0, "right by 80 px — backward");
    assert!(!touch("touchmove", 100), "touchmove is canceled");
    host.remove();
}

async fn vertical_orientation() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Carousel
                value=vec![1, 2, 3]
                orientation=CarouselOrientation::Vertical
                vertical_view_port_height="330px"
                item=Callback::new(show)
            />
        }
    });
    settle().await;
    assert!(q(&host, ".p-carousel").class_name().contains("p-carousel-vertical"));
    assert_eq!(q(&host, ".p-carousel-viewport").style().get_property_value("height").unwrap(), "330px");
    q(&host, ".p-carousel-next-button").click();
    settle().await;
    assert_eq!(transform(&host), translate(-1, 1, true));
    host.remove();
}

async fn changing_the_value_clamps_the_page() {
    let host = host();
    let value = RwSignal::new(vec![1, 2, 3, 4, 5, 6]);
    let log = Log::default();
    let on_page = log_page(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Carousel value=value num_visible=2 item=Callback::new(show) on_page=on_page /> }
    });
    settle().await;
    all(&host, ".p-carousel-indicator-button")[4].click();
    settle().await;
    assert_eq!(page_of(&host), 4);
    value.set(vec![1, 2, 3]);
    settle().await;
    assert_eq!(all(&host, ".p-carousel-indicator").len(), 2);
    assert_eq!(page_of(&host), 1, "the page is clamped to the last one");
    assert_eq!(log.lock().unwrap().last(), Some(&1));
    assert_eq!(all(&host, ".p-carousel-item").len(), 3);
    host.remove();
}

#[wasm_bindgen_test]
async fn carousel_behaviour_in_a_real_browser() {
    buttons_dots_borders_and_events().await;
    external_page_signal_is_two_way().await;
    remainder_page_shows_the_last_items().await;
    circular_wraps_and_jumps_after_the_transition().await;
    autoplay_runs_and_stops_on_manual_navigation().await;
    indicator_arrow_keys_move_focus().await;
    responsive_options_switch_the_page_size().await;
    swipe_pages_by_touch_events().await;
    vertical_orientation().await;
    changing_the_value_clamps_the_page().await;
}
