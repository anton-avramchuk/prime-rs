//! Accordion — порт `p-accordion` из PrimeNG 21.1.10.
//!
//! Составной компонент из четырёх частей, как в PrimeNG: [`Accordion`] хранит список активных значений,
//! [`AccordionPanel`] задаёт `value` панели, [`AccordionHeader`] — заголовок-кнопка, [`AccordionContent`] — раскрывающееся содержимое.
//! Части общаются через контекст Leptos и не принимают значений друг от друга.
//!
//! Значение — двусторонний сигнал `RwSignal<Vec<String>>`: в одиночном режиме в нём 0 или 1 значение, при `multiple` — любое число.
//! Раскрытие и сворачивание анимируют классы `p-collapsible-enter-active`/`p-collapsible-leave-active` через
//! [`prime_core::collapsible`], как у Panel и Fieldset.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel};
//!
//! let html = Owner::new().with(|| view! {
//!     <Accordion default_value=vec!["0".to_owned()]>
//!         <AccordionPanel value="0">
//!             <AccordionHeader>"Header I"</AccordionHeader>
//!             <AccordionContent><p>"Content I"</p></AccordionContent>
//!         </AccordionPanel>
//!         <AccordionPanel value="1">
//!             <AccordionHeader>"Header II"</AccordionHeader>
//!             <AccordionContent><p>"Content II"</p></AccordionContent>
//!         </AccordionPanel>
//!     </Accordion>
//! }
//! .to_html());
//! assert!(html.contains("p-accordionpanel-active") && html.contains(r#"aria-expanded="true""#) && html.contains("Header II"));
//! ```

mod state;

use leptos::either::EitherOf3;
use leptos::prelude::*;
use prime_core::collapsible::{CollapseIds, hidden_style};
use prime_core::{ClassNames, OptProp, Prop, Pt};
use prime_icons::{ChevronDownIcon, ChevronUpIcon};
use std::sync::Arc;

pub use state::{Nav, nav_target, toggle_value};

/// Событие `on_open` и `on_close`: исходное событие и значение панели.
#[derive(Clone, Debug)]
pub struct AccordionToggle {
    /// Исходное событие `click`.
    pub original_event: web_sys::Event,
    /// Значение панели (`value` у [`AccordionPanel`]).
    pub index: String,
}

/// Pass-through частей Accordion.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AccordionPt {
    /// Корневой элемент (`div.p-accordion`).
    pub root: Pt,
    /// Панель (`div.p-accordionpanel`).
    pub panel: Pt,
    /// Заголовок (`div.p-accordionheader`).
    pub header: Pt,
    /// Иконка переключателя: применяется только класс.
    pub toggle_icon: Pt,
    /// Контейнер содержимого (`div.p-accordioncontent`).
    pub content_container: Pt,
    /// Обёртка содержимого (`div.p-accordioncontent-wrapper`).
    pub content_wrapper: Pt,
    /// Содержимое (`div.p-accordioncontent-content`).
    pub content: Pt,
}

/// Общее состояние группы, доступное панелям.
#[derive(Clone)]
struct AccordionCtx {
    id: String,
    value: RwSignal<Vec<String>>,
    multiple: Prop<bool>,
    select_on_focus: Prop<bool>,
    on_open: Option<Callback<AccordionToggle>>,
    on_close: Option<Callback<AccordionToggle>>,
    pt: Arc<AccordionPt>,
}

/// Состояние панели, доступное заголовку и содержимому.
#[derive(Clone)]
struct PanelCtx {
    value: String,
    disabled: Prop<bool>,
    active: Signal<bool>,
}

const NO_ACCORDION: &str = "AccordionPanel, AccordionHeader и AccordionContent должны находиться внутри Accordion";
const NO_PANEL: &str = "AccordionHeader и AccordionContent должны находиться внутри AccordionPanel";

fn header_id(ctx: &AccordionCtx, value: &str) -> String {
    format!("{}_accordionheader_{value}", ctx.id)
}

