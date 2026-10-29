//! Структура DOM, классы, `data-p`, радиокнопки, подписи, иконки, слоты и `pt` Rating (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_rating::{Rating, RatingIcon, RatingPt};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn default_structure_has_five_stars() {
    let html = view! { <Rating /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains(r#"class="p-rating""#) && html.contains(r#"data-p="""#), "{html}");
    assert_eq!(count(&html, "p-rating-option"), 5, "{html}");
    assert_eq!(count(&html, r#"type="radio""#), 5, "{html}");
    assert_eq!(count(&html, r#"class="p-hidden-accessible""#), 5, "{html}");
    assert_eq!(count(&html, r#"data-p-hidden-accessible="true""#), 5, "{html}");
    assert_eq!(count(&html, "p-rating-off-icon"), 5, "{html}");
    for unwanted in [
        "p-rating-option-active",
        "p-rating-on-icon",
        "p-readonly",
        "p-disabled",
        "p-invalid",
        "required",
        "readonly",
        "disabled",
        "style=",
    ] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    let option = html.find("p-rating-option").unwrap();
    let input = html.find("<input").unwrap();
    let icon = html.find("<svg").unwrap();
    assert!(option < input && input < icon, "{html}");
}

#[test]
fn value_marks_active_stars_and_on_icons() {
    let html = view! { <Rating value=RwSignal::new(Some(3)) /> }.to_html();
    assert_eq!(count(&html, "p-rating-option-active"), 3, "{html}");
    assert_eq!(count(&html, "p-rating-on-icon"), 3, "{html}");
    assert_eq!(count(&html, "p-rating-off-icon"), 2, "{html}");
    // `checked` только у радиокнопки звезды, равной значению (в SSR — атрибутом `checked`)
    assert_eq!(count(&html, "checked"), 1, "{html}");
    let checked = html.find("checked").unwrap();
    let input_start = html[..checked].rfind("<input").unwrap();
    assert!(html[input_start..checked].contains(r#"value="3""#) || html[checked..].contains(r#"value="3""#), "{html}");
    // `default_value` без сигнала
    let html = view! { <Rating default_value=2 /> }.to_html();
    assert_eq!(count(&html, "p-rating-option-active"), 2, "{html}");
}

#[test]
fn stars_count_and_reactive_stars() {
    assert_eq!(count(&view! { <Rating stars=10 /> }.to_html(), r#"type="radio""#), 10);
    let stars = RwSignal::new(Some(3));
    assert_eq!(count(&view! { <Rating stars=stars /> }.to_html(), r#"type="radio""#), 3);
    stars.set(Some(7));
    assert_eq!(count(&view! { <Rating stars=stars /> }.to_html(), r#"type="radio""#), 7);
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <Rating readonly=true /> }.to_html();
    assert!(html.contains(r#"class="p-rating p-readonly""#) && html.contains(r#"data-p="readonly""#), "{html}");
    assert_eq!(count(&html, "readonly"), 1 + 1 + 5, "класс, data-p и 5 радиокнопок: {html}");
    let html = view! { <Rating disabled=true /> }.to_html();
    assert!(html.contains(r#"class="p-rating p-disabled""#) && html.contains(r#"data-p="disabled""#), "{html}");
    assert_eq!(count(&html, "disabled"), 1 + 1 + 5, "{html}");
    let html = view! { <Rating readonly=true disabled=true /> }.to_html();
    assert!(html.contains(r#"data-p="readonly disabled""#) && html.contains("p-rating p-readonly p-disabled"), "{html}");
}

#[test]
fn invalid_marks_every_icon() {
    let html = view! { <Rating invalid=true default_value=2 /> }.to_html();
    assert_eq!(count(&html, "p-invalid"), 5, "{html}");
    assert!(!view! { <Rating /> }.to_html().contains("p-invalid"));
}

#[test]
fn radio_attributes_and_group_name() {
    let html = view! { <Rating name="grade" required=true /> }.to_html();
    assert_eq!(count(&html, r#"name="grade""#), 5, "{html}");
    assert_eq!(count(&html, "required"), 5, "{html}");
    for n in 1..=5 {
        assert!(html.contains(&format!(r#"value="{n}""#)), "{n}: {html}");
    }
}

#[test]
fn default_name_is_shared_inside_and_unique_between_instances() {
    let name_of = |html: &str| {
        let from = html.find(r#"name=""#).unwrap() + 6;
        html[from..from + html[from..].find('"').unwrap()].to_owned()
    };
    let (a, b) = (view! { <Rating /> }.to_html(), view! { <Rating /> }.to_html());
    let (na, nb) = (name_of(&a), name_of(&b));
    assert!(na.starts_with("pn_id_") && na.ends_with("_name"), "{na}");
    assert_ne!(na, nb);
    assert_eq!(count(&a, &format!(r#"name="{na}""#)), 5, "{a}");
}

#[test]
fn aria_labels_come_from_the_locale() {
    let html = view! { <Rating stars=3 /> }.to_html();
    for label in ["1 star", "2 stars", "3 stars"] {
        assert!(html.contains(&format!(r#"aria-label="{label}""#)), "{label}: {html}");
    }
}

#[test]
fn custom_icons_and_templates() {
    let on = ViewFn::from(|| view! { <i class="on-ic" /> });
    let off = ViewFn::from(|| view! { <i class="off-ic" /> });
    let html = view! { <Rating default_value=1 stars=3 on_icon=on off_icon=off /> }.to_html();
    assert_eq!(count(&html, r#"class="on-ic""#), 1, "{html}");
    assert_eq!(count(&html, r#"class="off-ic""#), 2, "{html}");
    assert!(html.contains("p-rating-icon p-rating-on-icon") && !html.contains("<svg"), "{html}");

    let on_tpl = Callback::new(|i: RatingIcon| view! { <b class={i.class}>{format!("on{}", i.star)}</b> }.into_any());
    let off_tpl = Callback::new(|i: RatingIcon| view! { <b class={i.class}>{format!("off{}", i.star)}</b> }.into_any());
    let html = view! { <Rating default_value=2 stars=3 on_icon_template=on_tpl off_icon_template=off_tpl invalid=true /> }.to_html();
    for expected in [
        r#"<b class="p-rating-icon p-rating-on-icon p-invalid">on1</b>"#,
        "on2",
        r#"<b class="p-rating-icon p-rating-off-icon p-invalid">off3</b>"#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("<svg"), "{html}");
}

#[test]
fn style_and_style_class() {
    let html = view! { <Rating style="width: 9rem" style_class="mine" /> }.to_html();
    assert!(html.contains(r#"style="width: 9rem;""#) && html.contains(r#"class="p-rating mine""#), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = RatingPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        option: Pt::new().class("custom").attr("data-o", "2"),
        hidden_option_input_container: Pt::new().class("c"),
        hidden_option_input: Pt::new().class("i"),
        on_icon: Pt::new().class("on-pt"),
        off_icon: Pt::new().class("off-pt"),
    };
    let html = view! { <Rating pt=pt default_value=1 stars=2 /> }.to_html();
    for expected in [
        "p-rating r",
        r#"data-r="1""#,
        "p-rating-option custom",
        r#"data-o="2""#,
        "p-hidden-accessible c",
        r#"class="i""#,
        "p-rating-on-icon on-pt",
        "p-rating-off-icon off-pt",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn reactive_props_render_like_static() {
    let readonly = RwSignal::new(false);
    let reactive = view! { <Rating readonly=readonly default_value=2 name="n" /> }.to_html();
    let fixed = view! { <Rating default_value=2 name="n" /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    readonly.set(true);
    assert!(view! { <Rating readonly=readonly /> }.to_html().contains("p-readonly"));
}

#[test]
fn input_has_no_empty_class_attribute() {
    let html = view! { <Rating /> }.to_html();
    assert!(!html.contains(r#"class="""#), "{}", &html[..300]);
}
