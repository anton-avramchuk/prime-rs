//! Поведение Tabs в реальном браузере (wasm-bindgen-test): клик, фокус, клавиши, roving `tabindex`, `lazy`, индикатор
//! и кнопки прокрутки.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_tabs::{Tab, TabList, TabPanel, TabPanels, Tabs};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, selector: &str) -> HtmlElement {
    host.query_selector(selector).unwrap().unwrap_or_else(|| panic!("нет {selector}")).dyn_into().unwrap()
}

fn has(host: &HtmlElement, selector: &str) -> bool {
    host.query_selector(selector).unwrap().is_some()
}

fn tab(host: &HtmlElement, value: &str) -> HtmlElement {
    q(host, &format!(r#"[role="tab"][id$="_tab_{value}"]"#))
}

fn panel(host: &HtmlElement, value: &str) -> HtmlElement {
    q(host, &format!(r#"[role="tabpanel"][id$="_tabpanel_{value}"]"#))
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Нажатие клавиши по `code`; `true`, если действие не отменено.
fn press(target: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_code(code);
    target.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn focused_id() -> Option<String> {
    doc().active_element().and_then(|el| el.get_attribute("id"))
}

fn four_tabs(select_on_focus: bool, value: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <Tabs value=value select_on_focus=select_on_focus>
            <TabList>
                <Tab value="0">"Header I"</Tab>
                <Tab value="1">"Header II"</Tab>
                <Tab value="2" disabled=true>"Header III"</Tab>
                <Tab value="3">"Header IV"</Tab>
            </TabList>
            <TabPanels>
                <TabPanel value="0">"c0"</TabPanel>
                <TabPanel value="1">"c1"</TabPanel>
                <TabPanel value="2">"c2"</TabPanel>
                <TabPanel value="3">"c3"</TabPanel>
            </TabPanels>
        </Tabs>
    }
}

#[wasm_bindgen_test]
async fn click_activates_and_updates_attributes() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    assert!(tab(&host, "0").class_name().contains("p-tab-active") && !panel(&host, "0").has_attribute("hidden"));
    assert!(panel(&host, "1").has_attribute("hidden"));
    tab(&host, "1").click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("1"));
    assert!(tab(&host, "1").class_name().contains("p-tab-active") && !tab(&host, "0").class_name().contains("p-tab-active"));
    assert_eq!(tab(&host, "1").get_attribute("aria-selected").as_deref(), Some("true"));
    assert_eq!(tab(&host, "0").get_attribute("aria-selected").as_deref(), Some("false"));
    assert!(!panel(&host, "1").has_attribute("hidden") && panel(&host, "0").has_attribute("hidden"));
}

#[wasm_bindgen_test]
async fn disabled_tab_does_not_activate() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    tab(&host, "2").click();
    press(&tab(&host, "2"), "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("0"));
}

#[wasm_bindgen_test]
async fn roving_tabindex_follows_value() {
    let host = host();
    let value = RwSignal::new(Some("1".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    let indexes = |host: &HtmlElement| ["0", "1", "2", "3"].map(|v| tab(host, v).get_attribute("tabindex").unwrap());
    assert_eq!(indexes(&host), ["-1", "0", "-1", "-1"]);
    value.set(Some("3".to_owned()));
    leptos::task::tick().await;
    assert_eq!(indexes(&host), ["-1", "-1", "-1", "0"]);
}

#[wasm_bindgen_test]
async fn arrows_move_focus_skipping_disabled_and_wrapping() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    tab(&host, "0").focus().unwrap();
    assert!(!press(&tab(&host, "0"), "ArrowRight"), "стрелка отменяет действие по умолчанию");
    assert!(focused_id().unwrap().ends_with("_tab_1"));
    press(&tab(&host, "1"), "ArrowRight");
    assert!(focused_id().unwrap().ends_with("_tab_3"), "отключённая вкладка пропущена");
    press(&tab(&host, "3"), "ArrowRight");
    assert!(focused_id().unwrap().ends_with("_tab_0"), "по кругу к первой");
    press(&tab(&host, "0"), "ArrowLeft");
    assert!(focused_id().unwrap().ends_with("_tab_3"), "по кругу к последней");
    press(&tab(&host, "3"), "ArrowLeft");
    assert!(focused_id().unwrap().ends_with("_tab_1"));
    assert_eq!(value.get_untracked().as_deref(), Some("0"), "фокус без select_on_focus не активирует");
}

#[wasm_bindgen_test]
async fn home_and_end_focus_first_and_last_enabled() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    tab(&host, "1").focus().unwrap();
    assert!(!press(&tab(&host, "1"), "End"));
    assert!(focused_id().unwrap().ends_with("_tab_3"));
    press(&tab(&host, "3"), "Home");
    assert!(focused_id().unwrap().ends_with("_tab_0"));
}

#[wasm_bindgen_test]
async fn enter_space_and_numpad_enter_activate() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    for (code, target) in [("Enter", "1"), ("Space", "3"), ("NumpadEnter", "0")] {
        assert!(!press(&tab(&host, target), code), "{code}: действие по умолчанию отменено");
        leptos::task::tick().await;
        assert_eq!(value.get_untracked().as_deref(), Some(target), "{code}");
    }
}

#[wasm_bindgen_test]
async fn other_keys_pass_through_but_do_not_bubble() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(false, value));
    leptos::task::tick().await;
    assert!(press(&tab(&host, "0"), "KeyA"), "необработанная клавиша не отменяется");
    assert!(press(&tab(&host, "0"), "ArrowDown"));
}

