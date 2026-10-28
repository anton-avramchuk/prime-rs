//! Эффект ripple (порт директивы `pRipple`): элемент `span.p-ink` внутри хоста, активируемый при `mousedown`.

use crate::dom::{document, window};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, MouseEvent};

const ACTIVE_MS: i32 = 401;

fn set_active(ink: &Element, active: bool) {
    let _ = ink.class_list().toggle_with_force("p-ink-active", active);
    let _ = ink.set_attribute("data-p-ink-active", if active { "true" } else { "false" });
}

/// Подключённый ripple. При `Drop` удаляет `span.p-ink` и обработчики.
pub struct Ripple {
    host: HtmlElement,
    ink: HtmlElement,
    timeout: Rc<Cell<Option<i32>>>,
    _on_mouse_down: Closure<dyn FnMut(MouseEvent)>,
    _on_animation_end: Closure<dyn FnMut()>,
    _on_timeout: Rc<Closure<dyn FnMut()>>,
}

impl Ripple {
    /// Подключает ripple к `host` (добавляет класс `p-ripple` и элемент `p-ink`). Вне браузера возвращает `None`.
    pub fn attach(host: &HtmlElement) -> Option<Self> {
        let doc = document()?;
        let win = window()?;
        let ink = doc.create_element("span").ok()?.dyn_into::<HtmlElement>().ok()?;
        for (k, v) in
            [("class", "p-ink"), ("data-p-ink", "true"), ("data-p-ink-active", "false"), ("aria-hidden", "true"), ("role", "presentation")]
        {
            ink.set_attribute(k, v).ok()?;
        }
        host.class_list().add_1("p-ripple").ok()?;
        host.append_with_node_1(&ink).ok()?;

        let timeout: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
        let on_timeout = {
            let ink = ink.clone();
            Rc::new(Closure::<dyn FnMut()>::new(move || set_active(&ink, false)))
        };

        let on_mouse_down = {
            let (host, ink, timeout, win, on_timeout) = (host.clone(), ink.clone(), timeout.clone(), win.clone(), on_timeout.clone());
            Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
                if win.get_computed_style(&ink).ok().flatten().and_then(|s| s.get_property_value("display").ok()).as_deref() == Some("none")
                {
                    return;
                }
                set_active(&ink, false);
                let style = ink.style();
                if ink.offset_width() == 0 && ink.offset_height() == 0 {
                    let d = host.offset_width().max(host.offset_height());
                    let _ = style.set_property("width", &format!("{d}px"));
                    let _ = style.set_property("height", &format!("{d}px"));
                }
                let rect = host.get_bounding_client_rect();
                let x = f64::from(e.page_x()) - (rect.left() + win.page_x_offset().unwrap_or(0.0)) - f64::from(ink.offset_width()) / 2.0;
                let y = f64::from(e.page_y()) - (rect.top() + win.page_y_offset().unwrap_or(0.0)) - f64::from(ink.offset_height()) / 2.0;
                let _ = style.set_property("top", &format!("{y}px"));
                let _ = style.set_property("left", &format!("{x}px"));
                set_active(&ink, true);
                if let Some(id) = timeout.take() {
                    win.clear_timeout_with_handle(id);
                }
                if let Ok(id) = win.set_timeout_with_callback_and_timeout_and_arguments_0((*on_timeout).as_ref().unchecked_ref(), ACTIVE_MS)
                {
                    timeout.set(Some(id));
                }
            })
        };
        let on_animation_end = {
            let (ink, timeout, win) = (ink.clone(), timeout.clone(), win.clone());
            Closure::<dyn FnMut()>::new(move || {
                if let Some(id) = timeout.take() {
                    win.clear_timeout_with_handle(id);
                }
                set_active(&ink, false);
            })
        };

        host.add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref()).ok()?;
        ink.add_event_listener_with_callback("animationend", on_animation_end.as_ref().unchecked_ref()).ok()?;
        Some(Self {
            host: host.clone(),
            ink,
            timeout,
            _on_mouse_down: on_mouse_down,
            _on_animation_end: on_animation_end,
            _on_timeout: on_timeout,
        })
    }

    /// Элемент `span.p-ink`.
    pub fn ink(&self) -> &HtmlElement {
        &self.ink
    }
}

impl Drop for Ripple {
    fn drop(&mut self) {
        if let (Some(id), Some(win)) = (self.timeout.take(), window()) {
            win.clear_timeout_with_handle(id);
        }
        let _ = self.host.remove_event_listener_with_callback("mousedown", self._on_mouse_down.as_ref().unchecked_ref());
        self.ink.remove();
    }
}
