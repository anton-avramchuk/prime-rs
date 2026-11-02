//! BlockUI — порт `p-blockui` из PrimeNG 21.1.10.
//!
//! Маска `div.p-blockui.p-blockui-mask` с произвольным содержимым по центру: блокирует весь документ (маска в `body`,
//! прокрутка страницы заблокирована) либо выбранный элемент (`target`, маска внутри него). Пока `blocked = false`, в DOM
//! ничего нет; при снятии блокировки играет анимация выхода, затем маска убирается. Маска выводится только в браузере и
//! до гидратации не отдаётся сервером.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_blockui::BlockUI;
//!
//! let html = view! { <BlockUI blocked=true><span>"Loading"</span></BlockUI> }.to_html();
//! // на сервере маски нет
//! assert!(!html.contains("p-blockui") && !html.contains("Loading"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_dom::Portal;
use std::sync::Arc;

pub use state::{aria_busy, mask_classes, mask_style};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::{block_body_scroll, unblock_body_scroll};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Запасное время выхода: без стилей или при отключённой анимации `animationend` не придёт.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(400);

/// Блокируемый элемент: любой элемент, на который указывает `NodeRef`.
#[derive(Clone)]
pub struct BlockTarget(Arc<dyn Fn() -> Option<web_sys::Element> + Send + Sync>);

impl BlockTarget {
    /// Элемент цели, если он уже в DOM.
    pub fn resolve(&self) -> Option<web_sys::Element> {
        (self.0)()
    }
}

impl<E> From<NodeRef<E>> for BlockTarget
where
    E: leptos::tachys::html::element::ElementType + 'static,
    E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static,
{
    fn from(node: NodeRef<E>) -> Self {
        Self(Arc::new(move || node.get_untracked().map(leptos::wasm_bindgen::JsCast::unchecked_into::<web_sys::Element>)))
    }
}

/// Состояние, общее для обработчиков маски.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    blocked: Prop<bool>,
    target: Option<BlockTarget>,
    /// Маска в DOM (пока играет анимация выхода, она ещё в DOM).
    rendered: RwSignal<bool>,
    /// Идёт анимация выхода.
    leaving: RwSignal<bool>,
    mask: NodeRef<leptos::html::Div>,
    auto_z_index: OptProp<bool>,
    base_z_index: Prop<i32>,
    /// Выданный маске слой (он же часть её инлайн-стиля).
    z_index: RwSignal<Option<i32>>,
    scroll_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    saved_position: StoredValue<Option<(web_sys::HtmlElement, String)>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl Core {
    /// Освобождает слой, блокировку прокрутки и позицию цели.
    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if self.scroll_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            #[cfg(target_arch = "wasm32")]
            unblock_body_scroll();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some((el, old)) = self.saved_position.try_update_value(Option::take).flatten() {
            let style = el.style();
            if old.is_empty() {
                let _ = style.remove_property("position");
            } else {
                let _ = style.set_property("position", &old);
            }
        }
    }

    /// Полное освобождение при размонтировании компонента.
    fn teardown(&self) {
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Маска только что попала в DOM: слой, блокировка прокрутки страницы или `position: relative` цели.
    ///
    /// Повторное включение во время выхода вызывает это снова, пока слой и блокировка ещё удерживаются: второй раз их не берём.
    #[cfg(target_arch = "wasm32")]
    fn opened(&self) {
        if self.auto_z_index.get_untracked().unwrap_or(true) && self.z_index.get_untracked().is_none() {
            let base = use_prime_config().z_index.base(ZGroup::Modal) + self.base_z_index.get_untracked();
            self.z_index.set(Some(prime_core::zindex::generate(ZGroup::Modal, base)));
        }
        match &self.target {
            Some(target) => {
                let held = self.saved_position.with_value(Option::is_some);
                if let Some(el) = target.resolve().and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok()).filter(|_| !held) {
                    let style = el.style();
                    let old = style.get_property_value("position").unwrap_or_default();
                    let _ = style.set_property("position", "relative");
                    self.saved_position.set_value(Some((el, old)));
                }
            }
            None => {
                if !self.scroll_blocked.get_value() {
                    block_body_scroll();
                    self.scroll_blocked.set_value(true);
                }
            }
        }
    }

    /// Блокировка снята: играет анимация выхода.
    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.leaving.set(true);
        let this = self.clone();
        if let Ok(handle) = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK) {
            self.timer.set_value(Some(handle));
        }
    }

    /// Анимация выхода закончилась: маска убирается из DOM, слой и блокировка освобождаются.
    #[cfg(target_arch = "wasm32")]
    fn finish_leave(&self) {
        if self.rendered.is_disposed() || !self.leaving.get_untracked() {
            return;
        }
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        self.release_layer();
        self.leaving.set(false);
        self.rendered.set(false);
    }
}

