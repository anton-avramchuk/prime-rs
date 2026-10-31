//! Разметка Dialog (SSR): маска и окно выводятся в `body` на клиенте, поэтому сервер не отдаёт ничего.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_dialog::{Dialog, DialogPosition, DialogPt};

#[test]
fn hidden_by_default() {
    let html = Owner::new().with(|| view! { <Dialog header="Header"><p>"Body"</p></Dialog> }.to_html());
    assert!(!html.contains("p-dialog") && !html.contains("Body") && !html.contains("Header"), "{html}");
}

#[test]
fn visible_dialog_is_not_rendered_on_the_server() {
    let html = Owner::new().with(|| {
        let visible = RwSignal::new(true);
        view! {
            <Dialog visible=visible header="Top" position=DialogPosition::Top modal=true maximizable=true>
                <p>"Body"</p>
            </Dialog>
        }
        .to_html()
    });
    assert!(!html.contains("p-dialog") && !html.contains("p-overlay-mask") && !html.contains("Body"), "{html}");
}

#[test]
fn props_do_not_leak_while_hidden() {
    let html = Owner::new().with(|| {
        view! {
            <Dialog
                style_class="custom"
                mask_style_class="masked"
                content_style="opacity: 0.5"
                close_aria_label="Shut"
                breakpoints=vec![("575px".to_owned(), "90vw".to_owned())]
                pt=DialogPt { root: Pt::new().class("x"), ..Default::default() }
            >
                <p>"Body"</p>
            </Dialog>
        }
        .to_html()
    });
    for leaked in ["custom", "masked", "opacity", "Shut", "575px", "90vw"] {
        assert!(!html.contains(leaked), "{leaked}: {html}");
    }
}
