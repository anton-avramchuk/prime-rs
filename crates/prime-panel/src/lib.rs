//! Panel — порт `p-panel` из PrimeNG 21.1.10.
//!
//! Структура и классы совпадают с PrimeNG: шапка `p-panel-header` (заголовок `p-panel-title`, блок действий
//! `p-panel-header-actions` с кнопкой `p-panel-toggle-button`), контейнер содержимого `p-panel-content-container`
//! с обёрткой `p-panel-content-wrapper`, содержимым `p-panel-content` и подвалом `p-panel-footer`. Вместо хоста `<p-panel>`
//! отрисовывается `<div class="p-panel …">`.
//!
//! Сворачивание (`toggleable`) построено на [`prime_core::collapsible`]: состояние — двусторонний сигнал `collapsed`
//! (без него — собственное, начальное значение `default_collapsed`), кнопка получает `aria-controls` и `aria-expanded`,
//! область — `role="region"`, `aria-labelledby`, `aria-hidden`; раскрытие и сворачивание анимируют классы
//! `p-collapsible-enter-active`/`p-collapsible-leave-active`, а `on_after_toggle` вызывается по окончании анимации.
//!
//! Слоты — `ViewFn` (аналоги шаблонов `#header`, `#icons`, `#footer`), `header_icons_template` получает `collapsed`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_panel::Panel;
//!
//! let collapsed = RwSignal::new(true);
//! let html = view! { <Panel header="Заголовок" toggleable=true collapsed=collapsed><p>"Содержимое"</p></Panel> }.to_html();
//! assert!(html.contains("p-panel-collapsed") && html.contains(r#"aria-expanded="false""#) && html.contains("Заголовок"));
//! ```

mod state;

use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use prime_core::collapsible::{CollapseIds, CollapseState, aria_expanded, aria_hidden, region_tabindex};
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use prime_icons::{MinusIcon, PlusIcon};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{IconPos, Toggler, actions_class, data_p, write_root_classes};

/// Событие `on_before_toggle`: исходное событие и состояние до переключения.
#[derive(Clone, Debug)]
pub struct PanelBeforeToggle {
    /// Исходное событие `click`.
    pub original_event: web_sys::MouseEvent,
    /// Значение `collapsed` до переключения.
    pub collapsed: bool,
}

/// Событие `on_after_toggle`: новое состояние после окончания анимации.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PanelAfterToggle {
    /// Значение `collapsed` после переключения.
    pub collapsed: bool,
}

/// Pass-through частей Panel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PanelPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Шапка (`div.p-panel-header`).
    pub header: Pt,
    /// Заголовок (`span.p-panel-title`).
    pub title: Pt,
    /// Блок действий (`div.p-panel-header-actions`).
    pub header_actions: Pt,
    /// Кнопка-переключатель (`button.p-panel-toggle-button`).
    pub toggle_button: Pt,
    /// Контейнер содержимого (`div.p-panel-content-container`).
    pub content_container: Pt,
    /// Обёртка содержимого (`div.p-panel-content-wrapper`).
    pub content_wrapper: Pt,
    /// Содержимое (`div.p-panel-content`).
    pub content: Pt,
    /// Подвал (`div.p-panel-footer`).
    pub footer: Pt,
}

