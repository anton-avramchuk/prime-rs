//! Разметка Popover (SSR): скрыта по умолчанию, свойства корня не выводятся до показа.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_popover::{Popover, PopoverPt, PopoverRef};

#[test]
fn hidden_by_default() {
    let html = Owner::new().with(|| view! { <Popover popover_ref=PopoverRef::new()><p>"Body"</p></Popover> }.to_html());
    assert!(!html.contains("p-popover") && !html.contains("Body") && !html.contains("dialog"), "{html}");
}

#[test]
fn props_do_not_leak_while_hidden() {
    let html = Owner::new().with(|| {
        view! {
            <Popover
                popover_ref=PopoverRef::new()
                id="op"
                aria_label="Share"
                style_class="custom"
                pt=PopoverPt { root: Pt::new().class("x"), ..Default::default() }
            >
                <p>"Body"</p>
            </Popover>
        }
        .to_html()
    });
    assert!(!html.contains("Share") && !html.contains("custom"), "{html}");
}
