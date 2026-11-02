//! Stepper — порт `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item` и `p-stepper-separator`
//! из PrimeNG 21.1.10.
//!
//! Составной компонент, как Tabs: [`Stepper`] хранит активное значение, [`StepList`] и [`Step`] — список шагов с разделителями,
//! [`StepPanels`] и [`StepPanel`] — панели, [`StepItem`] оборачивает шаг и панель в вертикальной раскладке. Шаг и панель связаны
//! одинаковым `value` (число); части общаются через контекст Leptos. Активное значение — двусторонний сигнал `value`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_stepper::{Step, StepList, StepPanel, StepPanels, Stepper};
//!
//! let html = Owner::new().with(|| view! {
//!     <Stepper default_value=2u32>
//!         <StepList>
//!             <Step value=1u32>"Header I"</Step>
//!             <Step value=2u32>"Header II"</Step>
//!         </StepList>
//!         <StepPanels>
//!             <StepPanel value=1u32>"Content I"</StepPanel>
//!             <StepPanel value=2u32>"Content II"</StepPanel>
//!         </StepPanels>
//!     </Stepper>
//! }
//! .to_html());
//! assert!(html.contains("p-step-active") && html.contains("Content II") && !html.contains("Content I<"));
//! ```

mod state;

use leptos::prelude::*;
use leptos::tachys::reactive_graph::OwnedView;
use prime_core::collapsible::hidden_style;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{is_active, item_classes, panel_classes, panel_id, root_classes, separator_after, step_classes, step_disabled, step_id};

/// Состояние шага или панели, передаваемое `content` (аналог контекста шаблона `content` PrimeNG).
#[derive(Clone, Copy)]
pub struct StepContext {
    /// Значение шага или панели.
    pub value: u32,
    /// Активен ли шаг (панель).
    pub active: Signal<bool>,
    /// Делает активным шаг с указанным значением (`activateCallback`); работает и в линейном режиме.
    pub activate: Callback<u32>,
}

/// Pass-through частей [`Step`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepPt {
    /// Корень шага.
    pub root: Pt,
    /// Кнопка-заголовок.
    pub header: Pt,
    /// Номер.
    pub number: Pt,
    /// Заголовок.
    pub title: Pt,
    /// Разделитель.
    pub separator: Pt,
}

/// Pass-through частей [`StepPanel`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepPanelPt {
    /// Корень панели.
    pub root: Pt,
    /// Обёртка содержимого.
    pub content_wrapper: Pt,
    /// Содержимое.
    pub content: Pt,
    /// Разделитель (вертикальная раскладка).
    pub separator: Pt,
}

/// Общее состояние группы, доступное частям.
#[derive(Clone)]
struct StepperCtx {
    id: String,
    value: RwSignal<Option<u32>>,
    linear: Prop<bool>,
    /// Значения шагов `StepList` в порядке создания (для разделителей).
    steps: RwSignal<Vec<u32>>,
    /// Значения `StepItem` в порядке создания (для разделителей панелей).
    items: RwSignal<Vec<u32>>,
}

/// Значение ближайшего `StepItem`.
#[derive(Clone, Copy)]
struct ItemCtx {
    value: u32,
}

const NO_STEPPER: &str = "StepList, Step, StepPanels, StepPanel и StepItem должны находиться внутри Stepper";
const NO_VALUE: &str = "Step и StepPanel без value должны находиться внутри StepItem с value";

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

/// Строит `f` в дочернем владельце, чтобы положенный в `f` контекст не протёк на соседей, идущих после компонента.
///
/// Возвращаемый вид помнит владельца и строится, обновляется и гидратируется внутри него, поэтому реактивные потомки
/// (например, шаги, добавляемые условием) тоже находят контекст. Владелец держится до очистки родителя: сам по себе он
/// освободился бы сразу и унёс бы за собой реактивные значения потомков.
fn scoped(f: impl FnOnce() -> AnyView) -> OwnedView<AnyView> {
    match Owner::current() {
        Some(parent) => {
            let owner = parent.child();
            let out = owner.with(f);
            let keep = owner.clone();
            on_cleanup(move || drop(keep));
            OwnedView::new_with_owner(out, owner)
        }
        None => OwnedView::new_with_owner(f(), Owner::new()),
    }
}

