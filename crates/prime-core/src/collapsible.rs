//! Общая логика сворачиваемой области (Panel, Fieldset): состояние, идентификаторы, атрибуты ARIA и анимация раскрытия.
//!
//! Анимацию задают классы темы `p-collapsible-enter-active` и `p-collapsible-leave-active` (`grid-template-rows` `0fr` ↔ `1fr`).
//! Итоговое скрытие — встроенный `display: none`, который ставит [`CollapseState::initial_style`] при первом рендере
//! и затем (только в браузере) [`bind_region`].

use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Состояние «свёрнуто»: внешний двусторонний сигнал либо собственный.
#[derive(Clone, Copy, Debug)]
pub struct CollapseState {
    signal: RwSignal<bool>,
}

impl CollapseState {
    /// Берёт внешний сигнал, а без него создаёт собственный с начальным значением `initial`.
    ///
    /// ```
    /// use leptos::prelude::*;
    /// use prime_core::collapsible::CollapseState;
    ///
    /// let own = CollapseState::new(None, true);
    /// assert!(own.get_untracked());
    /// assert!(!own.toggle());
    ///
    /// let external = RwSignal::new(false);
    /// let state = CollapseState::new(Some(external), true);
    /// assert!(state.toggle() && external.get_untracked());
    /// ```
    pub fn new(external: Option<RwSignal<bool>>, initial: bool) -> Self {
        Self { signal: external.unwrap_or_else(|| RwSignal::new(initial)) }
    }

    /// Текущее значение с подпиской.
    pub fn get(&self) -> bool {
        self.signal.get()
    }

    /// Текущее значение без подписки.
    pub fn get_untracked(&self) -> bool {
        self.signal.get_untracked()
    }

    /// Инвертирует состояние и возвращает новое значение `collapsed`.
    pub fn toggle(&self) -> bool {
        let now = !self.signal.get_untracked();
        self.signal.set(now);
        now
    }

    /// Встроенный стиль области при первом рендере: свёрнутая область скрыта сразу, без анимации.
    pub fn initial_style(&self) -> Option<&'static str> {
        hidden_style(self.get_untracked())
    }
}

/// Стиль скрытой области.
pub fn hidden_style(collapsed: bool) -> Option<&'static str> {
    collapsed.then_some("display: none")
}

/// Идентификаторы частей: заголовок, область содержимого и строка заголовка.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollapseIds {
    /// Основа: `id` корня.
    pub root: String,
    /// `{id}_header`: заголовок, на него указывает `aria-labelledby`.
    pub header: String,
    /// `{id}_content`: область содержимого, на неё указывает `aria-controls`.
    pub content: String,
    /// `{id}-titlebar`: строка заголовка.
    pub titlebar: String,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

impl CollapseIds {
    /// Производные идентификаторы от `id`. Без `id` берётся номер из общего контекста гидратации (одинаковый на сервере и в браузере),
    /// а вне его — из глобального счётчика.
    ///
    /// ```
    /// use prime_core::collapsible::CollapseIds;
    ///
    /// let ids = CollapseIds::new(Some("p1"));
    /// assert_eq!((ids.header.as_str(), ids.content.as_str(), ids.titlebar.as_str()), ("p1_header", "p1_content", "p1-titlebar"));
    /// ```
    pub fn new(id: Option<&str>) -> Self {
        let root = match id {
            Some(id) => id.to_owned(),
            None => format!("pn_id_{}", next_unique()),
        };
        Self { header: format!("{root}_header"), content: format!("{root}_content"), titlebar: format!("{root}-titlebar"), root }
    }
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
fn next_unique() -> usize {
    match Owner::current_shared_context() {
        Some(ctx) => ctx.next_id().into_inner(),
        None => NEXT_ID.fetch_add(1, Ordering::Relaxed),
    }
}

#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
fn next_unique() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// `aria-expanded` переключателя.
pub fn aria_expanded(collapsed: bool) -> &'static str {
    if collapsed { "false" } else { "true" }
}

/// `aria-hidden` области.
pub fn aria_hidden(collapsed: bool) -> &'static str {
    if collapsed { "true" } else { "false" }
}

/// `tabindex` самой области: `-1` только когда она свёрнута.
pub fn region_tabindex(collapsed: bool) -> Option<&'static str> {
    collapsed.then_some("-1")
}

/// Класс анимации для раскрытия или сворачивания.
pub fn animation_class(expanding: bool) -> &'static str {
    if expanding { "p-collapsible-enter-active" } else { "p-collapsible-leave-active" }
}

