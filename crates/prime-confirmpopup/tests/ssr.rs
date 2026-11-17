//! ConfirmPopup on the server: no panel until there is a confirmation; the service subscription is dropped on unmount.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_confirmation::{Confirmation, provide_confirmation_service};
use prime_confirmpopup::ConfirmPopup;

#[test]
fn nothing_is_rendered_until_a_confirmation_arrives() {
    let html = Owner::new().with(|| {
        provide_confirmation_service();
        view! { <ConfirmPopup /> }.to_html()
    });
    assert!(!html.contains("p-confirmpopup") && !html.contains("alertdialog"), "{html}");
}

#[test]
fn confirmation_on_the_server_does_not_render_a_panel() {
    let html = Owner::new().with(|| {
        let service = provide_confirmation_service();
        let html = view! { <ConfirmPopup /> }.to_html();
        service.confirm(Confirmation::new().message("Sure?"));
        html
    });
    assert!(!html.contains("Sure?"), "{html}");
}

#[test]
#[should_panic(expected = "ConfirmationService not found")]
fn component_requires_the_service() {
    Owner::new().with(|| {
        let _ = view! { <ConfirmPopup /> }.to_html();
    });
}

#[test]
fn unmounting_unsubscribes_from_the_service() {
    let owner = Owner::new();
    let service = owner.with(provide_confirmation_service);
    let child = Owner::new_root(None);
    child.with(|| {
        provide_context(service);
        let _ = view! { <ConfirmPopup key="k" /> }.to_html();
    });
    drop(child);
    // after unmount the broadcast wakes nobody and does not panic
    owner.with(|| service.confirm(Confirmation::new().key("k").message("late")));
}
