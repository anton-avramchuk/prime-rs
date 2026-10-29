//! Slider — порт `p-slider` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-slider` (вместо хоста `<p-slider>`), внутри `span.p-slider-range` и одна
//! (`role=slider`) или, при `range`, две ручки `span.p-slider-handle`.
//!
//! Значение — двусторонний сигнал `value: RwSignal<f64>`, в режиме диапазона — `values: RwSignal<[f64; 2]>` (меньшее первым).
//! Без сигнала компонент ведёт внутреннее состояние (`default_value`, `default_values`). Ручку можно перетаскивать мышью и
//! касанием, двигать клавишами (стрелки, `PageUp`/`PageDown`, `Home`/`End`), а клик по полосе переносит туда активную ручку.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_slider::Slider;
//!
//! let value = RwSignal::new(25.0);
//! let html = view! { <Slider value=value max=200.0 /> }.to_html();
//! assert!(html.contains("p-slider-horizontal") && html.contains(r#"role="slider""#) && html.contains(r#"aria-valuenow="25""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use state::{
    KeyMove, SliderOrientation, SliderState, SliderValue, bar_percent, clamp, data_p, decimals, key_move, normalize, percent, step_change,
    touch_percent, value_from_percent, write_classes,
};

/// Событие изменения значения: новое значение и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct SliderChange {
    /// Новое значение (в диапазоне меньшее первым).
    pub value: SliderValue,
    /// Исходное событие (клик, перемещение указателя, касание или `keydown`).
    pub original_event: web_sys::Event,
}

/// Событие окончания перетаскивания или клика по полосе.
#[derive(Clone, Debug)]
pub struct SliderSlideEnd {
    /// Значение на момент окончания.
    pub value: SliderValue,
    /// Исходное событие (`mouseup`, `touchend` или `click`).
    pub original_event: web_sys::Event,
}

/// Pass-through частей Slider.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SliderPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Заполненная часть (`span.p-slider-range`).
    pub range: Pt,
    /// Ручка одиночного режима.
    pub handle: Pt,
    /// Первая ручка диапазона.
    pub start_handler: Pt,
    /// Вторая ручка диапазона.
    pub end_handler: Pt,
}

