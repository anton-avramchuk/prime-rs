//! Carousel — порт `p-carousel` из PrimeNG 21.1.10.
//!
//! Ряд элементов, показываемых страницами по `num_visible` штук. Листается кнопками, точками-индикаторами, автопрокруткой
//! (`autoplay_interval`) и свайпом; в бесконечном режиме (`circular`) по краям лежат клоны, а переход через границу завершается
//! скачком без анимации. Размеры страниц меняются по `responsive_options` на `resize` окна. Вся арифметика страниц и смещений — в
//! чистой модели [`Track`] (см. модуль `model`).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_carousel::Carousel;
//!
//! let html = Owner::new().with(|| {
//!     view! { <Carousel value=vec![1, 2, 3, 4, 5] num_visible=2 item=Callback::new(|n: i32| view! { <b>{n}</b> }.into_any()) /> }.to_html()
//! });
//! assert!(html.contains("p-carousel-viewport") && html.contains("p-carousel-indicator-button") && html.contains("<b>3</b>"));
//! ```

mod model;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{ClassNames, OptProp, Prop, Pt};
use prime_icons::{ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon};
use std::sync::Arc;

pub use model::{
    Dir, Responsive, SWIPE_THRESHOLD, Track, autoplay_next, fill, indicator_focus, is_circular, item_flex, match_responsive, swipe,
    total_dots, translate,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{IntervalHandle, set_interval_with_handle};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Ориентация.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    /// По горизонтали (по умолчанию).
    #[default]
    Horizontal,
    /// По вертикали.
    Vertical,
}

impl CarouselOrientation {
    /// Вертикальная ли ориентация.
    pub fn is_vertical(self) -> bool {
        self == Self::Vertical
    }
}

/// Pass-through частей [`Carousel`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CarouselPt {
    /// Корень.
    pub root: Pt,
    /// Шапка.
    pub header: Pt,
    /// Контейнер содержимого.
    pub content_container: Pt,
    /// Содержимое.
    pub content: Pt,
    /// Кнопка «назад».
    pub prev_button: Pt,
    /// Окно.
    pub viewport: Pt,
    /// Список элементов.
    pub item_list: Pt,
    /// Элемент.
    pub item: Pt,
    /// Клон элемента.
    pub item_clone: Pt,
    /// Кнопка «вперёд».
    pub next_button: Pt,
    /// Список индикаторов.
    pub indicator_list: Pt,
    /// Индикатор.
    pub indicator: Pt,
    /// Кнопка индикатора.
    pub indicator_button: Pt,
    /// Подвал.
    pub footer: Pt,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    track: RwSignal<Track>,
    /// Смещение, применённое к `transform` (может отличаться от логического при переходе через клоны).
    applied: RwSignal<i32>,
    transitioning: RwSignal<bool>,
    items_hidden: RwSignal<bool>,
    allow_autoplay: RwSignal<bool>,
    focus_dot: RwSignal<usize>,
    len: Memo<usize>,
    circular: Prop<bool>,
    vertical: Prop<CarouselOrientation>,
    num_visible: Prop<usize>,
    num_scroll: Prop<usize>,
    responsive: Arc<Vec<Responsive>>,
    autoplay_interval: Prop<u32>,
    page_signal: Option<RwSignal<usize>>,
    on_page: Option<Callback<usize>>,
    start: StoredValue<(f64, f64)>,
    indicators: NodeRef<leptos::html::Ul>,
    #[cfg(target_arch = "wasm32")]
    interval: StoredValue<Option<IntervalHandle>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
}

impl Core {
    fn circ(&self) -> bool {
        is_circular(self.circular.get_untracked(), self.len.get_untracked(), self.track.with_untracked(|t| t.num_visible))
    }

    fn emit_page(&self, page: usize) {
        if let Some(sig) = self.page_signal
            && sig.get_untracked() != page
        {
            sig.set(page);
        }
        if let Some(cb) = self.on_page {
            cb.run(page);
        }
    }

