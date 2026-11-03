//! Menu — порт `p-menu` из PrimeNG 21.1.10.
//!
//! Список команд и ссылок с группами и разделителями: статичный или всплывающий у элемента-цели. Всплывающим меню
//! управляет [`MenuRef`] (аналог `menu.toggle($event)` из PrimeNG): его создают заранее, передают в [`Menu`]
//! и вызывают из обработчиков.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::{Menu, MenuItem};
//!
//! let html = view! {
//!     <Menu model=vec![MenuItem::new().label("New"), MenuItem::new().label("Search")] />
//! }
//! .to_html();
//! assert!(html.contains("p-menu-item-label") && html.contains("New"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub use state::{
    MenuCommand, MenuItem, Row, can_run_command, enabled_ids, flatten, has_submenu, item_classes, item_id, next_id, prev_id, root_classes,
    root_style, should_prevent_default,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::set_timeout_with_handle;
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Pass-through частей [`Menu`].
#[derive(Clone, Debug, Default)]
pub struct MenuPt {
    /// Корень.
    pub root: Pt,
    /// Блок `start`.
    pub start: Pt,
    /// Список `ul`.
    pub list: Pt,
    /// Заголовок группы.
    pub submenu_label: Pt,
    /// Разделитель.
    pub separator: Pt,
    /// Блок `end`.
    pub end: Pt,
    /// Элемент `li` пункта.
    pub item: Pt,
    /// Содержимое пункта.
    pub item_content: Pt,
    /// Ссылка пункта.
    pub item_link: Pt,
    /// Значок пункта.
    pub item_icon: Pt,
    /// Подпись пункта.
    pub item_label: Pt,
}

/// Управляющий объект всплывающего меню: `toggle`/`show`/`hide` от события.
#[derive(Clone, Copy)]
pub struct MenuRef {
    visible: RwSignal<bool>,
    target: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
}

impl Default for MenuRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Цель по умолчанию: `currentTarget` события, а если его нет — `target`.
fn event_target(event: &web_sys::Event) -> Option<web_sys::HtmlElement> {
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()))
}

impl MenuRef {
    /// Создаёт скрытое меню. Вызывать внутри компонента (владелец сигналов).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false), target: StoredValue::new_local(None) }
    }

    /// Меню сейчас показано (с подпиской).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// Меню сейчас показано (без подписки).
    pub fn visible_untracked(&self) -> bool {
        self.visible.get_untracked()
    }

    /// Показывает меню у `currentTarget` события.
    pub fn show(&self, event: &web_sys::Event) {
        if let Some(target) = event_target(event) {
            self.show_at(&target);
        }
    }

    /// Показывает меню у явной цели.
    pub fn show_at(&self, target: &web_sys::HtmlElement) {
        self.target.set_value(Some(target.clone()));
        self.visible.set(true);
    }

    /// Переключает меню; цель — `currentTarget` события.
    pub fn toggle(&self, event: &web_sys::Event) {
        if self.visible.get_untracked() { self.hide() } else { self.show(event) }
    }

    /// Скрывает меню.
    pub fn hide(&self) {
        self.visible.try_set(false);
    }
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    r: MenuRef,
    popup: Prop<bool>,
    model: Prop<Vec<MenuItem>>,
    menu_id: String,
    panel: NodeRef<leptos::html::Div>,
    list: NodeRef<leptos::html::Ul>,
    focused: RwSignal<bool>,
    focused_id: RwSignal<Option<String>>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_visible: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_focus: Option<Callback<()>>,
    on_blur: Option<Callback<()>>,
}

impl Core {
    fn rows(&self) -> Vec<Row> {
        self.model.with(|m| flatten(m, &self.menu_id))
    }

    fn rows_untracked(&self) -> Vec<Row> {
        flatten(&self.model.get_untracked(), &self.menu_id)
    }

    fn enabled(&self) -> Vec<String> {
        enabled_ids(&self.rows_untracked())
    }

    /// Снимает подписки на окно и освобождает z-index.
    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    fn focus_target(&self) {
        self.r.target.with_value(|t| {
            if let Some(t) = t {
                let _ = t.focus();
            }
        });
    }

    fn set_focused_id(&self, id: Option<String>) {
        self.focused_id.set(id);
    }

