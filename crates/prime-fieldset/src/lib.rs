//! Fieldset — порт `p-fieldset` из PrimeNG 21.1.10.
//!
//! Структура и классы совпадают с PrimeNG: корень `<fieldset class="p-fieldset …">` с `<legend class="p-fieldset-legend">`
//! и контейнером содержимого `p-fieldset-content-container` (обёртка `p-fieldset-content-wrapper`, содержимое `p-fieldset-content`).
//! При `toggleable` внутри легенды стоит кнопка `p-fieldset-toggle-button` с иконкой «плюс»/«минус» и подписью.
//!
//! Сворачивание построено на [`prime_core::collapsible`], так же как у Panel: состояние — двусторонний сигнал `collapsed`
//! (без него — собственное, начальное значение `default_collapsed`), кнопка получает `aria-controls` и `aria-expanded`,
//! область — `role="region"`, `aria-labelledby`, `aria-hidden`; раскрытие и сворачивание анимируют классы
//! `p-collapsible-enter-active`/`p-collapsible-leave-active`, а `on_after_toggle` вызывается по окончании анимации.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_fieldset::Fieldset;
//!
//! let collapsed = RwSignal::new(true);
//! let html = view! { <Fieldset legend="Заголовок" toggleable=true collapsed=collapsed><p>"Содержимое"</p></Fieldset> }.to_html();
//! assert!(html.contains("p-fieldset-collapsed") && html.contains(r#"aria-expanded="false""#) && html.contains("Заголовок"));
//! ```

mod state;

use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use prime_core::collapsible::{CollapseIds, CollapseState, aria_expanded, aria_hidden, region_tabindex};
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use prime_icons::{MinusIcon, PlusIcon};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{data_p, write_root_classes};

/// Событие `on_before_toggle`: исходное событие и состояние до переключения.
#[derive(Clone, Debug)]
pub struct FieldsetBeforeToggle {
    /// Исходное событие `click`.
    pub original_event: web_sys::MouseEvent,
    /// Значение `collapsed` до переключения.
    pub collapsed: bool,
}

/// Событие `on_after_toggle`: новое состояние после окончания анимации.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetAfterToggle {
    /// Значение `collapsed` после переключения.
    pub collapsed: bool,
}

/// Pass-through частей Fieldset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldsetPt {
    /// Корневой элемент (`fieldset.p-fieldset`).
    pub root: Pt,
    /// Легенда (`legend.p-fieldset-legend`).
    pub legend: Pt,
    /// Подпись легенды (`span.p-fieldset-legend-label`).
    pub legend_label: Pt,
    /// Кнопка-переключатель (`button.p-fieldset-toggle-button`).
    pub toggle_button: Pt,
    /// Иконка переключателя: оболочка слота (`span.p-fieldset-toggle-icon`); для стандартных иконок применяется только класс.
    pub toggle_icon: Pt,
    /// Контейнер содержимого (`div.p-fieldset-content-container`).
    pub content_container: Pt,
    /// Обёртка содержимого (`div.p-fieldset-content-wrapper`).
    pub content_wrapper: Pt,
    /// Содержимое (`div.p-fieldset-content`).
    pub content: Pt,
}

