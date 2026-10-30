//! Breadcrumb — порт `p-breadcrumb` из PrimeNG 21.1.10.
//!
//! Корень `nav.p-breadcrumb` (вместо хоста `<p-breadcrumb>`), список `ol.p-breadcrumb-list`, пункт «домой»,
//! пункты модели и разделители. Вместо `MenuItem` с полями маршрутизатора Angular — [`BreadcrumbItem`] с `url`:
//! навигацию SPA ведёт `command`. Шаблоны `item` и `separator` — колбэки.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_breadcrumb::{Breadcrumb, BreadcrumbItem};
//!
//! let model = vec![BreadcrumbItem::new().label("Electronics"), BreadcrumbItem::new().label("Computer")];
//! let html = view! { <Breadcrumb model=model home=BreadcrumbItem::new() /> }.to_html();
//! assert!(html.contains("p-breadcrumb-home-item") && html.contains("Computer"));
//! ```

mod state;

use leptos::prelude::*;
use prime_badge::{Badge, BadgePt};
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{ChevronRightIcon, HomeIcon};
use std::sync::Arc;

pub use state::{BreadcrumbClick, BreadcrumbItem, ClickPlan, click_plan};

/// Pass-through частей Breadcrumb.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BreadcrumbPt {
    /// Корневой `nav`.
    pub root: Pt,
    /// Список `ol`.
    pub list: Pt,
    /// Пункт «домой».
    pub home_item: Pt,
    /// Пункт модели.
    pub item: Pt,
    /// Ссылка пункта.
    pub item_link: Pt,
    /// Значок пункта.
    pub item_icon: Pt,
    /// Подпись пункта.
    pub item_label: Pt,
    /// Разделитель.
    pub separator: Pt,
    /// Значок разделителя.
    pub separator_icon: Pt,
    /// Бейдж пункта.
    pub pc_badge: BadgePt,
}

/// Путь по иерархии страниц.
///
/// Каждое свойство принимает значение или сигнал; сигнал `model` или `home` перестраивает список целиком.
#[component]
pub fn Breadcrumb(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<BreadcrumbItem>>,
    /// Пункт «домой».
    #[prop(optional, into)]
    home: OptProp<BreadcrumbItem>,
    /// `aria-label` пункта «домой».
    #[prop(optional, into)]
    home_aria_label: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Шаблон пункта (заменяет содержимое `li`).
    #[prop(optional, into)]
    item: Option<Callback<BreadcrumbItem, AnyView>>,
    /// Шаблон разделителя (заменяет значок).
    #[prop(optional, into)]
    separator: Option<ViewFn>,
    /// Вызывается при нажатии на неотключённый пункт после его `command`.
    #[prop(optional, into)]
    on_item_click: Option<Callback<BreadcrumbClick>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<BreadcrumbPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let attrs = pt.root.attrs();
    let class = {
        let (pt, style_class) = (pt.clone(), style_class);
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-breadcrumb p-component");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        move || style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
    };
    let list_class = pt.list.merge_class("p-breadcrumb-list");
    let list_style = pt.list.has_style().then(|| pt.list.merge_style(""));
    let list_attrs = pt.list.attrs();
    let ctx = Ctx { pt: pt.clone(), item, separator, on_item_click };
    let rows = move || {
        let home = home.get().filter(|h| h.visible);
        let items: Vec<BreadcrumbItem> = model.get().into_iter().filter(|i| i.visible).collect();
        let mut out: Vec<AnyView> = Vec::new();
        if let Some(h) = &home {
            out.push(item_view(&ctx, h, true, home_aria_label.get()));
            if !items.is_empty() {
                out.push(separator_view(&ctx));
            }
        }
        let last = items.len().saturating_sub(1);
        for (i, it) in items.iter().enumerate() {
            out.push(item_view(&ctx, it, false, None));
            if i < last {
                out.push(separator_view(&ctx));
            }
        }
        out
    };
    view! {
        <nav class=class style={style} {..attrs}>
            <ol class=list_class style={list_style} {..list_attrs}>{rows}</ol>
        </nav>
    }
}

