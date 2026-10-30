//! Страница ScrollTop: демонстрации `basic` и `element` по `apps/showcase/doc/scrolltop` PrimeNG.
//!
//! В `basic` вместо анимированного значка `pi-angle-down` — стрелка `AngleDownIcon`; в `element` вместо `icon="pi pi-arrow-up"`
//! — `ArrowUpIcon`.

use super::Demo;
use leptos::prelude::*;
use prime_button::ButtonSeverity;
use prime_icons::{AngleDownIcon, ArrowUpIcon};
use prime_scrolltop::{ScrollTop, ScrollTopTarget};

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Vitae et leo duis ut diam. Ultricies mi quis hendrerit dolor magna eget est lorem. Amet consectetur adipiscing elit ut. Nam libero justo laoreet sit amet. Pharetra massa massa ultricies mi quis hendrerit dolor magna. Est ultricies integer quis auctor elit sed vulputate. Consequat ac felis donec et. Tellus orci ac auctor augue mauris. Semper feugiat nibh sed pulvinar proin gravida hendrerit lectus a. Tincidunt arcu non sodales neque sodales. Metus aliquam eleifend mi in nulla posuere sollicitudin aliquam ultrices. Sodales ut etiam sit amet nisl purus. Cursus sit amet dictum sit amet. Tristique senectus et netus et malesuada fames ac turpis egestas. Et tortor consequat id porta nibh venenatis cras sed. Diam maecenas ultricies mi eget mauris. Eget egestas purus viverra accumsan in nisl nisi. Suscipit adipiscing bibendum est ultricies integer. Mattis aliquam faucibus purus in massa tempor nec.";

/// Страница `/scrolltop`.
#[component]
pub fn ScrollTopPage() -> impl IntoView {
    view! {
        <h2>"ScrollTop"</h2>
        <Demo name="basic" title="Basic">
            <div style="display: flex; flex-direction: column; align-items: center">
                <p>"Scroll down the page to display the ScrollTop component."</p>
                <span style="margin-bottom: 30rem">
                    <AngleDownIcon size=32 />
                </span>
                <ScrollTop />
            </div>
        </Demo>
        <Demo name="element" title="Element">
            <div data-testid="scroller" style="width: 250px; height: 200px; overflow: auto">
                <p>{LOREM}</p>
                <ScrollTop
                    target=ScrollTopTarget::Parent
                    threshold=100.0
                    severity=ButtonSeverity::Contrast
                    raised=true
                    icon=ViewFn::from(|| view! { <ArrowUpIcon /> })
                />
            </div>
        </Demo>
    }
}