/// Маска, блокирующая документ или выбранный элемент.
///
/// Свойства принимают значение или сигнал. Содержимое (`children`) строится заново при каждом показе.
#[component]
pub fn BlockUI(
    /// Включена ли блокировка.
    #[prop(optional, into)]
    blocked: Prop<bool>,
    /// Блокируемый элемент; без него блокируется документ.
    #[prop(optional, into)]
    target: Option<BlockTarget>,
    /// Выдавать маске слой группы `Modal` (по умолчанию `true`).
    #[prop(optional, into)]
    auto_z_index: OptProp<bool>,
    /// Смещение слоя относительно базового значения группы `Modal`.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Встроенный стиль маски.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс маски.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through маски.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Содержимое маски (например, индикатор загрузки).
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        blocked,
        target,
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        mask: NodeRef::new(),
        auto_z_index,
        base_z_index,
        z_index: RwSignal::new(None),
        scroll_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        #[cfg(target_arch = "wasm32")]
        saved_position: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }
    // Показ, выход и побочные эффекты (слой, прокрутка, позиция цели) — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.blocked.get();
            // Маска попадает в DOM после этого эффекта, поэтому следим и за ссылкой на неё.
            let mounted = core.mask.get().is_some();
            let was_open = core.was_open.get_value();
            if shown {
                if let Some(handle) = core.timer.try_update_value(Option::take).flatten() {
                    handle.clear();
                }
                // Повторное включение во время выхода: маска ещё в DOM, возвращаем её в состояние «заблокирована».
                if core.leaving.get_untracked() {
                    core.leaving.set(false);
                }
                if !core.rendered.get_untracked() {
                    core.rendered.set(true);
                }
                if mounted && !was_open {
                    core.was_open.set_value(true);
                    core.opened();
                }
            } else if was_open {
                core.was_open.set_value(false);
                core.begin_leave();
            } else if core.rendered.get_untracked() && !core.leaving.get_untracked() {
                // Выключили, не дождавшись появления маски в DOM: ждать нечего.
                core.rendered.set(false);
            }
        });
    }

    let parts = Arc::new(Parts { style, style_class, children });
    move || {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        core.rendered
            .get()
            .then(move || {
                // Цель задана, но её элемента ещё нет: маска не выводится.
                let mount = match core.target.as_ref() {
                    Some(target) => Some(target.resolve()?),
                    None => None,
                };
                Some(match mount {
                    Some(element) => Either::Left(view! { <Portal mount=element>{build_mask(&core, &pt, &parts)}</Portal> }),
                    None => Either::Right(view! { <Portal>{build_mask(&core, &pt, &parts)}</Portal> }),
                })
            })
            .flatten()
    }
}

/// Свойства, нужные только при выводе маски.
struct Parts {
    style: OptProp<String>,
    style_class: OptProp<String>,
    children: Option<ChildrenFn>,
}

/// Маска с содержимым.
fn build_mask(core: &Arc<Core>, pt: &Arc<Pt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let document = core.target.is_none();
    let (leaving, z_index, mask_ref) = (core.leaving, core.z_index, core.mask);
    let class = {
        let (pt, parts) = (pt.clone(), parts.clone());
        move || {
            let extra = parts.style_class.with(|c| c.clone());
            let merged = mask_classes(document, leaving.get(), extra.as_deref());
            pt.merge_class(&merged).into_owned()
        }
    };
    let style = {
        let (pt, parts) = (pt.clone(), parts.clone());
        move || {
            let own = parts.style.with(|s| s.clone()).unwrap_or_default();
            mask_style(z_index.get(), &[&own, &pt.merge_style("")])
        }
    };
    let busy = move || aria_busy(leaving.get());
    let end = {
        let core = core.clone();
        move |e: web_sys::AnimationEvent| animation_end(&core, &e)
    };
    let attrs = pt.attrs();
    let content = parts.children.as_ref().map(|c| c());
    view! {
        <div node_ref={mask_ref} class={class} style={style} aria-busy={busy} on:animationend={end} {..attrs}>
            {content}
        </div>
    }
}

/// `animationend` самой маски (не вложенных элементов) завершает выход.
fn animation_end(core: &Arc<Core>, e: &web_sys::AnimationEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        let own = e.target().zip(e.current_target()).is_some_and(|(t, c)| t == c);
        if own && core.leaving.get_untracked() {
            core.finish_leave();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, e);
}
