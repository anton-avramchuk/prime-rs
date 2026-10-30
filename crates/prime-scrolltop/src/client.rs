//! Клиентская часть ScrollTop (только `wasm32`): подписка на `scroll`, фазы показа, анимация и z-index.

use crate::state::{Phase, ScrollTopTarget, enter_classes, is_visible, leave_classes};
use leptos::prelude::*;
use prime_core::{Prop, ZGroup};
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

/// Длительность перехода в теме (`transition: opacity 300ms`).
const TRANSITION: Duration = Duration::from_millis(300);

/// Состояние экземпляра, нужное клиенту.
#[derive(Clone, Copy)]
pub struct Setup {
    pub target: ScrollTopTarget,
    pub threshold: StoredValue<Prop<f64>>,
    pub phase: RwSignal<Phase>,
    pub animation: RwSignal<&'static str>,
    pub z_index: RwSignal<Option<i32>>,
    pub overlay_base: i32,
    pub epoch: StoredValue<u32>,
    pub id: usize,
}

impl Setup {
    fn alive(&self) -> bool {
        !self.phase.is_disposed()
    }

    /// Проверка прокрутки: показать или скрыть.
    fn check(&self, scroll: f64) {
        let visible = is_visible(scroll, self.threshold.with_value(|t| t.get_untracked()));
        match (visible, self.phase.get_untracked()) {
            (true, Phase::Hidden | Phase::Leaving) => self.enter(),
            (false, Phase::Entering | Phase::Shown) => self.leave(),
            _ => {}
        }
    }

    /// Поколение анимации: устаревшие таймеры ничего не делают.
    fn bump(&self) -> u32 {
        self.epoch.update_value(|e| *e = e.wrapping_add(1));
        self.epoch.get_value()
    }

    fn enter(&self) {
        if self.z_index.get_untracked().is_none() {
            self.z_index.set(Some(prime_core::zindex::generate(ZGroup::Overlay, self.overlay_base)));
        }
        self.phase.set(Phase::Entering);
        self.animation.set(enter_classes(false));
        let (epoch, me) = (self.epoch, self.bump());
        let this = *self;
        request_animation_frame(move || {
            if this.alive() && epoch.get_value() == me {
                this.animation.set(enter_classes(true));
            }
        });
        set_timeout(
            move || {
                if this.alive() && epoch.get_value() == me {
                    this.phase.set(Phase::Shown);
                    this.animation.set("");
                }
            },
            TRANSITION,
        );
    }

    fn leave(&self) {
        self.phase.set(Phase::Leaving);
        self.animation.set(leave_classes(false));
        let (epoch, me) = (self.epoch, self.bump());
        let this = *self;
        request_animation_frame(move || {
            if this.alive() && epoch.get_value() == me {
                this.animation.set(leave_classes(true));
            }
        });
        set_timeout(
            move || {
                if this.alive() && epoch.get_value() == me {
                    this.phase.set(Phase::Hidden);
                    this.animation.set("");
                    this.release_z();
                }
            },
            TRANSITION,
        );
    }

    fn release_z(&self) {
        if let Some(z) = self.z_index.try_update(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    /// Текущая прокрутка цели.
    fn scroll_of(&self, parent: Option<&web_sys::Element>) -> f64 {
        match parent {
            Some(el) => f64::from(el.scroll_top()),
            None => web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0),
        }
    }
}

/// Подписывается на прокрутку цели после монтирования и снимает подписку при уничтожении.
pub fn setup(s: Setup) {
    let listener = StoredValue::new_local(None::<Box<dyn FnOnce()>>);
    Effect::new(move |_| {
        if listener.with_value(Option::is_some) {
            return;
        }
        let remove: Box<dyn FnOnce()> = match s.target {
            ScrollTopTarget::Window => {
                s.check(s.scroll_of(None));
                let handle = window_event_listener(leptos::ev::scroll, move |_| s.check(s.scroll_of(None)));
                Box::new(move || handle.remove())
            }
            ScrollTopTarget::Parent => {
                let Some(parent) = crate::parent_of(s.id) else { return };
                s.check(s.scroll_of(Some(&parent)));
                let watched = parent.clone();
                let closure = Closure::<dyn FnMut()>::new(move || s.check(s.scroll_of(Some(&watched))));
                let _ = parent.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
                Box::new(move || {
                    let _ = parent.remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
                })
            }
        };
        listener.set_value(Some(remove));
    });
    on_cleanup(move || {
        if let Some(remove) = listener.try_update_value(Option::take).flatten() {
            remove();
        }
        if let Some(z) = s.z_index.try_get_untracked().flatten() {
            prime_core::zindex::revert(z);
        }
    });
}