    fn list_focus(&self) {
        if self.focused.get_untracked() {
            return;
        }
        self.focused.set(true);
        if !self.popup.get_untracked() {
            self.set_focused_id(self.enabled().into_iter().next());
        }
        if let Some(cb) = self.on_focus {
            cb.run(());
        }
    }

    fn list_blur(&self) {
        if !self.focused.get_untracked() {
            return;
        }
        self.focused.set(false);
        self.set_focused_id(None);
        if let Some(cb) = self.on_blur {
            cb.run(());
        }
    }

    fn item_click(&self, event: web_sys::MouseEvent, item: &MenuItem, id: &str) {
        if !self.focused.get_untracked() {
            self.focused.set(true);
            if let Some(cb) = self.on_focus {
                cb.run(());
            }
        }
        if should_prevent_default(item) {
            event.prevent_default();
        }
        if item.disabled {
            return;
        }
        if let Some(command) = item.command {
            command.run(MenuCommand { event, item: item.clone() });
        }
        if self.popup.get_untracked() {
            self.r.hide();
        } else if self.focused_id.get_untracked().as_deref() != Some(id) {
            self.set_focused_id(Some(id.to_string()));
        }
    }

    fn enter(&self) {
        let Some(id) = self.focused_id.get_untracked() else { return };
        let Some(li) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&id)) else { return };
        if self.popup.get_untracked() {
            self.focus_target();
        }
        let anchor = li.query_selector("a").ok().flatten().unwrap_or(li);
        if let Ok(el) = anchor.dyn_into::<web_sys::HtmlElement>() {
            el.click();
        }
    }

    fn key_down(&self, event: &web_sys::KeyboardEvent) {
        let ids = self.enabled();
        let current = self.focused_id.get_untracked();
        match event.code().as_str() {
            "ArrowDown" => {
                self.set_focused_id(next_id(&ids, current.as_deref()));
                event.prevent_default();
            }
            "ArrowUp" => {
                if event.alt_key() && self.popup.get_untracked() {
                    self.focus_target();
                    self.r.hide();
                } else {
                    self.set_focused_id(prev_id(&ids, current.as_deref()));
                }
                event.prevent_default();
            }
            "Home" => {
                self.set_focused_id(ids.first().cloned());
                event.prevent_default();
            }
            "End" => {
                self.set_focused_id(ids.last().cloned());
                event.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.enter();
                event.prevent_default();
            }
            "Escape" | "Tab" if self.popup.get_untracked() => {
                self.focus_target();
                self.r.hide();
            }
            _ => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn is_inside(&self, event_target: Option<web_sys::EventTarget>) -> (bool, bool) {
        let node = event_target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside_panel = self.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref()));
        let inside_target = self.r.target.with_value(|t| t.as_ref().is_some_and(|t| t.contains(node.as_ref())));
        (inside_panel, inside_target)
    }

    /// Меню только что показано: z-index, выравнивание, фокус, подписки, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(target)) = (self.panel.get_untracked(), self.r.target.get_value()) else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Menu) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Menu, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        align_overlay(&panel, &target, AlignMode::Relative, false);
        if let Some(list) = self.list.get_untracked() {
            prime_dom::focus_no_scroll(&list);
        }
        // Подписки — в следующем тике: клик, открывший меню, ещё всплывает к окну и закрыл бы его сам.
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.r.visible.is_disposed() || !this.r.visible.get_untracked() {
                    return;
                }
                this.bind_window();
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Клик снаружи, изменение размера окна и прокрутка предка цели скрывают меню.
    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            if !this.r.visible.get_untracked() {
                return;
            }
            let (panel, target) = this.is_inside(e.target());
            if !panel && !target {
                this.r.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.r.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || resize.remove()));
        });

        // Прокрутка предка цели (или сдвинувшая цель прокрутка страницы) закрывает меню.
        let this = self.clone();
        if let Some(target) = self.r.target.get_value()
            && let Some(unwatch) = prime_dom::watch_scroll(target, move || this.r.hide())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
    }

    /// Меню только что скрыто: подписки сняты, фокус и цель забыты, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.focused.set(false);
        self.set_focused_id(None);
        self.r.target.set_value(None);
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

fn join_class(base: &str, extra: &Option<String>) -> String {
    match extra.as_deref().filter(|c| !c.is_empty()) {
        Some(c) => format!("{base} {c}"),
        None => base.to_string(),
    }
}

