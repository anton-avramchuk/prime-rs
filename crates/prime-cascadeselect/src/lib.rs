//! CascadeSelect — порт `p-cascadeselect` из PrimeNG 21.1.10.
//!
//! Поле выбора значения из вложенной структуры: панель с корневым списком, из которого вбок раскрываются списки
//! вложенных групп. Варианты — [`CascadeOption`] (лист со значением либо группа с `children`), значение —
//! двусторонний сигнал `RwSignal<Option<String>>` со значением выбранного листа.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_cascadeselect::{CascadeOption, CascadeSelect};
//!
//! let options = vec![CascadeOption::new("Australia").children(vec![CascadeOption::new("Sydney")])];
//! let value = RwSignal::new(Some("Sydney".to_owned()));
//! let html = Owner::new().with(|| view! { <CascadeSelect options=options value=value placeholder="Select" /> }.to_html());
//! assert!(html.contains("p-cascadeselect-label") && html.contains("Sydney") && !html.contains("p-cascadeselect-overlay"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::leptos_dom::helpers::WindowListenerHandle;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{AngleRightIcon, ChevronDownIcon, SpinnerIcon, TimesIcon};
use std::sync::Arc;

pub use state::{CascadeOption, Click, Menu, Node, State, build, is_ancestor_or_self};

