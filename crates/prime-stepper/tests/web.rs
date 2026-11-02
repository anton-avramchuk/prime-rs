//! Поведение Stepper в реальном браузере (wasm-bindgen-test): клик по шагу, линейный режим, переходы из содержимого панели,
//! внешний сигнал, разделители при динамических шагах, вертикальная раскладка.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_stepper::{Step, StepContext, StepItem, StepList, StepPanel, StepPanels, Stepper};
use wasm_bindgen::JsCast;
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

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn count(host: &HtmlElement, sel: &str) -> u32 {
    host.query_selector_all(sel).unwrap().length()
}

fn header(host: &HtmlElement, value: u32) -> HtmlElement {
    q(host, &format!(".p-step-header[id$=\"_step_{value}\"]"))
}

fn active_steps(host: &HtmlElement) -> Vec<String> {
    let all = host.query_selector_all(".p-step-active .p-step-number").unwrap();
    (0..all.length()).map(|i| all.item(i).unwrap().text_content().unwrap_or_default()).collect()
}

fn panel_text(host: &HtmlElement) -> String {
    host.query_selector_all(".p-steppanel-content").unwrap().item(0).and_then(|n| n.text_content()).unwrap_or_default()
}

#[wasm_bindgen_test]
async fn clicking_a_step_activates_it_and_swaps_the_panel() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value>
                <StepList>
                    <Step value=1u32>"Header I"</Step>
                    <Step value=2u32>"Header II"</Step>
                    <Step value=3u32>"Header III"</Step>
                </StepList>
                <StepPanels>
                    <StepPanel value=1u32>"Content I"</StepPanel>
                    <StepPanel value=2u32>"Content II"</StepPanel>
                    <StepPanel value=3u32>"Content III"</StepPanel>
                </StepPanels>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    assert_eq!(active_steps(&host), ["1"]);
    assert_eq!(panel_text(&host), "Content I");
    header(&host, 3).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(3));
    assert_eq!(active_steps(&host), ["3"]);
    assert_eq!(panel_text(&host), "Content III");
    assert_eq!(count(&host, ".p-steppanel-content"), 1, "содержимое только у активной панели");
    assert_eq!(q(&host, ".p-step-active").get_attribute("aria-current").as_deref(), Some("step"));
    assert!(q(&host, "[role=presentation]:not(.p-step-active)").get_attribute("aria-current").is_none());
    host.remove();
}

#[wasm_bindgen_test]
async fn external_value_changes_move_the_active_step() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value>
                <StepList><Step value=1u32>"A"</Step><Step value=2u32>"B"</Step></StepList>
                <StepPanels><StepPanel value=1u32>"One"</StepPanel><StepPanel value=2u32>"Two"</StepPanel></StepPanels>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    value.set(Some(2));
    leptos::task::tick().await;
    assert_eq!(active_steps(&host), ["2"]);
    assert_eq!(panel_text(&host), "Two");
    value.set(None);
    leptos::task::tick().await;
    assert!(active_steps(&host).is_empty());
    assert_eq!(count(&host, ".p-steppanel-content"), 0);
    host.remove();
}

#[wasm_bindgen_test]
async fn linear_mode_blocks_clicks_but_not_programmatic_activation() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value linear=true>
                <StepList><Step value=1u32>"A"</Step><Step value=2u32>"B"</Step><Step value=3u32>"C"</Step></StepList>
                <StepPanels>
                    <StepPanel
                        value=1u32
                        content=Callback::new(|c: StepContext| {
                            view! { <button class="next" on:click=move |_| c.activate.run(2)>"Next"</button> }.into_any()
                        })
                    />
                    <StepPanel value=2u32>"Two"</StepPanel>
                </StepPanels>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    assert!(q(&host, ".p-stepper").class_name().contains("p-readonly"));
    assert!(header(&host, 3).has_attribute("disabled"));
    assert!(!header(&host, 1).has_attribute("disabled"), "активный шаг не блокируется");
    header(&host, 3).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(1), "клик по недоступному шагу ничего не меняет");
    q(&host, ".next").click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(2));
    assert_eq!(panel_text(&host), "Two");
    assert!(header(&host, 1).has_attribute("disabled"), "теперь недоступен шаг 1");
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_step_ignores_clicks() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value>
                <StepList><Step value=1u32>"A"</Step><Step value=2u32 disabled=true>"B"</Step></StepList>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    header(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(1));
    assert_eq!(header(&host, 2).get_attribute("tabindex").as_deref(), Some("-1"));
    host.remove();
}

#[wasm_bindgen_test]
async fn separators_follow_dynamically_added_and_removed_steps() {
    let extra = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper default_value=1u32>
                <StepList>
                    <Step value=1u32>"A"</Step>
                    <Step value=2u32>"B"</Step>
                    {move || extra.get().then(|| view! { <Step value=3u32>"C"</Step> })}
                </StepList>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-stepper-separator"), 1);
    extra.set(true);
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-stepper-separator"), 2, "второй шаг получил разделитель");
    extra.set(false);
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-stepper-separator"), 1, "разделитель снова только у первого");
    host.remove();
}

#[wasm_bindgen_test]
async fn vertical_layout_shows_and_hides_panels() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value>
                <StepItem value=1u32><Step>"A"</Step><StepPanel>"One"</StepPanel></StepItem>
                <StepItem value=2u32><Step>"B"</Step><StepPanel>"Two"</StepPanel></StepItem>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    let motion = |i: u32| doc_motion(&host, i);
    assert_eq!(count(&host, ".p-steppanel-content"), 2, "содержимое всех панелей в разметке");
    assert_eq!(motion(2).style().get_property_value("display").unwrap(), "none", "неактивная панель скрыта");
    assert_ne!(motion(1).style().get_property_value("display").unwrap(), "none", "активная показана");
    assert!(q(&host, ".p-stepitem-active").text_content().unwrap_or_default().contains("One"));
    header(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(active_steps(&host), ["2"]);
    assert!(q(&host, ".p-steppanel-active").text_content().unwrap_or_default().contains("Two"));
    host.remove();
}

fn doc_motion(host: &HtmlElement, panel: u32) -> HtmlElement {
    host.query_selector_all(".p-steppanel .p-motion").unwrap().item(panel - 1).unwrap().dyn_into().unwrap()
}

#[wasm_bindgen_test]
async fn vertical_inactive_panel_is_hidden_after_the_collapse_animation() {
    let value = RwSignal::new(Some(1u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Stepper value=value>
                <StepItem value=1u32><Step>"A"</Step><StepPanel>"One"</StepPanel></StepItem>
                <StepItem value=2u32><Step>"B"</Step><StepPanel>"Two"</StepPanel></StepItem>
            </Stepper>
        }
    });
    leptos::task::tick().await;
    assert_eq!(doc_motion(&host, 2).style().get_property_value("display").unwrap(), "none", "неактивная панель скрыта сразу");
    value.set(Some(2));
    leptos::task::tick().await;
    // запасной таймер анимации завершает сворачивание, даже если animationend не пришёл
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 900).unwrap();
    }))
    .await
    .unwrap();
    assert_eq!(doc_motion(&host, 1).style().get_property_value("display").unwrap(), "none", "бывшая активная панель скрыта");
    assert_ne!(doc_motion(&host, 2).style().get_property_value("display").unwrap(), "none", "новая активная показана");
    host.remove();
}
