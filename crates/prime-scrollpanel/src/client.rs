//! Браузерная часть ScrollPanel: пересчёт полос, перетаскивание, клавиатура, слушатели.
//!
//! Состояние (`ratio`, последние координаты, ось) живёт в обычных `Cell`, а не в сигналах: пересчёт полос не должен
//! вызывать перерисовку Leptos. `style` и служебные атрибуты полос пишутся прямо в DOM.

use crate::{KeyAction, bar_len, bar_offset, bar_ratio, clamp_scroll, key_action};
use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use web_sys::{Event, EventTarget, HtmlElement, KeyboardEvent, MouseEvent};

/// Задержка перед прокруткой клавишей, мс.
const KEY_DELAY_MS: i32 = 40;

/// Набор подписок; при `Drop` все слушатели снимаются.
#[derive(Default)]
struct Listeners(Vec<Subscription>);

type Subscription = (EventTarget, &'static str, Closure<dyn FnMut(Event)>);

impl Listeners {
    fn add(&mut self, target: &EventTarget, name: &'static str, f: impl FnMut(Event) + 'static) {
        let closure = Closure::<dyn FnMut(Event)>::new(f);
        if target.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref()).is_ok() {
            self.0.push((target.clone(), name, closure));
        }
    }
}

impl Drop for Listeners {
    fn drop(&mut self) {
        for (target, name, closure) in self.0.drain(..) {
            let _ = target.remove_event_listener_with_callback(name, closure.as_ref().unchecked_ref());
        }
    }
}

/// Живой экземпляр ScrollPanel в браузере.
pub(crate) struct Client {
    root: HtmlElement,
    content: HtmlElement,
    bar_x: HtmlElement,
    bar_y: HtmlElement,
    step: Rc<dyn Fn() -> f64>,
    ratio_x: Cell<f64>,
    ratio_y: Cell<f64>,
    last_page_x: Cell<f64>,
    last_page_y: Cell<f64>,
    x_grabbed: Cell<bool>,
    y_grabbed: Cell<bool>,
    last_scroll_left: Cell<i32>,
    last_scroll_top: Cell<i32>,
    horizontal: Cell<bool>,
    timer: Cell<Option<i32>>,
    own: RefCell<Listeners>,
    document: RefCell<Option<Listeners>>,
}

/// Привязывает клиентскую часть после монтирования и снимает всё при размонтировании.
pub(crate) fn mount(
    root: NodeRef<Div>,
    content: NodeRef<Div>,
    bar_x: NodeRef<Div>,
    bar_y: NodeRef<Div>,
    step: Rc<dyn Fn() -> f64>,
    panel_ref: Option<crate::ScrollPanelRef>,
) {
    let handle = StoredValue::new_local(None::<Rc<Client>>);
    Effect::new(move |_| {
        let (Some(root), Some(content), Some(bar_x), Some(bar_y)) = (root.get(), content.get(), bar_x.get(), bar_y.get()) else { return };
        if handle.with_value(|h| h.is_some()) {
            return;
        }
        let client = Client::attach((*root).clone(), (*content).clone(), (*bar_x).clone(), (*bar_y).clone(), step.clone());
        handle.set_value(Some(client.clone()));
        if let Some(r) = panel_ref {
            r.inner.set_value(Some(client));
        }
    });
    on_cleanup(move || {
        if let Some(client) = handle.try_update_value(Option::take).flatten() {
            client.dispose();
        }
        if let Some(r) = panel_ref {
            r.inner.try_set_value(None);
        }
    });
}

fn window() -> Option<web_sys::Window> {
    web_sys::window()
}

/// Выполняет `f` в кадре анимации; без `requestAnimationFrame` — через `setTimeout(0)`.
fn next_frame(f: impl FnOnce() + 'static) {
    let Some(win) = window() else { return };
    let cb = Closure::once_into_js(f);
    if win.request_animation_frame(cb.unchecked_ref()).is_err() {
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), 0);
    }
}

/// Значение CSS-свойства в пикселях, как `parseFloat` (нечисловое — 0).
fn px(style: &web_sys::CssStyleDeclaration, name: &str) -> f64 {
    let value = style.get_property_value(name).unwrap_or_default();
    let end = value.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).unwrap_or(value.len());
    value[..end].parse().unwrap_or(0.0)
}

fn set_hidden(bar: &HtmlElement, hidden: bool) {
    let _ = bar.set_attribute("data-p-scrollpanel-hidden", if hidden { "true" } else { "false" });
    let _ = bar.class_list().toggle_with_force("p-scrollpanel-hidden", hidden);
}

fn set_grabbed(bar: &HtmlElement, body: Option<&HtmlElement>, grabbed: bool) {
    let value = if grabbed { "true" } else { "false" };
    for el in std::iter::once(bar).chain(body) {
        let _ = el.set_attribute("data-p-scrollpanel-grabbed", value);
        let _ = el.class_list().toggle_with_force("p-scrollpanel-grabbed", grabbed);
    }
}

