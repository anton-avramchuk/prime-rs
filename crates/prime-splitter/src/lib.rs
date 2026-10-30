//! Splitter — порт `p-splitter` из PrimeNG 21.1.10.
//!
//! Панели в ряд или столбец, между ними разделители: размеры меняются перетаскиванием (мышь, касание) и стрелками на ручке.
//! Содержимое панелей — `panels` (вместо `ng-template #panel`). Размеры хранятся в процентах и выводятся как
//! `flex-basis: calc(<доля>% - <(n-1)·gutter_size>px)`; при заданном `state_key` они сохраняются в хранилище браузера.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_splitter::Splitter;
//!
//! let panels = vec![ViewFn::from(|| view! { <p>"One"</p> }), ViewFn::from(|| view! { <p>"Two"</p> })];
//! let html = view! { <Splitter panels=panels panel_sizes=vec![30.0, 70.0] /> }.to_html();
//! assert!(html.contains("p-splitter-horizontal") && html.contains("calc(30% - 4px)") && html.contains("calc(70% - 4px)"));
//! ```

mod state;

use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{
    SplitterLayout, SplitterStorage, aria_value, data_p, drag_delta, flex_basis, gutter_classes, handle_style, initial_sizes, is_arrow,
    is_valid_resize, key_step, panel_classes, parse_json, root_classes, root_style, to_json,
};

/// Событие изменения размеров.
#[derive(Clone, Debug, PartialEq)]
pub struct SplitterResizeEvent {
    /// Размеры панелей в процентах.
    pub sizes: Vec<f64>,
}

/// Pass-through частей [`Splitter`].
#[derive(Clone, Debug, Default)]
pub struct SplitterPt {
    /// Корень.
    pub root: Pt,
    /// Панель.
    pub panel: Pt,
    /// Разделитель.
    pub gutter: Pt,
    /// Ручка разделителя.
    pub gutter_handle: Pt,
}

/// Перетаскивание в процессе.
#[derive(Clone, Copy)]
struct Drag {
    index: usize,
    start: f64,
    prev: f64,
    next: f64,
    size: f64,
    rtl: bool,
}

/// Состояние и действия одного экземпляра.
#[derive(Clone, Copy)]
struct Ctl {
    layout: SplitterLayout,
    step: f64,
    storage: SplitterStorage,
    sizes: RwSignal<Vec<f64>>,
    resizing: RwSignal<Option<usize>>,
    drag: StoredValue<Option<Drag>>,
    key_active: StoredValue<bool>,
    min_sizes: StoredValue<Vec<f64>>,
    state_key: StoredValue<Option<String>>,
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    root_ref: NodeRef<html::Div>,
    on_start: Option<Callback<SplitterResizeEvent>>,
    on_end: Option<Callback<SplitterResizeEvent>>,
}

impl Ctl {
    fn horizontal(&self) -> bool {
        self.layout == SplitterLayout::Horizontal
    }

    /// Размер корня по оси раскладки и направление письма.
    fn measure(&self) -> Option<(f64, bool)> {
        let root = self.root_ref.get_untracked()?;
        let rect = web_sys::Element::get_bounding_client_rect(&root);
        let size = if self.horizontal() { rect.width() } else { rect.height() };
        let rtl = web_sys::window()
            .and_then(|w| w.get_computed_style(&root).ok().flatten())
            .and_then(|style| style.get_property_value("direction").ok())
            .is_some_and(|direction| direction == "rtl");
        (size > 0.0).then_some((size, rtl))
    }

    fn begin(&self, index: usize) {
        self.resizing.set(Some(index));
        if let Some(callback) = self.on_start {
            callback.run(SplitterResizeEvent { sizes: self.sizes.get_untracked() });
        }
    }

    /// Применяет новые размеры пары `(index, index + 1)`, если они допустимы.
    fn apply(&self, index: usize, new_prev: f64, new_next: f64) {
        let allowed = self.min_sizes.with_value(|min| {
            is_valid_resize(new_prev, new_next, min.get(index).copied().unwrap_or(0.0), min.get(index + 1).copied().unwrap_or(0.0))
        });
        if allowed {
            self.sizes.update(|sizes| {
                sizes[index] = new_prev;
                sizes[index + 1] = new_next;
            });
        }
    }

    fn start_drag(&self, index: usize, pos: f64) {
        let Some((size, rtl)) = self.measure() else { return };
        let sizes = self.sizes.get_untracked();
        self.drag.set_value(Some(Drag { index, start: pos, prev: sizes[index], next: sizes[index + 1], size, rtl }));
        self.begin(index);
    }

