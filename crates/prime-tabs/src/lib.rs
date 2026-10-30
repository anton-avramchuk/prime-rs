//! Tabs — порт `p-tabs`, `p-tablist`, `p-tab`, `p-tabpanels`, `p-tabpanel` из PrimeNG 21.1.10.
//!
//! Составной компонент из пяти частей, как в PrimeNG: [`Tabs`] хранит активное значение, [`TabList`] содержит вкладки,
//! полосу-индикатор и кнопки прокрутки, [`Tab`] — вкладка, [`TabPanels`] и [`TabPanel`] — панели. Вкладка и панель связаны
//! одинаковым `value`; части общаются через контекст Leptos.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_tabs::{Tab, TabList, TabPanel, TabPanels, Tabs};
//!
//! let html = Owner::new().with(|| view! {
//!     <Tabs default_value="1">
//!         <TabList>
//!             <Tab value="0">"Header I"</Tab>
//!             <Tab value="1">"Header II"</Tab>
//!         </TabList>
//!         <TabPanels>
//!             <TabPanel value="0"><p>"Content I"</p></TabPanel>
//!             <TabPanel value="1"><p>"Content II"</p></TabPanel>
//!         </TabPanels>
//!     </Tabs>
//! }
//! .to_html());
//! assert!(html.contains("p-tab-active") && html.contains(r#"aria-selected="true""#) && html.contains("Content II"));
//! ```

mod state;

use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use prime_core::{ClassNames, OptProp, Prop, Pt, use_prime_config};
use prime_icons::{ChevronLeftIcon, ChevronRightIcon};
use std::sync::Arc;

pub use state::{
    KeyAction, ink_style, is_active, key_action, next_scroll, next_visible, panel_classes, panel_id, prev_scroll, prev_visible,
    root_classes, should_render, tab_classes, tab_id, tabindex_for, wrap_target,
};

/// Pass-through частей [`TabList`].
#[derive(Clone, Debug, Default)]
pub struct TabListPt {
    /// Корень.
    pub root: Pt,
    /// Прокручиваемая область.
    pub content: Pt,
    /// Список вкладок.
    pub tab_list: Pt,
    /// Полоса-индикатор.
    pub active_bar: Pt,
    /// Кнопка «назад».
    pub prev_button: Pt,
    /// Кнопка «вперёд».
    pub next_button: Pt,
}

/// Общее состояние группы, доступное частям.
#[derive(Clone)]
struct TabsCtx {
    id: String,
    value: RwSignal<Option<String>>,
    lazy: Prop<bool>,
    select_on_focus: Prop<bool>,
    show_navigators: Prop<bool>,
    tabindex: Prop<i32>,
    /// Ширина и смещение полосы-индикатора.
    ink: RwSignal<Option<(f64, f64)>>,
}