/// Группирующий контейнер с легендой и необязательным сворачиванием.
///
/// Текстовые и логические свойства принимают значение или сигнал; `collapsed` — двусторонний сигнал.
#[component]
pub fn Fieldset(
    /// Атрибут `id` корня; от него образуются идентификаторы кнопки и содержимого. По умолчанию уникальный.
    #[prop(optional, into)]
    id: Option<String>,
    /// Текст легенды (`p-fieldset-legend-label`).
    #[prop(optional, into)]
    legend: OptProp<String>,
    /// Можно ли сворачивать содержимое.
    #[prop(optional, into)]
    toggleable: Prop<bool>,
    /// Двусторонний сигнал «свёрнуто».
    #[prop(optional)]
    collapsed: Option<RwSignal<bool>>,
    /// Начальное состояние, когда `collapsed` не передан.
    #[prop(optional)]
    default_collapsed: bool,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Слот легенды: выводится после подписи.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Слот иконки развёртывания (показывается у свёрнутого Fieldset).
    #[prop(optional, into)]
    expand_icon_template: Option<ViewFn>,
    /// Слот иконки сворачивания (показывается у развёрнутого Fieldset).
    #[prop(optional, into)]
    collapse_icon_template: Option<ViewFn>,
    /// Вызывается до переключения.
    #[prop(optional, into)]
    on_before_toggle: Option<Callback<FieldsetBeforeToggle>>,
    /// Вызывается после окончания анимации переключения.
    #[prop(optional, into)]
    on_after_toggle: Option<Callback<FieldsetAfterToggle>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<FieldsetPt>,
    /// Содержимое (`p-fieldset-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(legend, toggleable, style, style_class) && !toggleable.get() && collapsed.is_none();
    let state = CollapseState::new(collapsed, default_collapsed);
    let props = Arc::new(Props {
        ids: CollapseIds::new(id.as_deref()),
        state,
        legend,
        toggleable,
        style,
        style_class,
        header_template,
        expand_icon_template,
        collapse_icon_template,
        on_before_toggle,
    });

    // `NodeRef` нужен только клиенту (анимация и `tabindex`): при SSR его нет, а `()` — пустой атрибут вместо `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        if props.toggleable.get_untracked() {
            let after = on_after_toggle.map(|cb| Callback::new(move |collapsed| cb.run(FieldsetAfterToggle { collapsed })));
            prime_core::collapsible::bind_region(region, state, after);
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {
        let _ = on_after_toggle;
    };

    let (root_id, content_id) = (props.ids.root.clone(), props.ids.content.clone());
    let root_attrs = pt.root.attrs();
    let container_class = pt.content_container.merge_class("p-fieldset-content-container");
    let container_style = container_style_of(&props, &pt);
    let container_attrs = pt.content_container.attrs();
    let (wrapper_class, wrapper_attrs) = (pt.content_wrapper.merge_class("p-fieldset-content-wrapper"), pt.content_wrapper.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-fieldset-content"), pt.content.attrs());
    let content = children.map(|c| c());

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let legend = legend_of(&props, &pt);
        Either::Left(fieldset_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            legend,
            region,
            container_class,
            container_style,
            content_id,
            None::<String>,
            None::<&'static str>,
            None::<&'static str>,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content
        ))
    } else {
        let pt = Arc::new(pt);
        let (p, t) = (&props, &pt);
        let (class, style, data) = (rx(p, t, class_of), rx(p, t, style_of), rx(p, t, |p, _| data_of(p)));
        let legend = rx(p, t, legend_of);
        let labelled_by = rx(p, t, |p, _| p.toggleable.get().then(|| p.ids.header.clone()));
        let hidden = rx(p, t, |p, _| p.toggleable.get().then(|| aria_hidden(p.state.get())));
        let tabindex = rx(p, t, |p, _| p.toggleable.get().then(|| region_tabindex(p.state.get())).flatten());
        Either::Right(fieldset_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            legend,
            region,
            container_class,
            container_style,
            content_id,
            labelled_by,
            hidden,
            tabindex,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content
        ))
    }
}

/// Разметка Fieldset: одна для статической и реактивной ветвей (меняются только выражения атрибутов и легенды).
macro_rules! fieldset_markup {
    (
        $root_id:expr, $class:expr, $style:expr, $data:expr, $root_attrs:expr, $legend:expr, $region:expr, $container_class:expr,
        $container_style:expr, $content_id:expr, $labelled_by:expr, $hidden:expr, $tabindex:expr, $container_attrs:expr,
        $wrapper_class:expr, $wrapper_attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr
    ) => {
        view! {
            <fieldset id={$root_id} class={$class} style={$style} data-p={$data} {..$root_attrs}>
                {$legend}
                <div
                    {..$region}
                    id={$content_id}
                    class={$container_class}
                    style={$container_style}
                    role="region"
                    aria-labelledby={$labelled_by}
                    aria-hidden={$hidden}
                    tabindex={$tabindex}
                    {..$container_attrs}
                >
                    <div class={$wrapper_class} {..$wrapper_attrs}>
                        <div class={$content_class} {..$content_attrs}>
                            {$content}
                        </div>
                    </div>
                </div>
            </fieldset>
        }
    };
}
use fieldset_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<FieldsetPt>,
    f: fn(&Arc<Props>, &FieldsetPt) -> T,
) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn data_of(props: &Props) -> Option<Cow<'static, str>> {
    let data = data_p(props.toggleable.get());
    (!data.is_empty()).then_some(data)
}

