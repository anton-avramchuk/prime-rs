//! Структура DOM, классы, `style`, `pt` и паритет ветвей AvatarGroup (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_avatargroup::AvatarGroup;
use prime_core::Pt;

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn renders_div_with_classes_and_children_in_order() {
    let html = view! { <AvatarGroup><i>"1"</i><i>"2"</i><i>"3"</i></AvatarGroup> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains(r#"class="p-avatar-group p-component""#), "{html}");
    let (a, b, c) = (html.find(">1<").unwrap(), html.find(">2<").unwrap(), html.find(">3<").unwrap());
    assert!(a < b && b < c, "{html}");
    assert!(!html.contains("style="), "{html}");
}

#[test]
fn renders_without_children() {
    let html = view! { <AvatarGroup /> }.to_html();
    assert!(html.contains("p-avatar-group p-component") && html.ends_with("</div>"), "{html}");
}

#[test]
fn style_class_and_style_are_added() {
    let html = view! { <AvatarGroup style_class="mine" style="gap: 1px" /> }.to_html();
    assert!(html.contains("p-avatar-group p-component mine") && html.contains("gap: 1px"), "{html}");
}

#[test]
fn pt_root_class_style_and_attr_are_added() {
    let pt = Pt::new().class("custom").style("margin: 0").attr("data-x", "y");
    let html = view! { <AvatarGroup style_class="mine" pt=pt /> }.to_html();
    assert!(
        html.contains("p-avatar-group p-component mine custom") && html.contains("margin: 0") && html.contains(r#"data-x="y""#),
        "{html}"
    );
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <AvatarGroup style_class="x" style="gap: 1px"><b>"k"</b></AvatarGroup> }.to_html();
    let reactive = view! {
        <AvatarGroup style_class=RwSignal::new(Some("x".to_owned())) style=RwSignal::new(Some("gap: 1px".to_owned()))>
            <b>"k"</b>
        </AvatarGroup>
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