fn content_id(ctx: &AccordionCtx, value: &str) -> String {
    format!("{}_accordioncontent_{value}", ctx.id)
}

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// Группа вкладок: хранит список активных значений.
///
/// `value` — двусторонний сигнал; без него состояние собственное, начальное — `default_value`.
#[component]
pub fn Accordion(
    /// Атрибут `id`-основа для идентификаторов заголовков и содержимого. По умолчанию уникальный.
    #[prop(optional, into)]
    id: Option<String>,
    /// Двусторонний сигнал активных значений.
    #[prop(optional)]
    value: Option<RwSignal<Vec<String>>>,
    /// Начальные значения, когда `value` не передан.
    #[prop(optional)]
    default_value: Vec<String>,
    /// Можно ли открыть несколько панелей одновременно.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Переключать панель при получении фокуса заголовком.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается, когда панель открыта кликом по заголовку.
    #[prop(optional, into)]
    on_open: Option<Callback<AccordionToggle>>,
    /// Вызывается, когда панель закрыта кликом по заголовку.
    #[prop(optional, into)]
    on_close: Option<Callback<AccordionToggle>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<AccordionPt>,
    /// Панели.
    children: Children,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let ctx = AccordionCtx {
        id: CollapseIds::new(id.as_deref()).root,
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        multiple,
        select_on_focus,
        on_open,
        on_close,
        pt: pt.clone(),
    };
    provide_context(ctx);
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordion p-component");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        pt.root.has_style().then(|| pt.root.merge_style(""))
    };
    view! {
        <div class={class} style={style} {..pt.root.attrs()}>
            {children()}
        </div>
    }
}

/// Панель: задаёт `value` и состояние активности для заголовка и содержимого.
#[component]
pub fn AccordionPanel(
    /// Уникальное значение панели в группе.
    #[prop(into)]
    value: String,
    /// Панель недоступна для взаимодействия.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Дополнительный класс панели.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Заголовок и содержимое.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let active = {
        let (list, own) = (ctx.value, value.clone());
        Signal::derive(move || list.with(|v| v.contains(&own)))
    };
    provide_context(PanelCtx { value, disabled: disabled.clone(), active });
    let pt = ctx.pt.clone();
    let class = {
        let (pt, disabled) = (pt.clone(), disabled.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordionpanel").add_if(active.get(), "p-accordionpanel-active").add_if(disabled.get(), "p-disabled");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.panel.extra_class());
            cn.finish_cow()
        }
    };
    let data_disabled = move || bool_attr(disabled.get());
    let data_active = move || bool_attr(active.get());
    view! {
        <div class={class} data-p-disabled={data_disabled} data-p-active={data_active} {..pt.panel.attrs()}>
            {children()}
        </div>
    }
}

/// Заголовок панели: кнопка с иконкой переключателя.
#[component]
pub fn AccordionHeader(
    /// Дополнительный класс заголовка.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Слот иконки переключателя; получает `active` и заменяет шеврон.
    #[prop(optional, into)]
    toggle_icon_template: Option<Callback<bool, AnyView>>,
    /// Содержимое заголовка.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let panel = use_context::<PanelCtx>().expect(NO_PANEL);
    let (active, disabled) = (panel.active, panel.disabled.clone());
    let (id, controls) = (header_id(&ctx, &panel.value), content_id(&ctx, &panel.value));
    let pt = ctx.pt.clone();

    let toggle = {
        let (ctx, value, disabled) = (ctx.clone(), panel.value.clone(), disabled.clone());
        move |e: web_sys::Event, emit: bool| {
            if disabled.get_untracked() {
                return;
            }
            let was_active = active.get_untracked();
            let multiple = ctx.multiple.get_untracked();
            ctx.value.update(|list| *list = toggle_value(list, &value, multiple));
            let is_active = active.get_untracked();
            if !emit {
                return;
            }
            let event = AccordionToggle { original_event: e, index: value.clone() };
            match (was_active, is_active) {
                (false, true) => ctx.on_open.inspect(|cb| cb.run(event)),
                (true, false) => ctx.on_close.inspect(|cb| cb.run(event)),
                _ => None,
            };
        }
    };
    let click = {
        let toggle = toggle.clone();
        move |e: web_sys::MouseEvent| toggle(e.into(), true)
    };
    let focus = {
        let (toggle, ctx, disabled) = (toggle.clone(), ctx.clone(), disabled.clone());
        move |e: web_sys::FocusEvent| {
            if !disabled.get_untracked() && ctx.select_on_focus.get_untracked() {
                toggle(e.into(), false);
            }
        }
    };
    let keydown = move |e: web_sys::KeyboardEvent| {
        let nav = match e.code().as_str() {
            "ArrowDown" => Some(Nav::Next),
            "ArrowUp" => Some(Nav::Prev),
            "Home" => Some(Nav::First),
            "End" => Some(Nav::Last),
            "Enter" | "NumpadEnter" | "Space" => {
                toggle(e.clone().into(), false);
                e.prevent_default();
                return;
            }
            _ => return,
        };
        if let Some(nav) = nav {
            focus_neighbor(&e, nav);
            e.prevent_default();
        }
    };

    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordionheader");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.header.extra_class());
            cn.finish_cow()
        }
    };
    let style = pt.header.merge_style("user-select: none");
    let icon_class = pt.toggle_icon.merge_class("p-accordionheader-toggle-icon").into_owned();
    let icon = move || {
        let is_active = active.get();
        match toggle_icon_template {
            Some(template) => EitherOf3::A(template.run(is_active)),
            None if is_active => EitherOf3::B(view! { <ChevronUpIcon class=icon_class.clone() /> }),
            None => EitherOf3::C(view! { <ChevronDownIcon class=icon_class.clone() /> }),
        }
    };
    let expanded = move || bool_attr(active.get());
    let aria_disabled = {
        let disabled = disabled.clone();
        move || bool_attr(disabled.get())
    };
    let tabindex = {
        let disabled = disabled.clone();
        move || if disabled.get() { "-1" } else { "0" }
    };
    let data_disabled = move || bool_attr(disabled.get());
    let data_active = move || bool_attr(active.get());
    let data = move || active.get().then_some("active");
    view! {
        <div
            id={id}
            class={class}
            style={style}
            role="button"
            aria-expanded={expanded}
            aria-controls={controls}
            aria-disabled={aria_disabled}
            tabindex={tabindex}
            data-p-active={data_active}
            data-p-disabled={data_disabled}
            data-p={data}
            on:click={click}
            on:focus={focus}
            on:keydown={keydown}
            {..pt.header.attrs()}
        >
            {children()}
            {icon}
        </div>
    }
}

