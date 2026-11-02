//! BlockUI вне браузера: маска не выводится ни при каком `blocked`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_blockui::BlockUI;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn nothing_is_rendered_while_unblocked() {
    let html = render(|| view! { <BlockUI><span>"Loading"</span></BlockUI> }.to_html());
    assert!(!html.contains("p-blockui") && !html.contains("Loading"), "{html}");
}

#[test]
fn the_server_never_renders_the_mask() {
    let html = render(|| view! { <BlockUI blocked=true><span>"Loading"</span></BlockUI> }.to_html());
    assert!(!html.contains("p-blockui") && !html.contains("p-overlay-mask") && !html.contains("Loading"), "{html}");
    let signal = RwSignal::new(true);
    let reactive = render(|| view! { <BlockUI blocked=signal style_class="x" auto_z_index=false /> }.to_html());
    assert!(!reactive.contains("p-blockui") && !reactive.contains("aria-busy"), "{reactive}");
}