/// Ползунок выбора числа или диапазона.
///
/// Каждое свойство принимает значение или сигнал; `value` и `values` — двусторонние сигналы.
#[component]
pub fn Slider(
    /// Двусторонний сигнал значения (одиночный режим).
    #[prop(optional)]
    value: Option<RwSignal<f64>>,
    /// Двусторонний сигнал значений (режим `range`).
    #[prop(optional)]
    values: Option<RwSignal<[f64; 2]>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional)]
    default_value: f64,
    /// Начальные значения, если `values` не передан (по умолчанию `[0, 0]`).
    #[prop(optional)]
    default_values: Option<[f64; 2]>,
    /// Режим диапазона: две ручки.
    #[prop(optional, into)]
    range: Prop<bool>,
    /// Нижняя граница (по умолчанию 0).
    #[prop(optional, into)]
    min: OptProp<f64>,
    /// Верхняя граница (по умолчанию 100).
    #[prop(optional, into)]
    max: OptProp<f64>,
    /// Шаг изменения значения.
    #[prop(optional, into)]
    step: OptProp<f64>,
    /// Ориентация.
    #[prop(optional, into)]
    orientation: Prop<SliderOrientation>,
    /// Анимация положения при клике по полосе.
    #[prop(optional, into)]
    animate: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Атрибут `tabindex` ручек (по умолчанию 0; при `disabled` атрибута нет).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` ручек.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` ручек.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается при каждом изменении значения пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<SliderChange>>,
    /// Вызывается по окончании перетаскивания или клика по полосе.
    #[prop(optional, into)]
    on_slide_end: Option<Callback<SliderSlideEnd>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<SliderPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let props = Arc::new(Props {
        value: value.unwrap_or_else(|| RwSignal::new(default_value)),
        values: values.unwrap_or_else(|| RwSignal::new(default_values.unwrap_or([0.0, 0.0]))),
        range,
        min,
        max,
        step,
        orientation,
        animate,
        disabled,
        invalid,
        tabindex,
        aria_label,
        aria_labelledby,
        style,
        style_class,
        on_change,
        on_slide_end,
        active: RwSignal::new(0),
        dragging: RwSignal::new(false),
        handle_pressed: AtomicBool::new(false),
        root_ref: NodeRef::new(),
        handle_refs: [NodeRef::new(), NodeRef::new()],
        touch: StoredValue::new(Touch::default()),
        listeners: StoredValue::new_local(Vec::new()),
    });
    // Слушатели `window` живут только во время перетаскивания; при размонтировании снимаются.
    {
        let props = props.clone();
        on_cleanup(move || props.unbind_window());
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(
                &mut cn,
                &SliderState {
                    disabled: props.disabled.get(),
                    invalid: props.invalid.get(),
                    orientation: props.orientation.get(),
                    animate: props.animate.get() && !props.dragging.get(),
                },
            );
            props.style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let data = {
        let props = props.clone();
        move || data_p(props.orientation.get())
    };
    let sliding = {
        let props = props.clone();
        move || if props.dragging.get() { "true" } else { "false" }
    };
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.bar_click(e)
    };
    // Структура зависит только от `range`; значения и границы читаются внутри ручек и заполнения.
    let parts = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            if props.range.get() {
                Either::Left(view! {
                    {range_view(&props, &pt)}
                    {handle_view(&props, &pt.start_handler, 0, "startHandler")}
                    {handle_view(&props, &pt.end_handler, 1, "endHandler")}
                })
            } else {
                Either::Right(view! {
                    {range_view(&props, &pt)}
                    {handle_view(&props, &pt.handle, 0, "handle")}
                })
            }
        }
    };
    let root_attrs = pt.root.attrs();
    let root_ref = props.root_ref;

    view! {
        <div
            node_ref={root_ref}
            class={root_class}
            style={root_style}
            data-pc-name="slider"
            data-pc-section="root"
            data-p={data}
            data-p-sliding={sliding}
            on:click={click}
            {..root_attrs}
        >
            {parts}
        </div>
    }
}

/// Заполненная часть полосы.
fn range_view(props: &Arc<Props>, pt: &Arc<SliderPt>) -> impl IntoView + use<> {
    let class = pt.range.merge_class("p-slider-range");
    let style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let (from, len) = props.fill();
            let geometry = if props.orientation.get().is_vertical() {
                format!("bottom: {from}%; height: {len}%")
            } else {
                format!("inset-inline-start: {from}%; width: {len}%")
            };
            pt.range.merge_style_parts(&["position: absolute", &geometry])
        }
    };
    let attrs = pt.range.attrs();
    view! { <span class={class} data-pc-section="range" style={style} {..attrs}></span> }
}

