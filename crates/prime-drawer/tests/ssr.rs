//! Разметка Drawer (SSR): панель и маска выводятся в `body` на клиенте, поэтому сервер не отдаёт ничего.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_drawer::{Drawer, DrawerPosition, DrawerPt};

#[test]
fn hidden_by_default() {
    let html = Owner::new().with(|| view! { <Drawer header="Left Drawer"><p>"Body"</p></Drawer> }.to_html());
    assert!(!html.contains("p-drawer") && !html.contains("Body") && !html.contains("Left Drawer"), "{html}");
}

#[test]
fn visible_drawer_is_not_rendered_on_the_server() {
    let html = Owner::new().with(|| {
        let visible = RwSignal::new(true);
        view! {
            <Drawer visible=visible header="Right" position=DrawerPosition::Right modal=true block_scroll=true>
                <p>"Body"</p>
            </Drawer>
        }
        .to_html()
    });
    assert!(!html.contains("p-drawer") && !html.contains("p-overlay-mask") && !html.contains("Body"), "{html}");
}

#[test]
fn props_do_not_leak_while_hidden() {
    let html = Owner::new().with(|| {
        view! {
            <Drawer
                style_class="custom"
                mask_style="opacity: 0.5"
                aria_close_label="Shut"
                pt=DrawerPt { root: Pt::new().class("x"), ..Default::default() }
            >
                <p>"Body"</p>
            </Drawer>
        }
        .to_html()
    });
    assert!(!html.contains("custom") && !html.contains("Shut") && !html.contains("opacity"), "{html}");
}