/// Всё, что нужно для вывода строк списка.
#[derive(Clone)]
struct Parts {
    pt: Arc<MenuPt>,
    item: Option<Callback<MenuItem, AnyView>>,
    submenu_header: Option<Callback<MenuItem, AnyView>>,
}

fn render_row(core: &Arc<Core>, parts: &Parts, row: Row) -> AnyView {
    let pt = &parts.pt;
    match row {
        Row::Separator => {
            view! { <li class={pt.separator.merge_class("p-menu-separator")} role="separator" {..pt.separator.attrs()}></li> }.into_any()
        }
        Row::Header(group) => {
            let content = match parts.submenu_header {
                Some(cb) => untrack(|| cb.run(group.clone())),
                None => view! { <span>{group.label.clone()}</span> }.into_any(),
            };
            view! { <li class={pt.submenu_label.merge_class("p-menu-submenu-label")} role="none" {..pt.submenu_label.attrs()}>{content}</li> }.into_any()
        }
        Row::Item { id, item } => render_item(core, parts, id, item),
    }
}

fn render_item(core: &Arc<Core>, parts: &Parts, id: String, item: MenuItem) -> AnyView {
    let pt = &parts.pt;
    let focused = {
        let (core, id) = (core.clone(), id.clone());
        move || core.focused_id.get().as_deref() == Some(id.as_str())
    };
    let class = {
        let (item, focused, pt) = (item.clone(), focused.clone(), pt.clone());
        move || pt.item.merge_class(&item_classes(&item, focused()))
    };
    let content = match parts.item {
        Some(cb) => untrack(|| cb.run(item.clone())),
        None => {
            let icon = item.icon.clone().map(|icon| {
                let class = join_class("p-menu-item-icon", &item.icon_class);
                view! { <span class={pt.item_icon.merge_class(&class)} {..pt.item_icon.attrs()}>{icon.run()}</span> }
            });
            let link_class = join_class("p-menu-item-link", &item.link_class);
            view! {
                <a
                    class={pt.item_link.merge_class(&link_class)}
                    tabindex="-1"
                    href={item.url.clone()}
                    target={item.target.clone()}
                    title={item.title.clone()}
                    {..pt.item_link.attrs()}
                >
                    {icon}
                    <span class={pt.item_label.merge_class("p-menu-item-label")} {..pt.item_label.attrs()}>{item.label.clone()}</span>
                </a>
            }
            .into_any()
        }
    };
    let on_click = {
        let (core, item, id) = (core.clone(), item.clone(), id.clone());
        move |e: web_sys::MouseEvent| core.item_click(e, &item, &id)
    };
    let disabled = if item.disabled { "true" } else { "false" };
    let focused_attr = {
        let focused = focused.clone();
        move || if focused() { "true" } else { "false" }
    };
    view! {
        <li
            class=class
            id={id}
            role="menuitem"
            aria-label={item.label.clone()}
            aria-disabled=disabled
            data-p-disabled=disabled
            data-p-focused=focused_attr
            style={item.style.clone()}
            {..pt.item.attrs()}
        >
            <div class={pt.item_content.merge_class("p-menu-item-content")} on:click={on_click} {..pt.item_content.attrs()}>{content}</div>
        </li>
    }
    .into_any()
}

