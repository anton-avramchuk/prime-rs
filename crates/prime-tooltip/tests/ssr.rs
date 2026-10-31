//! Директива `tooltip` в SSR: не вызывается и не меняет разметку элемента.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_button::Button;
use prime_inputtext::InputText;
use prime_tooltip::{TooltipEvent, TooltipOptions, TooltipPosition, tooltip};

fn options() -> TooltipOptions {
    TooltipOptions::text("Enter your username").position(TooltipPosition::Top).event(TooltipEvent::Both).show_delay(300).auto_hide(false)
}

#[test]
fn plain_element_markup_is_unchanged() {
    let with = view! { <span id="a" class="x" use:tooltip=options()>"text"</span> }.to_html();
    let without = view! { <span id="a" class="x">"text"</span> }.to_html();
    assert_eq!(with, without, "{with}");
    assert!(!with.contains("tooltip") && !with.contains("Enter your username"));
}

#[test]
fn component_roots_are_unchanged() {
    let button = Owner::new().with(|| {
        let with = view! { <Button label="B" use:tooltip=options() /> }.to_html();
        let without = view! { <Button label="B" /> }.to_html();
        (with, without)
    });
    assert_eq!(button.0, button.1);
    let input = Owner::new().with(|| {
        let with = view! { <InputText placeholder="p" use:tooltip=TooltipOptions::html("<b>x</b>") /> }.to_html();
        let without = view! { <InputText placeholder="p" /> }.to_html();
        (with, without)
    });
    assert_eq!(input.0, input.1);
    assert!(!input.0.contains("<b>x</b>"));
}

#[test]
fn builder_sets_every_option() {
    let o = TooltipOptions::text("t")
        .position(TooltipPosition::Left)
        .event(TooltipEvent::Focus)
        .show_delay(1)
        .hide_delay(2)
        .life(3)
        .offset(4, 5)
        .auto_hide(false)
        .fit_content(false)
        .hide_on_escape(false)
        .show_on_ellipsis(true)
        .disabled(true)
        .position_style("fixed")
        .style_class("wide")
        .z_index(9)
        .id("tip");
    assert_eq!((o.position, o.event), (TooltipPosition::Left, TooltipEvent::Focus));
    assert_eq!((o.show_delay, o.hide_delay, o.life), (Some(1), Some(2), Some(3)));
    assert_eq!((o.position_top, o.position_left), (4, 5));
    assert!(!o.auto_hide && !o.fit_content && !o.hide_on_escape && o.show_on_ellipsis);
    assert!(Owner::new().with(|| o.disabled.get_untracked()));
    assert_eq!(
        (o.position_style.as_deref(), o.style_class.as_deref(), o.z_index, o.id.as_deref()),
        (Some("fixed"), Some("wide"), Some(9), Some("tip"))
    );

    let d = TooltipOptions::text("t");
    assert_eq!((d.position, d.event), (TooltipPosition::Right, TooltipEvent::Hover));
    assert!(d.auto_hide && d.fit_content && d.hide_on_escape && !d.show_on_ellipsis && d.z_index.is_none());
}
