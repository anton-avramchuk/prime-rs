//! Структура DOM, классы, подпись из локали, `pt` и паритет ветвей Chip (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_chip::{Chip, ChipPt};
use prime_core::{Locale, PrimeConfig, Pt, provide_prime_config};

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <svg data-testid="i"></svg> })
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет `{needle}` в {html}"))
}

#[test]
fn label_is_rendered_in_a_div() {
    let html = view! { <Chip label="Amy Elsner" /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains("p-chip p-component") && html.contains(r#"<div class="p-chip-label""#), "{html}");
    assert!(html.contains(">Amy Elsner<"), "{html}");
}

#[test]
fn no_label_no_label_element() {
    let html = view! { <Chip /> }.to_html();
    assert!(!html.contains("p-chip-label") && !html.contains("p-chip-remove-icon"), "{html}");
}

#[test]
fn disabled_and_removable_set_class_and_data_p() {
    let html = view! { <Chip label="x" disabled=true removable=true /> }.to_html();
    assert!(html.contains("p-disabled") && html.contains(r#"data-p="removable""#), "{html}");
}

#[test]
fn style_class_and_style_keep_own_classes() {
    let html = view! { <Chip label="x" style_class="custom" style="margin: 4px" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-chip p-component") && html.contains("margin: 4px"), "{html}");
}

#[test]
fn image_replaces_icon() {
    let html = view! { <Chip label="x" image="/a.png" alt="Аватар" icon=icon() /> }.to_html();
    assert!(
        html.contains("<img") && html.contains("p-chip-image") && html.contains(r#"src="/a.png""#) && html.contains(r#"alt="Аватар""#),
        "{html}"
    );
    assert!(!html.contains("p-chip-icon"), "{html}");
}

#[test]
fn icon_is_rendered_without_image() {
    let html = view! { <Chip label="x" icon=icon() /> }.to_html();
    assert!(html.contains("p-chip-icon") && !html.contains("p-chip-image"), "{html}");
}

#[test]
fn content_order_children_media_label_remove() {
    let html = view! {
        <Chip label="L" icon=icon() removable=true>
            <i>"перед"</i>
        </Chip>
    }
    .to_html();
    assert!(pos(&html, "<i>перед</i>") < pos(&html, "p-chip-icon"), "{html}");
    assert!(pos(&html, "p-chip-icon") < pos(&html, "p-chip-label"), "{html}");
    assert!(pos(&html, "p-chip-label") < pos(&html, "p-chip-remove-icon"), "{html}");
}

#[test]
fn remove_element_has_role_tabindex_and_default_label() {
    let html = view! { <Chip label="x" removable=true /> }.to_html();
    assert!(html.contains(r#"role="button""#) && html.contains(r#"tabindex="0""#) && html.contains(r#"aria-label="Remove""#), "{html}");
    assert!(html.contains("<svg"), "иконка по умолчанию: {html}");
}

#[test]
fn disabled_remove_element_is_not_focusable() {
    let html = view! { <Chip label="x" removable=true disabled=true /> }.to_html();
    assert!(html.contains(r#"tabindex="-1""#), "{html}");
}

#[test]
fn aria_label_comes_from_the_locale() {
    let owner = Owner::new();
    owner.with(|| {
        provide_prime_config(PrimeConfig { locale: Locale::default().with("aria.removeLabel", "Удалить"), ..Default::default() });
        let html = view! { <Chip label="x" removable=true /> }.to_html();
        assert!(html.contains(r#"aria-label="Удалить""#), "{html}");
    });
}

#[test]
fn custom_remove_icon_replaces_the_default() {
    let custom = ViewFn::from(|| view! { <b data-testid="custom">"x"</b> });
    let html = view! { <Chip label="x" removable=true remove_icon=custom /> }.to_html();
    assert!(html.contains(r#"<b data-testid="custom">x</b>"#) && !html.contains("<svg"), "{html}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = ChipPt {
        root: Pt::new().class("root-extra").attr("data-testid", "chip"),
        image: Pt::new().class("image-extra"),
        label: Pt::new().class("label-extra"),
        remove_icon: Pt::new().class("remove-extra"),
        ..Default::default()
    };
    let html = view! { <Chip label="x" image="/a.png" removable=true pt=pt /> }.to_html();
    for c in [
        "root-extra",
        "p-chip p-component",
        "image-extra",
        "p-chip-image",
        "label-extra",
        "p-chip-label",
        "remove-extra",
        "p-chip-remove-icon",
    ] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="chip""#), "{html}");
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (Option<&'static str>, Option<&'static str>, Option<&'static str>, bool, Option<&'static str>, Option<&'static str>);
    let cases: Vec<Case> = vec![
        (None, None, None, false, None, None),
        (Some("Amy"), None, None, false, None, None),
        (Some("Amy"), Some("/a.png"), Some("Фото"), false, Some("margin: 1px"), Some("a b")),
        (Some("x"), None, None, true, None, Some("custom")),
        (Some("я"), None, None, false, Some("color: red;"), None),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (label, image, alt, disabled, style, class) in cases {
            let o = |s: Option<&str>| s.map(str::to_owned);
            let fixed =
                view! { <Chip label=o(label) image=o(image) alt=o(alt) disabled=disabled style=o(style) style_class=o(class) /> }.to_html();
            let live = view! {
                <Chip
                    label=RwSignal::new(o(label))
                    image=RwSignal::new(o(image))
                    alt=RwSignal::new(o(alt))
                    disabled=RwSignal::new(disabled)
                    style=RwSignal::new(o(style))
                    style_class=RwSignal::new(o(class))
                />
            }
            .to_html();
            assert_eq!(fixed, live, "{label:?} {image:?} {disabled} {style:?} {class:?}");
        }
    });
}

#[test]
fn removable_chip_takes_the_reactive_path_with_identical_output_to_signals() {
    let owner = Owner::new();
    owner.with(|| {
        let fixed = view! { <Chip label="x" removable=true disabled=true /> }.to_html();
        let live = view! { <Chip label=RwSignal::new(Some("x".to_owned())) removable=RwSignal::new(true) disabled=RwSignal::new(true) /> }
            .to_html();
        assert_eq!(fixed, live);
    });
}