/// Ручка с номером `index` (в одиночном режиме 0).
fn handle_view(props: &Arc<Props>, handle_pt: &Pt, index: usize, section: &'static str) -> impl IntoView + use<> {
    let ranged = section != "handle";
    let class = {
        let (props, class) = (props.clone(), handle_pt.merge_class("p-slider-handle"));
        move || {
            let mut cn = ClassNames::new();
            cn.add(&class).add_if(ranged && props.active.get() == index, "p-slider-handle-active");
            cn.finish_cow()
        }
    };
    let style = {
        let (props, handle_pt) = (props.clone(), handle_pt.clone());
        move || {
            let p = props.handle_percent(index);
            let position =
                if props.orientation.get().is_vertical() { format!("bottom: {p}%") } else { format!("inset-inline-start: {p}%") };
            let transition = if props.dragging.get() { "transition: none" } else { "" };
            handle_pt.merge_style_parts(&["position: absolute", &position, transition])
        }
    };
    let tabindex = {
        let props = props.clone();
        move || (!props.disabled.get()).then(|| props.tabindex.get().unwrap_or(0))
    };
    let (min, max) = (
        {
            let props = props.clone();
            move || props.min_t()
        },
        {
            let props = props.clone();
            move || props.max_t()
        },
    );
    let now = {
        let props = props.clone();
        move || props.value_at(index)
    };
    let orientation = {
        let props = props.clone();
        move || props.orientation.get().as_str()
    };
    let aria_label = {
        let props = props.clone();
        move || props.aria_label.get()
    };
    let aria_labelledby = {
        let props = props.clone();
        move || props.aria_labelledby.get()
    };
    let data = {
        let props = props.clone();
        move || data_p(props.orientation.get())
    };
    let mouse_down = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.mouse_down(e, index)
    };
    let touch_start = {
        let props = props.clone();
        move |e: web_sys::TouchEvent| props.touch_start(e, index)
    };
    let touch_move = {
        let props = props.clone();
        move |e: web_sys::TouchEvent| props.touch_move(e)
    };
    let touch_end = {
        let props = props.clone();
        move |e: web_sys::TouchEvent| props.touch_end(e)
    };
    let key_down = {
        let props = props.clone();
        move |e: web_sys::KeyboardEvent| props.key_down(e, index)
    };
    let attrs = handle_pt.attrs();
    let node_ref = props.handle_refs[index];

    view! {
        <span
            node_ref={node_ref}
            class={class}
            style={style}
            role="slider"
            tabindex={tabindex}
            aria-valuemin={min}
            aria-valuenow={now}
            aria-valuemax={max}
            aria-labelledby={aria_labelledby}
            aria-label={aria_label}
            aria-orientation={orientation}
            data-pc-section={section}
            data-p={data}
            on:mousedown={mouse_down}
            on:touchstart={touch_start}
            on:touchmove={touch_move}
            on:touchend={touch_end}
            on:keydown={key_down}
            {..attrs}
        ></span>
    }
}

/// Начало касания: координата вдоль полосы, размер полосы и стартовая доля ручки.
#[derive(Clone, Copy, Debug, Default)]
struct Touch {
    start: f64,
    size: f64,
    percent: f64,
}

/// Свойства и состояние экземпляра.
struct Props {
    value: RwSignal<f64>,
    values: RwSignal<[f64; 2]>,
    range: Prop<bool>,
    min: OptProp<f64>,
    max: OptProp<f64>,
    step: OptProp<f64>,
    orientation: Prop<SliderOrientation>,
    animate: Prop<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    on_change: Option<Callback<SliderChange>>,
    on_slide_end: Option<Callback<SliderSlideEnd>>,
    /// Индекс активной ручки (`handleIndex`).
    active: RwSignal<usize>,
    /// Идёт перетаскивание.
    dragging: RwSignal<bool>,
    /// Нажатие пришлось на ручку: следующий `click` не считается кликом по полосе (`sliderHandleClick`).
    handle_pressed: AtomicBool,
    root_ref: NodeRef<html::Div>,
    handle_refs: [NodeRef<html::Span>; 2],
    touch: StoredValue<Touch>,
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>, LocalStorage>,
}

impl Props {
    fn min_t(&self) -> f64 {
        self.min.get().unwrap_or(0.0)
    }

    fn max_t(&self) -> f64 {
        self.max.get().unwrap_or(100.0)
    }

    fn bounds(&self) -> (f64, f64) {
        (self.min.get_untracked().unwrap_or(0.0), self.max.get_untracked().unwrap_or(100.0))
    }

    /// Значение ручки (с подпиской): в одиночном режиме одно, в диапазоне по индексу.
    fn value_at(&self, index: usize) -> f64 {
        if self.range.get() { self.values.get()[index] } else { self.value.get() }
    }

    /// Положение ручки, % (с подпиской).
    fn handle_percent(&self, index: usize) -> f64 {
        percent(self.value_at(index), self.min_t(), self.max_t())
    }