#[cfg(target_arch = "wasm32")]
use leptos::ev;
#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::TimeoutHandle;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use prime_core::ZGroup;
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Pass-through частей [`CascadeSelect`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CascadeSelectPt {
    /// Корень.
    pub root: Pt,
    /// Контейнер скрытого поля.
    pub hidden_input_wrapper: Pt,
    /// Скрытое поле.
    pub hidden_input: Pt,
    /// Метка.
    pub label: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Кнопка раскрытия.
    pub dropdown: Pt,
    /// Стрелка кнопки.
    pub dropdown_icon: Pt,
    /// Значок загрузки.
    pub loading_icon: Pt,
    /// Панель.
    pub overlay: Pt,
    /// Контейнер списка.
    pub list_container: Pt,
    /// Корневой список.
    pub list: Pt,
    /// Вариант `li`.
    pub option: Pt,
    /// Содержимое варианта.
    pub option_content: Pt,
    /// Текст варианта.
    pub option_text: Pt,
    /// Значок группы.
    pub group_icon: Pt,
    /// Вложенный список.
    pub option_list: Pt,
}

/// Контекст представления `option`.
#[derive(Clone, Debug)]
pub struct CascadeOptionContext {
    /// Подпись.
    pub label: String,
    /// Значение (подпись, если не задано).
    pub value: String,
    /// Уровень вложенности (корневые — `0`).
    pub level: usize,
    /// Вариант — группа.
    pub group: bool,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    nodes: Memo<Vec<Node>>,
    value: RwSignal<Option<String>>,
    state: RwSignal<State>,
    open: RwSignal<bool>,
    focused: RwSignal<bool>,
    mobile: RwSignal<bool>,
    clicked: StoredValue<bool>,
    disabled: Prop<bool>,
    loading: Prop<bool>,
    show_clear: Prop<bool>,
    focus_on_hover: Prop<bool>,
    auto_option_focus: Prop<bool>,
    root_ref: NodeRef<html::Div>,
    overlay_ref: NodeRef<html::Div>,
    input_ref: NodeRef<html::Input>,
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    search: StoredValue<String>,
    #[cfg(target_arch = "wasm32")]
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    on_change: Option<Callback<Option<String>>>,
    on_group_change: Option<Callback<String>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_clear: Option<Callback<()>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn tree<R>(&self, f: impl FnOnce(&Menu) -> R) -> R {
        self.nodes.with_untracked(|nodes| f(&Menu { nodes }))
    }

    fn update<R>(&self, f: impl FnOnce(&Menu, &mut State) -> R) -> R {
        let mut state = self.state.get_untracked();
        let out = self.tree(|m| f(m, &mut state));
        self.state.set(state);
        out
    }

    fn focus_input(&self) {
        if let Some(input) = self.input_ref.get_untracked() {
            let _ = input.focus();
        }
    }

    fn blocked(&self) -> bool {
        self.disabled.get_untracked() || self.loading.get_untracked()
    }

    /// `show()`: открывает панель, раскрывает путь к выбранному значению.
    fn show(&self) {
        let (value, auto) = (self.value.get_untracked(), self.auto_option_focus.get_untracked());
        self.update(|m, st| m.show(st, value.as_deref(), auto));
        self.open.set(true);
    }

    /// `hide()`: закрывает панель и сбрасывает путь и фокус.
    fn hide(&self, focus: bool) {
        self.open.try_set(false);
        self.clicked.try_set_value(false);
        self.state.try_update(|st| {
            st.active.clear();
            st.focus = None;
        });
        if focus {
            self.focus_input();
        }
    }

    fn select(&self, value: String) {
        self.value.set(Some(value.clone()));
        if let Some(cb) = self.on_change {
            cb.run(Some(value));
        }
    }

    fn clear(&self, e: Option<web_sys::MouseEvent>) {
        if self.value.get_untracked().is_some() && self.show_clear.get_untracked() {
            self.value.set(None);
            self.state.set(State::default());
            if let Some(cb) = self.on_clear {
                cb.run(());
            }
        }
        if let Some(e) = e {
            e.stop_propagation();
        }
    }

    /// Клик по корню: открывает или закрывает панель, кроме кликов внутри панели.
    fn root_click(&self, e: &web_sys::MouseEvent) {
        if self.blocked() {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let target = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            if self.overlay_ref.get_untracked().is_some_and(|o| o.contains(target.as_ref())) {
                return;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = e;
        if self.open.get_untracked() {
            self.hide(false);
        } else {
            self.show();
        }
        self.focus_input();
        self.clicked.set_value(true);
    }

    /// Клик по варианту.
    fn option_click(&self, key: &str) {
        let Some(node) = self.tree(|m| m.node(key).cloned()) else { return };
        if !node.valid {
            return;
        }
        let mobile = self.mobile.get_untracked();
        let result = self.update(|m, st| m.option_click(st, key, false, false, mobile));
        match result {
            Some(Click::Select(value)) => {
                self.select(value);
                self.hide(true);
            }
            Some(Click::Group) => {
                if let Some(cb) = self.on_group_change {
                    cb.run(node.value);
                }
                self.focus_input();
            }
            Some(Click::Closed) | None => self.focus_input(),
        }
    }

    fn option_hover(&self, key: &str) {
        let (has_value, hover, mobile) =
            (self.value.get_untracked().is_some(), self.focus_on_hover.get_untracked(), self.mobile.get_untracked());
        self.update(|m, st| m.hover(st, key, has_value, hover, mobile));
    }

    fn option_move(&self, key: &str) {
        if self.focused.get_untracked() && self.focus_on_hover.get_untracked() && self.tree(|m| m.node(key).is_some_and(|n| n.valid)) {
            self.state.update(|st| st.focus = Some(key.to_owned()));
        }
    }

    fn scroll_focused_into_view(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(key) = self.state.get_untracked().focus
            && let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&format!("{}_{key}", self.id)))
        {
            let init = web_sys::ScrollIntoViewOptions::new();
            init.set_block(web_sys::ScrollLogicalPosition::Nearest);
            init.set_inline(web_sys::ScrollLogicalPosition::Start);
            el.scroll_into_view_with_scroll_into_view_options(&init);
        }
    }

    /// Печатаемый символ: поиск по началу подписи; строка поиска сбрасывается через 500 мс.
    fn type_char(&self, ch: &str) {
        self.search.update_value(|s| s.push_str(ch));
        let text = self.search.get_value();
        let (hover, mobile) = (self.focus_on_hover.get_untracked(), self.mobile.get_untracked());
        self.update(|m, st| m.search(st, &text, hover, mobile));
        self.scroll_focused_into_view();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
            let search = self.search;
            if let Ok(handle) = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || drop(search.try_set_value(String::new())),
                Duration::from_millis(500),
            ) {
                self.search_timer.set_value(Some(handle));
            }
        }
    }

    fn key_down(&self, e: &web_sys::KeyboardEvent) {
        if self.blocked() {
            e.prevent_default();
            return;
        }
        let (hover, mobile, clicked, open) =
            (self.focus_on_hover.get_untracked(), self.mobile.get_untracked(), self.clicked.get_value(), self.open.get_untracked());
        let meta = e.meta_key() || e.ctrl_key();
        match e.code().as_str() {
            "ArrowDown" => {
                if open {
                    self.update(|m, st| m.arrow_down(st, clicked, hover, mobile));
                    self.scroll_focused_into_view();
                } else {
                    self.show();
                }
                e.prevent_default();
            }
            "ArrowUp" => {
                if e.alt_key() {
                    self.update(|m, st| m.tab(st, mobile));
                    if open {
                        self.hide(false);
                    }
                } else if open {
                    self.update(|m, st| m.arrow_up(st, clicked, hover, mobile));
                    self.scroll_focused_into_view();
                } else {
                    self.show();
                }
                e.prevent_default();
            }
            "ArrowLeft" => {
                if open {
                    self.update(|m, st| m.arrow_left(st, hover, mobile));
                    self.search.set_value(String::new());
                    self.scroll_focused_into_view();
                    e.prevent_default();
                }
            }
            "ArrowRight" => {
                if open {
                    self.update(|m, st| m.arrow_right(st, hover, mobile));
                    self.search.set_value(String::new());
                    self.scroll_focused_into_view();
                    e.prevent_default();
                }
            }
            "Home" => {
                if open {
                    self.update(|m, st| m.home(st, hover, mobile));
                    self.scroll_focused_into_view();
                } else {
                    self.show();
                }
                e.prevent_default();
            }
            "End" => {
                if open {
                    self.update(|m, st| m.end(st, hover, mobile));
                    self.scroll_focused_into_view();
                } else {
                    self.show();
                }
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                if !open {
                    self.show();
                } else if let Some(key) = self.state.get_untracked().focus {
                    let result = self.update(|m, st| m.option_click(st, &key, false, false, mobile));
                    match result {
                        Some(Click::Select(value)) => {
                            self.select(value);
                            self.hide(false);
                        }
                        Some(Click::Group) => {
                            if let (Some(cb), Some(node)) = (self.on_group_change, self.tree(|m| m.node(&key).cloned())) {
                                cb.run(node.value);
                            }
                        }
                        _ => {}
                    }
                }
                e.prevent_default();
            }
            "Escape" => {
                if open {
                    self.hide(true);
                }
                e.prevent_default();
            }
            "Tab" => {
                self.update(|m, st| m.tab(st, mobile));
                if open {
                    self.hide(false);
                }
            }
            "Backspace" => {
                if self.value.get_untracked().is_some() && self.show_clear.get_untracked() {
                    self.clear(None);
                }
                e.stop_propagation();
            }
            "PageDown" | "PageUp" | "ShiftLeft" | "ShiftRight" => {}
            _ => {
                if !meta && e.key().chars().count() == 1 {
                    if !open {
                        self.show();
                    }
                    self.type_char(&e.key());
                }
            }
        }
        self.clicked.set_value(false);
    }

    /// Панель только что открылась: z-index, положение, слушатели окна, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        if let (Some(overlay), Some(root)) = (self.overlay_ref.get_untracked(), self.root_ref.get_untracked()) {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let overlay: web_sys::HtmlElement = overlay.unchecked_into();
            let _ = overlay.style().set_property("z-index", &z.to_string());
            align_overlay(&overlay, &root.unchecked_into::<web_sys::HtmlElement>(), AlignMode::Relative, true);
        }
        let this = self.clone();
        let outside = window_event_listener(ev::click, move |e: web_sys::MouseEvent| {
            let inside = this
                .root_ref
                .get_untracked()
                .is_some_and(|root| e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|n| root.contains(Some(&n))));
            if !inside && this.open.get_untracked() {
                this.hide(false);
            }
        });
        let this = self.clone();
        let resize = window_event_listener(ev::resize, move |_| {
            if this.open.get_untracked() {
                this.hide(false);
            }
        });
        self.listeners.update_value(|l| l.extend([outside, resize]));
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Панель только что закрылась: слушатели и z-index освобождаются, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    fn release(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }
}