/// Значение шага/панели: своё или значение ближайшего `StepItem`.
fn resolve_value(own: Option<u32>) -> (u32, bool) {
    let item = use_context::<ItemCtx>();
    (own.or(item.map(|i| i.value)).expect(NO_VALUE), item.is_some())
}

/// Мастер из шагов: хранит активное значение.
///
/// `value` — двусторонний сигнал; без него состояние собственное, начальное — `default_value`.
#[component]
pub fn Stepper(
    /// Основа для `id` шагов и панелей; по умолчанию уникальная.
    #[prop(optional, into)]
    id: Option<String>,
    /// Двусторонний сигнал активного значения.
    #[prop(optional)]
    value: Option<RwSignal<Option<u32>>>,
    /// Начальное значение, когда `value` не передан.
    #[prop(optional, into)]
    default_value: Option<u32>,
    /// Линейный режим: клики по неактивным шагам недоступны, переходы — программно.
    #[prop(optional, into)]
    linear: Prop<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through корня.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Список шагов, панели или элементы.
    children: Children,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let id = id.unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let ctx = StepperCtx {
        id: id.clone(),
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        linear: linear.clone(),
        steps: RwSignal::new(Vec::new()),
        items: RwSignal::new(Vec::new()),
    };
    let content = scoped(|| {
        provide_context(ctx);
        children().into_any()
    });
    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = ClassNames::new();
            cn.add(root_classes(linear.get()));
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
        <div class={class} style={style} id={id} role="tablist" {..pt.attrs()}>
            {content}
        </div>
    }
}

