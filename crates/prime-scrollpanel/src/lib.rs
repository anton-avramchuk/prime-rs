//! ScrollPanel — порт `p-scrollpanel` из PrimeNG 21.1.10.
//!
//! Разметка и классы совпадают с PrimeNG: корень `div.p-scrollpanel`, контейнер `p-scrollpanel-content-container` с
//! `p-scrollpanel-content` внутри и две полосы `p-scrollpanel-bar-x` / `p-scrollpanel-bar-y` (`role="scrollbar"`).
//! Вместо хоста `<p-scrollpanel>` отрисовывается `<div class="p-scrollpanel …">`.
//!
//! Размер и положение полос считаются только в браузере и пишутся прямо в DOM (см. `docs/scrollpanel.md`);
//! SSR отдаёт разметку без обращения к DOM. Свойства — [`Prop`]/[`OptProp`]; если `style` и `style_class` статические,
//! компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_scrollpanel::ScrollPanel;
//!
//! let html = view! { <ScrollPanel style="height:150px">"Текст"</ScrollPanel> }.to_html();
//! assert!(html.contains("p-scrollpanel-content") && html.contains("p-scrollpanel-bar-x") && html.contains("p-scrollpanel-bar-y"));
//! ```

#[cfg(target_arch = "wasm32")]
mod client;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::collapsible::CollapseIds;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Шаг прокрутки клавишами по умолчанию, пикселей.
pub const DEFAULT_STEP: f64 = 5.0;

/// Минимальная длина полосы, процентов.
const MIN_BAR_PERCENT: f64 = 10.0;

/// Доля видимой части: `own / total`; при пустом содержимом — 1 (полоса не нужна).
///
/// ```
/// use prime_scrollpanel::bar_ratio;
///
/// assert_eq!(bar_ratio(150.0, 300.0), 0.5);
/// assert_eq!(bar_ratio(150.0, 0.0), 1.0);
/// ```
pub fn bar_ratio(own: f64, total: f64) -> f64 {
    if total > 0.0 { own / total } else { 1.0 }
}

/// Длина полосы в процентах: доля × 100, но не меньше 10.
///
/// ```
/// use prime_scrollpanel::bar_len;
///
/// assert_eq!(bar_len(0.5), 50.0);
/// assert_eq!(bar_len(0.02), 10.0);
/// ```
pub fn bar_len(ratio: f64) -> f64 {
    (ratio * 100.0).max(MIN_BAR_PERCENT)
}

/// Смещение полосы в процентах: `scroll × (100 − len) / (total − own)`; без хода прокрутки — 0.
///
/// ```
/// use prime_scrollpanel::bar_offset;
///
/// // содержимое 400, область 100, полоса 25%, прокручено на половину хода (150 из 300)
/// assert_eq!(bar_offset(150.0, 25.0, 400.0, 100.0), 37.5);
/// assert_eq!(bar_offset(10.0, 25.0, 100.0, 100.0), 0.0);
/// ```
pub fn bar_offset(scroll: f64, len: f64, total: f64, own: f64) -> f64 {
    let travel = total - own;
    if travel > 0.0 { scroll * (100.0 - len) / travel } else { 0.0 }
}

/// Реакция на клавишу, нажатую на полосе.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyAction {
    /// Прокрутить: `horizontal` — по оси X, `delta` — знак шага (`1.0` или `-1.0`).
    Scroll {
        /// Ось X (иначе Y).
        horizontal: bool,
        /// Знак шага.
        delta: f64,
    },
    /// Только отменить действие браузера по умолчанию.
    Block,
    /// Ничего не делать.
    Ignore,
}

/// Таблица клавиш: вертикальная ось слушает `ArrowUp`/`ArrowDown`, горизонтальная — `ArrowLeft`/`ArrowRight`;
/// остальные стрелки лишь блокируются.
///
/// ```
/// use prime_scrollpanel::{KeyAction, key_action};
///
/// assert_eq!(key_action(false, "ArrowDown"), KeyAction::Scroll { horizontal: false, delta: 1.0 });
/// assert_eq!(key_action(true, "ArrowLeft"), KeyAction::Scroll { horizontal: true, delta: -1.0 });
/// assert_eq!(key_action(false, "ArrowLeft"), KeyAction::Block);
/// assert_eq!(key_action(false, "Enter"), KeyAction::Ignore);
/// ```
pub fn key_action(horizontal: bool, code: &str) -> KeyAction {
    match (horizontal, code) {
        (false, "ArrowDown") => KeyAction::Scroll { horizontal: false, delta: 1.0 },
        (false, "ArrowUp") => KeyAction::Scroll { horizontal: false, delta: -1.0 },
        (true, "ArrowRight") => KeyAction::Scroll { horizontal: true, delta: 1.0 },
        (true, "ArrowLeft") => KeyAction::Scroll { horizontal: true, delta: -1.0 },
        (_, "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown") => KeyAction::Block,
        _ => KeyAction::Ignore,
    }
}

