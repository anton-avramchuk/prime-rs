//! Структура DOM, классы, атрибуты, `pt` и a11y Button (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_badge::BadgeSeverity;
use prime_button::{Button, ButtonPt, ButtonSeverity, ButtonVariant, IconPos};
use prime_core::{Pt, Size};
use prime_icons::CheckIcon;

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <CheckIcon /> })
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет `{needle}` в {html}"))
}

#[test]
fn default_button() {
    let html = view! { <Button /> }.to_html();
    assert!(html.starts_with("<button"), "{html}");
    assert!(html.contains(r#"type="button""#), "{html}");
    assert!(html.contains("p-button p-component") && html.contains("p-ripple"), "{html}");
    assert!(!html.contains("disabled") && !html.contains("p-button-label"), "{html}");
}

#[test]
fn label_is_rendered_in_a_span() {
    let html = view! { <Button label="Сохранить" /> }.to_html();
    assert!(html.contains(r#"<span class="p-button-label""#) || html.contains("p-button-label"), "{html}");
    assert!(html.contains(">Сохранить<"), "{html}");
}

#[test]
fn severity_variant_and_size() {
    let html = view! { <Button severity=ButtonSeverity::Danger variant=ButtonVariant::Outlined size=Size::Small /> }.to_html();
    for c in ["p-button-danger", "p-button-outlined", "p-button-sm"] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-p-severity="danger""#), "{html}");
}

#[test]
fn icon_only_and_vertical() {
    let only = view! { <Button icon=icon() /> }.to_html();
    assert!(only.contains("p-button-icon-only") && only.contains("icon-only"), "{only}");
    assert!(only.contains("p-button-icon") && !only.contains("p-button-icon-left"), "{only}");

    let vertical = view! { <Button icon=icon() label="x" icon_pos=IconPos::Top /> }.to_html();
    assert!(vertical.contains("p-button-vertical") && vertical.contains("p-button-icon-top"), "{vertical}");
}

#[test]
fn icon_position_class() {
    let html = view! { <Button icon=icon() label="x" icon_pos=IconPos::Right /> }.to_html();
    assert!(html.contains("p-button-icon-right"), "{html}");
    assert!(html.contains(r#"data-p="right""#), "{html}");
}

#[test]
fn style_class_and_style_keep_own_classes() {
    let html = view! { <Button style_class="custom" style="margin: 4px" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-button p-component"), "{html}");
    assert!(html.contains("margin: 4px"), "{html}");
}

#[test]
fn disabled_button() {
    let html = view! { <Button disabled=true /> }.to_html();
    assert!(html.contains("disabled"), "{html}");
    assert!(html.contains(r#"data-p-disabled="true""#), "{html}");
    let enabled = view! { <Button disabled=false /> }.to_html();
    assert!(!enabled.contains("disabled"), "{enabled}");
}

#[test]
fn loading_shows_spinner_instead_of_icon_and_disables() {
    let html = view! { <Button loading=true icon=icon() label="x" /> }.to_html();
    assert!(html.contains("p-button-loading") && html.contains("disabled"), "{html}");
    assert!(html.contains("p-button-loading-icon") && html.contains("p-icon-spin"), "{html}");
    assert!(html.contains(r#"aria-hidden="true""#), "{html}");
    assert!(!html.contains(r#"class="p-button-icon"#), "иконки быть не должно: {html}");
}

#[test]
fn custom_loading_icon_is_wrapped() {
    let custom = ViewFn::from(|| view! { <b>"..."</b> });
    let html = view! { <Button loading=true loading_icon=custom /> }.to_html();
    assert!(html.contains("<span") && html.contains("p-button-loading-icon"), "{html}");
    assert!(html.contains("<b>...</b>") && !html.contains("p-icon-spin"), "{html}");
}

#[test]
fn badge_defaults_to_secondary_and_takes_severity() {
    let html = view! { <Button label="x" badge="3" /> }.to_html();
    assert!(html.contains("p-badge") && html.contains("p-badge-secondary") && html.contains(">3<"), "{html}");
    let danger = view! { <Button label="x" badge="3" badge_severity=BadgeSeverity::Danger /> }.to_html();
    assert!(danger.contains("p-badge-danger"), "{danger}");
}

#[test]
fn children_come_first_and_replace_label_and_badge() {
    let html = view! {
        <Button label="не будет" badge="9">
            <i>"свой контент"</i>
        </Button>
    }
    .to_html();
    assert!(html.contains("<i>свой контент</i>"), "{html}");
    assert!(!html.contains("p-button-label") && !html.contains("p-badge"), "{html}");
}

#[test]
fn child_order_is_icon_label_badge() {
    let html = view! { <Button icon=icon() label="Текст" badge="2" /> }.to_html();
    let (i, l, b) = (pos(&html, "p-button-icon"), pos(&html, "p-button-label"), pos(&html, "p-badge"));
    assert!(i < l && l < b, "{html}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = ButtonPt {
        root: Pt::new().class("root-extra").attr("data-testid", "btn").style("color: red"),
        label: Pt::new().class("label-extra"),
        icon: Pt::new().class("icon-extra"),
        badge: Pt::new().class("badge-extra"),
        ..Default::default()
    };
    let html = view! { <Button icon=icon() label="x" badge="1" pt=pt /> }.to_html();
    for c in
        ["root-extra", "p-button p-component", "label-extra", "p-button-label", "icon-extra", "p-button-icon", "badge-extra", "p-badge"]
    {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="btn""#) && html.contains("color: red"), "{html}");
}

#[test]
fn pt_on_loading_icon() {
    let pt = ButtonPt { loading_icon: Pt::new().class("li-extra"), ..Default::default() };
    let html = view! { <Button loading=true pt=pt /> }.to_html();
    assert!(html.contains("li-extra") && html.contains("p-button-loading-icon"), "{html}");
}

#[test]
fn a11y_attributes_and_native_button_type() {
    let html = view! { <Button aria_label="Закрыть" tabindex=2 button_type="submit" /> }.to_html();
    assert!(html.contains(r#"aria-label="Закрыть""#) && html.contains(r#"tabindex="2""#) && html.contains(r#"type="submit""#), "{html}");
}

#[test]
fn reactive_loading_and_label() {
    let owner = Owner::new();
    owner.with(|| {
        let loading = RwSignal::new(false);
        let label = RwSignal::new(Some("a".to_owned()));
        let html = view! { <Button loading=loading label=label /> }.to_html();
        assert!(!html.contains("p-button-loading"), "{html}");
        loading.set(true);
        label.set(Some("b".into()));
        let html = view! { <Button loading=loading label=label /> }.to_html();
        assert!(html.contains("p-button-loading") && html.contains(">b<"), "{html}");
    });
}

#[test]
fn ssr_with_ripple_and_autofocus_never_touches_the_browser() {
    // Effect-ы не выполняются при SSR: никакого window, фокуса и p-ink; тест падал бы паникой web-sys на нативной цели.
    let owner = Owner::new();
    owner.with(|| {
        prime_core::provide_prime_config(prime_core::PrimeConfig { ripple: true, ..Default::default() });
        let html = view! { <Button label="x" autofocus=true /> }.to_html();
        assert!(html.contains("p-ripple") && !html.contains("p-ink"), "{html}");
    });
}