    fn moved(&self, pos: f64) {
        let Some(drag) = self.drag.get_value() else { return };
        let delta = drag_delta(drag.start, pos, drag.size, drag.rtl, self.horizontal());
        self.apply(drag.index, drag.prev + delta, drag.next - delta);
    }

    fn stop_listening(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
    }

    fn finish(&self) {
        self.resizing.set(None);
        self.drag.set_value(None);
        self.save();
        if let Some(callback) = self.on_end {
            callback.run(SplitterResizeEvent { sizes: self.sizes.get_untracked() });
        }
    }

    fn storage_handle(&self) -> Option<web_sys::Storage> {
        let window = web_sys::window()?;
        match self.storage {
            SplitterStorage::Session => window.session_storage().ok().flatten(),
            SplitterStorage::Local => window.local_storage().ok().flatten(),
        }
    }

    fn save(&self) {
        let Some(key) = self.state_key.get_value() else { return };
        if let Some(storage) = self.storage_handle() {
            let _ = storage.set_item(&key, &to_json(&self.sizes.get_untracked()));
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn restore(&self) {
        let Some(key) = self.state_key.get_value() else { return };
        let saved = self.storage_handle().and_then(|storage| storage.get_item(&key).ok().flatten()).and_then(|text| parse_json(&text));
        if let Some(saved) = saved.filter(|saved| saved.len() == self.sizes.with_untracked(Vec::len)) {
            self.sizes.set(saved);
        }
    }

    fn mouse_down(&self, index: usize, event: &web_sys::MouseEvent) {
        let pos = |e: &web_sys::MouseEvent, horizontal: bool| f64::from(if horizontal { e.page_x() } else { e.page_y() });
        self.start_drag(index, pos(event, self.horizontal()));
        let (this, horizontal) = (*self, self.horizontal());
        let moving = window_event_listener(leptos::ev::mousemove, move |e| this.moved(pos(&e, horizontal)));
        let up = window_event_listener(leptos::ev::mouseup, move |_| {
            this.stop_listening();
            this.finish();
        });
        self.listeners.set_value(vec![moving, up]);
    }

    fn touch_start(&self, index: usize, event: &web_sys::TouchEvent) {
        let pos = |e: &web_sys::TouchEvent, horizontal: bool| {
            e.changed_touches().get(0).map_or(0.0, |t| f64::from(if horizontal { t.page_x() } else { t.page_y() }))
        };
        self.start_drag(index, pos(event, self.horizontal()));
        let (this, horizontal) = (*self, self.horizontal());
        let moving = window_event_listener(leptos::ev::touchmove, move |e| this.moved(pos(&e, horizontal)));
        let end = window_event_listener(leptos::ev::touchend, move |_| {
            this.stop_listening();
            this.finish();
        });
        let cancel = window_event_listener(leptos::ev::touchcancel, move |_| {
            this.stop_listening();
            this.finish();
        });
        self.listeners.set_value(vec![moving, end, cancel]);
    }

    fn key_down(&self, index: usize, event: &web_sys::KeyboardEvent) {
        let key = event.key();
        if let Some(step) = key_step(self.layout, &key, self.step)
            && let Some((size, _)) = self.measure()
        {
            if !self.key_active.get_value() {
                self.key_active.set_value(true);
                self.begin(index);
            }
            let delta = 100.0 * step / size;
            let sizes = self.sizes.get_untracked();
            self.apply(index, sizes[index] + delta, sizes[index + 1] - delta);
        }
        if is_arrow(&key) {
            event.prevent_default();
        }
    }

    fn key_up(&self) {
        if self.key_active.get_value() {
            self.key_active.set_value(false);
            self.finish();
        }
    }
}

/// Панели с изменяемыми размерами.
///
/// Число панелей, раскладка, шаг и ограничения читаются один раз при создании.
#[component]
pub fn Splitter(
    /// Содержимое панелей по порядку.
    panels: Vec<ViewFn>,
    /// Раскладка (по умолчанию в ряд).
    #[prop(optional)]
    layout: SplitterLayout,
    /// Начальные размеры в процентах; недостающие и нулевые — поровну.
    #[prop(optional)]
    panel_sizes: Vec<f64>,
    /// Минимальные размеры в процентах; нулевые и отсутствующие не ограничивают.
    #[prop(optional)]
    min_sizes: Vec<f64>,
    /// Толщина разделителя в пикселях.
    #[prop(default = 4.0)]
    gutter_size: f64,
    /// Шаг клавиш в пикселях.
    #[prop(default = 5.0)]
    step: f64,
    /// Ключ хранилища; без него состояние не сохраняется.
    #[prop(optional, into)]
    state_key: Option<String>,
    /// Хранилище (по умолчанию `sessionStorage`).
    #[prop(optional)]
    state_storage: SplitterStorage,
    /// Панели содержат вложенный Splitter (в PrimeNG определяется автоматически).
    #[prop(optional, into)]
    nested: Prop<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль панелей.
    #[prop(optional, into)]
    panel_style: OptProp<String>,
    /// Дополнительный класс панелей.
    #[prop(optional, into)]
    panel_style_class: OptProp<String>,
    /// Вызывается в начале изменения размеров.
    #[prop(optional, into)]
    on_resize_start: Option<Callback<SplitterResizeEvent>>,
    /// Вызывается в конце изменения размеров.
    #[prop(optional, into)]
    on_resize_end: Option<Callback<SplitterResizeEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<SplitterPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let n = panels.len();
    let root_ref = NodeRef::<html::Div>::new();
    let ctl = Ctl {
        layout,
        step,
        storage: state_storage,
        sizes: RwSignal::new(initial_sizes(&panel_sizes, n)),
        resizing: RwSignal::new(None),
        drag: StoredValue::new(None),
        key_active: StoredValue::new(false),
        min_sizes: StoredValue::new(min_sizes),
        state_key: StoredValue::new(state_key),
        listeners: StoredValue::new_local(Vec::new()),
        root_ref,
        on_start: on_resize_start,
        on_end: on_resize_end,
    };
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| ctl.restore());
    on_cleanup(move || ctl.stop_listening());

    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = ClassNames::new();
            cn.add(&root_classes(layout, ctl.resizing.get().is_some()));
            style_class.with(|c| cn.add_user(c.as_deref()));
            pt.root.merge_class(&cn.finish())
        })
    };
    let root_style_value = {
        let pt = pt.clone();
        Signal::derive(move || {
            let user = style.get().unwrap_or_default();
            pt.root.merge_style_parts(&[root_style(layout), &user])
        })
    };
    let data = {
        let nested = nested.clone();
        Signal::derive(move || data_p(layout, nested.get()))
    };

    let items = panels
        .into_iter()
        .enumerate()
        .flat_map(|(i, panel)| {
            let panel_class = {
                let (pt, nested, panel_style_class) = (pt.clone(), nested.clone(), panel_style_class.clone());
                Signal::derive(move || {
                    let mut cn = ClassNames::new();
                    cn.add(panel_classes(nested.get()));
                    panel_style_class.with(|c| cn.add_user(c.as_deref()));
                    pt.panel.merge_class(&cn.finish())
                })
            };
            let panel_style_value = {
                let (pt, panel_style) = (pt.clone(), panel_style.clone());
                Signal::derive(move || {
                    let user = panel_style.get().unwrap_or_default();
                    let basis = format!("flex-basis: {}", flex_basis(ctl.sizes.with(|s| s[i]), n, gutter_size));
                    pt.panel.merge_style_parts(&[&user, &basis])
                })
            };
            let panel_view = view! {
                <div class={panel_class} style={panel_style_value} tabindex="-1" data-pc-section="panel" {..pt.panel.attrs()}>
                    {panel.run()}
                </div>
            }
            .into_any();
            let gutter_view = (i + 1 < n).then(|| {
                let gutter_class = {
                    let pt = pt.clone();
                    move || pt.gutter.merge_class(gutter_classes(ctl.resizing.get() == Some(i)))
                };
                let handle_class = pt.gutter_handle.merge_class("p-splitter-gutter-handle");
                let handle_style_value = pt.gutter_handle.merge_style(&handle_style(layout, gutter_size));
                view! {
                    <div
                        class={gutter_class}
                        role="separator"
                        tabindex="-1"
                        data-p-gutter-resizing={move || (ctl.resizing.get() == Some(i)).to_string()}
                        data-p={move || data.get()}
                        on:mousedown=move |ev| ctl.mouse_down(i, &ev)
                        on:touchstart=move |ev| ctl.touch_start(i, &ev)
                        {..pt.gutter.attrs()}
                    >
                        <div
                            class={handle_class}
                            style={handle_style_value}
                            tabindex="0"
                            aria-orientation={layout.as_str()}
                            aria-valuenow={move || aria_value(ctl.sizes.with(|s| s[i]))}
                            on:keydown=move |ev| ctl.key_down(i, &ev)
                            on:keyup=move |_| ctl.key_up()
                            {..pt.gutter_handle.attrs()}
                        ></div>
                    </div>
                }
                .into_any()
            });
            [Some(panel_view), gutter_view]
        })
        .flatten()
        .collect_view();

    view! {
        <div node_ref={root_ref} class={class} style={root_style_value} data-p-gutter-resizing="false" data-p={move || data.get()} {..pt.root.attrs()}>
            {items}
        </div>
    }
}
