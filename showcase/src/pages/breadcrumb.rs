//! Страница Breadcrumb: демонстрации `basic` и `template` по `apps/showcase/doc/breadcrumb` PrimeNG.
//!
//! Демонстрация `router` не переносится (Angular Router). В `template` вместо `sitemap`, `book`, `wallet`,
//! `shopping-bag`, `calculator` взяты имеющиеся иконки `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_breadcrumb::{Breadcrumb, BreadcrumbItem};
use prime_icons::{CalendarIcon, EyeIcon, InfoCircleIcon, SearchIcon, StarIcon};

fn icon_item(icon: impl Fn() -> AnyView + Send + Sync + 'static) -> BreadcrumbItem {
    BreadcrumbItem::new().icon(ViewFn::from(icon)).url("#")
}

/// Страница `/breadcrumb`.
#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    let basic: Vec<BreadcrumbItem> =
        ["Electronics", "Computer", "Accessories", "Keyboard", "Wireless"].into_iter().map(|l| BreadcrumbItem::new().label(l)).collect();
    let icons = vec![
        icon_item(|| view! { <StarIcon /> }.into_any()),
        icon_item(|| view! { <CalendarIcon /> }.into_any()),
        icon_item(|| view! { <EyeIcon /> }.into_any()),
        icon_item(|| view! { <InfoCircleIcon /> }.into_any()),
        icon_item(|| view! { <SearchIcon /> }.into_any()),
    ];
    let template = Callback::new(|item: BreadcrumbItem| {
        let icon = item.icon.map(|i| i.run());
        view! { <a class="p-breadcrumb-item-link" href=item.url style="cursor: pointer">{icon}</a> }.into_any()
    });
    view! {
        <h2>"Breadcrumb"</h2>
        <Demo name="basic" title="Basic">
            <Breadcrumb model=basic home=BreadcrumbItem::new() />
        </Demo>
        <Demo name="template" title="Template">
            <Breadcrumb
                model=icons
                home=icon_item(|| view! { <StarIcon /> }.into_any())
                item=template
                separator=ViewFn::from(|| view! { " / " })
            />
        </Demo>
    }
}