    /// Начало и длина заполненной части, % (с подпиской): в диапазоне — между ручками независимо от порядка.
    fn fill(&self) -> (f64, f64) {
        if self.range.get() {
            let (a, b) = (self.handle_percent(0), self.handle_percent(1));
            (a.min(b), (a - b).abs())
        } else {
            (0.0, self.handle_percent(0))
        }
    }

    fn current(&self) -> SliderValue {
        if self.range.get_untracked() {
            SliderValue::Range(self.values.get_untracked())
        } else {
            SliderValue::Single(self.value.get_untracked())
        }
    }

    fn emit_change(&self, event: web_sys::Event) {
        if let Some(cb) = self.on_change {
            cb.run(SliderChange { value: self.current(), original_event: event });
        }
    }

    fn emit_slide_end(&self, event: web_sys::Event) {
        if let Some(cb) = self.on_slide_end {
            cb.run(SliderSlideEnd { value: self.current(), original_event: event });
        }
    }

    fn focus_active(&self) {
        if let Some(handle) = self.handle_refs[self.active.get_untracked()].get_untracked() {
            let _ = handle.focus();
        }
    }

    /// `updateValue`: ограничение, нормализация, запись, событие и фокус активной ручки.
    /// В диапазоне значение пишется в `index`; пара остаётся упорядоченной, при пересечении активной становится вторая ручка.
    fn update_value(&self, raw: f64, event: web_sys::Event, index: usize) {
        let (min, max) = self.bounds();
        let v = normalize(clamp(raw, min, max), self.step.get_untracked());
        if self.range.get_untracked() {
            let mut pair = self.values.get_untracked();
            pair[index] = v;
            let crossed = pair[0] > pair[1];
            if crossed {
                pair.swap(0, 1);
            }
            self.values.set(pair);
            self.active.set(if crossed { 1 - index } else { index });
        } else {
            self.value.set(v);
        }
        self.emit_change(event);
        self.focus_active();
    }

    /// Значение активной ручки (без подписки).
    fn active_value(&self) -> f64 {
        if self.range.get_untracked() { self.values.get_untracked()[self.active.get_untracked()] } else { self.value.get_untracked() }
    }

    /// `setValueFromHandle`: значение по доле полосы, с шагом — через `handleStepChange`.
    fn set_from_percent(&self, percent: f64, event: web_sys::Event) {
        let (min, max) = self.bounds();
        let new = value_from_percent(percent, min, max);
        let target = match self.step.get_untracked() {
            Some(step) if step > 0.0 => step_change(new, self.active_value(), step),
            _ => new,
        };
        self.update_value(target, event, self.active.get_untracked());
    }

    /// Геометрия полосы: начало и размер вдоль оси, RTL.
    fn geometry(&self) -> Option<(f64, f64, bool)> {
        let root: web_sys::HtmlElement = self.root_ref.get_untracked()?.unchecked_into();
        let rect = root.get_bounding_client_rect();
        let rtl = web_sys::window()
            .and_then(|w| w.get_computed_style(&root).ok().flatten())
            .and_then(|s| s.get_property_value("direction").ok())
            .is_some_and(|d| d == "rtl");
        Some(if self.orientation.get_untracked().is_vertical() {
            (rect.top(), rect.height(), rtl)
        } else {
            (rect.left(), rect.width(), rtl)
        })
    }

    /// Доля полосы под указателем с клиентскими координатами `(x, y)`.
    fn pointer_percent(&self, x: f64, y: f64) -> Option<f64> {
        let vertical = self.orientation.get_untracked().is_vertical();
        let (start, size, rtl) = self.geometry()?;
        (size > 0.0).then(|| bar_percent(if vertical { y } else { x }, start, size, vertical, rtl))
    }

    fn bar_click(&self, e: web_sys::MouseEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        if !self.handle_pressed.swap(false, Ordering::Relaxed) {
            if let Some(p) = self.pointer_percent(e.client_x() as f64, e.client_y() as f64) {
                self.set_from_percent(p, e.clone().into());
            }
            self.emit_slide_end(e.into());
        }
    }

