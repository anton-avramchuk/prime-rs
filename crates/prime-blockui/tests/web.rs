//! BlockUI behaviour in a real browser (wasm-bindgen-test): the mask in a target and in the document, the layer, page scrolling,
//! target position, exit, re-enabling and unmounting.
//!
//! The scenarios run as one sequential test: the `z-index` layer stack and the scroll-lock counter are shared by the page,
//! so parallel tests would affect each other.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_blockui::BlockUI;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn find(root: &HtmlElement, sel: &str) -> Option<HtmlElement> {
    root.query_selector(sel).unwrap().and_then(|e| e.dyn_into().ok())
}

fn body() -> HtmlElement {
    doc().body().unwrap()
}

fn body_locked() -> bool {
    body().class_list().contains("p-overflow-hidden")
}

/// Gives effects and `Portal` time to insert the mask and apply the styles (several microtasks).
async fn settle() {
    sleep(30).await;
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn mask_appears_inside_the_target_and_leaves_with_an_animation() {
    let blocked = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let target = NodeRef::<leptos::html::Div>::new();
        view! {
            <div node_ref=target class="target" style="position: static">"Panel"</div>
            <BlockUI blocked=blocked target=target><span class="spinner">"Loading"</span></BlockUI>
        }
    });
    leptos::task::tick().await;
    assert!(find(&host, ".p-blockui").is_none(), "there is no mask without blocking");
    blocked.set(true);
    settle().await;
    let target = find(&host, ".target").unwrap();
    let mask = find(&target, ".p-blockui").expect("mask inside the target");
    let class = mask.class_name();
    assert!(
        class.contains("p-blockui-mask") && class.contains("p-overlay-mask") && class.contains("p-overlay-mask-enter-active"),
        "{class}"
    );
    assert!(!class.contains("p-blockui-mask-document"));
    assert_eq!(mask.get_attribute("aria-busy").as_deref(), Some("true"));
    assert_eq!(mask.style().get_property_value("display").unwrap(), "flex");
    assert_eq!(target.style().get_property_value("position").unwrap(), "relative", "the target got position: relative");
    assert!(find(&mask, ".spinner").is_some(), "content inside the mask");
    assert!(!body_locked(), "the page scroll is not blocked with a target");
    let z: i32 = mask.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1100, "layer of the Modal group: {z}");
    blocked.set(false);
    leptos::task::tick().await;
    let leaving = find(&target, ".p-blockui").expect("the mask is still in the DOM during the exit");
    assert!(leaving.class_name().contains("p-overlay-mask-leave-active"));
    assert_eq!(leaving.get_attribute("aria-busy").as_deref(), Some("false"));
    sleep(700).await;
    assert!(find(&host, ".p-blockui").is_none(), "the mask is removed after the exit");
    assert_eq!(target.style().get_property_value("position").unwrap(), "static", "the previous position is restored");
    host.remove();
}

async fn target_without_inline_position_gets_it_removed_afterwards() {
    let blocked = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let target = NodeRef::<leptos::html::Div>::new();
        view! {
            <div node_ref=target class="target">"Panel"</div>
            <BlockUI blocked=blocked target=target />
        }
    });
    leptos::task::tick().await;
    blocked.set(true);
    settle().await;
    let target = find(&host, ".target").unwrap();
    assert_eq!(target.style().get_property_value("position").unwrap(), "relative");
    blocked.set(false);
    sleep(700).await;
    assert_eq!(target.style().get_property_value("position").unwrap(), "", "the inline property is removed");
    host.remove();
}

async fn document_mask_lives_in_body_and_locks_the_page_scroll() {
    let blocked = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <BlockUI blocked=blocked style_class="mine" /> });
    leptos::task::tick().await;
    blocked.set(true);
    settle().await;
    let mask = find(&body(), "body > div > .p-blockui").expect("mask in body");
    assert!(mask.class_name().contains("p-blockui-mask-document") && mask.class_name().contains("mine"), "{}", mask.class_name());
    assert!(find(&host, ".p-blockui").is_none(), "the mask is not inside the component host");
    assert!(body_locked(), "the page does not scroll");
    blocked.set(false);
    sleep(700).await;
    assert!(find(&body(), ".p-blockui").is_none());
    assert!(!body_locked(), "the block is lifted");
    host.remove();
}

async fn auto_z_index_can_be_turned_off() {
    let blocked = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <BlockUI blocked=blocked auto_z_index=false /> });
    settle().await;
    let mask = find(&body(), ".p-blockui").expect("mask shown immediately");
    assert_eq!(mask.style().get_property_value("z-index").unwrap(), "");
    blocked.set(false);
    sleep(700).await;
    host.remove();
}

async fn base_z_index_shifts_the_layer() {
    let blocked = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <BlockUI blocked=blocked base_z_index=500 /> });
    settle().await;
    let mask = find(&body(), ".p-blockui").unwrap();
    let z: i32 = mask.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1600, "the base_z_index shift is applied: {z}");
    blocked.set(false);
    sleep(700).await;
    host.remove();
}

async fn reblocking_during_the_leave_animation_keeps_a_single_mask() {
    let blocked = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <BlockUI blocked=blocked /> });
    settle().await;
    blocked.set(false);
    leptos::task::tick().await;
    assert!(find(&body(), ".p-overlay-mask-leave-active").is_some());
    blocked.set(true);
    leptos::task::tick().await;
    let masks = body().query_selector_all(".p-blockui").unwrap().length();
    assert_eq!(masks, 1, "one mask");
    let mask = find(&body(), ".p-blockui").unwrap();
    assert!(mask.class_name().contains("p-overlay-mask-enter-active") && mask.get_attribute("aria-busy").as_deref() == Some("true"));
    sleep(700).await;
    assert!(find(&body(), ".p-blockui").is_some(), "the mask did not disappear after the fallback timer");
    assert!(body_locked());
    blocked.set(false);
    sleep(700).await;
    assert!(!body_locked(), "the block is lifted exactly once");
    host.remove();
}

async fn unmounting_while_blocked_removes_the_mask_and_frees_the_page() {
    let blocked = RwSignal::new(true);
    let host = host();
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <BlockUI blocked=blocked /> });
    settle().await;
    assert!(find(&body(), ".p-blockui").is_some());
    assert!(body_locked());
    drop(handle);
    leptos::task::tick().await;
    assert!(find(&body(), ".p-blockui").is_none(), "the mask is removed");
    assert!(!body_locked(), "page scrolling is allowed again");
    host.remove();
}

/// All scenarios in a row: each waits for the mask to fully leave, so the layers and the scroll block are free between them.
#[wasm_bindgen_test]
async fn blockui_scenarios() {
    mask_appears_inside_the_target_and_leaves_with_an_animation().await;
    target_without_inline_position_gets_it_removed_afterwards().await;
    document_mask_lives_in_body_and_locks_the_page_scroll().await;
    auto_z_index_can_be_turned_off().await;
    base_z_index_shifts_the_layer().await;
    reblocking_during_the_leave_animation_keeps_a_single_mask().await;
    unmounting_while_blocked_removes_the_mask_and_frees_the_page().await;
}