/// Ограничивает `scrollTop` диапазоном `0..=scroll_height − client_height`.
///
/// ```
/// use prime_scrollpanel::clamp_scroll;
///
/// assert_eq!(clamp_scroll(100000.0, 500.0, 150.0), 350.0);
/// assert_eq!(clamp_scroll(-5.0, 500.0, 150.0), 0.0);
/// ```
pub fn clamp_scroll(value: f64, scroll_size: f64, client_size: f64) -> f64 {
    value.min(scroll_size - client_size).max(0.0)
}

/// Pass-through частей ScrollPanel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrollPanelPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Контейнер содержимого (`div.p-scrollpanel-content-container`).
    pub content_container: Pt,
    /// Содержимое (`div.p-scrollpanel-content`).
    pub content: Pt,
    /// Горизонтальная полоса (`div.p-scrollpanel-bar-x`).
    pub bar_x: Pt,
    /// Вертикальная полоса (`div.p-scrollpanel-bar-y`).
    pub bar_y: Pt,
}

/// Ручка для вызова методов ScrollPanel из кода (аналог `@ViewChild`): `refresh` и `scroll_top`.
///
/// Создаётся до компонента и передаётся свойством `panel_ref`; вне браузера и до монтирования методы ничего не делают.
#[derive(Clone, Copy)]
pub struct ScrollPanelRef {
    #[cfg(target_arch = "wasm32")]
    inner: StoredValue<Option<std::rc::Rc<client::Client>>, LocalStorage>,
}

impl ScrollPanelRef {
    /// Пустая ручка; компонент заполняет её при монтировании.
    pub fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            inner: StoredValue::new_local(None),
        }
    }

    /// Пересчитывает положение и размер полос.
    pub fn refresh(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(c) = self.inner.try_get_value().flatten() {
            c.move_bar();
        }
    }

    /// Прокручивает содержимое к `value` пикселей сверху (значение ограничено доступным ходом).
    pub fn scroll_top(&self, value: f64) {
        #[cfg(target_arch = "wasm32")]
        if let Some(c) = self.inner.try_get_value().flatten() {
            c.scroll_top(value);
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = value;
    }
}