/// Контейнер с шапкой, необязательным сворачиванием и подвалом.
///
/// Текстовые и логические свойства принимают значение или сигнал; `collapsed` — двусторонний сигнал.
#[component]
pub fn Panel(
    /// Атрибут `id` корня; от него образуются идентификаторы заголовка и содержимого. По умолчанию уникальный.
    #[prop(optional, into)]
    id: Option<String>,
    /// Текст заголовка (`p-panel-title`).
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Можно ли сворачивать содержимое.
    #[prop(optional, into)]
    toggleable: Prop<bool>,
    /// Двусторонний сигнал «свёрнуто».
    #[prop(optional)]
    collapsed: Option<RwSignal<bool>>,
    /// Начальное состояние, когда `collapsed` не передан.
    #[prop(optional)]
    default_collapsed: bool,
    /// Положение иконок в шапке.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Показывать ли шапку.
    #[prop(default = Prop::Static(true), into)]
    show_header: Prop<bool>,
    /// Чем переключается панель.
    #[prop(optional, into)]
    toggler: Prop<Toggler>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Слот шапки: выводится после заголовка.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Слот действий: выводится в блоке действий перед переключателем.
    #[prop(optional, into)]
    icons_template: Option<ViewFn>,
    /// Слот значка переключателя; получает `collapsed` и заменяет «минус»/«плюс».
    #[prop(optional, into)]
    header_icons_template: Option<Callback<bool, AnyView>>,
    /// Слот подвала (`p-panel-footer`).
    #[prop(optional, into)]
    footer_template: Option<ViewFn>,
    /// Вызывается до переключения.
    #[prop(optional, into)]
    on_before_toggle: Option<Callback<PanelBeforeToggle>>,
    /// Вызывается после окончания анимации переключения.
    #[prop(optional, into)]
    on_after_toggle: Option<Callback<PanelAfterToggle>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<PanelPt>,
    /// Содержимое (`p-panel-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static =
        all_static!(header, toggleable, show_header, icon_pos, toggler, style, style_class) && !toggleable.get() && collapsed.is_none();
    let state = CollapseState::new(collapsed, default_collapsed);
    let props = Arc::new(Props {
        ids: CollapseIds::new(id.as_deref()),
        state,
        header,
        toggleable,
        show_header,
        icon_pos,
        toggler,
        style,
        style_class,
        header_template,
        icons_template,
        header_icons_template,
        on_before_toggle,
        footer_template,
    });

    // `NodeRef` нужен только клиенту (анимация и `tabindex`): при SSR его нет, а `()` — пустой атрибут вместо `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let region = {
        let region = NodeRef::<leptos::html::Div>::new();
        if props.toggleable.get_untracked() {
            let after = on_after_toggle.map(|cb| Callback::new(move |collapsed| cb.run(PanelAfterToggle { collapsed })));
            prime_core::collapsible::bind_region(region, state, after);
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Div, _>(region)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let region = {
        let _ = on_after_toggle;
    };

    let (root_id, content_id, labelled_by) = (props.ids.root.clone(), props.ids.content.clone(), props.ids.header.clone());
    let root_attrs = pt.root.attrs();
    let container_class = pt.content_container.merge_class("p-panel-content-container");
    let container_style = container_style_of(&props, &pt);
    let container_attrs = pt.content_container.attrs();
    let (wrapper_class, wrapper_attrs) = (pt.content_wrapper.merge_class("p-panel-content-wrapper"), pt.content_wrapper.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-panel-content"), pt.content.attrs());
    let content = children.map(|c| c());
    let footer = footer_of(&props, &pt);

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let header = header_of(&props, &pt);
        Either::Left(panel_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            header,
            region,
            container_class,
            container_style,
            content_id,
            labelled_by,
            None::<&'static str>,
            None::<&'static str>,
            container_attrs,
            wrapper_class,
            wrapper_attrs,
            content_class,
            content_attrs,
            content,
            footer
        ))
    } else {
        let pt = Arc::new(pt);
        let (p, t) = (&props, &pt);
        let (class, style, data) = (rx(p, t, class_of), rx(p, t, style_of), rx(p, t, |p, _| data_of(p)));
        let header = rx(p, t, header_of);
        let hidden = rx(p, t, |p, _| p.toggleable.get().then(|| aria_hidden(p.state.get())));
        let tabindex = rx(p, t, |p, _| p.toggleable.get().then(|| region_tabindex(p.state.get())).flatten());
        Either::Right(panel_markup!(
            root_id,
            class,
            style,
            data,
            root_attrs,
            header,
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
            content,
            footer
        ))
    }
}

/// Разметка Panel: одна для статической и реактивной ветвей (меняются только выражения атрибутов и шапки).
macro_rules! panel_markup {
    (
        $root_id:expr, $class:expr, $style:expr, $data:expr, $root_attrs:expr, $header:expr, $region:expr, $container_class:expr,
        $container_style:expr, $content_id:expr, $labelled_by:expr, $hidden:expr, $tabindex:expr, $container_attrs:expr,
        $wrapper_class:expr, $wrapper_attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr, $footer:expr
    ) => {
        view! {
            <div id={$root_id} class={$class} style={$style} data-p={$data} {..$root_attrs}>
                {$header}
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
                        {$footer}
                    </div>
                </div>
            </div>
        }
    };
}
use panel_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<PanelPt>,
    f: fn(&Arc<Props>, &PanelPt) -> T,
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

