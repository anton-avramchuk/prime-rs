//! Разметка Dock (SSR): состав, положения, пункты, значок, бейдж, представление `item`, ссылка, `aria-*`, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_dock::{Dock, DockItem, DockPosition, DockPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn items() -> Vec<DockItem> {
    vec![
        DockItem::new().label("Finder").icon(ViewFn::from(|| view! { <i class="ic">"F"</i> })),
        DockItem::new().label("Terminal").url("/terminal").target("_blank").title("Open terminal"),
        DockItem::new().label("Trash").disabled(true),
    ]
}

#[test]
fn markup_has_root_container_list_and_items() {
    let html = render(|| view! { <Dock id="d" model=items() /> }.to_html());
    assert!(html.contains("p-dock p-component p-dock-bottom"), "{html}");
    assert!(html.contains("p-dock-list-container") && html.contains("id=\"d\"") && html.contains("class=\"p-dock-list\""), "{html}");
    assert!(
        html.contains("role=\"menu\"") && html.contains("tabindex=\"0\"") && html.contains("aria-orientation=\"horizontal\""),
        "{html}"
    );
    assert_eq!(html.matches("role=\"menuitem\"").count(), 3, "{html}");
    assert_eq!(html.matches("class=\"p-dock-item-content\"").count(), 3, "{html}");
    assert_eq!(html.matches("p-dock-item-link").count(), 3, "{html}");
    assert!(html.contains("id=\"d_0\"") && html.contains("id=\"d_2\""), "{html}");
    assert!(html.contains("aria-label=\"Finder\"") && html.contains("aria-label=\"Terminal\""), "{html}");
    assert!(!html.contains("aria-activedescendant") && !html.contains("p-dock-mobile"), "{html}");
}

#[test]
fn positions_set_class_and_orientation() {
    for (position, class, orientation) in [
        (DockPosition::Bottom, "p-dock-bottom", "horizontal"),
        (DockPosition::Top, "p-dock-top", "horizontal"),
        (DockPosition::Left, "p-dock-left", "vertical"),
        (DockPosition::Right, "p-dock-right", "vertical"),
    ] {
        let html = render(|| view! { <Dock model=items() position=position /> }.to_html());
        assert!(html.contains(class) && html.contains(&format!("aria-orientation=\"{orientation}\"")), "{class}: {html}");
    }
}

#[test]
fn items_carry_disabled_state_and_link_attributes() {
    let html = render(|| view! { <Dock model=items() /> }.to_html());
    assert_eq!(html.matches("data-p-disabled=\"true\"").count(), 1, "{html}");
    assert_eq!(html.matches("data-p-disabled=\"false\"").count(), 2, "{html}");
    assert!(html.contains("class=\"p-dock-item p-disabled\""), "{html}");
    assert_eq!(html.matches("data-p-focused=\"false\"").count(), 3, "{html}");
    assert!(html.contains("aria-disabled=\"true\"") && html.contains("aria-disabled=\"false\""), "{html}");
    assert!(
        html.contains("href=\"/terminal\"") && html.contains("target=\"_blank\"") && html.contains("title=\"Open terminal\""),
        "{html}"
    );
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 3, "ссылки скрыты от чтения с экрана: {html}");
}

#[test]
fn icon_is_wrapped_and_badge_rendered() {
    let model = vec![DockItem::new().label("Mail").icon(ViewFn::from(|| view! { <i class="ic">"M"</i> })).badge("5").icon_class("big")];
    let html = render(|| view! { <Dock model=model /> }.to_html());
    assert!(html.contains("p-dock-item-icon big") && html.contains("class=\"ic\""), "{html}");
    assert!(html.contains("p-badge") && html.contains(">5<"), "{html}");
    let plain = render(|| view! { <Dock model=items() /> }.to_html());
    assert!(!plain.contains("p-badge"), "{plain}");
    assert_eq!(plain.matches("p-dock-item-icon").count(), 1, "значок только у пункта с иконкой: {plain}");
}

#[test]
fn item_template_replaces_the_icon() {
    let html = render(|| {
        view! {
            <Dock
                model=items()
                item=Callback::new(|it: DockItem| view! { <b class="tpl">{it.label.unwrap_or_default()}</b> }.into_any())
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"tpl\"").count(), 3, "{html}");
    assert!(!html.contains("p-dock-item-icon") && !html.contains("class=\"ic\""), "{html}");
}

#[test]
fn item_style_class_and_ids() {
    let model = vec![DockItem::new().id("own").label("A").style_class("special").style("opacity: .5"), DockItem::new().label("B")];
    let html = render(|| view! { <Dock id="dk" model=model /> }.to_html());
    assert!(html.contains("id=\"own\"") && html.contains("id=\"dk_1\""), "{html}");
    assert!(html.contains("p-dock-item special") && html.contains("opacity: .5"), "{html}");
}

#[test]
fn aria_labels_style_and_pt() {
    let pt = DockPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        item_link: Pt::new().class("k"),
        ..DockPt::default()
    };
    let html = render(|| {
        view! { <Dock model=items() aria_label="Dock" aria_labelledby="lbl" style="width: 10rem" style_class="mine" pt=pt /> }.to_html()
    });
    assert!(html.contains("aria-label=\"Dock\"") && html.contains("aria-labelledby=\"lbl\""), "{html}");
    assert!(
        html.contains("p-dock p-component p-dock-bottom mine r") && html.contains("data-x=\"1\"") && html.contains("width: 10rem"),
        "{html}"
    );
    assert!(html.contains("p-dock-list l") && html.contains("p-dock-item i") && html.contains("p-dock-item-link k"), "{html}");
}

#[test]
fn empty_model_renders_an_empty_list() {
    let html = render(|| view! { <Dock /> }.to_html());
    assert!(html.contains("p-dock-list") && !html.contains("role=\"menuitem\""), "{html}");
}