#[wasm_bindgen_test]
async fn select_on_focus_activates() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || four_tabs(true, value));
    leptos::task::tick().await;
    tab(&host, "1").focus().unwrap();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("1"));
    press(&tab(&host, "1"), "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("3"), "перенос фокуса активирует следующую доступную");
}

#[wasm_bindgen_test]
async fn lazy_content_appears_on_first_activation_and_stays() {
    let host = host();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Tabs value=value lazy=true>
                <TabList>
                    <Tab value="0">"A"</Tab>
                    <Tab value="1">"B"</Tab>
                </TabList>
                <TabPanels>
                    <TabPanel value="0"><span class="c0">"zero"</span></TabPanel>
                    <TabPanel value="1"><span class="c1">"one"</span></TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    leptos::task::tick().await;
    assert!(has(&host, ".c0") && !has(&host, ".c1"));
    tab(&host, "1").click();
    leptos::task::tick().await;
    assert!(has(&host, ".c1"));
    tab(&host, "0").click();
    leptos::task::tick().await;
    assert!(has(&host, ".c1") && panel(&host, "1").has_attribute("hidden"), "после деактивации содержимое остаётся");
}

#[wasm_bindgen_test]
async fn ink_bar_follows_active_tab() {
    let host = host();
    host.style().set_property("width", "600px").unwrap();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Tabs value=value>
                <TabList>
                    <Tab value="0"><span style="display: inline-block; width: 80px">"A"</span></Tab>
                    <Tab value="1"><span style="display: inline-block; width: 120px">"B"</span></Tab>
                </TabList>
            </Tabs>
        }
    });
    leptos::task::tick().await;
    sleep(80).await;
    let bar = q(&host, ".p-tablist-active-bar");
    let style = |bar: &HtmlElement| (bar.style().get_property_value("width").unwrap(), bar.style().get_property_value("left").unwrap());
    let (width0, left0) = style(&bar);
    assert_eq!(width0, format!("{}px", tab(&host, "0").offset_width()));
    assert_eq!(left0, "0px");
    tab(&host, "1").click();
    leptos::task::tick().await;
    sleep(80).await;
    let (width1, left1) = style(&bar);
    assert_eq!(width1, format!("{}px", tab(&host, "1").offset_width()));
    let expected_left =
        tab(&host, "1").get_bounding_client_rect().left() - q(&host, ".p-tablist-tab-list").get_bounding_client_rect().left();
    assert!((left1.trim_end_matches("px").parse::<f64>().unwrap() - expected_left).abs() < 1.0, "{left1} / {expected_left}");
}

#[wasm_bindgen_test]
async fn navigation_buttons_appear_and_scroll() {
    let host = host();
    host.style().set_property("width", "300px").unwrap();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Tabs value=value scrollable=true>
                <TabList>
                    {(0..20)
                        .map(|i| view! { <Tab value=i.to_string()><span style="display: inline-block; width: 90px">{format!("Tab {i}")}</span></Tab> })
                        .collect_view()}
                </TabList>
            </Tabs>
        }
    });
    leptos::task::tick().await;
    sleep(120).await;
    // без темы список не прокручивается сам: даём содержимому переполнение вручную
    let content = q(&host, ".p-tablist-content");
    content.style().set_property("overflow-x", "auto").unwrap();
    q(&host, ".p-tablist-tab-list").style().set_property("display", "flex").unwrap();
    content.dispatch_event(&web_sys::Event::new("scroll").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(has(&host, ".p-tablist-next-button"), "справа есть скрытая часть");
    assert!(!has(&host, ".p-tablist-prev-button"));
    assert_eq!(q(&host, ".p-tablist-next-button").get_attribute("aria-label").as_deref(), Some("Next"));
    q(&host, ".p-tablist-next-button").click();
    sleep(50).await;
    leptos::task::tick().await;
    assert!(content.scroll_left() > 0, "клик прокрутил содержимое");
    assert!(has(&host, ".p-tablist-prev-button"));
    assert_eq!(q(&host, ".p-tablist-prev-button").get_attribute("aria-label").as_deref(), Some("Previous"));
    q(&host, ".p-tablist-prev-button").click();
    sleep(50).await;
    assert_eq!(content.scroll_left(), 0);
}

#[wasm_bindgen_test]
async fn show_navigators_false_hides_buttons() {
    let host = host();
    host.style().set_property("width", "200px").unwrap();
    let value = RwSignal::new(Some("0".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Tabs value=value scrollable=true show_navigators=false>
                <TabList>
                    {(0..10)
                        .map(|i| view! { <Tab value=i.to_string()><span style="display: inline-block; width: 90px">{format!("T{i}")}</span></Tab> })
                        .collect_view()}
                </TabList>
            </Tabs>
        }
    });
    leptos::task::tick().await;
    sleep(80).await;
    assert!(!has(&host, ".p-tablist-nav-button"));
}