/// Привязывает область к состоянию (только браузер): `tabindex` фокусируемых потомков и анимация при смене состояния.
/// Первый запуск ничего не анимирует. `on_done` вызывается с новым `collapsed` после окончания анимации.
#[cfg(target_arch = "wasm32")]
pub fn bind_region(region: NodeRef<leptos::html::Div>, state: CollapseState, on_done: Option<Callback<bool>>) {
    Effect::new(move |prev: Option<bool>| {
        let collapsed = state.get();
        let Some(el) = region.get() else { return collapsed };
        let el: web_sys::HtmlElement = (*el).clone();
        match prev {
            None => {
                if collapsed {
                    set_descendants_tabindex(&el, true);
                }
            }
            Some(p) if p == collapsed => {}
            Some(_) => {
                set_descendants_tabindex(&el, collapsed);
                animate(&el, !collapsed, move || {
                    if let Some(cb) = on_done {
                        cb.run(collapsed);
                    }
                });
            }
        }
        collapsed
    });
}

/// Проставляет или снимает `tabindex="-1"` у фокусируемых потомков области.
#[cfg(target_arch = "wasm32")]
fn set_descendants_tabindex(region: &web_sys::HtmlElement, collapsed: bool) {
    let Ok(list) = region.query_selector_all("input, button, select, a, textarea, [tabindex]") else { return };
    for i in 0..list.length() {
        let Some(node) = list.item(i) else { continue };
        let el: web_sys::Element = leptos::wasm_bindgen::JsCast::unchecked_into(node);
        let _ = if collapsed { el.set_attribute("tabindex", "-1") } else { el.remove_attribute("tabindex") };
    }
}

/// Запускает анимацию области и вызывает `done` один раз по `animationend`; если события нет (отключённые анимации,
/// скрытый элемент), завершает по запасному таймеру.
#[cfg(target_arch = "wasm32")]
pub fn animate(el: &web_sys::HtmlElement, expanding: bool, done: impl FnOnce() + 'static) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    let class = animation_class(expanding);
    let _ = el.style().remove_property("display");
    let _ = el.class_list().add_1(class);

    type Done = Rc<RefCell<Option<Box<dyn FnOnce()>>>>;
    let done: Done = Rc::new(RefCell::new(Some(Box::new(done))));
    let finish: Rc<dyn Fn()> = {
        let el = el.clone();
        Rc::new(move || {
            let Some(done) = done.borrow_mut().take() else { return };
            let _ = el.class_list().remove_1(class);
            if !expanding {
                let _ = el.style().set_property("display", "none");
            }
            done();
        })
    };
    let on_end = {
        let finish = finish.clone();
        Closure::once_into_js(move || finish())
    };
    let _ = el.add_event_listener_with_callback("animationend", on_end.unchecked_ref());
    set_timeout(move || finish(), Duration::from_millis(400));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_follow_state() {
        assert_eq!((aria_expanded(false), aria_hidden(false), region_tabindex(false)), ("true", "false", None));
        assert_eq!((aria_expanded(true), aria_hidden(true), region_tabindex(true)), ("false", "true", Some("-1")));
        assert_eq!(hidden_style(true), Some("display: none"));
        assert_eq!(hidden_style(false), None);
        assert_eq!(animation_class(true), "p-collapsible-enter-active");
        assert_eq!(animation_class(false), "p-collapsible-leave-active");
    }

    #[test]
    fn ids_are_derived_and_unique() {
        let a = CollapseIds::new(Some("x"));
        assert_eq!(
            (a.root.as_str(), a.header.as_str(), a.content.as_str(), a.titlebar.as_str()),
            ("x", "x_header", "x_content", "x-titlebar")
        );
        let (b, c) = (CollapseIds::new(None), CollapseIds::new(None));
        assert_ne!(b.root, c.root);
        assert!(b.root.starts_with("pn_id_"));
    }

    #[test]
    fn state_toggles_own_and_external() {
        let owner = Owner::new();
        owner.with(|| {
            let own = CollapseState::new(None, true);
            assert_eq!((own.get_untracked(), own.toggle(), own.get_untracked()), (true, false, false));
            let ext = RwSignal::new(false);
            let s = CollapseState::new(Some(ext), true);
            assert!(!s.get_untracked(), "внешний сигнал важнее начального значения");
            assert!(s.toggle() && ext.get_untracked());
            assert_eq!(s.initial_style(), Some("display: none"));
        });
    }
}