fn class_of(props: &Arc<Props>, pt: &FieldsetPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    let toggleable = props.toggleable.get();
    write_root_classes(&mut cn, toggleable, toggleable && props.state.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Arc<Props>, pt: &FieldsetPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Стиль контейнера содержимого: свёрнутый при первом рендере Fieldset скрыт сразу; дальше им управляет анимация.
fn container_style_of(props: &Props, pt: &FieldsetPt) -> Option<String> {
    let hidden = if props.toggleable.get_untracked() { props.state.initial_style() } else { None };
    (hidden.is_some() || pt.content_container.has_style()).then(|| pt.content_container.merge_style(hidden.unwrap_or("")))
}

/// Легенда: подпись и слот, а при `toggleable` — внутри кнопки; читает `collapsed` только внутри вложенных замыканий.
fn legend_of(props: &Arc<Props>, pt: &FieldsetPt) -> AnyView {
    let toggleable = props.toggleable.get();
    let text = props.legend.get();
    let label = view! {
        <span class=pt.legend_label.merge_class("p-fieldset-legend-label") {..pt.legend_label.attrs()}>{text.clone()}</span>
    };
    let slot = props.header_template.clone().map(|slot| slot.run());
    let (class, data) = (pt.legend.merge_class("p-fieldset-legend"), data_of(props));
    if toggleable {
        let button = toggle_button(props, pt, text, label, slot);
        view! { <legend class=class data-p={data} {..pt.legend.attrs()}>{button}</legend> }.into_any()
    } else {
        view! { <legend class=class data-p={data} {..pt.legend.attrs()}>{label}{slot}</legend> }.into_any()
    }
}

/// Кнопка-переключатель в легенде с `aria-controls` и `aria-expanded`; иконка зависит от `collapsed`.
fn toggle_button(
    props: &Arc<Props>,
    pt: &FieldsetPt,
    aria_label: Option<String>,
    label: impl IntoView,
    slot: Option<AnyView>,
) -> impl IntoView {
    let state = props.state;
    let (expand, collapse) = (props.expand_icon_template.clone(), props.collapse_icon_template.clone());
    let icon_class = pt.toggle_icon.merge_class("p-fieldset-toggle-icon").into_owned();
    let (icon_span_class, icon_pt) = (icon_class.clone(), pt.toggle_icon.clone());
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| toggle(&props, e)
    };
    let icon = move || {
        let collapsed = state.get();
        match if collapsed { &expand } else { &collapse } {
            Some(slot) => EitherOf3::A(view! { <span class=icon_span_class.clone() {..icon_pt.attrs()}>{slot.run()}</span> }),
            None if collapsed => EitherOf3::B(view! { <PlusIcon class=icon_class.clone() /> }),
            None => EitherOf3::C(view! { <MinusIcon class=icon_class.clone() /> }),
        }
    };
    view! {
        <button
            id=props.ids.header.clone()
            type="button"
            class=pt.toggle_button.merge_class("p-fieldset-toggle-button")
            aria-controls=props.ids.content.clone()
            aria-expanded=move || aria_expanded(state.get())
            aria-label=aria_label
            on:click={click}
            {..pt.toggle_button.attrs()}
        >
            {icon}
            {label}
            {slot}
        </button>
    }
}

/// Переключает Fieldset: `on_before_toggle` с прежним состоянием, затем смена `collapsed`.
fn toggle(props: &Props, e: web_sys::MouseEvent) {
    if !props.toggleable.get_untracked() {
        return;
    }
    e.prevent_default();
    if let Some(cb) = props.on_before_toggle {
        cb.run(FieldsetBeforeToggle { original_event: e, collapsed: props.state.get_untracked() });
    }
    props.state.toggle();
}

/// Свойства экземпляра.
struct Props {
    ids: CollapseIds,
    state: CollapseState,
    legend: OptProp<String>,
    toggleable: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    header_template: Option<ViewFn>,
    expand_icon_template: Option<ViewFn>,
    collapse_icon_template: Option<ViewFn>,
    on_before_toggle: Option<Callback<FieldsetBeforeToggle>>,
}
