//! Страница Skeleton: демонстрации `shapes`, `card`, `list` из showcase PrimeNG (`apps/showcase/doc/skeleton`).
//!
//! Демонстрация `datatable` не перенесена (нужна таблица), утилиты Tailwind заменены инлайн-стилями.

use super::Demo;
use leptos::prelude::*;
use prime_skeleton::{Skeleton, SkeletonShape};

const MB2: &str = "margin-bottom: 0.5rem";

/// Страница `/skeleton`.
#[component]
pub fn SkeletonPage() -> impl IntoView {
    let list_item = || {
        view! {
            <li style="margin-bottom: 1rem">
                <div style="display: flex">
                    <Skeleton shape=SkeletonShape::Circle size="4rem" pt=prime_core::Pt::new().style("margin-right: 0.5rem") />
                    <div style="flex: 1; align-self: center">
                        <Skeleton width="100%" pt=prime_core::Pt::new().style(MB2) />
                        <Skeleton width="75%" />
                    </div>
                </div>
            </li>
        }
    };
    view! {
        <h2>"Skeleton"</h2>
        <Demo name="shapes" title="Shapes">
            <div style="display: flex; flex-wrap: wrap; width: 100%">
                <div style="width: 50%; padding: 1rem; box-sizing: border-box">
                    <h5>"Rectangle"</h5>
                    <Skeleton pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="10rem" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="5rem" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton height="2rem" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="10rem" height="4rem" />
                </div>
                <div style="width: 50%; padding: 1rem; box-sizing: border-box">
                    <h5>"Rounded"</h5>
                    <Skeleton border_radius="16px" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="10rem" border_radius="16px" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="5rem" border_radius="16px" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton height="2rem" border_radius="16px" pt=prime_core::Pt::new().style(MB2) />
                    <Skeleton width="10rem" height="4rem" border_radius="16px" />
                </div>
                <div style="width: 50%; padding: 1rem; box-sizing: border-box">
                    <h5>"Square"</h5>
                    <div style="display: flex; align-items: flex-end; gap: 0.5rem">
                        <Skeleton size="2rem" />
                        <Skeleton size="3rem" />
                        <Skeleton size="4rem" />
                        <Skeleton size="5rem" />
                    </div>
                </div>
                <div style="width: 50%; padding: 1rem; box-sizing: border-box">
                    <h5>"Circle"</h5>
                    <div style="display: flex; align-items: flex-end; gap: 0.5rem">
                        <Skeleton shape=SkeletonShape::Circle size="2rem" />
                        <Skeleton shape=SkeletonShape::Circle size="3rem" />
                        <Skeleton shape=SkeletonShape::Circle size="4rem" />
                        <Skeleton shape=SkeletonShape::Circle size="5rem" />
                    </div>
                </div>
            </div>
        </Demo>
        <Demo name="card" title="Card">
            <div style="width: 100%; border: 1px solid var(--p-content-border-color); border-radius: 6px; padding: 1.5rem; box-sizing: border-box">
                <div style="display: flex; margin-bottom: 1rem">
                    <Skeleton shape=SkeletonShape::Circle size="4rem" pt=prime_core::Pt::new().style("margin-right: 0.5rem") />
                    <div>
                        <Skeleton width="10rem" pt=prime_core::Pt::new().style(MB2) />
                        <Skeleton width="5rem" pt=prime_core::Pt::new().style(MB2) />
                        <Skeleton height=".5rem" />
                    </div>
                </div>
                <Skeleton width="100%" height="150px" />
                <div style="display: flex; justify-content: space-between; margin-top: 1rem">
                    <Skeleton width="4rem" height="2rem" />
                    <Skeleton width="4rem" height="2rem" />
                </div>
            </div>
        </Demo>
        <Demo name="list" title="List">
            <div style="width: 100%; border: 1px solid var(--p-content-border-color); border-radius: 6px; padding: 1.5rem; box-sizing: border-box">
                <ul style="margin: 0; padding: 0; list-style: none">{list_item()} {list_item()} {list_item()} {list_item()}</ul>
            </div>
        </Demo>
    }
}