    /// Ручка нажата: выбирается ручка, фокус, подписка на перемещение и отпускание на `window`.
    fn mouse_down(self: &Arc<Self>, e: web_sys::MouseEvent, index: usize) {
        if self.disabled.get_untracked() {
            return;
        }
        self.dragging.set(true);
        self.handle_pressed.store(true, Ordering::Relaxed);
        let first_at_max = self.range.get_untracked() && self.values.get_untracked()[0] == self.bounds().1;
        self.active.set(if first_at_max { 0 } else { index });
        self.bind_window();
        if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
            let _ = target.focus();
        }
        e.prevent_default();
    }

    fn bind_window(self: &Arc<Self>) {
        self.unbind_window();
        let (on_move, on_up) = (self.clone(), self.clone());
        let move_handle = window_event_listener(ev::mousemove, move |e: web_sys::MouseEvent| {
            if on_move.dragging.get_untracked()
                && let Some(p) = on_move.pointer_percent(e.client_x() as f64, e.client_y() as f64)
            {
                on_move.set_from_percent(p, e.into());
            }
        });
        let up_handle = window_event_listener(ev::mouseup, move |e: web_sys::MouseEvent| {
            if on_up.dragging.get_untracked() {
                on_up.dragging.set(false);
                on_up.unbind_window();
                on_up.emit_slide_end(e.into());
                // `click` после отпускания приходит синхронно, а если отпустили вне полосы, флаг нужно сбросить
                let reset = on_up.clone();
                set_timeout(move || reset.handle_pressed.store(false, Ordering::Relaxed), std::time::Duration::ZERO);
            }
        });
        self.listeners.update_value(|l| l.extend([move_handle, up_handle]));
    }

    fn unbind_window(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
    }

    fn touch_start(&self, e: web_sys::TouchEvent, index: usize) {
        if self.disabled.get_untracked() {
            return;
        }
        let Some(touch) = e.changed_touches().get(0) else { return };
        let vertical = self.orientation.get_untracked().is_vertical();
        let Some((_, size, _)) = self.geometry() else { return };
        let first_at_max = self.range.get_untracked() && self.values.get_untracked()[0] == self.bounds().1;
        self.active.set(if first_at_max { 0 } else { index });
        let start = if vertical { touch.client_y() } else { touch.client_x() } as f64;
        self.touch.set_value(Touch { start, size, percent: self.handle_percent_untracked(self.active.get_untracked()) });
        self.dragging.set(true);
        e.prevent_default();
    }

    fn touch_move(&self, e: web_sys::TouchEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        let Some(touch) = e.changed_touches().get(0) else { return };
        let vertical = self.orientation.get_untracked().is_vertical();
        let t = self.touch.get_value();
        if t.size <= 0.0 {
            return;
        }
        let pointer = if vertical { touch.client_y() } else { touch.client_x() } as f64;
        self.set_from_percent(touch_percent(pointer, t.start, t.size, vertical, t.percent), e.clone().into());
        e.prevent_default();
    }

    fn touch_end(&self, e: web_sys::TouchEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        self.dragging.set(false);
        self.emit_slide_end(e.clone().into());
        e.prevent_default();
    }

    fn handle_percent_untracked(&self, index: usize) -> f64 {
        let (min, max) = self.bounds();
        let v = if self.range.get_untracked() { self.values.get_untracked()[index] } else { self.value.get_untracked() };
        percent(v, min, max)
    }

    fn key_down(&self, e: web_sys::KeyboardEvent, index: usize) {
        let range = self.range.get_untracked();
        if range {
            self.active.set(index);
        }
        let Some(action) = key_move(&e.code(), self.step.get_untracked(), range) else { return };
        let (min, max) = self.bounds();
        let current = if range { self.values.get_untracked()[index] } else { self.value.get_untracked() };
        let raw = match action {
            KeyMove::By(delta) => current + delta,
            KeyMove::ToMin => min,
            KeyMove::ToMax => max,
        };
        e.prevent_default();
        if !self.disabled.get_untracked() {
            self.update_value(raw, e.into(), index);
        }
    }
}
