//! Страница MegaMenu: демонстрации `basic`, `vertical`, `command` и `template` из showcase PrimeNG
//! (`apps/showcase/doc/megamenu`; `router` не переносится). Значки PrimeIcons заменены значками `prime-icons` там, где аналог есть.

use super::Demo;
use leptos::prelude::*;
use prime_icons::{HomeIcon, RefreshIcon, SearchIcon, StarIcon, ThLargeIcon};
use prime_megamenu::{MegaMenu, MegaMenuItem, MegaMenuItemContext, MegaMenuOrientation, MegaMenuSubmenu};
use prime_menu::{MenuCommand, MenuItem};
use prime_toast::{MessageService, Toast, ToastMessage, ToastSeverity, provide_message_service};

fn icon<F, V>(f: F) -> ViewFn
where
    F: Fn() -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    ViewFn::from(move || f().into_any())
}

fn sub(label: &str, items: &[&str]) -> MegaMenuSubmenu {
    MegaMenuSubmenu::new(label).items(items.iter().map(|l| MenuItem::new().label(*l)).collect())
}

fn items() -> Vec<MegaMenuItem> {
    vec![
        MegaMenuItem::new().label("Furniture").icon(icon(|| view! { <ThLargeIcon /> })).columns(vec![
            vec![sub("Living Room", &["Accessories", "Armchair", "Coffee Table", "Couch", "TV Stand"])],
            vec![sub("Kitchen", &["Bar stool", "Chair", "Table"]), sub("Bathroom", &["Accessories"])],
            vec![sub("Bedroom", &["Bed", "Chaise lounge", "Cupboard", "Dresser", "Wardrobe"])],
            vec![sub("Office", &["Bookcase", "Cabinet", "Chair", "Desk", "Executive Chair"])],
        ]),
        MegaMenuItem::new().label("Electronics").icon(icon(|| view! { <SearchIcon /> })).columns(vec![
            vec![sub("Computer", &["Monitor", "Mouse", "Notebook", "Keyboard", "Printer", "Storage"])],
            vec![sub("Home Theater", &["Projector", "Speakers", "TVs"])],
            vec![sub("Gaming", &["Accessories", "Console", "PC", "Video Games"])],
            vec![sub("Appliances", &["Coffee Machine", "Fridge", "Oven", "Vaccum Cleaner", "Washing Machine"])],
        ]),
        MegaMenuItem::new().label("Sports").icon(icon(|| view! { <StarIcon /> })).columns(vec![
            vec![sub("Football", &["Kits", "Shoes", "Shorts", "Training"])],
            vec![sub("Running", &["Accessories", "Shoes", "T-Shirts", "Shorts"])],
            vec![sub("Swimming", &["Kickboard", "Nose Clip", "Swimsuits", "Paddles"])],
            vec![sub("Tennis", &["Balls", "Rackets", "Shoes", "Training"])],
        ]),
    ]
}

fn say(toast: MessageService, severity: ToastSeverity, summary: &'static str, detail: &'static str) -> Callback<MenuCommand> {
    Callback::new(move |_| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

fn command_items(toast: MessageService) -> Vec<MegaMenuItem> {
    let leaf = |label: &str, cmd| MenuItem::new().label(label).command(cmd);
    vec![
        MegaMenuItem::new().label("Furniture").icon(icon(|| view! { <ThLargeIcon /> })).columns(vec![
            vec![MegaMenuSubmenu::new("Living Room").items(vec![
                leaf("Accessories", say(toast, ToastSeverity::Info, "Accessories", "Accessories selected")),
                leaf("Armchair", say(toast, ToastSeverity::Success, "Armchair", "Armchair added")),
            ])],
            vec![
                MegaMenuSubmenu::new("Kitchen").items(vec![leaf("Chair", say(toast, ToastSeverity::Warn, "Chair", "Chair out of stock"))]),
            ],
        ]),
        MegaMenuItem::from(MenuItem::new().label("Refresh").icon(icon(|| view! { <RefreshIcon /> })).command(say(
            toast,
            ToastSeverity::Success,
            "Refreshed",
            "Catalog refreshed",
        ))),
    ]
}

#[component]
fn CommandDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <Toast />
        <MegaMenu id="command_menu" model=command_items(toast) />
    }
}

fn template_item(ctx: MegaMenuItemContext) -> AnyView {
    let MegaMenuItemContext { item, has_submenu, root } = ctx;
    view! {
        <a style="display: flex; align-items: center; padding: 0.5rem 0.75rem; cursor: pointer; gap: 0.5rem" data-testid="template-item">
            {item.icon.clone().map(|i| i.run())}
            <span>{item.label.clone().unwrap_or_default()}</span>
            {(has_submenu && root).then(|| view! { <span style="margin-left: auto">"▾"</span> })}
        </a>
    }
    .into_any()
}

/// Страница `/megamenu`.
#[component]
pub fn MegaMenuPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <div style="width: 100%"><MegaMenu id="basic_menu" model=items() /></div>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <MegaMenu id="vertical_menu" model=items() orientation=MegaMenuOrientation::Vertical style="width: 15rem" />
        </Demo>
        <Demo name="command" title="Command">
            <div style="width: 100%"><CommandDemo /></div>
        </Demo>
        <Demo name="template" title="Template">
            <div style="width: 100%">
                <MegaMenu
                    id="template_menu"
                    model=items()
                    item=Callback::new(template_item)
                    style="border-radius: 3rem"
                    start=ViewFn::from(|| view! { <span data-testid="megamenu-start" style="margin-right: 1rem; display: inline-flex"><HomeIcon /></span> })
                    end=ViewFn::from(|| view! { <span data-testid="megamenu-end">"v1"</span> })
                />
            </div>
        </Demo>
    }
}
