//! Страница Carousel: демонстрации `basic`, `responsive`, `circular` (автопрокрутка) и `vertical` из showcase PrimeNG
//! (`apps/showcase/doc/carousel`). Товары — локальный список, фотографии заменены цветными плитками; значки PrimeIcons
//! (`pi-heart`, `pi-shopping-cart`) заменены значками `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_carousel::{Carousel, CarouselOrientation, Responsive};
use prime_icons::{PlusIcon, StarIcon};
use prime_tag::{Tag, TagSeverity};

#[derive(Clone)]
struct Product {
    name: &'static str,
    price: u32,
    status: &'static str,
    color: &'static str,
}

fn products() -> Vec<Product> {
    vec![
        Product { name: "Bamboo Watch", price: 65, status: "INSTOCK", color: "#0ea5e9" },
        Product { name: "Black Watch", price: 72, status: "INSTOCK", color: "#334155" },
        Product { name: "Blue Band", price: 79, status: "LOWSTOCK", color: "#3b82f6" },
        Product { name: "Blue T-Shirt", price: 29, status: "INSTOCK", color: "#6366f1" },
        Product { name: "Bracelet", price: 15, status: "INSTOCK", color: "#ec4899" },
        Product { name: "Brown Purse", price: 120, status: "OUTOFSTOCK", color: "#92400e" },
        Product { name: "Chakra Bracelet", price: 32, status: "LOWSTOCK", color: "#f59e0b" },
        Product { name: "Galaxy Earrings", price: 34, status: "INSTOCK", color: "#8b5cf6" },
        Product { name: "Game Controller", price: 99, status: "LOWSTOCK", color: "#10b981" },
    ]
}

fn severity(status: &str) -> TagSeverity {
    match status {
        "INSTOCK" => TagSeverity::Success,
        "LOWSTOCK" => TagSeverity::Warn,
        _ => TagSeverity::Danger,
    }
}

fn card(product: Product) -> AnyView {
    view! {
        <div style="border: 1px solid var(--p-content-border-color); border-radius: 0.375rem; margin: 0.5rem; padding: 1rem">
            <div style="position: relative; margin-bottom: 1rem">
                <div style=format!("height: 8rem; border-radius: 0.375rem; background: {}", product.color)></div>
                <span style="position: absolute; left: 5px; top: 5px">
                    <Tag value=product.status severity=severity(product.status) />
                </span>
            </div>
            <div style="margin-bottom: 1rem; font-weight: 500">{product.name}</div>
            <div style="display: flex; justify-content: space-between; align-items: center">
                <div style="font-weight: 600; font-size: 1.25rem">{format!("${}", product.price)}</div>
                <span style="display: flex; gap: 0.5rem">
                    <Button severity=ButtonSeverity::Secondary outlined=true icon=ViewFn::from(|| view! { <StarIcon /> }) />
                    <Button icon=ViewFn::from(|| view! { <PlusIcon /> }) />
                </span>
            </div>
        </div>
    }
    .into_any()
}

fn responsive() -> Vec<Responsive> {
    vec![
        Responsive { breakpoint: "1400px".into(), num_visible: 2, num_scroll: 1 },
        Responsive { breakpoint: "1199px".into(), num_visible: 3, num_scroll: 1 },
        Responsive { breakpoint: "767px".into(), num_visible: 2, num_scroll: 1 },
        Responsive { breakpoint: "575px".into(), num_visible: 1, num_scroll: 1 },
    ]
}

/// Страница `/carousel`.
#[component]
pub fn CarouselPage() -> impl IntoView {
    view! {
        <h2>"Carousel"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%" data-testid="basic">
                <Carousel value=products() num_visible=3 num_scroll=3 responsive_options=responsive() item=Callback::new(card) />
            </div>
        </Demo>
        <Demo name="responsive" title="Responsive">
            <div style="width: 100%" data-testid="responsive">
                <Carousel value=products() num_visible=3 num_scroll=1 responsive_options=responsive() item=Callback::new(card) />
            </div>
        </Demo>
        <Demo name="circular" title="Circular and Autoplay">
            <div style="width: 100%" data-testid="circular">
                <Carousel
                    value=products()
                    num_visible=3
                    num_scroll=1
                    circular=true
                    autoplay_interval=3000
                    responsive_options=responsive()
                    item=Callback::new(card)
                />
            </div>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <div style="width: 100%" data-testid="vertical">
                <Carousel
                    value=products()
                    num_visible=1
                    num_scroll=1
                    orientation=CarouselOrientation::Vertical
                    vertical_view_port_height="330px"
                    content_class="p-carousel-demo-vertical"
                    item=Callback::new(card)
                />
            </div>
        </Demo>
    }
}