/// Раскрывающееся содержимое панели.
#[component]
pub fn AccordionContent(
    /// Дополнительный класс контейнера содержимого.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Содержимое.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>().expect(NO_ACCORDION);
    let panel = use_context::<PanelCtx>().expect(NO_PANEL);
    let active = panel.active;
    let (id, labelled_by) = (content_id(&ctx, &panel.value), header_id(&ctx, &panel.value));
    let pt = ctx.pt.clone();

    // `NodeRef` нужен только клиенту (анимация и `tabindex`): при SSR его нет, а `()` — пустой атрибут вместо `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        prime_core::collapsible::bind_region_with(region, move || !active.get(), None);
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {};

    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-accordioncontent");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.content_container.extra_class());
            cn.finish_cow()
        }
    };
    let data_active = move || bool_attr(active.get());
    let hidden = hidden_style(!active.get_untracked());
    view! {
        <div id={id} class={class} role="region" data-p-active={data_active} aria-labelledby={labelled_by} {..pt.content_container.attrs()}>
            <div {..region} class="p-motion" style={hidden}>
                <div class=pt.content_wrapper.merge_class("p-accordioncontent-wrapper") {..pt.content_wrapper.attrs()}>
                    <div class=pt.content.merge_class("p-accordioncontent-content") {..pt.content.attrs()}>{children()}</div>
                </div>
            </div>
        </div>
    }
}

/// Переносит фокус на заголовок соседней не отключённой панели (только браузер).
fn focus_neighbor(e: &web_sys::KeyboardEvent, nav: Nav) {
    use leptos::wasm_bindgen::JsCast;
    let Some(header) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
    let Some(panel) = header.closest(".p-accordionpanel").ok().flatten() else { return };
    let Some(parent) = panel.parent_element() else { return };
    let children = parent.children();
    let panels: Vec<web_sys::Element> =
        (0..children.length()).filter_map(|i| children.item(i)).filter(|c| c.class_list().contains("p-accordionpanel")).collect();
    let Some(from) = panels.iter().position(|p| *p == panel) else { return };
    let disabled: Vec<bool> = panels.iter().map(|p| p.get_attribute("data-p-disabled").as_deref() == Some("true")).collect();
    let Some(to) = nav_target(&disabled, from, nav) else { return };
    if let Ok(Some(target)) = panels[to].query_selector(".p-accordionheader")
        && let Ok(target) = target.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = target.focus();
    }
}