const NO_TABS: &str = "TabList, Tab, TabPanels и TabPanel должны находиться внутри Tabs";

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// Группа вкладок: хранит активное значение.
///
/// `value` — двусторонний сигнал; без него состояние собственное, начальное — `default_value`.
#[component]
pub fn Tabs(
    /// Основа для `id` вкладок и панелей; по умолчанию уникальная.
    #[prop(optional, into)]
    id: Option<String>,
    /// Двусторонний сигнал активного значения.
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Начальное значение, когда `value` не передан.
    #[prop(optional, into)]
    default_value: Option<String>,
    /// Горизонтальная прокрутка списка вкладок.
    #[prop(optional, into)]
    scrollable: Prop<bool>,
    /// Выводить содержимое панелей только после первой активации.
    #[prop(optional, into)]
    lazy: Prop<bool>,
    /// Активировать вкладку при получении фокуса.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Кнопки прокрутки (по умолчанию да).
    #[prop(default = Prop::Static(true), into)]
    show_navigators: Prop<bool>,
    /// `tabindex` активной вкладки и кнопок прокрутки (по умолчанию 0).
    #[prop(default = Prop::Static(0), into)]
    tabindex: Prop<i32>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through корня.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Список вкладок и панели.
    children: Children,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let id = id.unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    provide_context(TabsCtx {
        id: id.clone(),
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        lazy,
        select_on_focus,
        show_navigators,
        tabindex,
        ink: RwSignal::new(None),
    });
    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = ClassNames::new();
            cn.add(root_classes(scrollable.get()));
            style_class.with(|c| cn.add_user(c.as_deref()));
            pt.merge_class(&cn.finish())
        })
    };
    let style = {
        let pt = pt.clone();
        Signal::derive(move || {
            let merged = pt.merge_style_parts(&[&style.get().unwrap_or_default()]);
            (!merged.is_empty()).then_some(merged)
        })
    };
    view! {
        <div class={class} style={style} id={id} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Список вкладок с полосой-индикатором и кнопками прокрутки.
#[component]
pub fn TabList(
    /// Значок кнопки «назад» вместо `ChevronLeftIcon`.
    #[prop(optional, into)]
    prev_icon: Option<ViewFn>,
    /// Значок кнопки «вперёд» вместо `ChevronRightIcon`.
    #[prop(optional, into)]
    next_icon: Option<ViewFn>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<TabListPt>,
    /// Вкладки.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<TabsCtx>().expect(NO_TABS);
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let (prev_label, next_label) = (locale.text("aria.previous").to_owned(), locale.text("aria.next").to_owned());
    let root_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<html::Div>::new();
    let list_ref = NodeRef::<html::Div>::new();
    let nav = Nav { root_ref, content_ref, list_ref, prev: RwSignal::new(false), next: RwSignal::new(false) };
    #[cfg(target_arch = "wasm32")]
    client::install(&ctx, nav);
    let ink = ctx.ink;
    let show = ctx.show_navigators.clone();
    let tabindex = ctx.tabindex.clone();

    let prev_button = {
        let (pt, show, tabindex, label, icon) = (pt.clone(), show.clone(), tabindex.clone(), prev_label, prev_icon);
        move || {
            (show.get() && nav.prev.get()).then(|| {
                let icon = match icon.clone() {
                    Some(icon) => icon.run(),
                    None => view! { <ChevronLeftIcon /> }.into_any(),
                };
                view! {
                    <button
                        type="button"
                        class={pt.prev_button.merge_class("p-tablist-prev-button p-tablist-nav-button")}
                        aria-label={label.clone()}
                        tabindex={tabindex.get()}
                        data-pc-group-section="navigator"
                        on:click=move |_| nav.scroll_prev()
                        {..pt.prev_button.attrs()}
                    >
                        {icon}
                    </button>
                }
            })
        }
    };
    let next_button = {
        let (pt, show, tabindex, label, icon) = (pt.clone(), show, tabindex, next_label, next_icon);
        move || {
            (show.get() && nav.next.get()).then(|| {
                let icon = match icon.clone() {
                    Some(icon) => icon.run(),
                    None => view! { <ChevronRightIcon /> }.into_any(),
                };
                view! {
                    <button
                        type="button"
                        class={pt.next_button.merge_class("p-tablist-next-button p-tablist-nav-button")}
                        aria-label={label.clone()}
                        tabindex={tabindex.get()}
                        data-pc-group-section="navigator"
                        on:click=move |_| nav.scroll_next()
                        {..pt.next_button.attrs()}
                    >
                        {icon}
                    </button>
                }
            })
        }
    };
    let bar_style = {
        let pt = pt.clone();
        move || {
            let base = ink.get().map(|(width, left)| ink_style(width, left)).unwrap_or_default();
            let merged = pt.active_bar.merge_style(&base);
            (!merged.is_empty()).then_some(merged)
        }
    };

    view! {
        <div node_ref={root_ref} class={pt.root.merge_class("p-tablist")} {..pt.root.attrs()}>
            {prev_button}
            <div
                node_ref={content_ref}
                class={pt.content.merge_class("p-tablist-content p-tablist-viewport")}
                on:scroll=move |_| nav.update_buttons()
                {..pt.content.attrs()}
            >
                <div node_ref={list_ref} class={pt.tab_list.merge_class("p-tablist-tab-list")} role="tablist" {..pt.tab_list.attrs()}>
                    {children()}
                    <span
                        role="presentation"
                        class={pt.active_bar.merge_class("p-tablist-active-bar")}
                        data-pc-section="activebar"
                        style={bar_style}
                        {..pt.active_bar.attrs()}
                    ></span>
                </div>
            </div>
            {next_button}
        </div>
    }
}

/// Ссылки на элементы списка и видимость кнопок прокрутки.
#[derive(Clone, Copy)]
struct Nav {
    root_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    list_ref: NodeRef<html::Div>,
    prev: RwSignal<bool>,
    next: RwSignal<bool>,
}

impl Nav {
    fn rtl(content: &web_sys::Element) -> bool {
        web_sys::window()
            .and_then(|w| w.get_computed_style(content).ok().flatten())
            .and_then(|style| style.get_property_value("direction").ok())
            .is_some_and(|direction| direction == "rtl")
    }

    /// Обновляет видимость кнопок по текущей прокрутке.
    fn update_buttons(&self) {
        let (Some(content), Some(root)) = (self.content_ref.get_untracked(), self.root_ref.get_untracked()) else { return };
        let scroll_left = f64::from(content.scroll_left());
        let (scroll_width, width) = (f64::from(content.scroll_width()), f64::from(content.client_width()));
        self.prev.set(prev_visible(scroll_left));
        self.next.set(next_visible(f64::from(root.offset_width()), f64::from(content.offset_width()), scroll_left, scroll_width, width));
    }

    fn buttons_width(&self) -> f64 {
        let Some(root) = self.root_ref.get_untracked() else { return 0.0 };
        let sum = |selector: &str| {
            root.query_selector(selector)
                .ok()
                .flatten()
                .and_then(|el| leptos::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el).ok())
                .map_or(0.0, |el| f64::from(el.client_width()))
        };
        sum(".p-tablist-prev-button") + sum(".p-tablist-next-button")
    }

    fn scroll_prev(&self) {
        let Some(content) = self.content_ref.get_untracked() else { return };
        let target = prev_scroll(f64::from(content.scroll_left()), f64::from(content.client_width()));
        content.set_scroll_left(if Self::rtl(&content) { -target } else { target } as i32);
    }

    fn scroll_next(&self) {
        let Some(content) = self.content_ref.get_untracked() else { return };
        let target = next_scroll(
            f64::from(content.scroll_left()),
            f64::from(content.client_width()),
            self.buttons_width(),
            f64::from(content.scroll_width()),
        );
        content.set_scroll_left(if Self::rtl(&content) { -target } else { target } as i32);
    }
}

