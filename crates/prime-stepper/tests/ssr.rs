//! Разметка Stepper (SSR): корень, шаги и их состояния, разделители, панели, вертикальная раскладка, `content`, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_stepper::{Step, StepContext, StepItem, StepList, StepPanel, StepPanelPt, StepPanels, StepPt, Stepper};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

/// Горизонтальный мастер из трёх шагов и панелей.
fn horizontal(default_value: Option<u32>, linear: bool) -> String {
    render(|| {
        view! {
            <Stepper id="s" default_value=default_value.unwrap_or(0) linear=linear>
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
        .to_html()
    })
}

#[test]
fn root_markup_and_linear_class() {
    let html = horizontal(Some(1), false);
    assert!(html.contains("p-stepper p-component") && html.contains("role=\"tablist\"") && html.contains("id=\"s\""), "{html}");
    assert!(!html.contains("p-readonly"), "{html}");
    let linear = horizontal(Some(1), true);
    assert!(linear.contains("p-stepper p-component p-readonly"), "{linear}");
}

#[test]
fn steps_markup() {
    let html = horizontal(Some(2), false);
    assert!(html.contains("p-steplist") && html.contains("p-steppanels"), "{html}");
    assert_eq!(html.matches("role=\"tab\"").count(), 3, "{html}");
    assert_eq!(html.matches("class=\"p-step-header\"").count(), 3, "{html}");
    assert!(html.contains("id=\"s_step_1\"") && html.contains("aria-controls=\"s_steppanel_1\""), "{html}");
    assert!(html.contains("class=\"p-step-number\">1<") || html.contains("p-step-number"), "{html}");
    assert!(html.contains("p-step-title") && html.contains("Header II"), "{html}");
    assert_eq!(html.matches("class=\"p-step p-step-active\"").count(), 1, "{html}");
    assert_eq!(html.matches("aria-current=\"step\"").count(), 1, "{html}");
    assert!(html.matches("data-p-active=\"true\"").count() >= 2, "шаг и панель: {html}");
    assert!(html.contains("role=\"presentation\""), "{html}");
}

#[test]
fn separators_are_between_steps_only() {
    let html = horizontal(Some(1), false);
    assert_eq!(html.matches("p-stepper-separator").count(), 2, "{html}");
    let two = render(|| {
        view! {
            <Stepper default_value=1u32>
                <StepList>
                    <Step value=1u32>"A"</Step>
                    <Step value=2u32>"B"</Step>
                </StepList>
            </Stepper>
        }
        .to_html()
    });
    assert_eq!(two.matches("p-stepper-separator").count(), 1, "{two}");
    let one = render(|| view! { <Stepper default_value=1u32><StepList><Step value=1u32>"A"</Step></StepList></Stepper> }.to_html());
    assert_eq!(one.matches("p-stepper-separator").count(), 0, "{one}");
}

#[test]
fn only_the_active_horizontal_panel_has_content() {
    let html = horizontal(Some(2), false);
    assert!(html.contains("Content II") && !html.contains("Content I<") && !html.contains("Content III"), "{html}");
    assert_eq!(html.matches("role=\"tabpanel\"").count(), 3, "{html}");
    assert_eq!(html.matches("p-steppanel-content-wrapper").count(), 1, "{html}");
    assert!(
        html.contains("id=\"s_steppanel_2\"") && html.contains("aria-controls=\"s_step_2\"") && html.contains("data-pc-name=\"steppanel\""),
        "{html}"
    );
    assert!(!html.contains("p-steppanel-active"), "в горизонтальной раскладке класса нет: {html}");
}

#[test]
fn no_value_means_no_active_step_or_panel_content() {
    let html = render(|| {
        view! {
            <Stepper>
                <StepList><Step value=1u32>"A"</Step></StepList>
                <StepPanels><StepPanel value=1u32>"Content"</StepPanel></StepPanels>
            </Stepper>
        }
        .to_html()
    });
    assert!(!html.contains("p-step-active") && !html.contains("Content") && !html.contains("aria-current"), "{html}");
}

#[test]
fn linear_disables_inactive_steps_but_not_the_active_one() {
    let html = horizontal(Some(1), true);
    assert_eq!(html.matches("p-step p-disabled").count(), 2, "{html}");
    assert_eq!(html.matches("tabindex=\"-1\"").count(), 2, "{html}");
    let buttons: Vec<&str> = html.split("<button").skip(1).map(|b| b.split('>').next().unwrap()).collect();
    assert_eq!(buttons.len(), 3, "{html}");
    let disabled: Vec<bool> = buttons.iter().map(|b| b.split_whitespace().any(|t| t == "disabled" || t.starts_with("disabled="))).collect();
    assert_eq!(disabled, [false, true, true], "{html}");
    assert!(html.contains("data-p-disabled=\"true\"") && html.contains("data-p-disabled=\"false\""), "{html}");
}

#[test]
fn own_disabled_step_is_marked_unless_active() {
    let html = render(|| {
        view! {
            <Stepper default_value=1u32>
                <StepList>
                    <Step value=1u32 disabled=true>"A"</Step>
                    <Step value=2u32 disabled=true>"B"</Step>
                </StepList>
            </Stepper>
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"p-step p-disabled\"").count(), 1, "заблокирован только неактивный шаг: {html}");
    assert!(html.contains("class=\"p-step p-step-active\""), "активный шаг не блокируется: {html}");
}

#[test]
fn vertical_layout_keeps_all_panels_and_marks_the_active_one() {
    let html = render(|| {
        view! {
            <Stepper id="v" default_value=2u32>
                <StepItem value=1u32>
                    <Step>"Header I"</Step>
                    <StepPanel>"Content I"</StepPanel>
                </StepItem>
                <StepItem value=2u32>
                    <Step>"Header II"</Step>
                    <StepPanel>"Content II"</StepPanel>
                </StepItem>
                <StepItem value=3u32>
                    <Step>"Header III"</Step>
                    <StepPanel>"Content III"</StepPanel>
                </StepItem>
            </Stepper>
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"p-stepitem\"").count() + html.matches("p-stepitem p-stepitem-active").count(), 3, "{html}");
    assert_eq!(html.matches("p-stepitem-active").count(), 1, "{html}");
    assert_eq!(html.matches("p-steppanel-active").count(), 1, "{html}");
    for text in ["Content I<", "Content II<", "Content III<"] {
        assert!(html.contains(text), "содержимое {text} в разметке: {html}");
    }
    assert_eq!(html.matches("display: none").count(), 2, "две неактивные панели скрыты: {html}");
    assert_eq!(html.matches("class=\"p-motion\"").count(), 3, "{html}");
    // шаги внутри StepItem получают значение элемента и без разделителя; разделитель панели — кроме последнего элемента
    assert!(html.contains("id=\"v_step_1\"") && html.contains("id=\"v_steppanel_3\""), "{html}");
    assert_eq!(html.matches("p-stepper-separator").count(), 2, "{html}");
    assert!(html.contains("p-step-number\">1<") && html.contains("p-step-number\">3<"), "{html}");
}

#[test]
fn custom_content_replaces_the_header_and_the_panel_body() {
    let html = render(|| {
        view! {
            <Stepper default_value=1u32>
                <StepList>
                    <Step
                        value=1u32
                        content=Callback::new(|c: StepContext| view! { <i class="my">{format!("v{}", c.value)}</i> }.into_any())
                    />
                    <Step value=2u32>"B"</Step>
                </StepList>
                <StepPanels>
                    <StepPanel
                        value=1u32
                        content=Callback::new(|c: StepContext| view! { <b class="body">{format!("active={}", c.active.get_untracked())}</b> }.into_any())
                    />
                </StepPanels>
            </Stepper>
        }
        .to_html()
    });
    assert!(html.contains("class=\"my\"") && html.contains("v1"), "{html}");
    assert_eq!(html.matches("class=\"p-step-header\"").count(), 1, "кнопка только у второго шага: {html}");
    assert!(html.contains("class=\"body\"") && html.contains("active=true"), "{html}");
    assert_eq!(html.matches("p-stepper-separator").count(), 1, "разделитель остаётся после первого шага: {html}");
}

#[test]
fn style_class_and_pt() {
    let step_pt = StepPt { root: Pt::new().class("sr"), header: Pt::new().class("sh"), number: Pt::new().class("sn"), ..StepPt::default() };
    let panel_pt = StepPanelPt { root: Pt::new().class("pr"), content: Pt::new().class("pc"), ..StepPanelPt::default() };
    let html = render(|| {
        view! {
            <Stepper default_value=1u32 style="width: 20rem" style_class="mine" pt=Pt::new().class("r").attr("data-x", "1")>
                <StepList pt=Pt::new().class("l")>
                    <Step value=1u32 pt=step_pt>"A"</Step>
                </StepList>
                <StepPanels pt=Pt::new().class("ps")>
                    <StepPanel value=1u32 pt=panel_pt>"Content"</StepPanel>
                </StepPanels>
            </Stepper>
        }
        .to_html()
    });
    assert!(html.contains("p-stepper p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 20rem"), "{html}");
    assert!(html.contains("p-steplist l") && html.contains("p-steppanels ps"), "{html}");
    assert!(html.contains("p-step p-step-active sr") && html.contains("p-step-header sh") && html.contains("p-step-number sn"), "{html}");
    assert!(html.contains("p-steppanel pr") && html.contains("p-steppanel-content pc"), "{html}");
}

#[test]
fn external_value_signal_is_used() {
    let value = RwSignal::new(Some(3u32));
    let html = render(|| {
        view! {
            <Stepper value=value>
                <StepList><Step value=1u32>"A"</Step><Step value=3u32>"C"</Step></StepList>
                <StepPanels><StepPanel value=3u32>"Content C"</StepPanel></StepPanels>
            </Stepper>
        }
        .to_html()
    });
    assert!(html.contains("p-step-active") && html.contains("Content C"), "{html}");
}

#[test]
fn vertical_stepper_context_does_not_leak_to_a_following_horizontal_one() {
    // оба Stepper в одном владельце: контекст StepItem не должен сделать вторую панель «вертикальной»
    let html = render(|| {
        view! {
            <Stepper default_value=1u32>
                <StepItem value=1u32><Step>"A"</Step><StepPanel>"V1"</StepPanel></StepItem>
                <StepItem value=2u32><Step>"B"</Step><StepPanel>"V2"</StepPanel></StepItem>
            </Stepper>
            <Stepper default_value=1u32>
                <StepList><Step value=1u32>"H1"</Step><Step value=2u32>"H2"</Step></StepList>
                <StepPanels>
                    <StepPanel value=1u32>"Horizontal one"</StepPanel>
                    <StepPanel value=2u32>"Horizontal two"</StepPanel>
                </StepPanels>
            </Stepper>
        }
        .to_html()
    });
    assert!(html.contains("Horizontal one") && !html.contains("Horizontal two"), "{html}");
    assert_eq!(html.matches("class=\"p-motion\"").count(), 2, "p-motion только у вертикальных панелей: {html}");
    assert_eq!(html.matches("p-stepper-separator").count(), 1 + 1, "разделитель панели вертикального и шага горизонтального: {html}");
}