/// Всё, что нужно для вывода списков.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<CascadeSelectPt>,
    option: Option<Callback<CascadeOptionContext, AnyView>>,
    group_icon: Option<ViewFn>,
}

/// Поле выбора значения из каскада вложенных списков.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn CascadeSelect(
    /// Варианты.
    #[prop(optional, into)]
    options: Prop<Vec<CascadeOption>>,
    /// Значение выбранного листа (двусторонний сигнал).
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Подсказка пустого поля.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Показывать значок очистки.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Идёт загрузка.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Вариант оформления.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Ширина окна, до которой включён мобильный режим (по умолчанию `960px`).
    #[prop(optional, into)]
    breakpoint: OptProp<String>,
    /// Раскрывать группы при наведении и переносе фокуса клавишами.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_on_hover: Prop<bool>,
    /// Фокус на первый вариант при открытии.
    #[prop(optional, into)]
    auto_option_focus: Prop<bool>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Атрибут `id` скрытого поля.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Порядок обхода Tab.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Дополнительный класс панели.
    #[prop(optional, into)]
    panel_style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<CascadeSelectPt>,
    /// Своё представление выбранного (вместо метки): подпись выбранного листа.
    #[prop(optional)]
    value_view: Option<Callback<Option<String>, AnyView>>,
    /// Своё содержимое варианта.
    #[prop(optional)]
    option: Option<Callback<CascadeOptionContext, AnyView>>,
    /// Свой значок группы.
    #[prop(optional, into)]
    group_icon: Option<ViewFn>,
    /// Значение изменено.
    #[prop(optional)]
    on_change: Option<Callback<Option<String>>>,
    /// Группа раскрыта кликом (значение — значение группы, т. е. её подпись).
    #[prop(optional)]
    on_group_change: Option<Callback<String>>,
    /// Панель показана.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Панель скрыта.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
    /// Значение очищено.
    #[prop(optional)]
    on_clear: Option<Callback<()>>,
    /// Скрытое поле получило фокус.
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Скрытое поле потеряло фокус.
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let nodes = Memo::new({
        let options = options.clone();
        move |_| options.with(|o| build(o))
    });
    let core = Arc::new(Core {
        id: menu_id.clone(),
        nodes,
        value: value.unwrap_or_else(|| RwSignal::new(None)),
        state: RwSignal::new(State::default()),
        open: RwSignal::new(false),
        focused: RwSignal::new(false),
        mobile: RwSignal::new(false),
        clicked: StoredValue::new(false),
        disabled: disabled.clone(),
        loading: loading.clone(),
        show_clear: show_clear.clone(),
        focus_on_hover,
        auto_option_focus,
        root_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        input_ref: NodeRef::new(),
        listeners: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        search: StoredValue::new(String::new()),
        #[cfg(target_arch = "wasm32")]
        search_timer: StoredValue::new_local(None),
        on_change,
        on_group_change,
        on_show,
        on_hide,
        on_clear,
        on_focus,
        on_blur,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        client::bind_media(breakpoint.get_untracked().unwrap_or_else(|| "960px".to_owned()), core.mobile, move || c.hide(false));
        let c = core.clone();
        Effect::new(move |_| {
            let is_open = c.open.get();
            if is_open != c.was_open.get_value() {
                c.was_open.set_value(is_open);
                if is_open {
                    c.opened();
                } else {
                    c.closed();
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = breakpoint;

    let ctx = Ctx { core: core.clone(), pt: pt.clone(), option, group_icon };
    let selected_label = {
        let core = core.clone();
        move || -> Option<String> { core.value.get().and_then(|v| core.nodes.with(|n| Menu { nodes: n }.label_of(&v).map(str::to_owned))) }
    };
    let filled = {
        let core = core.clone();
        move || core.value.with(Option::is_some)
    };
    let root_class = {
        let (core, pt, style_class, filled) = (core.clone(), pt.clone(), style_class.clone(), filled.clone());
        let (disabled, invalid, variant, size, fluid, show_clear) =
            (disabled.clone(), invalid.clone(), variant.clone(), size.clone(), fluid.clone(), show_clear.clone());
        move || {
            let filled_variant = variant.get().or(use_prime_config().input_variant) == Some(Variant::Filled);
            let (open, focused) = (core.open.get(), core.focused.get());
            let mut cn = ClassNames::new();
            cn.add("p-cascadeselect p-component p-inputwrapper")
                .add_if(show_clear.get() && !disabled.get(), "p-cascadeselect-clearable")
                .add_if(core.mobile.get(), "p-cascadeselect-mobile")
                .add_if(disabled.get(), "p-disabled")
                .add_if(invalid.get(), "p-invalid")
                .add_if(focused, "p-focus")
                .add_if(filled(), "p-inputwrapper-filled")
                .add_if(filled_variant, "p-variant-filled")
                .add_if(focused || open, "p-inputwrapper-focus")
                .add_if(open, "p-cascadeselect-open")
                .add_if(fluid.get(), "p-cascadeselect-fluid")
                .add_if(size.get() == Some(Size::Small), "p-cascadeselect-sm p-inputfield-sm")
                .add_if(size.get() == Some(Size::Large), "p-cascadeselect-lg p-inputfield-lg");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (pt, style) = (pt.clone(), style.clone());
        move || {
            let st = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!st.is_empty()).then_some(st)
        }
    };
    let label_class = {
        let (pt, selected_label, placeholder) = (pt.clone(), selected_label.clone(), placeholder.clone());
        move || {
            let empty = selected_label().is_none();
            let has_placeholder = placeholder.with(|p| p.as_deref().is_some_and(|p| !p.is_empty()));
            let mut cn = ClassNames::new();
            cn.add("p-cascadeselect-label")
                .add_if(empty && has_placeholder, "p-placeholder")
                .add_if(empty && !has_placeholder, "p-cascadeselect-label-empty");
            cn.add_user(pt.label.extra_class());
            cn.finish_cow()
        }
    };
    let label_content = {
        let (core, selected_label, placeholder) = (core.clone(), selected_label.clone(), placeholder.clone());
        move || -> AnyView {
            if let Some(cb) = value_view {
                return cb.run(selected_label());
            }
            let _ = &core;
            selected_label().or_else(|| placeholder.get()).unwrap_or_else(|| "p-emptylabel".to_owned()).into_any()
        }
    };
    let clear = {
        let (core, pt, filled, disabled, show_clear) = (core.clone(), pt.clone(), filled.clone(), disabled.clone(), show_clear.clone());
        move || {
            (filled() && !disabled.get() && show_clear.get()).then(|| {
                let core = core.clone();
                let class = pt.clear_icon.merge_class("p-cascadeselect-clear-icon").into_owned();
                view! { <TimesIcon class={class} on:click={move |e: web_sys::MouseEvent| core.clear(Some(e))} {..pt.clear_icon.attrs()} /> }
            })
        }
    };
    let dropdown_content = {
        let (loading, pt) = (loading.clone(), pt.clone());
        move || -> AnyView {
            if loading.get() {
                view! { <SpinnerIcon spin=true class={pt.loading_icon.merge_class("p-cascadeselect-loading-icon").into_owned()} /> }
                    .into_any()
            } else {
                view! { <ChevronDownIcon class={pt.dropdown_icon.merge_class("p-cascadeselect-dropdown-icon").into_owned()} /> }.into_any()
            }
        }
    };
    let panel = {
        let (ctx, panel_style_class) = (ctx.clone(), panel_style_class.clone());
        move || ctx.core.open.get().then(|| panel_view(&ctx, &panel_style_class))
    };
    let input_attrs: Vec<AnyAttribute> =
        {
            let (core, disabled, input_id, tabindex, aria_label, aria_labelledby, placeholder) = (
                core.clone(),
                disabled.clone(),
                input_id.clone(),
                tabindex.clone(),
                aria_label.clone(),
                aria_labelledby.clone(),
                placeholder.clone(),
            );
            let attr = |name: &'static str, f: Box<dyn Fn() -> Option<String> + Send + Sync>| custom_attribute(name, f).into_any_attr();
            vec![
                attr("id", Box::new(move || input_id.get())),
                attr("aria-label", Box::new(move || aria_label.get())),
                attr("aria-labelledby", Box::new(move || aria_labelledby.get())),
                attr("placeholder", Box::new(move || placeholder.get())),
                attr("aria-expanded", {
                    let core = core.clone();
                    Box::new(move || Some(core.open.get().to_string()))
                }),
                attr("aria-controls", {
                    let core = core.clone();
                    Box::new(move || core.open.get().then(|| format!("{}_tree", core.id)))
                }),
                attr("aria-activedescendant", {
                    let core = core.clone();
                    Box::new(move || {
                        if core.focused.get() { core.state.with(|s| s.focus.clone()).map(|k| format!("{}_{k}", core.id)) } else { None }
                    })
                }),
                attr("disabled", {
                    let disabled = disabled.clone();
                    Box::new(move || disabled.get().then(String::new))
                }),
                attr("tabindex", Box::new(move || Some(if disabled.get() { -1 } else { tabindex.get().unwrap_or(0) }.to_string()))),
            ]
        };
    let (focus_core, blur_core, key_core, click_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let (root_attrs, hidden_wrapper_attrs, hidden_attrs, label_attrs, dropdown_attrs) =
        (pt.root.attrs(), pt.hidden_input_wrapper.attrs(), pt.hidden_input.attrs(), pt.label.attrs(), pt.dropdown.attrs());
    let (hidden_wrapper_class, dropdown_class) = (
        pt.hidden_input_wrapper.merge_class("p-hidden-accessible").into_owned(),
        pt.dropdown.merge_class("p-cascadeselect-dropdown").into_owned(),
    );
    let dropdown_expanded = {
        let core = core.clone();
        move || core.open.get().to_string()
    };
    let (root_ref, input_ref) = (core.root_ref, core.input_ref);
    view! {
        <div node_ref={root_ref} id={menu_id} class={root_class} style={root_style} on:click={move |e| click_core.root_click(&e)} {..root_attrs}>
            <div class={hidden_wrapper_class} data-p-hidden-accessible="true" {..hidden_wrapper_attrs}>
                <input
                    node_ref={input_ref}
                    type="text"
                    role="combobox"
                    readonly
                    aria-haspopup="tree"
                    on:focus={move |e: web_sys::FocusEvent| {
                        if focus_core.disabled.get_untracked() {
                            return;
                        }
                        focus_core.focused.set(true);
                        if let Some(cb) = focus_core.on_focus {
                            cb.run(e);
                        }
                    }}
                    on:blur={move |e: web_sys::FocusEvent| {
                        blur_core.focused.set(false);
                        blur_core.state.update(|st| st.focus = None);
                        blur_core.search.set_value(String::new());
                        if let Some(cb) = blur_core.on_blur {
                            cb.run(e);
                        }
                    }}
                    on:keydown={move |e| key_core.key_down(&e)}
                    {..input_attrs}
                    {..hidden_attrs}
                />
            </div>
            <span class={label_class} {..label_attrs}>{label_content}</span>
            {clear}
            <div class={dropdown_class} role="button" aria-haspopup="listbox" aria-expanded={dropdown_expanded} aria-hidden="true" {..dropdown_attrs}>
                {dropdown_content}
            </div>
            {panel}
        </div>
    }
}

/// Панель с корневым списком.
fn panel_view(ctx: &Ctx, extra: &OptProp<String>) -> impl IntoView + use<> {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let class = {
        let (pt, extra, mobile) = (pt.clone(), extra.clone(), core.mobile);
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-cascadeselect-overlay p-component-overlay p-component").add_if(mobile.get(), "p-cascadeselect-mobile-active");
            extra.with(|c| cn.add_user(c.as_deref()));
            pt.overlay.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    // В PrimeNG положение задаёт обёртка `p-overlay`; у нас панель внутри корня, поэтому позиционирование — встроенным стилем.
    let overlay_style = pt.overlay.merge_style("position: absolute; top: 0; left: 0");
    let (container_class, container_attrs, overlay_attrs) =
        (pt.list_container.merge_class("p-cascadeselect-list-container").into_owned(), pt.list_container.attrs(), pt.overlay.attrs());
    let (list_class, list_attrs) = (pt.list.merge_class("p-cascadeselect-list").into_owned(), pt.list.attrs());
    let list_id = format!("{}_tree", core.id);
    let items = {
        let ctx = ctx.clone();
        move || level_view(&ctx, "")
    };
    let overlay_ref = core.overlay_ref;
    view! {
        <div node_ref={overlay_ref} class={class} style={overlay_style} {..overlay_attrs}>
            <div class={container_class} {..container_attrs}>
                <ul id={list_id} class={list_class} role="tree" {..list_attrs}>{items}</ul>
            </div>
        </div>
    }
}

/// Варианты одного списка (дети узла `parent`).
fn level_view(ctx: &Ctx, parent: &str) -> AnyView {
    let nodes: Vec<Node> = ctx.core.nodes.with_untracked(|n| n.iter().filter(|x| x.parent_key == parent).cloned().collect());
    nodes.iter().map(|n| option_view(ctx, n)).collect_view().into_any()
}

#[allow(clippy::clone_on_copy)] // замыкание над `Memo` — `Copy`, `clone()` оставлен для единообразия с остальными замыканиями
fn option_view(ctx: &Ctx, node: &Node) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let id = format!("{}_{key}", core.id);
    let state = core.state;
    // Memo: вложенный список перестраивается только при смене флага этого варианта, а не при любом изменении состояния.
    let is_active = {
        let key = key.clone();
        let memo = Memo::new(move |_| state.with(|s| s.active.contains(&key)));
        move || memo.get()
    };
    let is_focused = {
        let (key, focused) = (key.clone(), core.focused);
        move || focused.get() && state.with(|s| s.focus.as_deref() == Some(key.as_str()))
    };
    let is_selected = {
        let (core, value) = (core.clone(), node.value.clone());
        let group = node.group;
        move || !group && core.value.with(|v| v.as_deref() == Some(value.as_str()))
    };
    let class = {
        let (pt, is_active, is_focused, is_selected) = (pt.clone(), is_active.clone(), is_focused.clone(), is_selected.clone());
        let (group, disabled) = (node.group, !node.valid);
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-cascadeselect-option")
                .add_if(group, "p-cascadeselect-option-group")
                .add_if(is_active(), "p-cascadeselect-option-active")
                .add_if(is_selected(), "p-cascadeselect-option-selected")
                .add_if(is_focused(), "p-focus")
                .add_if(disabled, "p-disabled");
            pt.option.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let text = match ctx.option {
        Some(template) => untrack(|| template.run(CascadeOptionContext { label: node.label.clone(), value: node.value.clone(), level: node.level, group: node.group })),
        None => view! { <span class={pt.option_text.merge_class("p-cascadeselect-option-text").into_owned()} {..pt.option_text.attrs()}>{node.label.clone()}</span> }.into_any(),
    };
    let group_icon = node.group.then(|| {
        let inner = match ctx.group_icon.clone() {
            Some(custom) => custom.run(),
            None => view! { <AngleRightIcon /> }.into_any(),
        };
        view! { <span class={pt.group_icon.merge_class("p-cascadeselect-group-icon").into_owned()} {..pt.group_icon.attrs()}>{inner}</span> }
    });
    let sublist = node.group.then(|| {
        let (ctx, key, is_active) = (ctx.clone(), key.clone(), is_active.clone());
        let (class, attrs) = (
            ctx.pt.option_list.merge_class("p-cascadeselect-list p-cascadeselect-overlay p-cascadeselect-option-list").into_owned(),
            ctx.pt.option_list.attrs(),
        );
        move || {
            is_active().then(|| {
                let children = untrack(|| level_view(&ctx, &key));
                let list_ref: NodeRef<html::Ul> = NodeRef::new();
                #[cfg(target_arch = "wasm32")]
                Effect::new(move |_| {
                    if let Some(ul) = list_ref.get() {
                        flip_if_clipped(&ul);
                    }
                });
                view! { <ul node_ref={list_ref} class={class.clone()} role="group" {..attrs.clone()}>{children}</ul> }
            })
        }
    });
    let selected_attr = {
        let is_selected = is_selected.clone();
        let group = node.group;
        move || (!group).then(|| is_selected().to_string())
    };
    let (click_core, enter_core, move_core) = (core.clone(), core.clone(), core.clone());
    let (click_key, enter_key, move_key) = (key.clone(), key.clone(), key.clone());
    let (li_attrs, content_attrs) = {
        let mut li = pt.option.attrs();
        li.push(custom_attribute("aria-level", (node.level + 1).to_string()).into_any_attr());
        (li, pt.option_content.attrs())
    };
    let content_class = pt.option_content.merge_class("p-cascadeselect-option-content").into_owned();
    view! {
        <li
            id={id}
            class={class}
            role="treeitem"
            aria-setsize={node.siblings.to_string()}
            aria-posinset={(node.index + 1).to_string()}
            aria-label={node.label.clone()}
            aria-selected={selected_attr}
            {..li_attrs}
        >
            <div
                class={content_class}
                on:click={move |_| click_core.option_click(&click_key)}
                on:mouseenter={move |_| enter_core.option_hover(&enter_key)}
                on:mousemove={move |_| move_core.option_move(&move_key)}
                {..content_attrs}
            >
                {text}
                {group_icon}
            </div>
            {sublist}
        </li>
    }
    .into_any()
}

/// Если вложенный список выходит за правый край окна, он раскрывается влево (аналог `position()` PrimeNG).
#[cfg(target_arch = "wasm32")]
fn flip_if_clipped(ul: &web_sys::HtmlUListElement) {
    let el: &web_sys::HtmlElement = ul.as_ref();
    let Some(window) = web_sys::window() else { return };
    let width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(f64::MAX);
    if el.get_bounding_client_rect().right() > width - 16.0 {
        let style = el.style();
        let _ = style.set_property("left", "auto");
        let _ = style.set_property("right", "100%");
    }
}

#[cfg(target_arch = "wasm32")]
mod client {
    //! Мобильный режим: `matchMedia("(max-width: <breakpoint>)")`.

    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    /// Ведёт `mobile` по медиазапросу, вызывает `on_change` при смене и снимает слушатель при размонтировании.
    pub fn bind_media(breakpoint: String, mobile: RwSignal<bool>, on_change: impl Fn() + 'static) {
        let Some(window) = web_sys::window() else { return };
        let Some(query) = window.match_media(&format!("(max-width: {breakpoint})")).ok().flatten() else { return };
        mobile.set(query.matches());
        let closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(move |e: web_sys::MediaQueryListEvent| {
            mobile.try_set(e.matches());
            on_change();
        });
        let _ = query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        let held = StoredValue::new_local((query, closure));
        on_cleanup(move || {
            held.try_with_value(|(query, closure)| {
                let _ = query.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            });
        });
    }
}