/// Вкладка.
#[component]
pub fn Tab(
    /// Значение, связывающее вкладку с панелью.
    #[prop(into)]
    value: String,
    /// Недоступна.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Pass-through вкладки.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Заголовок.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<TabsCtx>().expect(NO_TABS);
    let pt = Arc::new(pt.unwrap_or_default());
    let element = NodeRef::<html::Div>::new();
    let active = {
        let (current, value) = (ctx.value, value.clone());
        Signal::derive(move || current.with(|c| is_active(c.as_deref(), &value)))
    };
    let activate = {
        let (current, value, disabled) = (ctx.value, value.clone(), disabled.clone());
        move || {
            if !disabled.get_untracked() {
                current.set(Some(value.clone()));
            }
        }
    };
    let class = {
        let (pt, disabled) = (pt.clone(), disabled.clone());
        move || pt.merge_class(tab_classes(active.get(), disabled.get()))
    };
    let tabindex = {
        let (disabled, tabindex) = (disabled.clone(), ctx.tabindex.clone());
        move || tabindex_for(disabled.get(), active.get(), tabindex.get())
    };
    let (tab_id_value, panel_id_value) = (tab_id(&ctx.id, &value), panel_id(&ctx.id, &value));
    let on_focus = {
        let (select_on_focus, activate) = (ctx.select_on_focus.clone(), activate.clone());
        move |_| {
            if select_on_focus.get_untracked() {
                activate();
            }
        }
    };
    let on_keydown = {
        let activate = activate.clone();
        move |ev: web_sys::KeyboardEvent| {
            if let Some(action) = key_action(&ev.code()) {
                if action == KeyAction::Activate {
                    activate();
                } else {
                    navigate(element, action);
                }
                ev.prevent_default();
            }
            ev.stop_propagation();
        }
    };
    let on_click = {
        let activate = activate.clone();
        move |_| activate()
    };
    let data_disabled = {
        let disabled = disabled.clone();
        move || disabled.get().then_some("true")
    };
    let aria_disabled = {
        let disabled = disabled.clone();
        move || bool_attr(disabled.get())
    };

    view! {
        <div
            node_ref={element}
            class={class}
            id={tab_id_value}
            role="tab"
            aria-controls={panel_id_value}
            aria-selected={move || bool_attr(active.get())}
            aria-disabled={aria_disabled}
            data-p-disabled={data_disabled}
            data-p-active={move || bool_attr(active.get())}
            data-pc-name="tab"
            tabindex={tabindex}
            on:click={on_click}
            on:focus={on_focus}
            on:keydown={on_keydown}
            {..pt.attrs()}
        >
            {children()}
        </div>
    }
}