impl Default for ScrollPanelRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Панель с тематизируемыми полосами прокрутки.
///
/// `style` и `style_class` принимают значение или сигнал. Размеры (`width`, `height` или `max-height`) задаются через `style`.
/// Содержимое — `children` либо слот `content_template` (при заданном слоте `children` не выводятся).
#[component]
pub fn ScrollPanel(
    /// Шаг прокрутки клавишами стрелок, пикселей (по умолчанию 5).
    #[prop(optional, into)]
    step: OptProp<f64>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Слот содержимого (аналог шаблона `#content`).
    #[prop(optional, into)]
    content_template: Option<ViewFn>,
    /// Ручка для `refresh` и `scroll_top`.
    #[prop(optional)]
    panel_ref: Option<ScrollPanelRef>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ScrollPanelPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(style, style_class);
    let props = Props { style, style_class };
    let content_id = CollapseIds::new(None).content;

    // `NodeRef` нужны только клиенту (измерение и слушатели): при SSR их нет, а `()` — пустой атрибут вместо `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let (root_ref, content_ref, x_ref, y_ref) = {
        use leptos::html::Div;
        use leptos::tachys::html::node_ref::node_ref;
        let (root, content, x, y) = (NodeRef::<Div>::new(), NodeRef::<Div>::new(), NodeRef::<Div>::new(), NodeRef::<Div>::new());
        let step = step.clone();
        client::mount(root, content, x, y, std::rc::Rc::new(move || step.get_untracked().unwrap_or(DEFAULT_STEP)), panel_ref);
        (node_ref::<Div, _>(root), node_ref::<Div, _>(content), node_ref::<Div, _>(x), node_ref::<Div, _>(y))
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let (root_ref, content_ref, x_ref, y_ref) = {
        let _ = (step, panel_ref);
        ((), (), (), ())
    };

    let content = match content_template {
        Some(slot) => Some(slot.run()),
        None => children.map(|c| c().into_any()),
    };
    let root_attrs = pt.root.attrs();
    let (container_class, container_attrs) =
        (pt.content_container.merge_class("p-scrollpanel-content-container"), pt.content_container.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-scrollpanel-content"), pt.content.attrs());
    let (bar_x_class, bar_x_style, bar_x_attrs) = bar_of(&pt.bar_x, "p-scrollpanel-bar p-scrollpanel-bar-x");
    let (bar_y_class, bar_y_style, bar_y_attrs) = bar_of(&pt.bar_y, "p-scrollpanel-bar p-scrollpanel-bar-y");
    let controls = content_id.clone();

    if is_static {
        let (class, style) = (class_of(&props, &pt), style_of(&props, &pt));
        Either::Left(scrollpanel_markup!(
            class,
            style,
            root_attrs,
            root_ref,
            container_class,
            container_attrs,
            content_ref,
            content_id,
            content_class,
            content_attrs,
            content,
            x_ref,
            bar_x_class,
            bar_x_style,
            bar_x_attrs,
            y_ref,
            bar_y_class,
            bar_y_style,
            bar_y_attrs,
            controls
        ))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = move || style_of(&props, &pt);
        Either::Right(scrollpanel_markup!(
            class,
            style,
            root_attrs,
            root_ref,
            container_class,
            container_attrs,
            content_ref,
            content_id,
            content_class,
            content_attrs,
            content,
            x_ref,
            bar_x_class,
            bar_x_style,
            bar_x_attrs,
            y_ref,
            bar_y_class,
            bar_y_style,
            bar_y_attrs,
            controls
        ))
    }
}

/// Разметка ScrollPanel: одна для статической и реактивной ветвей (меняются только выражения `class` и `style` корня).
macro_rules! scrollpanel_markup {
    (
        $class:expr, $style:expr, $root_attrs:expr, $root_ref:expr, $container_class:expr, $container_attrs:expr, $content_ref:expr,
        $content_id:expr, $content_class:expr, $content_attrs:expr, $content:expr, $x_ref:expr, $bar_x_class:expr, $bar_x_style:expr,
        $bar_x_attrs:expr, $y_ref:expr, $bar_y_class:expr, $bar_y_style:expr, $bar_y_attrs:expr, $controls:expr
    ) => {
        view! {
            <div class={$class} style={$style} {..$root_attrs} {..$root_ref}>
                <div class={$container_class} {..$container_attrs}>
                    <div {..$content_ref} id={$content_id} class={$content_class} {..$content_attrs}>
                        {$content}
                    </div>
                </div>
                <div
                    {..$x_ref}
                    class={$bar_x_class}
                    style={$bar_x_style}
                    tabindex="0"
                    role="scrollbar"
                    aria-orientation="horizontal"
                    aria-valuenow="0"
                    aria-controls={$controls.clone()}
                    data-pc-group-section="bar"
                    {..$bar_x_attrs}
                ></div>
                <div
                    {..$y_ref}
                    class={$bar_y_class}
                    style={$bar_y_style}
                    tabindex="0"
                    role="scrollbar"
                    aria-orientation="vertical"
                    aria-valuenow="0"
                    aria-controls={$controls}
                    data-pc-group-section="bar"
                    {..$bar_y_attrs}
                ></div>
            </div>
        }
    };
}
use scrollpanel_markup;

fn class_of(props: &Props, pt: &ScrollPanelPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-scrollpanel p-component");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &ScrollPanelPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Класс, начальный стиль и атрибуты полосы; дальше `style` полосы ведёт клиентский пересчёт, а не Leptos.
fn bar_of(pt: &Pt, base: &'static str) -> (Cow<'static, str>, Option<String>, Vec<leptos::attr::any_attribute::AnyAttribute>) {
    (pt.merge_class(base), pt.has_style().then(|| pt.merge_style("")), pt.attrs())
}

/// Свойства экземпляра.
struct Props {
    style: OptProp<String>,
    style_class: OptProp<String>,
}
