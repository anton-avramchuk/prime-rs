//! Структура DOM, классы, приоритет блоков, `pt` и паритет ветвей Avatar (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarPt, AvatarShape, AvatarSize};
use prime_core::Pt;

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <svg data-testid="i"></svg> })
}

#[test]
fn label_is_rendered_in_a_span() {
    let html = view! { <Avatar label="P" /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains("p-avatar p-component") && html.contains("p-avatar-label") && html.contains(">P<"), "{html}");
}

#[test]
fn defaults_give_square_normal_data_p() {
    let html = view! { <Avatar /> }.to_html();
    assert!(html.contains(r#"data-p="square normal""#), "{html}");
    for unwanted in ["p-avatar-image", "p-avatar-circle", "p-avatar-lg", "p-avatar-xl", "p-avatar-label", "p-avatar-icon"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn shape_and_size_classes_and_data_p() {
    let html = view! { <Avatar label="P" shape=AvatarShape::Circle size=AvatarSize::XLarge /> }.to_html();
    assert!(html.contains("p-avatar-circle") && html.contains("p-avatar-xl"), "{html}");
    assert!(html.contains(r#"data-p="circle xlarge""#), "{html}");
    let large = view! { <Avatar label="P" size=AvatarSize::Large /> }.to_html();
    assert!(large.contains("p-avatar-lg"), "{large}");
}

#[test]
fn image_sets_root_class_and_renders_img() {
    let html = view! { <Avatar image="/a.png" aria_label="Фото" /> }.to_html();
    assert!(html.contains("p-avatar-image"), "{html}");
    assert!(html.contains("<img") && html.contains(r#"src="/a.png""#), "{html}");
    assert_eq!(html.matches(r#"aria-label="Фото""#).count(), 2, "aria-label на корне и на img: {html}");
}

#[test]
fn aria_attributes_go_to_the_root() {
    let html = view! { <Avatar label="P" aria_label="Фото" aria_labelledby="id1" /> }.to_html();
    assert!(html.contains(r#"aria-label="Фото""#) && html.contains(r#"aria-labelledby="id1""#), "{html}");
}

#[test]
fn priority_label_then_icon_then_image() {
    let all = view! { <Avatar label="P" icon=icon() image="/a.png" /> }.to_html();
    assert!(all.contains("p-avatar-label") && !all.contains("p-avatar-icon") && !all.contains("<img"), "{all}");
    let icon_image = view! { <Avatar icon=icon() image="/a.png" /> }.to_html();
    assert!(icon_image.contains("p-avatar-icon") && !icon_image.contains("<img"), "{icon_image}");
    let image_only = view! { <Avatar image="/a.png" /> }.to_html();
    assert!(image_only.contains("<img") && !image_only.contains("p-avatar-icon"), "{image_only}");
}

#[test]
fn children_come_first() {
    let html = view! {
        <Avatar label="P">
            <i>"перед"</i>
        </Avatar>
    }
    .to_html();
    assert!(html.find("<i>перед</i>").unwrap() < html.find("p-avatar-label").unwrap(), "{html}");
}

#[test]
fn blocks_carry_the_same_data_p_as_the_root() {
    let html = view! { <Avatar label="P" shape=AvatarShape::Circle /> }.to_html();
    assert_eq!(html.matches(r#"data-p="circle normal""#).count(), 2, "{html}");
}

#[test]
fn style_class_and_style_keep_own_classes() {
    let html = view! { <Avatar label="P" style_class="custom" style="color: red" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-avatar p-component") && html.contains("color: red"), "{html}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = AvatarPt {
        root: Pt::new().class("root-extra").attr("data-testid", "av"),
        label: Pt::new().class("label-extra"),
        ..Default::default()
    };
    let html = view! { <Avatar label="P" pt=pt /> }.to_html();
    for c in ["root-extra", "p-avatar p-component", "label-extra", "p-avatar-label"] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="av""#), "{html}");
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (
        Option<&'static str>,
        Option<&'static str>,
        AvatarSize,
        AvatarShape,
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    );
    let cases: Vec<Case> = vec![
        (None, None, AvatarSize::Normal, AvatarShape::Square, None, None, None),
        (Some("P"), None, AvatarSize::XLarge, AvatarShape::Circle, None, None, None),
        (None, Some("/a.png"), AvatarSize::Large, AvatarShape::Circle, Some("Фото"), None, None),
        (Some("U"), None, AvatarSize::Normal, AvatarShape::Square, None, Some("color: red;"), Some("a b")),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (label, image, size, shape, aria, style, class) in cases {
            let o = |s: Option<&str>| s.map(str::to_owned);
            let fixed = view! { <Avatar label=o(label) image=o(image) size=size shape=shape aria_label=o(aria) style=o(style) style_class=o(class) /> }
                .to_html();
            let live = view! {
                <Avatar
                    label=RwSignal::new(o(label))
                    image=RwSignal::new(o(image))
                    size=RwSignal::new(size)
                    shape=RwSignal::new(shape)
                    aria_label=RwSignal::new(o(aria))
                    style=RwSignal::new(o(style))
                    style_class=RwSignal::new(o(class))
                />
            }
            .to_html();
            assert_eq!(fixed, live, "{label:?} {image:?} {size:?} {shape:?}");
        }
    });
}

#[test]
fn reactive_shape_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let shape = RwSignal::new(AvatarShape::Square);
        assert!(!view! { <Avatar label="P" shape=shape /> }.to_html().contains("p-avatar-circle"));
        shape.set(AvatarShape::Circle);
        assert!(view! { <Avatar label="P" shape=shape /> }.to_html().contains("p-avatar-circle"));
    });
}