impl Client {
    fn attach(root: HtmlElement, content: HtmlElement, bar_x: HtmlElement, bar_y: HtmlElement, step: Rc<dyn Fn() -> f64>) -> Rc<Self> {
        let this = Rc::new(Self {
            root,
            content,
            bar_x,
            bar_y,
            step,
            ratio_x: Cell::new(1.0),
            ratio_y: Cell::new(1.0),
            last_page_x: Cell::new(0.0),
            last_page_y: Cell::new(0.0),
            x_grabbed: Cell::new(false),
            y_grabbed: Cell::new(false),
            last_scroll_left: Cell::new(0),
            last_scroll_top: Cell::new(0),
            horizontal: Cell::new(false),
            timer: Cell::new(None),
            own: RefCell::default(),
            document: RefCell::new(None),
        });
        this.move_bar();
        let mut own = Listeners::default();
        let content: EventTarget = this.content.clone().into();
        let (x, y): (EventTarget, EventTarget) = (this.bar_x.clone().into(), this.bar_y.clone().into());
        let on = |f: fn(&Rc<Client>, Event)| {
            let this = this.clone();
            move |e| f(&this, e)
        };
        own.add(&content, "scroll", on(Client::on_scroll));
        own.add(&content, "mouseenter", on(|c, _| c.move_bar()));
        if let Some(win) = window() {
            own.add(&win.into(), "resize", on(|c, _| c.move_bar()));
        }
        own.add(&x, "mousedown", on(|c, e| c.on_bar_mouse_down(true, e)));
        own.add(&y, "mousedown", on(|c, e| c.on_bar_mouse_down(false, e)));
        for (bar, horizontal) in [(&x, true), (&y, false)] {
            let this = this.clone();
            own.add(bar, "keydown", {
                let this = this.clone();
                move |e| this.on_key_down(e)
            });
            own.add(bar, "keyup", {
                let this = this.clone();
                move |_| this.clear_timer()
            });
            own.add(bar, "focus", {
                let this = this.clone();
                move |_| this.horizontal.set(horizontal)
            });
            own.add(bar, "blur", move |_| {
                if this.horizontal.get() {
                    this.horizontal.set(false);
                }
            });
        }
        *this.own.borrow_mut() = own;
        this.calculate_container_height();
        this
    }

    /// Снимает все слушатели и таймер (размонтирование). Разрывает цикл `Rc` между экземпляром и замыканиями.
    pub(crate) fn dispose(&self) {
        self.clear_timer();
        *self.own.borrow_mut() = Listeners::default();
        *self.document.borrow_mut() = None;
        self.release_grab();
    }

    /// Высота корня при `max-height` без собственной высоты (как `calculateContainerHeight`).
    fn calculate_container_height(&self) {
        let Some(win) = window() else { return };
        let (Ok(Some(cs)), Ok(Some(bar_cs))) = (win.get_computed_style(&self.root), win.get_computed_style(&self.bar_x)) else { return };
        let (pad_top, pad_bottom) = (px(&cs, "padding-top"), px(&cs, "padding-bottom"));
        let (border_top, border_bottom) = (px(&cs, "border-top-width"), px(&cs, "border-bottom-width"));
        let bar_height = px(&bar_cs, "height").trunc();
        let pure_height = self.root.offset_height() as f64 - pad_top - pad_bottom - border_top - border_bottom - bar_height;
        let max_height = cs.get_property_value("max-height").unwrap_or_default();
        if max_height != "none" && pure_height == 0.0 {
            let content_height = self.content.offset_height() as f64;
            let height = if content_height + bar_height > px(&cs, "max-height").trunc() {
                max_height
            } else {
                format!("{}px", content_height + pad_top + pad_bottom + border_top + border_bottom)
            };
            let _ = self.root.style().set_property("height", &height);
        }
    }

    /// Пересчитывает размер и положение полос (кадром анимации, как в PrimeNG).
    pub(crate) fn move_bar(self: &Rc<Self>) {
        let (c, root) = (&self.content, &self.root);
        let (total_w, own_w) = (c.scroll_width() as f64, c.client_width() as f64);
        let (total_h, own_h) = (c.scroll_height() as f64, c.client_height() as f64);
        let bottom = -((root.client_height() - self.bar_x.client_height()) as f64);
        let right = -((root.client_width() - self.bar_y.client_width()) as f64);
        let (ratio_x, ratio_y) = (bar_ratio(own_w, total_w), bar_ratio(own_h, total_h));
        self.ratio_x.set(ratio_x);
        self.ratio_y.set(ratio_y);
        let this = self.clone();
        next_frame(move || {
            if ratio_x >= 1.0 {
                set_hidden(&this.bar_x, true);
            } else {
                set_hidden(&this.bar_x, false);
                let len = bar_len(ratio_x);
                let left = bar_offset(this.content.scroll_left() as f64, len, total_w, own_w).abs();
                this.bar_x.style().set_css_text(&format!("width:{len}%; inset-inline-start:{left}%;bottom:{bottom}px;"));
            }
            if ratio_y >= 1.0 {
                set_hidden(&this.bar_y, true);
            } else {
                set_hidden(&this.bar_y, false);
                let len = bar_len(ratio_y);
                let top = bar_offset(this.content.scroll_top() as f64, len, total_h, own_h);
                let x_height = this.bar_x.client_height();
                this.bar_y.style().set_css_text(&format!("height:{len}%; top: calc({top}% - {x_height}px); inset-inline-end:{right}px;"));
            }
        });
    }