    /// Шаг: смещение применяется с переходом, логическое состояние приводится сразу.
    fn step(&self, dir: Dir, page: Option<usize>) {
        let (len, circular) = (self.len.get_untracked(), self.circular.get_untracked());
        let mut track = self.track.get_untracked();
        let applied = track.step(len, circular, dir, page);
        self.items_hidden.set(false);
        self.transitioning.set(true);
        self.applied.set(applied);
        track.settle(len, circular);
        self.focus_dot.set(track.page);
        let page_now = track.page;
        self.track.set(track);
        self.emit_page(page_now);
    }

    fn nav_forward(&self, index: Option<usize>) {
        let (len, track) = (self.len.get_untracked(), self.track.get_untracked());
        if self.circ() || track.page + 1 < track.dots(len) {
            self.step(Dir::Forward, index);
        }
        self.stop_autoplay(true);
    }

    fn nav_backward(&self, index: Option<usize>) {
        if self.circ() || self.track.with_untracked(|t| t.page) != 0 {
            self.step(Dir::Backward, index);
        }
        self.stop_autoplay(true);
    }

    fn dot_click(&self, index: usize) {
        self.stop_autoplay(true);
        let page = self.track.with_untracked(|t| t.page);
        if index > page {
            self.nav_forward(Some(index));
        } else if index < page {
            self.nav_backward(Some(index));
        }
    }

    fn transition_end(&self) {
        self.transitioning.set(false);
        self.items_hidden.set(true);
        let (len, circular) = (self.len.get_untracked(), self.circular.get_untracked());
        if let Some(shift) = self.track.with_untracked(|t| t.after_transition(len, circular)) {
            self.applied.set(shift);
        }
    }

