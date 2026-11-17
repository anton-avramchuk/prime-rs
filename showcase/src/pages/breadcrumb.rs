//! Breadcrumb page: demos `basic` and `template` after PrimeNG's `apps/showcase/doc/breadcrumb`.
//!
//! The `router` demo is not ported (Angular Router). In `template` the available `prime-icons` icons are used instead of `sitemap`, `book`, `wallet`,
//! `shopping-bag`, `calculator`.

use super::Demo;
use leptos::prelude::*;
use prime_breadcrumb::{Breadcrumb, BreadcrumbItem};
use prime_icons::{CalendarIcon, EyeIcon, InfoCircleIcon, SearchIcon, StarIcon};

fn icon_item(icon: impl Fn() -> AnyView + Send + Sync + 'static) -> BreadcrumbItem {
    BreadcrumbItem::new().icon(ViewFn::from(icon)).url("#")
}

/// The `/breadcrumb` page.
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
