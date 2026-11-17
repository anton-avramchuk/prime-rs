//! Password hint panel: show phases, class-based animation, z-index and alignment below the field.

use crate::state::Phase;
use leptos::html;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use prime_core::ZGroup;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Animation duration (`300ms` in the theme's `/* For PrimeNG */` block).
#[cfg(target_arch = "wasm32")]
const TRANSITION: Duration = Duration::from_millis(300);

/// Control of a single instance's panel.
#[derive(Clone, Copy)]
pub struct Panel {
    pub phase: RwSignal<Phase>,
    pub panel_ref: NodeRef<html::Div>,
    pub root_ref: NodeRef<html::Span>,
    epoch: StoredValue<u32>,
    z_index: RwSignal<Option<i32>>,
    #[cfg(target_arch = "wasm32")]
    overlay_base: i32,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            phase: RwSignal::new(Phase::Hidden),
            panel_ref: NodeRef::new(),
            root_ref: NodeRef::new(),
            epoch: StoredValue::new(0),
            z_index: RwSignal::new(None),
            #[cfg(target_arch = "wasm32")]
            overlay_base: prime_core::use_prime_config().z_index.base(ZGroup::Overlay),
        }
    }

        /// Shows the panel (if it is hidden or leaving).
    pub fn open(&self) {
        if matches!(self.phase.get_untracked(), Phase::Hidden | Phase::Leaving) {
            self.enter();
        }
    }

        /// Hides the panel (if it is shown or appearing).
    pub fn close(&self) {
        if self.phase.get_untracked().is_visible() {
            self.leave();
        }
    }

        /// Releases the z-index when the component is destroyed.
    pub fn on_cleanup(&self) {
        let z_index = self.z_index;
        on_cleanup(move || {
            if let Some(z) = z_index.try_get_untracked().flatten() {
                prime_core::zindex::revert(z);
            }
        });
    }

    fn bump(&self) -> u32 {
        self.epoch.update_value(|e| *e = e.wrapping_add(1));
        self.epoch.get_value()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn enter(&self) {
        self.bump();
        self.phase.set(Phase::Shown);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn leave(&self) {
        self.bump();
        self.phase.set(Phase::Hidden);
    }

    #[cfg(target_arch = "wasm32")]
    fn alive(&self) -> bool {
        !self.phase.is_disposed()
    }

    #[cfg(target_arch = "wasm32")]
    fn enter(&self) {
        self.phase.set(Phase::Entering);
        let (epoch, me, this) = (self.epoch, self.bump(), *self);
        request_animation_frame(move || {
            if this.alive() && epoch.get_value() == me {
                this.position();
            }
        });
        set_timeout(
            move || {
                if this.alive() && epoch.get_value() == me {
                    this.phase.set(Phase::Shown);
                }
            },
            TRANSITION,
        );
    }

    #[cfg(target_arch = "wasm32")]
    fn leave(&self) {
        self.phase.set(Phase::Leaving);
        let (epoch, me, this) = (self.epoch, self.bump(), *self);
        set_timeout(
            move || {
                if this.alive() && epoch.get_value() == me {
                    this.phase.set(Phase::Hidden);
                    if let Some(z) = this.z_index.try_update(Option::take).flatten() {
                        prime_core::zindex::revert(z);
                    }
                }
            },
            TRANSITION,
        );
    }

        /// Issues a z-index and aligns the panel below the field.
    #[cfg(target_arch = "wasm32")]
    fn position(&self) {
        let (Some(panel), Some(root)) = (self.panel_ref.get_untracked(), self.root_ref.get_untracked()) else { return };
        let z = match self.z_index.get_untracked() {
            Some(z) => z,
            None => {
                let z = prime_core::zindex::generate(ZGroup::Overlay, self.overlay_base);
                self.z_index.set(Some(z));
                z
            }
        };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        let _ = panel.style().set_property("z-index", &z.to_string());
        prime_dom::dom::align_overlay(&panel, &root.unchecked_into::<web_sys::HtmlElement>(), prime_dom::dom::AlignMode::Relative, true);
    }
}
