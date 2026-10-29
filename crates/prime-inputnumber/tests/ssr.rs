//! Форматы, структура DOM, классы и `data-p`, раскладки кнопок, значок очистки, атрибуты и `pt` InputNumber (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_inputnumber::{ButtonLayout, CurrencyDisplay, InputNumber, InputNumberMode, InputNumberPt};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn default_structure() {
    let html = view! { <InputNumber default_value=1234.5 /> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    for expected in [
        "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled",
        r#"role="spinbutton""#,
        r#"inputmode="decimal""#,
        r#"value="1,234.5""#,
        r#"aria-valuenow="1234.5""#,
        r#"step="1""#,
        "p-inputtext p-component p-filled p-inputnumber-input",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    for unwanted in
        ["p-inputnumber-stacked", "p-invalid", "p-inputwrapper-focus", "<button", "<svg", "required", "readonly", "disabled", "aria-label"]
    {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    // пустое поле
    let html = view! { <InputNumber /> }.to_html();
    assert!(html.contains(r#"value="""#) && !html.contains("p-filled") && !html.contains("aria-valuenow"), "{html}");
    assert!(html.contains(r#"data-p="empty""#), "{html}");
}

#[test]
fn formats_follow_options() {
    let text = |html: String, expected: &str| assert!(html.contains(&format!(r#"value="{expected}""#)), "{expected}: {html}");
    text(view! { <InputNumber value=RwSignal::new(Some(1234.5)) min_fraction_digits=2 /> }.to_html(), "1,234.50");
    text(view! { <InputNumber value=RwSignal::new(Some(1234.5)) min_fraction_digits=2 locale="de-DE" /> }.to_html(), "1.234,50");
    text(view! { <InputNumber default_value=1234567.891 locale="en-IN" /> }.to_html(), "12,34,567.891");
    text(view! { <InputNumber default_value=1234567.0 use_grouping=false /> }.to_html(), "1234567");
    text(view! { <InputNumber default_value=7.0 prefix="Expires in " suffix=" days" /> }.to_html(), "Expires in 7 days");
    text(view! { <InputNumber default_value=1.005 max_fraction_digits=2 /> }.to_html(), "1.01");
    text(view! { <InputNumber default_value=1234.5 format=false /> }.to_html(), "1234.5");
    text(
        view! { <InputNumber default_value=-1234.5 mode=InputNumberMode::Currency currency="USD" locale="en-US" /> }.to_html(),
        "-$1,234.50",
    );
    text(
        view! { <InputNumber default_value=1234.5 mode=InputNumberMode::Currency currency="EUR" locale="de-DE" /> }.to_html(),
        "1.234,50\u{a0}€",
    );
    text(
        view! { <InputNumber default_value=1234.0 mode=InputNumberMode::Currency currency="INR" currency_display=CurrencyDisplay::Code locale="en-IN" /> }
            .to_html(),
        "INR\u{a0}1,234.00",
    );
    text(
        view! { <InputNumber default_value=1234.5 mode=InputNumberMode::Currency currency="JPY" locale="ja-JP" /> }.to_html(),
        "\u{ffe5}1,235",
    );
}

#[test]
fn allow_empty_false_shows_zero_and_is_always_filled() {
    let html = view! { <InputNumber allow_empty=false /> }.to_html();
    assert!(html.contains(r#"value="0""#) && html.contains("p-inputwrapper-filled"), "{html}");
}

#[test]
fn stacked_buttons_are_grouped() {
    let html = view! { <InputNumber show_buttons=true /> }.to_html();
    assert!(html.contains("p-inputnumber-stacked"), "{html}");
    assert_eq!(count(&html, "p-inputnumber-button-group"), 1, "{html}");
    assert_eq!(count(&html, "<button"), 2, "{html}");
    for expected in [
        "p-inputnumber-button p-inputnumber-increment-button",
        "p-inputnumber-button p-inputnumber-decrement-button",
        r#"tabindex="-1""#,
        r#"aria-hidden="true""#,
        r#"data-p="empty stacked""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    let group = html.find("p-inputnumber-button-group").unwrap();
    assert!(html.find("<input").unwrap() < group && group < html.find("increment-button").unwrap(), "{html}");
    assert_eq!(count(&html, "<svg"), 2, "{html}");
}

#[test]
fn horizontal_and_vertical_buttons_are_not_grouped() {
    for (layout, class) in [(ButtonLayout::Horizontal, "p-inputnumber-horizontal"), (ButtonLayout::Vertical, "p-inputnumber-vertical")] {
        let html = view! { <InputNumber show_buttons=true button_layout=layout /> }.to_html();
        assert!(html.contains(class) && !html.contains("p-inputnumber-button-group"), "{class}: {html}");
        assert_eq!(count(&html, "<button"), 2, "{html}");
        assert!(html.contains(&format!("{}\"", layout.as_str())) || html.contains(layout.as_str()), "{html}");
    }
    // без `show_buttons` раскладка не влияет
    let html = view! { <InputNumber button_layout=ButtonLayout::Vertical /> }.to_html();
    assert!(!html.contains("p-inputnumber-vertical") && !html.contains("<button"), "{html}");
}

#[test]
fn clear_icon_follows_value_and_layout() {
    let html = view! { <InputNumber default_value=5.0 show_clear=true /> }.to_html();
    assert_eq!(count(&html, "p-inputnumber-clear-icon"), 1, "{html}");
    assert!(!view! { <InputNumber show_clear=true /> }.to_html().contains("clear-icon"));
    assert!(!view! { <InputNumber default_value=5.0 /> }.to_html().contains("clear-icon"));
    assert!(
        !view! { <InputNumber default_value=5.0 show_clear=true show_buttons=true button_layout=ButtonLayout::Vertical /> }
            .to_html()
            .contains("clear-icon")
    );
    let icon = ViewFn::from(|| view! { <b class="custom-clear" /> });
    let html = view! { <InputNumber default_value=5.0 show_clear=true clear_icon=icon /> }.to_html();
    assert!(html.contains("custom-clear") && html.contains("p-inputnumber-clear-icon"), "{html}");
}

#[test]
fn custom_button_icons_and_classes() {
    let up = ViewFn::from(|| view! { <i class="plus" /> });
    let down = ViewFn::from(|| view! { <i class="minus" /> });
    let html = view! {
        <InputNumber
            show_buttons=true
            increment_button_icon=up
            decrement_button_icon=down
            increment_button_class="inc-extra"
            decrement_button_class="dec-extra"
        />
    }
    .to_html();
    for expected in
        ["class=\"plus\"", "class=\"minus\"", "p-inputnumber-increment-button inc-extra", "p-inputnumber-decrement-button dec-extra"]
    {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("<svg"), "{html}");
}

#[test]
fn modifiers_classes_and_data_p() {
    let html = view! { <InputNumber default_value=1.0 invalid=true fluid=true size=Size::Small variant=Variant::Filled /> }.to_html();
    for expected in [
        "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled p-inputnumber-fluid p-invalid",
        r#"data-p="invalid fluid filled small""#,
        "p-inputtext p-component p-filled p-inputtext-sm p-invalid p-variant-filled p-inputtext-fluid p-inputnumber-input",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    let html = view! { <InputNumber disabled=true show_buttons=true /> }.to_html();
    assert!(html.contains(r#"data-p="disabled empty stacked""#), "{html}");
    assert_eq!(count(&html, r#"disabled="""#), 3, "поле и две кнопки: {html}");
}

#[test]
fn attributes_are_set_only_when_given() {
    let html = view! {
        <InputNumber
            default_value=30.0
            min=0.0
            max=100.0
            step=0.5
            input_id="amount"
            name="amount"
            placeholder="Amount"
            title="Price"
            autocomplete="off"
            tabindex=2
            max_length=12
            aria_label="Amount"
            aria_labelledby="l"
            aria_describedby="d"
            aria_required=true
            required=true
            readonly=true
        />
    }
    .to_html();
    for expected in [
        r#"id="amount""#,
        r#"name="amount""#,
        r#"placeholder="Amount""#,
        r#"title="Price""#,
        r#"autocomplete="off""#,
        r#"tabindex="2""#,
        r#"maxlength="12""#,
        r#"aria-label="Amount""#,
        r#"aria-labelledby="l""#,
        r#"aria-describedby="d""#,
        r#"aria-required="true""#,
        r#"aria-valuemin="0""#,
        r#"aria-valuemax="100""#,
        r#"aria-valuenow="30""#,
        r#"min="0""#,
        r#"max="100""#,
        r#"step="0.5""#,
        "required",
        "readonly",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn style_and_style_classes() {
    let html =
        view! { <InputNumber style="width: 9rem" style_class="mine" input_style="width: 3rem" input_style_class="inner" /> }.to_html();
    assert!(html.contains(r#"style="width: 9rem;""#) && html.contains("p-inputwrapper mine"), "{html}");
    assert!(html.contains(r#"style="width: 3rem;""#) && html.contains("p-inputnumber-input inner"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = InputNumberPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        input: Pt::new().class("custom").attr("data-i", "2"),
        clear_icon: Pt::new().class("ci"),
        button_group: Pt::new().class("bg"),
        increment_button: Pt::new().class("ib"),
        increment_button_icon: Pt::new().attr("data-ii", "3"),
        decrement_button: Pt::new().class("db"),
        decrement_button_icon: Pt::new().attr("data-di", "4"),
    };
    let html = view! { <InputNumber pt=pt default_value=5.0 show_clear=true show_buttons=true /> }.to_html();
    for expected in [
        "p-inputwrapper-filled p-inputnumber-stacked r",
        r#"data-r="1""#,
        "p-inputnumber-input custom",
        r#"data-i="2""#,
        "p-inputnumber-clear-icon ci",
        "p-inputnumber-button-group bg",
        "p-inputnumber-increment-button ib",
        "p-inputnumber-decrement-button db",
        r#"data-ii="3""#,
        r#"data-di="4""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn reactive_props_render_like_static() {
    let locale = RwSignal::new(Some("en-US".to_owned()));
    let reactive = view! { <InputNumber default_value=1234.5 min_fraction_digits=2 locale=locale /> }.to_html();
    let fixed = view! { <InputNumber default_value=1234.5 min_fraction_digits=2 locale="en-US" /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    locale.set(Some("de-DE".to_owned()));
    assert!(view! { <InputNumber default_value=1234.5 min_fraction_digits=2 locale=locale /> }.to_html().contains("1.234,50"));
}

#[test]
fn fluid_inherited_from_context() {
    let outside = view! { <InputNumber/> }.to_html();
    assert!(!outside.contains("p-inputnumber-fluid"), "{outside}");
    Owner::new().with(|| {
        prime_core::fluid::provide_fluid();
        let inherited = view! { <InputNumber/> }.to_html();
        assert!(inherited.contains("p-inputnumber-fluid"), "{inherited}");
        let forced_off = view! { <InputNumber fluid=false/> }.to_html();
        assert!(!forced_off.contains("p-inputnumber-fluid"), "{forced_off}");
        let live = RwSignal::new(None::<bool>);
        let reactive = view! { <InputNumber fluid=live/> }.to_html();
        assert!(reactive.contains("p-inputnumber-fluid"), "{reactive}");
        live.set(Some(false));
        let reactive_off = view! { <InputNumber fluid=live/> }.to_html();
        assert!(!reactive_off.contains("p-inputnumber-fluid"), "{reactive_off}");
    });
}