    fn stop_autoplay(&self, change_allow: bool) {
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.interval.try_update_value(Option::take).flatten() {
            handle.clear();
            if change_allow {
                self.allow_autoplay.set(false);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = change_allow;
    }

    /// Свайп по разности координат вдоль оси ориентации.
    fn swipe_diff(&self, diff: f64) {
        match swipe(diff) {
            Some(Dir::Forward) => self.nav_forward(None),
            Some(Dir::Backward) => self.nav_backward(None),
            None => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn start_autoplay(self: &Arc<Self>) {
        let interval = self.autoplay_interval.get_untracked();
        let this = self.clone();
        let handle = set_interval_with_handle(
            move || {
                let (len, track) = (this.len.get_untracked(), this.track.get_untracked());
                if let Some(next) = autoplay_next(track.page, track.dots(len)) {
                    this.step(Dir::Forward, Some(next));
                }
            },
            Duration::from_millis(u64::from(interval)),
        );
        if let Ok(handle) = handle {
            self.interval.set_value(Some(handle));
            self.allow_autoplay.set(true);
        }
    }

    /// Пересчёт после смены свойств, длины или размеров окна (`onAfterContentChecked` и `calculatePosition`).
    #[cfg(target_arch = "wasm32")]
    fn recompute(self: &Arc<Self>) {
        let (nv, ns) = (self.num_visible.get_untracked(), self.num_scroll.get_untracked());
        let (len, circular) = (self.len.get_untracked(), self.circular.get_untracked());
        let mut track = self.track.get_untracked();
        track.num_visible = nv;
        track.num_scroll = ns;
        let notify = if self.responsive.is_empty() {
            track.recalc(len, circular)
        } else {
            let width = web_sys::window().and_then(|w| w.inner_width().ok()).and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
            track.apply_responsive(&self.responsive, width, (nv, ns), len, circular)
        };
        self.applied.set(track.total_shifted);
        self.focus_dot.set(track.page);
        let page = track.page;
        self.track.set(track);
        if notify {
            self.emit_page(page);
        }
        if self.autoplay_interval.get_untracked() > 0 && self.allow_autoplay.get_untracked() {
            self.stop_autoplay(false);
            self.start_autoplay();
        }
    }

    /// Переносит фокус на кнопку точки `index`.
    fn focus_indicator(&self, index: usize) {
        #[cfg(target_arch = "wasm32")]
        if let Some(list) = self.indicators.get_untracked()
            && let Some(button) = list
                .children()
                .item(index as u32)
                .and_then(|li| li.first_element_child())
                .and_then(|b| b.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = button.focus();
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = index;
    }

    fn release(&self) {
        self.stop_autoplay(false);
        #[cfg(target_arch = "wasm32")]
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
    }
}

/// Ряд элементов со страницами, индикаторами, автопрокруткой и бесконечным режимом.
///
/// Свойства принимают значение или сигнал. Элементы строятся шаблоном `item` один раз на смену `value`.
#[component]
pub fn Carousel<T>(
    /// Элементы.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// Текущая страница; двусторонний сигнал.
    #[prop(optional)]
    page: Option<RwSignal<usize>>,
    /// Число видимых элементов.
    #[prop(optional, default = Prop::Static(1), into)]
    num_visible: Prop<usize>,
    /// Число элементов за прокрутку.
    #[prop(optional, default = Prop::Static(1), into)]
    num_scroll: Prop<usize>,
    /// Варианты адаптивности.
    #[prop(optional, into)]
    responsive_options: Vec<Responsive>,
    /// Ориентация.
    #[prop(optional, into)]
    orientation: Prop<CarouselOrientation>,
    /// Высота окна при вертикальной ориентации.
    #[prop(optional, default = Prop::Static("300px".to_owned()), into)]
    vertical_view_port_height: Prop<String>,
    /// Дополнительный класс контейнера содержимого.
    #[prop(optional, into)]
    content_class: OptProp<String>,
    /// Дополнительный класс списка индикаторов.
    #[prop(optional, into)]
    indicators_content_class: OptProp<String>,
    /// Встроенный стиль списка индикаторов.
    #[prop(optional, into)]
    indicators_content_style: OptProp<String>,
    /// Дополнительный класс кнопки индикатора.
    #[prop(optional, into)]
    indicator_style_class: OptProp<String>,
    /// Встроенный стиль кнопки индикатора.
    #[prop(optional, into)]
    indicator_style: OptProp<String>,
    /// Бесконечный режим.
    #[prop(optional, into)]
    circular: Prop<bool>,
    /// Показывать индикаторы.
    #[prop(optional, default = Prop::Static(true), into)]
    show_indicators: Prop<bool>,
    /// Показывать кнопки «назад» и «вперёд».
    #[prop(optional, default = Prop::Static(true), into)]
    show_navigators: Prop<bool>,
    /// Период автопрокрутки, мс (`0` — выключена).
    #[prop(optional, into)]
    autoplay_interval: Prop<u32>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Цвет кнопки «назад».
    #[prop(optional, default = Prop::Static(Some(ButtonSeverity::Secondary)), into)]
    prev_button_severity: OptProp<ButtonSeverity>,
    /// Цвет кнопки «вперёд».
    #[prop(optional, default = Prop::Static(Some(ButtonSeverity::Secondary)), into)]
    next_button_severity: OptProp<ButtonSeverity>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<CarouselPt>,
    /// Шаблон элемента.
    item: Callback<T, AnyView>,
    /// Шапка.
    #[prop(optional, into)]
    header: Option<ViewFn>,
    /// Подвал.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Значок кнопки «назад».
    #[prop(optional, into)]
    previous_icon: Option<ViewFn>,
    /// Значок кнопки «вперёд».
    #[prop(optional, into)]
    next_icon: Option<ViewFn>,
    /// Страница изменилась.
    #[prop(optional)]
    on_page: Option<Callback<usize>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let pt = Arc::new(pt.unwrap_or_default());
    let root_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let len = Memo::new({
        let value = value.clone();
        move |_| value.with(Vec::len)
    });
    let circular_now = circular.get_untracked();
    let initial = Track::new(
        page.map_or(0, |p| p.get_untracked()),
        num_visible.get_untracked(),
        num_scroll.get_untracked(),
        len.get_untracked(),
        circular_now,
    );
    let track = RwSignal::new(initial.clone());
    let core = Arc::new(Core {
        track,
        applied: RwSignal::new(initial.total_shifted),
        transitioning: RwSignal::new(false),
        items_hidden: RwSignal::new(false),
        allow_autoplay: RwSignal::new(autoplay_interval.get_untracked() > 0),
        focus_dot: RwSignal::new(initial.page),
        len,
        circular: circular.clone(),
        vertical: orientation.clone(),
        num_visible: num_visible.clone(),
        num_scroll: num_scroll.clone(),
        responsive: Arc::new(responsive_options),
        autoplay_interval: autoplay_interval.clone(),
        page_signal: page,
        on_page,
        start: StoredValue::new((0.0, 0.0)),
        indicators: NodeRef::new(),
        #[cfg(target_arch = "wasm32")]
        interval: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        cleanups: StoredValue::new_local(Vec::new()),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // Пересчёт, адаптивность, автопрокрутка и внешняя смена страницы — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let (c1, nv, ns, circ) = (core.clone(), num_visible.clone(), num_scroll.clone(), circular.clone());
        Effect::new(move |_| {
            let _ = (nv.get(), ns.get(), circ.get(), c1.len.get());
            c1.recompute();
        });
        if !core.responsive.is_empty() {
            let c2 = core.clone();
            let handle = window_event_listener(leptos::ev::resize, move |_| c2.recompute());
            core.cleanups.update_value(|c| c.push(Box::new(move || handle.remove())));
        }
        if let Some(sig) = page {
            let c3 = core.clone();
            Effect::new(move |_| {
                let wanted = sig.get();
                let (len, current) = (c3.len.get_untracked(), c3.track.get_untracked());
                if wanted == current.page {
                    return;
                }
                c3.stop_autoplay(true);
                let dots = current.dots(len);
                if wanted > current.page && wanted < dots {
                    c3.step(Dir::Forward, Some(wanted));
                } else if wanted < current.page {
                    c3.step(Dir::Backward, Some(wanted));
                } else {
                    sig.set(current.page);
                }
            });
        }
    }

    let nv_memo = Memo::new(move |_| track.with(|t| t.num_visible));
    let dots = Memo::new(move |_| track.with(|t| t.dots(len.get())));

    let root_class = {
        let (pt, orientation) = (pt.clone(), orientation.clone());
        move || {
            let extra = style_class.with(|c| c.clone());
            let mut cn = ClassNames::new();
            cn.add("p-carousel p-component")
                .add(if orientation.get().is_vertical() { "p-carousel-vertical" } else { "p-carousel-horizontal" })
                .add_user(extra.as_deref());
            pt.root.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let locale_text = |key: &str| prime_core::use_prime_config().locale.text(key).to_owned();
    let (slide_label, slide_number, page_label) =
        (locale_text("aria.slide"), locale_text("aria.slideNumber"), locale_text("aria.pageLabel"));

    // Элементы: строятся на смену значения, состояние — реактивные атрибуты.
    let items = {
        let (core, pt, value, item) = (core.clone(), pt.clone(), value.clone(), item);
        let (slide_label, slide_number) = (slide_label.clone(), slide_number.clone());
        move || {
            value
                .get()
                .into_iter()
                .enumerate()
                .map(|(index, entry)| real_item(&core, &pt, index, item.run(entry), &slide_label, &slide_number))
                .collect_view()
        }
    };
    let clones = {
        let (core, pt, value, circular) = (core.clone(), pt.clone(), value.clone(), circular.clone());
        move || {
            let n = nv_memo.get();
            let all = value.get();
            if !is_circular(circular.get(), all.len(), n) {
                return ().into_any();
            }
            let start: Vec<T> = all[all.len() - n..].to_vec();
            let finish: Vec<T> = all[..n].to_vec();
            let (core2, pt2) = (core.clone(), pt.clone());
            let starting = start
                .into_iter()
                .enumerate()
                .map(|(index, entry)| clone_item(&core, &pt, index, n, item.run(entry), true, &slide_label, &slide_number))
                .collect_view();
            let finishing = finish
                .into_iter()
                .enumerate()
                .map(|(index, entry)| clone_item(&core2, &pt2, index, n, item.run(entry), false, "", ""))
                .collect_view();
            (starting, finishing).into_any()
        }
    };
    // Кнопки навигации.
    let nav_button = |forward: bool| -> AnyView {
        let (core, pt) = (core.clone(), pt.clone());
        let (severity, icon) =
            if forward { (next_button_severity.clone(), next_icon.clone()) } else { (prev_button_severity.clone(), previous_icon.clone()) };
        let class = {
            let (core, pt) = (core.clone(), pt.clone());
            Signal::derive(move || {
                let (len, circular) = (core.len.get(), core.circular.get());
                let disabled =
                    core.track.with(|t| if forward { t.forward_disabled(circular, len) } else { t.backward_disabled(circular, len) });
                let mut cn = ClassNames::new();
                cn.add(if forward { "p-carousel-next-button" } else { "p-carousel-prev-button" }).add_if(disabled, "p-disabled");
                let part = if forward { &pt.next_button } else { &pt.prev_button };
                Some(part.merge_class(&cn.finish_cow()).into_owned())
            })
        };
        let vertical = core.vertical.clone();
        let icon = icon.unwrap_or_else(|| {
            ViewFn::from(move || match (forward, vertical.get().is_vertical()) {
                (false, false) => view! { <ChevronLeftIcon /> }.into_any(),
                (false, true) => view! { <ChevronUpIcon /> }.into_any(),
                (true, false) => view! { <ChevronRightIcon /> }.into_any(),
                (true, true) => view! { <ChevronDownIcon /> }.into_any(),
            })
        });
        let label = locale_text(if forward { "aria.nextPageLabel" } else { "aria.prevPageLabel" });
        let on_click = {
            let core = core.clone();
            Callback::new(move |e: web_sys::MouseEvent| {
                if forward {
                    core.nav_forward(None);
                } else {
                    core.nav_backward(None);
                }
                if e.cancelable() {
                    e.prevent_default();
                }
            })
        };
        let part = if forward { pt.next_button.clone() } else { pt.prev_button.clone() };
        view! {
            <Button text=true rounded=true severity=severity style_class=class aria_label=label icon=icon on_click=on_click pt=ButtonPt { root: part, ..Default::default() } />
        }
        .into_any()
    };
    let (prev, next) = if show_navigators.get_untracked() { (Some(nav_button(false)), Some(nav_button(true))) } else { (None, None) };

    let viewport_style = {
        let (vertical, height) = (orientation.clone(), vertical_view_port_height.clone());
        move || if vertical.get().is_vertical() { format!("height: {}", height.get()) } else { "height: auto".to_owned() }
    };
    let list_class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-carousel-item-list").add_if(core.items_hidden.get(), "p-items-hidden");
            pt.item_list.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let list_style = {
        let core = core.clone();
        move || {
            let transform = translate(core.applied.get(), core.track.with(|t| t.num_visible), core.vertical.get().is_vertical());
            let base = format!("transform: {transform}");
            if core.transitioning.get() { format!("{base}; transition: transform 500ms ease 0s") } else { base }
        }
    };
    let on_transition_end = {
        let core = core.clone();
        move |_| core.transition_end()
    };
    let touch_start = {
        let core = core.clone();
        move |e: web_sys::TouchEvent| {
            if let Some(t) = e.changed_touches().get(0) {
                core.start.set_value((f64::from(t.page_x()), f64::from(t.page_y())));
            }
        }
    };
    let touch_end = {
        let core = core.clone();
        move |e: web_sys::TouchEvent| {
            if let Some(t) = e.changed_touches().get(0) {
                let (sx, sy) = core.start.get_value();
                let diff =
                    if core.vertical.get_untracked().is_vertical() { f64::from(t.page_y()) - sy } else { f64::from(t.page_x()) - sx };
                core.swipe_diff(diff);
            }
        }
    };
    let touch_move = |e: web_sys::TouchEvent| {
        if e.cancelable() {
            e.prevent_default();
        }
    };

    // Индикаторы.
    let indicators = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            show_indicators.get().then(|| {
                let (indicators_content_class, indicators_content_style) =
                    (indicators_content_class.clone(), indicators_content_style.clone());
                let (indicator_style_class, indicator_style) = (indicator_style_class.clone(), indicator_style.clone());
                let (list_core, key_core) = (core.clone(), core.clone());
                let list_class = {
                    let pt = pt.clone();
                    move || {
                        let extra = indicators_content_class.with(|c| c.clone());
                        let mut cn = ClassNames::new();
                        cn.add("p-carousel-indicator-list").add_user(extra.as_deref());
                        pt.indicator_list.merge_class(&cn.finish_cow()).into_owned()
                    }
                };
                let list_style = {
                    let pt = pt.clone();
                    move || {
                        let merged = indicators_content_style.with(|s| pt.indicator_list.merge_style(s.as_deref().unwrap_or_default()));
                        (!merged.is_empty()).then_some(merged)
                    }
                };
                let on_key = move |e: web_sys::KeyboardEvent| {
                    let delta = match e.code().as_str() {
                        "ArrowRight" => 1,
                        "ArrowLeft" => -1,
                        _ => return,
                    };
                    let next = indicator_focus(key_core.focus_dot.get_untracked(), dots.get_untracked(), delta);
                    key_core.focus_dot.set(next);
                    key_core.focus_indicator(next);
                };
                let (page_label, dot_pt) = (page_label.clone(), pt.clone());
                let dot_views = move || {
                    (0..dots.get())
                        .map(|i| dot_view(&list_core, &dot_pt, i, &page_label, indicator_style_class.clone(), indicator_style.clone()))
                        .collect_view()
                };
                let list_attrs = pt.indicator_list.attrs();
                view! {
                    <ul node_ref=core.indicators class={list_class} style={list_style} on:keydown={on_key} {..list_attrs}>
                        {dot_views}
                    </ul>
                }
            })
        }
    };

    let header_view = header.map(|h| {
        let class = pt.header.merge_class("p-carousel-header").into_owned();
        view! { <div class={class} {..pt.header.attrs()}>{h.run()}</div> }
    });
    let footer_view = footer.map(|f| {
        let class = pt.footer.merge_class("p-carousel-footer").into_owned();
        view! { <div class={class} {..pt.footer.attrs()}>{f.run()}</div> }
    });
    let container_class = {
        let pt = pt.clone();
        move || {
            let extra = content_class.with(|c| c.clone());
            let mut cn = ClassNames::new();
            cn.add("p-carousel-content-container").add_user(extra.as_deref());
            pt.content_container.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content_live = {
        let core = core.clone();
        move || if core.allow_autoplay.get() { "polite" } else { "off" }
    };
    let (root_attrs, container_attrs, content_attrs, viewport_attrs, list_attrs) =
        (pt.root.attrs(), pt.content_container.attrs(), pt.content.attrs(), pt.viewport.attrs(), pt.item_list.attrs());
    let content_class_attr = pt.content.merge_class("p-carousel-content").into_owned();
    let viewport_class = pt.viewport.merge_class("p-carousel-viewport").into_owned();
    view! {
        <div id={root_id} role="region" class={root_class} style={root_style} {..root_attrs}>
            {header_view}
            <div class={container_class} {..container_attrs}>
                <div class={content_class_attr} aria-live={content_live} {..content_attrs}>
                    {prev}
                    <div
                        class={viewport_class}
                        style={viewport_style}
                        on:touchstart={touch_start}
                        on:touchend={touch_end}
                        on:touchmove={touch_move}
                        {..viewport_attrs}
                    >
                        <div class={list_class} style={list_style} on:transitionend={on_transition_end} {..list_attrs}>
                            {items}
                            {clones}
                        </div>
                    </div>
                    {next}
                </div>
                {indicators}
            </div>
            {footer_view}
        </div>
    }
}

/// Точка-индикатор страницы.
fn dot_view(
    core: &Arc<Core>,
    pt: &Arc<CarouselPt>,
    index: usize,
    page_label: &str,
    style_class: OptProp<String>,
    style: OptProp<String>,
) -> AnyView {
    let track = core.track;
    let li_class = {
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-carousel-indicator").add_if(track.with(|t| t.page == index), "p-carousel-indicator-active");
            pt.indicator.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let button_class = {
        let pt = pt.clone();
        move || {
            let extra = style_class.with(|c| c.clone());
            let mut cn = ClassNames::new();
            cn.add("p-carousel-indicator-button").add_user(extra.as_deref());
            pt.indicator_button.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let button_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.indicator_button.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let label = fill(page_label, "page", index + 1);
    let (focus_dot, click_core) = (core.focus_dot, core.clone());
    let (li_attrs, button_attrs) = (pt.indicator.attrs(), pt.indicator_button.attrs());
    view! {
        <li class={li_class} data-p-active={move || track.with(|t| t.page == index).to_string()} {..li_attrs}>
            <button
                type="button"
                class={button_class}
                style={button_style}
                aria-label={label}
                aria-current={move || track.with(|t| t.page == index).then_some("page")}
                tabindex={move || if focus_dot.get() == index { "0" } else { "-1" }}
                on:click={move |_| click_core.dot_click(index)}
                {..button_attrs}
            ></button>
        </li>
    }
    .into_any()
}

/// Реальный элемент.
fn real_item(core: &Arc<Core>, pt: &Arc<CarouselPt>, index: usize, content: AnyView, slide_label: &str, slide_number: &str) -> AnyView {
    let (track, len, circular) = (core.track, core.len, core.circular.clone());
    let range = move || {
        let (first, last) = track.with(|t| (t.first_index(circular.get(), len.get()), t.last_index(circular.get(), len.get())));
        (first, last)
    };
    let idx = index as i32;
    let active = {
        let range = range.clone();
        move || {
            let (f, l) = range();
            f <= idx && l >= idx
        }
    };
    let class = {
        let (range, pt) = (range.clone(), pt.clone());
        move || {
            let (f, l) = range();
            let mut cn = ClassNames::new();
            cn.add("p-carousel-item")
                .add_if(f <= idx && l >= idx, "p-carousel-item-active")
                .add_if(f == idx, "p-carousel-item-start")
                .add_if(l == idx, "p-carousel-item-end");
            pt.item.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let flex = {
        let pt = pt.clone();
        move || pt.item.merge_style(&format!("flex: {}", item_flex(track.with(|t| t.num_visible))))
    };
    let attrs = pt.item.attrs();
    let label = fill(slide_number, "slideNumber", index + 1);
    let role = slide_label.to_owned();
    let (hidden, a1) = (active.clone(), active.clone());
    let start = {
        let range = range.clone();
        move || (range().0 == idx).to_string()
    };
    let end = move || (range().1 == idx).to_string();
    view! {
        <div
            class={class}
            style={flex}
            role="group"
            aria-hidden={move || (!hidden()).to_string()}
            aria-label={label}
            aria-roledescription={role}
            data-p-carousel-item-active={move || a1().to_string()}
            data-p-carousel-item-start={start}
            data-p-carousel-item-end={end}
            {..attrs}
        >
            {content}
        </div>
    }
    .into_any()
}

/// Клон элемента (бесконечный режим); у клонов начала есть `aria`-подписи, у клонов конца — нет.
#[allow(clippy::too_many_arguments)]
fn clone_item(
    core: &Arc<Core>,
    pt: &Arc<CarouselPt>,
    index: usize,
    clones: usize,
    content: AnyView,
    starting: bool,
    slide_label: &str,
    slide_number: &str,
) -> AnyView {
    let (track, len) = (core.track, core.len);
    let class = {
        let pt = pt.clone();
        move || {
            let active = track.with(|t| -t.total_shifted == len.get() as i32);
            let mut cn = ClassNames::new();
            cn.add("p-carousel-item p-carousel-item-clone")
                .add_if(active, "p-carousel-item-active")
                .add_if(index == 0, "p-carousel-item-start")
                .add_if(index + 1 == clones, "p-carousel-item-end");
            pt.item_clone.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let flex = {
        let pt = pt.clone();
        move || pt.item_clone.merge_style(&format!("flex: {}", item_flex(track.with(|t| t.num_visible))))
    };
    let attrs = pt.item_clone.attrs();
    if starting {
        let label = fill(slide_number, "slideNumber", index + 1);
        let role = slide_label.to_owned();
        let hidden = move || (!track.with(|t| -t.total_shifted == len.get() as i32)).to_string();
        let active = move || track.with(|t| -t.total_shifted == len.get() as i32 + t.num_visible as i32).to_string();
        let end = (index + 1 == clones).to_string();
        view! {
            <div class={class} style={flex} aria-hidden={hidden} aria-label={label} aria-roledescription={role} data-p-carousel-item-active={active} data-p-carousel-item-start={(index == 0).to_string()} data-p-carousel-item-end={end} {..attrs}>
                {content}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class={class} style={flex} data-p-carousel-item-active="false" data-p-carousel-item-start="false" data-p-carousel-item-end="false" {..attrs}>
                {content}
            </div>
        }
        .into_any()
    }
}