/// Общие для разметки пунктов данные.
#[derive(Clone)]
struct Ctx {
    pt: Arc<BreadcrumbPt>,
    item: Option<Callback<BreadcrumbItem, AnyView>>,
    separator: Option<ViewFn>,
    on_item_click: Option<Callback<BreadcrumbClick>>,
}

fn part_class(pt: &Pt, base: &str, extra: Option<&str>) -> String {
    let mut cn = prime_core::ClassNames::new();
    cn.add(base);
    cn.add_user(extra);
    cn.add_user(pt.extra_class());
    cn.finish()
}

fn separator_view(ctx: &Ctx) -> AnyView {
    let pt = &ctx.pt;
    let inner = match &ctx.separator {
        Some(s) => s.run(),
        None => view! { <ChevronRightIcon /> }.into_any(),
    };
    view! {
        <li class=pt.separator.merge_class("p-breadcrumb-separator") style=pt.separator.has_style().then(|| pt.separator.merge_style("")) {..pt.separator.attrs()}>
            {inner}
        </li>
    }
    .into_any()
}

fn item_view(ctx: &Ctx, it: &BreadcrumbItem, home: bool, aria_label: Option<String>) -> AnyView {
    let pt = &ctx.pt;
    let (part, base) = if home { (&pt.home_item, "p-breadcrumb-home-item") } else { (&pt.item, "p-breadcrumb-item") };
    let mut cn = prime_core::ClassNames::new();
    cn.add(base);
    cn.add_if(!home && it.disabled, "p-disabled");
    cn.add_user(it.style_class.as_deref());
    cn.add_user(part.extra_class());
    let class = cn.finish();
    let style = match (&it.style, part.has_style()) {
        (None, false) => None,
        (st, _) => Some(part.merge_style(st.as_deref().unwrap_or(""))),
    };
    let body = match ctx.item {
        Some(t) => t.run(it.clone()),
        None => link_view(ctx, it, home, aria_label),
    };
    view! {
        <li id=it.id.clone() class=class style={style} {..part.attrs()}>
            {body}
        </li>
    }
    .into_any()
}

fn link_view(ctx: &Ctx, it: &BreadcrumbItem, home: bool, aria_label: Option<String>) -> AnyView {
    let pt = &ctx.pt;
    let tabindex = (!it.disabled).then(|| it.tabindex.clone().unwrap_or_else(|| "0".to_owned()));
    let icon = match (&it.icon, home) {
        (Some(i), _) => Some(
            view! { <span class=part_class(&pt.item_icon, "p-breadcrumb-item-icon", None) {..pt.item_icon.attrs()}>{i.run()}</span> }
                .into_any(),
        ),
        (None, true) => Some(view! { <HomeIcon class="p-breadcrumb-item-icon" /> }.into_any()),
        (None, false) => None,
    };
    let label = it.label.clone().filter(|l| !l.is_empty()).map(|l| {
        view! { <span class=part_class(&pt.item_label, "p-breadcrumb-item-label", None) {..pt.item_label.attrs()}>{l}</span> }
    });
    let badge = it.badge.clone().map(|b| view! { <Badge value=b pt=pt.pc_badge.clone() /> });
    let click = {
        let (it, on_item_click) = (it.clone(), ctx.on_item_click);
        move |event: web_sys::MouseEvent| {
            let plan = click_plan(it.disabled, it.url.is_some());
            if plan.prevent_default {
                event.prevent_default();
            }
            if plan.run_commands {
                let payload = BreadcrumbClick { event, item: it.clone() };
                if let Some(cmd) = it.command {
                    cmd.run(payload.clone());
                }
                if let Some(cb) = on_item_click {
                    cb.run(payload);
                }
            }
        }
    };
    view! {
        <a
            href=it.url.clone()
            class=part_class(&pt.item_link, "p-breadcrumb-item-link", it.link_class.as_deref())
            target=it.target.clone()
            title=it.title.clone()
            tabindex=tabindex
            aria-label=aria_label
            on:click={click}
            {..pt.item_link.attrs()}
        >
            {icon}
            {label}
            {badge}
        </a>
    }
    .into_any()
}