fn class_of(props: &Arc<Props>, pt: &PanelPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    let toggleable = props.toggleable.get();
    write_root_classes(&mut cn, toggleable, toggleable && props.state.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Arc<Props>, pt: &PanelPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Стиль контейнера содержимого: свёрнутая при первом рендере панель скрыта сразу; дальше им управляет анимация.
fn container_style_of(props: &Props, pt: &PanelPt) -> Option<String> {
    let hidden = if props.toggleable.get_untracked() { props.state.initial_style() } else { None };
    (hidden.is_some() || pt.content_container.has_style()).then(|| pt.content_container.merge_style(hidden.unwrap_or("")))
}

/// Подвал: только при заданном слоте.
fn footer_of(props: &Props, pt: &PanelPt) -> Option<AnyView> {
    props
        .footer_template
        .clone()
        .map(|slot| view! { <div class=pt.footer.merge_class("p-panel-footer") {..pt.footer.attrs()}>{slot.run()}</div> }.into_any())
}

/// Шапка: заголовок, слот и блок действий с переключателем; читает `collapsed` только внутри вложенных замыканий.
fn header_of(props: &Arc<Props>, pt: &PanelPt) -> Option<AnyView> {
    if !props.show_header.get() {
        return None;
    }
    let toggleable = props.toggleable.get();
    let label = props.header.get();
    let title = label.clone().map(|text| {
        view! { <span id=props.ids.header.clone() class=pt.title.merge_class("p-panel-title") {..pt.title.attrs()}>{text}</span> }
    });
    let slot = props.header_template.clone().map(|slot| slot.run());
    let icons = props.icons_template.clone().map(|slot| slot.run());
    let button = toggleable.then(|| toggle_button(props, pt, label.clone()));
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            if props.toggler.get_untracked() == Toggler::Header {
                toggle(&props, e);
            }
        }
    };
    let actions_class = pt.header_actions.merge_class(actions_class(props.icon_pos.get()));
    Some(
        view! {
            <div
                id=props.ids.titlebar.clone()
                class=pt.header.merge_class("p-panel-header")
                data-p=data_of(props)
                on:click={click}
                {..pt.header.attrs()}
            >
                {title}
                {slot}
                <div class={actions_class} {..pt.header_actions.attrs()}>{icons}{button}</div>
            </div>
        }
        .into_any(),
    )
}

/// Кнопка-переключатель с `aria-controls` и `aria-expanded`; без заголовка берёт его идентификатор.
fn toggle_button(props: &Arc<Props>, pt: &PanelPt, label: Option<String>) -> impl IntoView + use<> {
    let state = props.state;
    let template = props.header_icons_template;
    let id = label.is_none().then(|| props.ids.header.clone());
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            if props.toggler.get_untracked() == Toggler::Icon {
                toggle(&props, e);
            }
        }
    };
    let icon = move || {
        let collapsed = state.get();
        match template {
            Some(template) => EitherOf3::A(template.run(collapsed)),
            None if collapsed => EitherOf3::B(view! { <PlusIcon /> }),
            None => EitherOf3::C(view! { <MinusIcon /> }),
        }
    };
    view! {
        <button
            id=id
            type="button"
            class=pt.toggle_button.merge_class(
                "p-button p-component p-button-icon-only p-button-secondary p-button-rounded p-button-text p-panel-toggle-button",
            )
            data-p="icon-only rounded text"
            aria-label=label
            aria-controls=props.ids.content.clone()
            aria-expanded=move || aria_expanded(state.get())
            on:click={click}
            {..pt.toggle_button.attrs()}
        >
            <span class="p-button-icon" data-p="left">
                {icon}
            </span>
        </button>
    }
}

/// Переключает панель: `on_before_toggle` с прежним состоянием, затем смена `collapsed`.
fn toggle(props: &Props, e: web_sys::MouseEvent) {
    if !props.toggleable.get_untracked() {
        return;
    }
    e.prevent_default();
    if let Some(cb) = props.on_before_toggle {
        cb.run(PanelBeforeToggle { original_event: e, collapsed: props.state.get_untracked() });
    }
    props.state.toggle();
}

/// Свойства экземпляра.
struct Props {
    ids: CollapseIds,
    state: CollapseState,
    header: OptProp<String>,
    toggleable: Prop<bool>,
    show_header: Prop<bool>,
    icon_pos: Prop<IconPos>,
    toggler: Prop<Toggler>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    header_template: Option<ViewFn>,
    icons_template: Option<ViewFn>,
    header_icons_template: Option<Callback<bool, AnyView>>,
    on_before_toggle: Option<Callback<PanelBeforeToggle>>,
    footer_template: Option<ViewFn>,
}