/// Список команд и ссылок, статичный или всплывающий.
///
/// Каждое свойство принимает значение или сигнал. Всплывающее меню выводится внутри собственного места в разметке
/// (не в `body`), поэтому для выравнивания родитель должен быть `position: relative`, а не `overflow: hidden`.
#[component]
pub fn Menu(
    /// Пункты меню; пункты с подпунктами становятся группами.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Всплывающий режим: меню показывается через [`MenuRef`].
    #[prop(optional, into)]
    popup: Prop<bool>,
    /// Управляющий объект всплывающего меню.
    #[prop(optional)]
    menu_ref: Option<MenuRef>,
    /// Автоматически назначать `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Базовый `z-index` поверх значения конфигурации.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Атрибут `aria-label` списка.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` списка.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Атрибут `id` корня; по умолчанию создаётся уникальный.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `tabindex` списка.
    #[prop(optional, default = Prop::Static(0), into)]
    tabindex: Prop<i32>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается после показа и выравнивания всплывающего меню.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Вызывается после скрытия всплывающего меню.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Вызывается, когда список получает фокус.
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    /// Вызывается, когда список теряет фокус.
    #[prop(optional, into)]
    on_blur: Option<Callback<()>>,
    /// Представление содержимого пункта вместо ссылки со значком и подписью.
    #[prop(optional, into)]
    item: Option<Callback<MenuItem, AnyView>>,
    /// Представление заголовка группы вместо подписи.
    #[prop(optional, into)]
    submenu_header: Option<Callback<MenuItem, AnyView>>,
    /// Содержимое перед списком.
    #[prop(optional, into)]
    start: Option<ViewFn>,
    /// Содержимое после списка.
    #[prop(optional, into)]
    end: Option<ViewFn>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<MenuPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let r = menu_ref.unwrap_or_default();
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let core = Arc::new(Core {
        r,
        popup: popup.clone(),
        model,
        menu_id: menu_id.clone(),
        panel: NodeRef::new(),
        list: NodeRef::new(),
        focused: RwSignal::new(false),
        focused_id: RwSignal::new(None),
        auto_z_index,
        base_z_index,
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_visible: StoredValue::new(false),
        on_show,
        on_hide,
        on_focus,
        on_blur,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // Побочные эффекты показа и скрытия (положение, слушатели, фокус) — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let visible = core.r.visible.get() && core.popup.get();
            // Панель попадает в DOM после этого эффекта, поэтому следим и за ссылкой на неё.
            let mounted = core.panel.get().is_some();
            let was = core.was_visible.get_value();
            if visible && mounted && !was {
                core.was_visible.set_value(true);
                core.opened();
            } else if !visible && was {
                core.was_visible.set_value(false);
                core.closed();
            }
        });
    }

    let parts = Parts { pt: pt.clone(), item, submenu_header };
    let panel = core.panel;
    let list = core.list;
    let visible = r.visible;
    let root_id = menu_id.clone();
    move || {
        let is_popup = popup.get();
        (!is_popup || visible.get()).then(|| {
            let (core, parts, pt) = (core.clone(), parts.clone(), pt.clone());
            let (style, style_class, aria_label, aria_labelledby) =
                (style.clone(), style_class.clone(), aria_label.clone(), aria_labelledby.clone());
            let class = {
                let (pt, style_class) = (pt.clone(), style_class.clone());
                move || {
                    let mut cn = String::from(root_classes(is_popup));
                    if let Some(c) = style_class.get().filter(|c| !c.is_empty()) {
                        cn.push(' ');
                        cn.push_str(&c);
                    }
                    pt.root.merge_class(&cn)
                }
            };
            let root_style_attr = {
                let pt = pt.clone();
                move || {
                    let user = style.get().unwrap_or_default();
                    pt.root.merge_style_parts(&[root_style(is_popup), &user])
                }
            };
            let activedescendant = {
                let core = core.clone();
                move || if core.focused.get() { core.focused_id.get() } else { None }
            };
            let rows = {
                let (core, parts) = (core.clone(), parts.clone());
                move || core.rows().into_iter().map(|row| render_row(&core, &parts, row)).collect_view()
            };
            let (on_focus_c, on_blur_c, on_key_c) = (core.clone(), core.clone(), core.clone());
            let start =
                start.clone().map(|s| view! { <div class={pt.start.merge_class("p-menu-start")} {..pt.start.attrs()}>{s.run()}</div> });
            let end = end.clone().map(|s| view! { <div class={pt.end.merge_class("p-menu-end")} {..pt.end.attrs()}>{s.run()}</div> });
            view! {
                <div
                    node_ref=panel
                    class=class
                    style=root_style_attr
                    id={root_id.clone()}
                    data-p={is_popup.then_some("overlay")}
                    {..pt.root.attrs()}
                >
                    {start}
                    <ul
                        node_ref=list
                        class={pt.list.merge_class("p-menu-list")}
                        role="menu"
                        id={format!("{}_list", root_id)}
                        tabindex={let tabindex = tabindex.clone(); move || tabindex.get().to_string()}
                        aria-activedescendant=activedescendant
                        aria-label={move || aria_label.get()}
                        aria-labelledby={move || aria_labelledby.get()}
                        on:focus=move |_| on_focus_c.list_focus()
                        on:blur=move |_| on_blur_c.list_blur()
                        on:keydown=move |e| on_key_c.key_down(&e)
                        {..pt.list.attrs()}
                    >
                        {rows}
                    </ul>
                    {end}
                </div>
            }
        })
    }
}
