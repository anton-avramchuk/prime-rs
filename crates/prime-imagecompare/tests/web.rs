//! Поведение ImageCompare в реальном браузере (wasm-bindgen-test): обрезка по вводу в ползунок, `rtl`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_imagecompare::ImageCompare;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, HtmlInputElement};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

/// Монтирует сравнение двух `<img>` (`.l` и `.r`) в `host`.
macro_rules! mount {
    ($host:expr) => {
        leptos::mount::mount_to($host.clone(), || {
            view! {
                <ImageCompare
                    left=|| view! { <img class="l" /> }
                    right=|| view! { <img class="r" /> }
                />
            }
        })
    };
}

fn slide(host: &HtmlElement, value: &str) {
    let input: HtmlInputElement = q(host, ".p-imagecompare-slider").dyn_into().unwrap();
    input.set_value(value);
    input
        .dispatch_event(&{
            let init = web_sys::EventInit::new();
            init.set_bubbles(true);
            Event::new_with_event_init_dict("input", &init).unwrap()
        })
        .unwrap();
}

fn clip(host: &HtmlElement) -> String {
    q(host, ".r").style().get_property_value("clip-path").unwrap()
}

#[wasm_bindgen_test]
async fn input_clips_the_element_before_the_slider() {
    let host = host();
    let _h = mount!(host);
    leptos::task::tick().await;
    assert_eq!(clip(&host), "", "до ввода инлайн-обрезки нет");
    slide(&host, "30");
    assert_eq!(clip(&host), "polygon(0px 0px, 30% 0px, 30% 100%, 0px 100%)");
    assert_eq!(q(&host, ".l").style().get_property_value("clip-path").unwrap(), "", "левое изображение не обрезается");
    slide(&host, "80");
    assert_eq!(clip(&host), "polygon(0px 0px, 80% 0px, 80% 100%, 0px 100%)");
    host.remove();
}

#[wasm_bindgen_test]
async fn rtl_ancestor_mirrors_the_clip() {
    let host = host();
    host.set_attribute("dir", "rtl").unwrap();
    let _h = mount!(host);
    leptos::task::tick().await;
    slide(&host, "30");
    assert_eq!(clip(&host), "polygon(70% 0px, 100% 0px, 100% 100%, 70% 100%)");
    host.remove();
}

#[wasm_bindgen_test]
async fn input_without_a_previous_sibling_does_not_fail() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <ImageCompare left=|| () right=|| () /> });
    leptos::task::tick().await;
    slide(&host, "40");
    assert!(q(&host, ".p-imagecompare-slider").previous_element_sibling().is_none());
    host.remove();
}