/// Список шагов.
#[component]
pub fn StepList(
    /// Pass-through списка.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Шаги.
    children: Children,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    view! {
        <div class={pt.merge_class("p-steplist")} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Шаг: номер, заголовок и разделитель; внутри `StepItem` — без разделителя.
#[component]
pub fn Step(
    /// Значение, связывающее шаг с панелью; внутри `StepItem` можно не задавать.
    #[prop(optional)]
    value: Option<u32>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Свой вид шага вместо кнопки-заголовка.
    #[prop(optional, into)]
    content: Option<Callback<StepContext, AnyView>>,
    /// Pass-through частей шага.
    #[prop(optional)]
    pt: Option<StepPt>,
    /// Заголовок.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = Arc::new(pt.unwrap_or_default());
    let (value, in_item) = resolve_value(value);
    if !in_item {
        ctx.steps.update(|s| s.push(value));
        let steps = ctx.steps;
        on_cleanup(move || {
            steps.try_update(|s| s.retain(|v| *v != value));
        });
    }
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let is_disabled = {
        let (linear, disabled) = (ctx.linear.clone(), disabled.clone());
        Signal::derive(move || step_disabled(active.get(), linear.get(), disabled.get()))
    };
    let activate = {
        let current = ctx.value;
        Callback::new(move |v: u32| current.set(Some(v)))
    };
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add(&step_classes(active.get(), is_disabled.get()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let separator = {
        let (steps, pt) = (ctx.steps, pt.clone());
        move || {
            (!in_item && steps.with(|s| separator_after(s, value))).then(|| {
                view! { <div class={pt.separator.merge_class("p-stepper-separator")} {..pt.separator.attrs()}></div> }
            })
        }
    };
    let body = match content {
        Some(template) => template.run(StepContext { value, active, activate }),
        None => {
            let (id, controls) = (step_id(&ctx.id, value), panel_id(&ctx.id, value));
            let click = move |_| activate.run(value);
            let tabindex = move || is_disabled.get().then_some("-1");
            let disabled_attr = move || is_disabled.get().then_some("");
            let (header_class, number_class, title_class) =
                (pt.header.merge_class("p-step-header"), pt.number.merge_class("p-step-number"), pt.title.merge_class("p-step-title"));
            let (header_attrs, number_attrs, title_attrs) = (pt.header.attrs(), pt.number.attrs(), pt.title.attrs());
            view! {
                <button
                    type="button"
                    id={id}
                    class={header_class}
                    role="tab"
                    aria-controls={controls}
                    tabindex={tabindex}
                    disabled={disabled_attr}
                    on:click={click}
                    {..header_attrs}
                >
                    <span class={number_class} {..number_attrs}>{value.to_string()}</span>
                    <span class={title_class} {..title_attrs}>{children.map(|c| c())}</span>
                </button>
            }
            .into_any()
        }
    };
    view! {
        <div
            class={class}
            role="presentation"
            aria-current={move || active.get().then_some("step")}
            data-p-active={move || bool_attr(active.get())}
            data-p-disabled={move || bool_attr(is_disabled.get())}
            {..pt.root.attrs()}
        >
            {body}
            {separator}
        </div>
    }
}

/// Контейнер панелей.
#[component]
pub fn StepPanels(
    /// Pass-through контейнера.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Панели.
    children: Children,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    view! {
        <div class={pt.merge_class("p-steppanels")} {..pt.attrs()}>
            {children()}
        </div>
    }
}

/// Панель шага: в горизонтальной раскладке содержимое есть только у активной, в вертикальной (внутри `StepItem`) — у всех, а
/// неактивные скрыты и раскрываются анимацией.
#[component]
pub fn StepPanel(
    /// Значение, связывающее панель с шагом; внутри `StepItem` можно не задавать.
    #[prop(optional)]
    value: Option<u32>,
    /// Свой вид содержимого; получает [`StepContext`].
    #[prop(optional, into)]
    content: Option<Callback<StepContext, AnyView>>,
    /// Pass-through частей панели.
    #[prop(optional)]
    pt: Option<StepPanelPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = Arc::new(pt.unwrap_or_default());
    let (value, vertical) = resolve_value(value);
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let activate = {
        let current = ctx.value;
        Callback::new(move |v: u32| current.set(Some(v)))
    };
    let context = StepContext { value, active, activate };
    let content_view = {
        let pt = pt.clone();
        move || {
            let inner = match (content, children.as_ref()) {
                (Some(template), _) => template.run(context),
                (None, Some(children)) => children().into_any(),
                (None, None) => ().into_any(),
            };
            let separator = (vertical).then(|| {
                let items = ctx.items;
                let pt = pt.clone();
                move || {
                    items
                        .with(|i| separator_after(i, value))
                        .then(|| view! { <div class={pt.separator.merge_class("p-stepper-separator")} {..pt.separator.attrs()}></div> })
                }
            });
            view! {
                <div class={pt.content_wrapper.merge_class("p-steppanel-content-wrapper")} {..pt.content_wrapper.attrs()}>
                    {separator}
                    <div class={pt.content.merge_class("p-steppanel-content")} {..pt.content.attrs()}>{inner}</div>
                </div>
            }
        }
    };
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add(panel_classes(vertical, active.get()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let (id, controls) = (panel_id(&ctx.id, value), step_id(&ctx.id, value));
    let data_active = move || bool_attr(active.get());
    if vertical {
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
        let hidden = hidden_style(!active.get_untracked());
        view! {
            <div class={class} id={id} role="tabpanel" aria-controls={controls} data-p-active={data_active} data-pc-name="steppanel" {..pt.root.attrs()}>
                <div {..region} class="p-motion" style={hidden}>
                    {content_view()}
                </div>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class={class} id={id} role="tabpanel" aria-controls={controls} data-p-active={data_active} data-pc-name="steppanel" {..pt.root.attrs()}>
                {move || active.get().then(&content_view)}
            </div>
        }
        .into_any()
    }
}

/// Элемент вертикальной раскладки: оборачивает [`Step`] и [`StepPanel`] и раздаёт им своё значение.
#[component]
pub fn StepItem(
    /// Значение элемента.
    value: u32,
    /// Pass-through элемента.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Шаг и панель.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<StepperCtx>().expect(NO_STEPPER);
    let pt = pt.unwrap_or_default();
    ctx.items.update(|i| i.push(value));
    let items = ctx.items;
    on_cleanup(move || {
        items.try_update(|i| i.retain(|v| *v != value));
    });
    let content = scoped(|| {
        provide_context(ItemCtx { value });
        children().into_any()
    });
    let active = {
        let current = ctx.value;
        Signal::derive(move || current.with(|c| is_active(*c, value)))
    };
    let base = pt.clone();
    view! {
        <div
            class={move || base.merge_class(item_classes(active.get()))}
            data-p-active={move || bool_attr(active.get())}
            {..pt.attrs()}
        >
            {content}
        </div>
    }
}
