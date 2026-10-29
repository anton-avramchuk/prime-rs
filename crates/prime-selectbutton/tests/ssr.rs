//! Структура DOM, метки, выбор, классы, `data-p`, слот и `pt` SelectButton (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_selectbutton::{SelectButton, SelectButtonOption, SelectButtonPt};
use prime_togglebutton::ToggleButtonPt;

fn options() -> Vec<SelectButtonOption> {
    vec![
        SelectButtonOption::new("One-Way", "one-way"),
        SelectButtonOption::new("Return", "return"),
        SelectButtonOption::new("Multi", "multi"),
    ]
}

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn root_structure_and_one_button_per_option_in_order() {
    let html = view! { <SelectButton options=options() /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for expected in [r#"class="p-selectbutton p-component""#, r#"role="group""#, r#"data-p="""#] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    for unwanted in ["aria-labelledby", "p-invalid", "p-selectbutton-fluid", "style="] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert_eq!(count(&html, r#"data-pc-name="togglebutton""#), 3, "{html}");
    let (a, b, c) = (html.find("One-Way").unwrap(), html.find("Return").unwrap(), html.find("Multi").unwrap());
    assert!(a < b && b < c, "{html}");
    assert_eq!(count(&html, r#"aria-pressed="false""#), 3, "{html}");
}

#[test]
fn empty_options_render_an_empty_group() {
    let html = view! { <SelectButton /> }.to_html();
    assert!(html.contains("p-selectbutton") && !html.contains("togglebutton"), "{html}");
}

#[test]
fn single_value_marks_only_the_selected_button() {
    let html = view! { <SelectButton options=options() value=RwSignal::new(Some("return".to_owned())) /> }.to_html();
    assert_eq!(count(&html, r#"aria-pressed="true""#), 1, "{html}");
    assert_eq!(count(&html, "p-togglebutton-checked"), 1, "{html}");
    let button = &html[html.find(r#"aria-pressed="true""#).unwrap()..];
    let button = &button[..button.find("</div>").unwrap()];
    assert!(button.contains("Return"), "{html}");
    // `default_value` без сигнала
    let html = view! { <SelectButton options=options() default_value="multi" /> }.to_html();
    assert_eq!(count(&html, r#"aria-pressed="true""#), 1, "{html}");
}

#[test]
fn multiple_values_mark_every_selected_button() {
    let values = RwSignal::new(vec!["one-way".to_owned(), "multi".to_owned()]);
    let html = view! { <SelectButton options=options() multiple=true values=values /> }.to_html();
    assert_eq!(count(&html, r#"aria-pressed="true""#), 2, "{html}");
    let html = view! { <SelectButton options=options() multiple=true default_values=vec!["return".to_owned()] /> }.to_html();
    assert_eq!(count(&html, r#"aria-pressed="true""#), 1, "{html}");
    // в множественном режиме одиночное значение игнорируется
    let html = view! { <SelectButton options=options() multiple=true value=RwSignal::new(Some("return".to_owned())) /> }.to_html();
    assert_eq!(count(&html, r#"aria-pressed="true""#), 0, "{html}");
}

#[test]
fn disabled_component_and_disabled_option() {
    let html = view! { <SelectButton options=options() disabled=true /> }.to_html();
    assert_eq!(count(&html, r#"data-p-disabled="true""#), 3, "{html}");
    let opts = vec![SelectButtonOption::new("A", "a"), SelectButtonOption::new("B", "b").disabled(true)];
    let html = view! { <SelectButton options=opts /> }.to_html();
    assert_eq!(count(&html, r#"data-p-disabled="true""#), 1, "{html}");
    assert_eq!(count(&html, r#"data-p-disabled="false""#), 1, "{html}");
    assert!(html.find(" p-disabled").unwrap() > html.find(">A<").unwrap(), "disabled только у второй кнопки: {html}");
}

#[test]
fn root_modifiers_and_aria() {
    let html = view! { <SelectButton options=options() invalid=true fluid=true aria_labelledby="lbl" /> }.to_html();
    for expected in
        ["p-selectbutton p-component p-invalid p-selectbutton-fluid", r#"role="group""#, r#"aria-labelledby="lbl""#, r#"data-p="invalid""#]
    {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    // `invalid` не передаётся кнопкам, `fluid` передаётся
    assert_eq!(count(&html, "p-invalid"), 1, "{html}");
    assert_eq!(count(&html, "p-togglebutton-fluid"), 3, "{html}");
}

#[test]
fn size_and_style_class_go_to_buttons() {
    let html = view! { <SelectButton options=options() size=Size::Small style_class="mine" /> }.to_html();
    assert_eq!(count(&html, "p-togglebutton-sm p-inputfield-sm"), 3, "{html}");
    assert_eq!(count(&html, "mine"), 3, "{html}");
    assert!(html.contains(r#"class="p-selectbutton p-component""#), "{html}");
    let html = view! { <SelectButton options=options() size=Size::Large /> }.to_html();
    assert_eq!(count(&html, "p-togglebutton-lg"), 3, "{html}");
}

#[test]
fn root_style_is_applied() {
    let html = view! { <SelectButton options=options() style="width: 20rem" /> }.to_html();
    assert!(html.contains(r#"style="width: 20rem;""#), "{html}");
}

#[test]
fn item_template_replaces_button_content() {
    let tpl = Callback::new(|(o, i): (SelectButtonOption, usize)| view! { <b class="tpl">{format!("{i}:{}", o.label)}</b> }.into_any());
    let html = view! { <SelectButton options=options() item_template=tpl /> }.to_html();
    for expected in ["0:One-Way", "1:Return", "2:Multi"] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("p-togglebutton-label"), "{html}");
    assert_eq!(count(&html, "p-togglebutton-content"), 3, "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = SelectButtonPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        toggle_button: ToggleButtonPt { label: Pt::new().class("custom"), ..Default::default() },
    };
    let html = view! { <SelectButton options=options() pt=pt /> }.to_html();
    assert!(html.contains("p-selectbutton p-component r") && html.contains(r#"data-r="1""#), "{html}");
    assert_eq!(count(&html, "p-togglebutton-label custom"), 3, "{html}");
}

#[test]
fn reactive_props_render_like_static() {
    let opts = RwSignal::new(options());
    let disabled = RwSignal::new(false);
    let reactive = view! { <SelectButton options=opts disabled=disabled default_value="return" /> }.to_html();
    let fixed = view! { <SelectButton options=options() default_value="return" /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    disabled.set(true);
    opts.update(|o| drop(o.pop()));
    let html = view! { <SelectButton options=opts disabled=disabled /> }.to_html();
    assert_eq!(count(&html, "data-pc-name"), 2, "{html}");
    assert_eq!(count(&html, r#"data-p-disabled="true""#), 2, "{html}");
}
