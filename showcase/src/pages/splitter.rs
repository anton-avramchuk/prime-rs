//! Страница Splitter: демонстрации `horizontal`, `vertical`, `size`, `nested` по `apps/showcase/doc/splitter` PrimeNG.
//!
//! Карточка демонстрации у нас flex, а у PrimeNG блочная, поэтому корню задана ширина 100 %. Утилитные классы Tailwind из исходных демонстраций заменены встроенными стилями; во вложенном Splitter признак
//! вложенности (`nested`) задан явно.

use super::Demo;
use leptos::prelude::*;
use prime_splitter::{Splitter, SplitterLayout};

const CENTER: &str = "display: flex; align-items: center; justify-content: center; height: 100%";

fn cell(text: &'static str) -> ViewFn {
    ViewFn::from(move || view! { <div style=CENTER>{text}</div> })
}

/// Страница `/splitter`.
#[component]
pub fn SplitterPage() -> impl IntoView {
    let inner = ViewFn::from(|| {
        view! {
            <Splitter
                layout=SplitterLayout::Vertical
                panel_sizes=vec![50.0, 50.0]
                nested=true
                panels=vec![
                    ViewFn::from(|| view! { <div style="flex-grow: 1; display: flex; align-items: center; justify-content: center">"Panel 2"</div> }),
                    ViewFn::from(|| {
                        view! {
                            <Splitter panel_sizes=vec![20.0, 80.0] panels=vec![cell("Panel 3"), cell("Panel 4")] />
                        }
                    }),
                ]
            />
        }
    });
    view! {
        <h2>"Splitter"</h2>
        <Demo name="horizontal" title="Horizontal">
            <Splitter style="width: 100%; height: 300px" panels=vec![cell("Panel 1"), cell("Panel 2")] />
        </Demo>
        <Demo name="vertical" title="Vertical">
            <Splitter
                style="width: 100%; height: 300px"
                layout=SplitterLayout::Vertical
                panel_sizes=vec![50.0, 50.0]
                panels=vec![cell("Panel 1"), cell("Panel 2")]
            />
        </Demo>
        <Demo name="size" title="Size">
            <Splitter style="width: 100%; height: 300px" panel_sizes=vec![25.0, 75.0] panels=vec![cell("Panel 1"), cell("Panel 2")] />
        </Demo>
        <Demo name="nested" title="Nested">
            <Splitter
                style="width: 100%; height: 300px"
                panel_sizes=vec![20.0, 80.0]
                min_sizes=vec![10.0, 0.0]
                nested=true
                panels=vec![cell("Panel 1"), inner]
            />
        </Demo>
    }
}