    fn on_scroll(self: &Rc<Self>, _: Event) {
        let (left, top) = (self.content.scroll_left(), self.content.scroll_top());
        if self.last_scroll_left.get() != left {
            self.last_scroll_left.set(left);
            self.horizontal.set(true);
            let _ = self.bar_x.set_attribute("aria-valuenow", &left.to_string());
        } else if self.last_scroll_top.get() != top {
            self.last_scroll_top.set(top);
            self.horizontal.set(false);
            let _ = self.bar_y.set_attribute("aria-valuenow", &top.to_string());
        }
        self.move_bar();
    }

    fn on_key_down(self: &Rc<Self>, event: Event) {
        let Ok(key) = event.clone().dyn_into::<KeyboardEvent>() else { return };
        match key_action(self.horizontal.get(), &key.code()) {
            KeyAction::Scroll { horizontal, delta } => {
                self.set_timer(horizontal, delta * (self.step)());
                event.prevent_default();
            }
            KeyAction::Block => event.prevent_default(),
            KeyAction::Ignore => {}
        }
    }

    fn set_timer(self: &Rc<Self>, horizontal: bool, step: f64) {
        self.clear_timer();
        let Some(win) = window() else { return };
        let this = self.clone();
        let cb = Closure::once_into_js(move || {
            this.timer.set(None);
            if horizontal {
                this.content.set_scroll_left((this.content.scroll_left() as f64 + step).round() as i32);
            } else {
                this.content.set_scroll_top((this.content.scroll_top() as f64 + step).round() as i32);
            }
            this.move_bar();
        });
        if let Ok(id) = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), KEY_DELAY_MS) {
            self.timer.set(Some(id));
        }
    }

    fn clear_timer(&self) {
        if let (Some(id), Some(win)) = (self.timer.take(), window()) {
            win.clear_timeout_with_handle(id);
        }
    }

    fn on_bar_mouse_down(self: &Rc<Self>, horizontal: bool, event: Event) {
        let Ok(mouse) = event.clone().dyn_into::<MouseEvent>() else { return };
        let (bar, grabbed, last, page) = if horizontal {
            (&self.bar_x, &self.x_grabbed, &self.last_page_x, mouse.page_x())
        } else {
            (&self.bar_y, &self.y_grabbed, &self.last_page_y, mouse.page_y())
        };
        grabbed.set(true);
        let _ = bar.focus();
        last.set(page as f64);
        set_grabbed(bar, body().as_ref(), true);
        self.bind_document();
        event.prevent_default();
    }

    fn bind_document(self: &Rc<Self>) {
        if self.document.borrow().is_some() {
            return;
        }
        let Some(doc) = window().and_then(|w| w.document()) else { return };
        let target: EventTarget = doc.into();
        let mut listeners = Listeners::default();
        let this = self.clone();
        listeners.add(&target, "mousemove", move |e| this.on_document_mouse_move(e));
        let this = self.clone();
        listeners.add(&target, "mouseup", move |_| this.on_document_mouse_up());
        *self.document.borrow_mut() = Some(listeners);
    }

    fn on_document_mouse_move(self: &Rc<Self>, event: Event) {
        let Ok(mouse) = event.dyn_into::<MouseEvent>() else { return };
        let (x, y) = (mouse.page_x() as f64, mouse.page_y() as f64);
        let (move_x, move_y) = match (self.x_grabbed.get(), self.y_grabbed.get()) {
            (true, _) => (true, false),
            (false, true) => (false, true),
            (false, false) => (true, true),
        };
        if move_x {
            let delta = x - self.last_page_x.replace(x);
            let this = self.clone();
            next_frame(move || {
                this.content.set_scroll_left((this.content.scroll_left() as f64 + delta / this.ratio_x.get()).round() as i32)
            });
        }
        if move_y {
            let delta = y - self.last_page_y.replace(y);
            let this = self.clone();
            next_frame(move || this.content.set_scroll_top((this.content.scroll_top() as f64 + delta / this.ratio_y.get()).round() as i32));
        }
    }

    fn on_document_mouse_up(&self) {
        self.release_grab();
        *self.document.borrow_mut() = None;
    }

    fn release_grab(&self) {
        let body = body();
        set_grabbed(&self.bar_x, body.as_ref(), false);
        set_grabbed(&self.bar_y, body.as_ref(), false);
        self.x_grabbed.set(false);
        self.y_grabbed.set(false);
    }

    /// Прокручивает содержимое к `value` пикселей сверху в пределах доступного хода.
    pub(crate) fn scroll_top(&self, value: f64) {
        let c = &self.content;
        c.set_scroll_top(clamp_scroll(value, c.scroll_height() as f64, c.client_height() as f64) as i32);
    }
}

fn body() -> Option<HtmlElement> {
    window()?.document()?.body()
}