/// Переносит фокус (или прокручивает) к вкладке, выбранной действием клавиши.
fn navigate(element: NodeRef<html::Div>, action: KeyAction) {
    use leptos::wasm_bindgen::JsCast;
    let Some(current) = element.get_untracked() else { return };
    let Some(list) = current.parent_element() else { return };
    let tabs: Vec<web_sys::HtmlElement> = {
        let children = list.children();
        (0..children.length())
            .filter_map(|i| children.item(i))
            .filter(|el| el.get_attribute("role").as_deref() == Some("tab"))
            .filter_map(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
            .collect()
    };
    let enabled: Vec<bool> = tabs.iter().map(|tab| !tab.has_attribute("data-p-disabled")).collect();
    let Some(index) = tabs.iter().position(|tab| tab.is_same_node(Some(&current))) else { return };
    let Some(target) = wrap_target(index, &enabled, action).map(|i| &tabs[i]) else { return };
    if matches!(action, KeyAction::ScrollFirst | KeyAction::ScrollLast) {
        scroll_in_view(target);
    } else {
        let _ = target.focus();
        scroll_in_view(target);
    }
}

fn scroll_in_view(element: &web_sys::HtmlElement) {
    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
    element.scroll_into_view_with_scroll_into_view_options(&options);
}

/// Контейнер панелей.
#[component]
pub fn TabPanels(
    /// Pass-through контейнера.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Панели.
    children: Children,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    view! {
        <div class={pt.merge_class("p-tabpanels")} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Панель вкладки.
#[component]
pub fn TabPanel(
    /// Значение, связывающее панель с вкладкой.
    #[prop(into)]
    value: String,
    /// Выводить содержимое только после первой активации (в дополнение к `lazy` у `Tabs`).
    #[prop(optional, into)]
    lazy: Prop<bool>,
    /// Pass-through панели.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Содержимое.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<TabsCtx>().expect(NO_TABS);
    let pt = pt.unwrap_or_default();
    let active = {
        let (current, value) = (ctx.value, value.clone());
        Signal::derive(move || current.with(|c| is_active(c.as_deref(), &value)))
    };
    let has_rendered = StoredValue::new(false);
    let rendered = {
        let group_lazy = ctx.lazy.clone();
        Memo::new(move |_| {
            let render = should_render(group_lazy.get() || lazy.get(), has_rendered.get_value(), active.get());
            if render && active.get() {
                has_rendered.set_value(true);
            }
            render
        })
    };
    let class = {
        let pt = pt.clone();
        move || pt.merge_class(panel_classes(active.get()))
    };
    view! {
        <div
            class={class}
            id={panel_id(&ctx.id, &value)}
            role="tabpanel"
            aria-labelledby={tab_id(&ctx.id, &value)}
            data-p-active={move || bool_attr(active.get())}
            hidden={move || !active.get()}
            {..pt.attrs()}
        >
            {move || rendered.get().then(|| children())}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
mod client {
    //! Клиентская часть TabList: полоса-индикатор, кнопки прокрутки и наблюдатель за размерами.

    use super::{Nav, TabsCtx};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::prelude::Closure;
    use leptos::web_sys;
    use std::time::Duration;

    /// Измеряет активную вкладку и обновляет полосу.
    fn update_ink(ctx: &TabsCtx, nav: Nav) {
        let Some(list) = nav.list_ref.get_untracked() else { return };
        let active = list.query_selector(r#"[role="tab"][data-p-active="true"]"#).ok().flatten();
        let measured = active.map(|tab| {
            let (tab_rect, list_rect) = (tab.get_bounding_client_rect(), list.get_bounding_client_rect());
            let width = tab.dyn_into::<web_sys::HtmlElement>().map_or(0.0, |el| f64::from(el.offset_width()));
            (width, tab_rect.left() - list_rect.left())
        });
        if ctx.ink.try_get_untracked().is_some() {
            ctx.ink.set(measured);
        }
    }

    pub fn install(ctx: &TabsCtx, nav: Nav) {
        let ink_ctx = ctx.clone();
        Effect::new(move |_| {
            ink_ctx.value.track();
            let inner = ink_ctx.clone();
            set_timeout(move || update_ink(&inner, nav), Duration::ZERO);
        });
        let (show, observer_ctx) = (ctx.show_navigators.clone(), ctx.clone());
        let observer = StoredValue::new_local(None::<(web_sys::ResizeObserver, Closure<dyn FnMut()>)>);
        Effect::new(move |_| {
            if observer.with_value(Option::is_some) {
                return;
            }
            let (Some(root), Some(list)) = (nav.root_ref.get(), nav.list_ref.get()) else { return };
            if show.get_untracked() {
                nav.update_buttons();
            }
            let (ctx, show) = (observer_ctx.clone(), show.clone());
            let closure = Closure::<dyn FnMut()>::new(move || {
                if show.get_untracked() {
                    nav.update_buttons();
                }
                update_ink(&ctx, nav);
            });
            if let Ok(resize) = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref()) {
                resize.observe(&root);
                resize.observe(&list);
                observer.set_value(Some((resize, closure)));
            }
        });
        on_cleanup(move || {
            if let Some(Some((resize, _closure))) = observer.try_update_value(Option::take) {
                resize.disconnect();
            }
        });
    }
}
