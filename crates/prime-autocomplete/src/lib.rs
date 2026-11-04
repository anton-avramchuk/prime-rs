//! AutoComplete — порт `p-autocomplete` из PrimeNG 21.1.10.
//!
//! Поле ввода с подсказками: при вводе от `min_length` символов через `delay` вызывается `on_complete`, пользователь
//! обновляет `suggestions`, и варианты показываются в панели внутри корня. Варианты — [`AutoCompleteOption`] (группы —
//! через `items` и `group = true`). Значение одиночного режима — `RwSignal<String>`, множественного (`multiple`, чипы) —
//! `RwSignal<Vec<String>>`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_autocomplete::{AutoComplete, AutoCompleteOption};
//!
//! let items = vec![AutoCompleteOption::new("Apple"), AutoCompleteOption::new("Banana")];
//! let value = RwSignal::new("Apple".to_owned());
//! let html = Owner::new().with(|| view! { <AutoComplete suggestions=items value=value /> }.to_html());
//! assert!(html.contains("p-autocomplete-input") && html.contains("Apple") && !html.contains("p-autocomplete-overlay"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::leptos_dom::helpers::{TimeoutHandle, WindowListenerHandle};
use leptos::prelude::*;
use prime_chip::Chip;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{ChevronDownIcon, SpinnerIcon, TimesIcon};
use prime_inputtext::{InputTextState, write_classes as input_classes};
use std::sync::Arc;

pub use state::{
    AutoCompleteOption, Row, first, first_focused, flatten, input_text, is_selected, label_for_value, last, last_focused, matched, next,
    pos_inset, prev, set_size, split_separator,
};

#[cfg(target_arch = "wasm32")]
use leptos::ev;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use prime_core::ZGroup;
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Что отправляет кнопка-стрелка.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AutoCompleteDropdownMode {
    /// Пустой запрос.
    #[default]
    Blank,
    /// Текущий текст поля.
    Current,
}

/// Событие запроса вариантов.
#[derive(Clone, Debug)]
pub struct AutoCompleteCompleteEvent {
    /// Запрос.
    pub query: String,
}

/// Pass-through частей [`AutoComplete`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AutoCompletePt {
    /// Корень.
    pub root: Pt,
    /// Поле ввода.
    pub input: Pt,
    /// Контейнер чипов.
    pub input_multiple: Pt,
    /// Элемент чипа.
    pub chip_item: Pt,
    /// Контейнер поля внутри чипов.
    pub input_chip: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Спиннер запроса.
    pub loader: Pt,
    /// Кнопка раскрытия.
    pub dropdown: Pt,
    /// Панель.
    pub overlay: Pt,
    /// Контейнер списка.
    pub list_container: Pt,
    /// Список.
    pub list: Pt,
    /// Вариант.
    pub option: Pt,
    /// Заголовок группы.
    pub option_group: Pt,
    /// Сообщение о пустом результате.
    pub empty_message: Pt,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    options: Prop<Vec<AutoCompleteOption>>,
    rows: Memo<Vec<Row>>,
    value: RwSignal<String>,
    values: RwSignal<Vec<String>>,
    text: RwSignal<String>,
    last_typed: StoredValue<String>,
    multiple: bool,
    unique: Prop<bool>,
    open: RwSignal<bool>,
    focused: RwSignal<bool>,
    loading: RwSignal<bool>,
    focus_index: RwSignal<Option<usize>>,
    focused_chip: RwSignal<Option<usize>>,
    dirty: StoredValue<bool>,
    min_length: Prop<usize>,
    delay: u32,
    force_selection: Prop<bool>,
    complete_on_focus: Prop<bool>,
    auto_option_focus: Prop<bool>,
    focus_on_hover: Prop<bool>,
    typeahead: Prop<bool>,
    add_on_tab: Prop<bool>,
    add_on_blur: Prop<bool>,
    separator: OptProp<String>,
    dropdown_mode: AutoCompleteDropdownMode,
    disabled: Prop<bool>,
    root_ref: NodeRef<html::Div>,
    input_ref: NodeRef<html::Input>,
    container_ref: NodeRef<html::Ul>,
    overlay_ref: NodeRef<html::Div>,
    dropdown_ref: NodeRef<html::Button>,
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    on_complete: Option<Callback<AutoCompleteCompleteEvent>>,
    on_select: Option<Callback<String>>,
    on_unselect: Option<Callback<String>>,
    on_add: Option<Callback<String>>,
    on_clear: Option<Callback<()>>,
    on_dropdown_click: Option<Callback<String>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn blocked(&self) -> bool {
        self.disabled.get_untracked()
    }

    fn filled(&self) -> bool {
        if self.multiple { self.values.with_untracked(|v| !v.is_empty()) } else { self.value.with_untracked(|v| !v.is_empty()) }
    }

    fn focus_input(&self) {
        if let Some(input) = self.input_ref.get_untracked() {
            let _ = input.focus();
        }
    }

    fn selected(&self, row: &Row) -> bool {
        let (value, values) = (self.value.get_untracked(), self.values.get_untracked());
        is_selected(row, &value, &values, self.multiple, self.unique.get_untracked())
    }

    fn selected_pred(&self) -> impl Fn(&Row) -> bool + '_ {
        let (value, values, unique) = (self.value.get_untracked(), self.values.get_untracked(), self.unique.get_untracked());
        move |row: &Row| is_selected(row, &value, &values, self.multiple, unique)
    }

    fn first_focused_index(&self) -> Option<usize> {
        self.rows.with_untracked(|rows| first_focused(rows, self.selected_pred()))
    }

    fn last_focused_index(&self) -> Option<usize> {
        self.rows.with_untracked(|rows| last_focused(rows, self.selected_pred()))
    }

    fn show(&self, focus: bool) {
        self.dirty.set_value(true);
        self.open.set(true);
        let index = self
            .focus_index
            .get_untracked()
            .or_else(|| if self.auto_option_focus.get_untracked() { self.first_focused_index() } else { None });
        self.focus_index.set(index);
        if focus {
            self.focus_input();
        }
    }

    fn hide(&self, focus: bool) {
        self.dirty.try_set_value(focus);
        self.open.try_set(false);
        self.focus_index.try_set(None);
        if focus {
            self.focus_input();
        }
        self.force_selection_check();
    }

    /// `updateInputWithForceSelection`: несовпавший ввод очищается, совпавший выбирается.
    fn force_selection_check(&self) {
        if !self.force_selection.get_untracked() || self.open.get_untracked() {
            return;
        }
        let text = self.text.get_untracked();
        let cleared = text.is_empty() && self.filled();
        if text.is_empty() && !cleared {
            return;
        }
        if !cleared && text.chars().count() < self.min_length.get_untracked() {
            return;
        }
        let found = self.rows.with_untracked(|rows| matched(rows, &text));
        match found {
            None => {
                self.text.set(String::new());
                if !self.multiple {
                    self.clear();
                }
            }
            Some(i) => {
                if !self.rows.with_untracked(|rows| self.selected(&rows[i])) {
                    self.option_select(i, false);
                }
            }
        }
    }

    fn clear(&self) {
        self.value.set(String::new());
        self.values.set(Vec::new());
        self.text.set(String::new());
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }

    /// `onOptionSelect`.
    fn option_select(&self, index: usize, hide: bool) {
        let Some(row) = self.rows.with_untracked(|rows| rows.get(index).cloned()) else { return };
        if !row.valid() {
            return;
        }
        if self.multiple {
            self.text.set(String::new());
            if !self.selected(&row) {
                self.values.update(|v| v.push(row.value.clone()));
            }
        } else {
            self.last_typed.set_value(String::new());
            self.value.set(row.value.clone());
            self.text.set(row.label.clone());
        }
        if let Some(cb) = self.on_select {
            cb.run(row.value);
        }
        if hide {
            self.hide(true);
        }
    }

    fn remove_chip(&self, index: usize) {
        let removed = self.values.with_untracked(|v| v.get(index).cloned());
        let Some(removed) = removed else { return };
        self.values.update(|v| {
            v.remove(index);
        });
        if let Some(cb) = self.on_unselect {
            cb.run(removed);
        }
        self.focused_chip.set(None);
        self.focus_input();
    }

    /// Добавляет набранный текст чипом (режим `multiple` без `typeahead`).
    fn add_typed(&self, text: &str) -> bool {
        let text = text.trim();
        if text.is_empty() || self.values.with_untracked(|v| v.iter().any(|x| x == text)) {
            return false;
        }
        self.values.update(|v| v.push(text.to_owned()));
        if let Some(cb) = self.on_add {
            cb.run(text.to_owned());
        }
        self.text.set(String::new());
        true
    }

    /// `search`: запрос вариантов.
    fn search(&self, query: &str, source: &str) {
        if source == "input" && query.trim().is_empty() {
            return;
        }
        self.loading.set(true);
        if let Some(cb) = self.on_complete {
            cb.run(AutoCompleteCompleteEvent { query: query.to_owned() });
        }
    }

    fn cancel_search(&self) {
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Ввод в поле (`onInput`).
    fn input(self: &Arc<Self>, query: String) {
        self.text.set(query.clone());
        if !self.typeahead.get_untracked() {
            return;
        }
        self.cancel_search();
        if !self.multiple && !self.force_selection.get_untracked() {
            self.last_typed.set_value(query.clone());
            self.value.set(query.clone());
        }
        #[cfg(target_arch = "wasm32")]
        {
            if query.is_empty() && !self.multiple {
                if let Some(cb) = self.on_clear {
                    cb.run(());
                }
                let this = self.clone();
                set_timeout(move || this.hide(false), Duration::from_millis(u64::from(self.delay / 2)));
            } else if query.chars().count() >= self.min_length.get_untracked() {
                self.focus_index.set(None);
                let this = self.clone();
                let q = query;
                if let Ok(handle) = set_timeout_with_handle(move || this.search(&q, "input"), Duration::from_millis(u64::from(self.delay)))
                {
                    self.search_timer.set_value(Some(handle));
                }
            } else {
                self.hide(false);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = query;
    }

    fn dropdown_click(self: &Arc<Self>) {
        if self.blocked() {
            return;
        }
        let mut query = String::new();
        if self.open.get_untracked() {
            self.hide(true);
        } else {
            self.focus_input();
            query = self.text.get_untracked();
            match self.dropdown_mode {
                AutoCompleteDropdownMode::Blank => self.search("", "dropdown"),
                AutoCompleteDropdownMode::Current => self.search(&query, "dropdown"),
            }
        }
        if let Some(cb) = self.on_dropdown_click {
            cb.run(query);
        }
    }

    fn container_click(&self, e: &web_sys::MouseEvent) {
        if self.blocked() || self.loading.get_untracked() {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let target = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let in_input = self.input_ref.get_untracked().is_some_and(|i| i.contains(target.as_ref()));
            let in_dropdown = self.dropdown_ref.get_untracked().is_some_and(|d| d.contains(target.as_ref()));
            let in_overlay = self.overlay_ref.get_untracked().is_some_and(|o| o.contains(target.as_ref()));
            if in_input || in_dropdown || in_overlay {
                return;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = e;
        self.focus_input();
    }

    fn change_focus(&self, index: usize) {
        if self.focus_index.get_untracked() != Some(index) {
            self.focus_index.set(Some(index));
            self.scroll_into_view();
        }
    }

    fn scroll_into_view(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(i) = self.focus_index.get_untracked()
            && let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&format!("{}_{i}", self.id)))
        {
            let init = web_sys::ScrollIntoViewOptions::new();
            init.set_block(web_sys::ScrollLogicalPosition::Nearest);
            init.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&init);
        }
    }

    fn input_focus(self: &Arc<Self>, e: web_sys::FocusEvent) {
        if self.blocked() {
            return;
        }
        if !self.dirty.get_value() && self.complete_on_focus.get_untracked() {
            let text = self.text.get_untracked();
            self.search(&text, "focus");
        }
        self.dirty.set_value(true);
        self.focused.set(true);
        let index = self.focus_index.get_untracked().or_else(|| {
            if self.open.get_untracked() && self.auto_option_focus.get_untracked() { self.first_focused_index() } else { None }
        });
        self.focus_index.set(index);
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn input_blur(&self, e: web_sys::FocusEvent) {
        self.dirty.set_value(false);
        self.focused.set(false);
        self.focus_index.set(None);
        if self.add_on_blur.get_untracked() && self.multiple && !self.typeahead.get_untracked() {
            let text = self.text.get_untracked();
            self.add_typed(&text);
        }
        if let Some(cb) = self.on_blur {
            cb.run(e);
        }
    }

    fn key_down(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        if self.blocked() {
            e.prevent_default();
            return;
        }
        let open = self.open.get_untracked();
        match e.code().as_str() {
            "ArrowDown" => {
                if open {
                    let next_index = match self.focus_index.get_untracked() {
                        Some(i) => Some(self.rows.with_untracked(|r| next(r, i))),
                        None => self.first_focused_index(),
                    };
                    if let Some(i) = next_index {
                        self.change_focus(i);
                    }
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            "ArrowUp" => {
                if open {
                    if e.alt_key() {
                        if let Some(i) = self.focus_index.get_untracked() {
                            self.option_select(i, true);
                        } else {
                            self.hide(false);
                        }
                        e.prevent_default();
                    } else {
                        let prev_index = match self.focus_index.get_untracked() {
                            Some(i) => Some(self.rows.with_untracked(|r| prev(r, i))),
                            None => self.last_focused_index(),
                        };
                        if let Some(i) = prev_index {
                            self.change_focus(i);
                        }
                        e.prevent_default();
                        e.stop_propagation();
                    }
                }
            }
            "ArrowLeft" => {
                self.focus_index.set(None);
                if self.multiple && self.text.get_untracked().is_empty() && self.filled() {
                    if let Some(c) = self.container_ref.get_untracked() {
                        let _ = c.focus();
                    }
                    self.focused_chip.set(Some(self.values.with_untracked(Vec::len).saturating_sub(1)));
                }
            }
            "ArrowRight" => self.focus_index.set(None),
            "Enter" | "NumpadEnter" => {
                if !self.typeahead.get_untracked() && !self.force_selection.get_untracked() && self.multiple {
                    let text = self.text.get_untracked();
                    self.add_typed(&text);
                }
                if open {
                    if let Some(i) = self.focus_index.get_untracked() {
                        self.option_select(i, true);
                    } else {
                        self.hide(false);
                    }
                    e.prevent_default();
                }
            }
            "Escape" => {
                if open {
                    self.hide(true);
                }
                e.prevent_default();
            }
            "Tab" => {
                if let Some(i) = self.focus_index.get_untracked() {
                    self.option_select(i, true);
                } else if self.multiple && !self.typeahead.get_untracked() && self.add_on_tab.get_untracked() {
                    let text = self.text.get_untracked();
                    if self.add_typed(&text) {
                        e.prevent_default();
                        if open {
                            self.hide(false);
                        }
                    }
                } else if open {
                    self.hide(false);
                }
            }
            "Backspace" => {
                if self.multiple {
                    if self.text.get_untracked().is_empty() && self.filled() {
                        let last_index = self.values.with_untracked(Vec::len) - 1;
                        self.remove_chip(last_index);
                    }
                    e.stop_propagation();
                }
            }
            "ShiftLeft" | "ShiftRight" | "Home" | "End" | "PageDown" | "PageUp" => {}
            _ => {
                if let Some(sep) = self.separator.get_untracked()
                    && self.multiple
                    && !self.typeahead.get_untracked()
                    && e.key() == sep
                {
                    let text = self.text.get_untracked();
                    if self.add_typed(&text) {
                        e.prevent_default();
                    }
                }
            }
        }
    }

    /// Клавиши контейнера чипов (`onMultipleContainerKeyDown`): только события на самом контейнере.
    fn container_key_down(&self, e: &web_sys::KeyboardEvent) {
        if self.blocked() {
            e.prevent_default();
            return;
        }
        if e.target() != e.current_target() {
            return;
        }
        let len = self.values.with_untracked(Vec::len);
        match e.code().as_str() {
            "ArrowLeft" => {
                let i = self.focused_chip.get_untracked().filter(|i| *i > 0).map_or(0, |i| i - 1);
                self.focused_chip.set(Some(i));
            }
            "ArrowRight" => {
                let i = self.focused_chip.get_untracked().map_or(0, |i| i + 1);
                if i >= len {
                    self.focused_chip.set(None);
                    self.focus_input();
                } else {
                    self.focused_chip.set(Some(i));
                }
            }
            "Backspace" => {
                if let Some(i) = self.focused_chip.get_untracked() {
                    self.remove_chip(i);
                }
            }
            _ => {}
        }
    }

    fn paste(&self, e: &web_sys::ClipboardEvent) {
        let Some(sep) = self.separator.get_untracked() else { return };
        if !self.multiple || self.typeahead.get_untracked() {
            return;
        }
        let Some(data) = e.clipboard_data().and_then(|d| d.get_data("Text").ok()) else { return };
        let mut changed = false;
        for part in split_separator(&data, &sep) {
            changed |= self.add_typed(&part);
        }
        if changed {
            e.prevent_default();
        }
    }

    /// Панель только что открылась: z-index, положение, слушатели окна, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        self.realign();
        if let Some(overlay) = self.overlay_ref.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let overlay: web_sys::HtmlElement = overlay.unchecked_into();
            let _ = overlay.style().set_property("z-index", &z.to_string());
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
        self.cancel_search();
    }

    /// Выравнивание панели по корню.
    #[cfg(target_arch = "wasm32")]
    fn realign(&self) {
        if let (Some(overlay), Some(root)) = (self.overlay_ref.get_untracked(), self.root_ref.get_untracked()) {
            align_overlay(
                &overlay.unchecked_into::<web_sys::HtmlElement>(),
                &root.unchecked_into::<web_sys::HtmlElement>(),
                AlignMode::Relative,
                true,
            );
        }
    }
}

/// Всё, что нужно для вывода вариантов.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<AutoCompletePt>,
    item: Option<Callback<AutoCompleteOption, AnyView>>,
    group_view: Option<Callback<String, AnyView>>,
}

/// Поле ввода с подсказками.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn AutoComplete(
    /// Варианты, предложенные на запрос.
    #[prop(optional, into)]
    suggestions: Prop<Vec<AutoCompleteOption>>,
    /// Значение одиночного режима: набранный текст либо значение выбранного варианта.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Значения множественного режима.
    #[prop(optional)]
    values: Option<RwSignal<Vec<String>>>,
    /// Множественный выбор с чипами.
    #[prop(optional)]
    multiple: bool,
    /// Варианты сгруппированы (верхний уровень — заголовки).
    #[prop(optional, into)]
    group: Prop<bool>,
    /// Кнопка-стрелка.
    #[prop(optional, into)]
    dropdown: Prop<bool>,
    /// Что отправляет кнопка-стрелка.
    #[prop(optional)]
    dropdown_mode: AutoCompleteDropdownMode,
    /// Значок очистки.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Очищать ввод, не совпавший ни с одним вариантом.
    #[prop(optional, into)]
    force_selection: Prop<bool>,
    /// Запрашивать варианты при первом фокусе.
    #[prop(optional, into)]
    complete_on_focus: Prop<bool>,
    /// Фокус на первый вариант при открытии панели.
    #[prop(optional, into)]
    auto_option_focus: Prop<bool>,
    /// Переносить фокус на вариант под курсором.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_on_hover: Prop<bool>,
    /// Минимальная длина запроса.
    #[prop(optional, default = Prop::Static(1), into)]
    min_length: Prop<usize>,
    /// Задержка запроса в миллисекундах.
    #[prop(optional, default = 300)]
    delay: u32,
    /// Подсказка пустого поля.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Высота списка (по умолчанию `200px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Запрашивать варианты при вводе (иначе поле — только ввод, `multiple` добавляет набранное).
    #[prop(optional, default = Prop::Static(true), into)]
    typeahead: Prop<bool>,
    /// Не добавлять повторяющиеся чипы.
    #[prop(optional, default = Prop::Static(true), into)]
    unique: Prop<bool>,
    /// Показывать сообщение при пустом результате.
    #[prop(optional, default = Prop::Static(true), into)]
    show_empty_message: Prop<bool>,
    /// Сообщение при пустом результате (по умолчанию — из `locale`).
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Добавлять набранное чипом по `Tab` (`multiple`, без `typeahead`).
    #[prop(optional, into)]
    add_on_tab: Prop<bool>,
    /// Добавлять набранное чипом при потере фокуса (`multiple`, без `typeahead`).
    #[prop(optional, into)]
    add_on_blur: Prop<bool>,
    /// Разделитель чипов (клавиша и вставка).
    #[prop(optional, into)]
    separator: OptProp<String>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только чтение.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Вариант оформления.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Тип поля (`text` по умолчанию).
    #[prop(optional, into)]
    input_type: OptProp<String>,
    /// Атрибут `id` поля.
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
    /// Атрибут `id` корня (основа идентификаторов вариантов).
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Дополнительный класс поля.
    #[prop(optional, into)]
    input_style_class: OptProp<String>,
    /// Дополнительный класс панели.
    #[prop(optional, into)]
    panel_style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<AutoCompletePt>,
    /// Своё содержимое варианта.
    #[prop(optional)]
    item: Option<Callback<AutoCompleteOption, AnyView>>,
    /// Свой заголовок группы (подпись группы).
    #[prop(optional)]
    group_view: Option<Callback<String, AnyView>>,
    /// Своё содержимое чипа (значение).
    #[prop(optional)]
    selected_item: Option<Callback<String, AnyView>>,
    /// Запрос вариантов.
    #[prop(optional)]
    on_complete: Option<Callback<AutoCompleteCompleteEvent>>,
    /// Вариант выбран (значение).
    #[prop(optional)]
    on_select: Option<Callback<String>>,
    /// Чип удалён (значение).
    #[prop(optional)]
    on_unselect: Option<Callback<String>>,
    /// Чип добавлен набранным текстом.
    #[prop(optional)]
    on_add: Option<Callback<String>>,
    /// Значение очищено.
    #[prop(optional)]
    on_clear: Option<Callback<()>>,
    /// Клик по кнопке-стрелке (запрос).
    #[prop(optional)]
    on_dropdown_click: Option<Callback<String>>,
    /// Панель показана.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Панель скрыта.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
    /// Поле получило фокус.
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Поле потеряло фокус.
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let root_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let rows = Memo::new({
        let (suggestions, group) = (suggestions.clone(), group.clone());
        move |_| suggestions.with(|s| flatten(s, group.get()))
    });
    let value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let values = values.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let text = RwSignal::new(if multiple { String::new() } else { suggestions.with(|s| input_text(s, &value.get_untracked())) });
    let core = Arc::new(Core {
        id: root_id.clone(),
        options: suggestions.clone(),
        rows,
        value,
        values,
        text,
        last_typed: StoredValue::new(String::new()),
        multiple,
        unique,
        open: RwSignal::new(false),
        focused: RwSignal::new(false),
        loading: RwSignal::new(false),
        focus_index: RwSignal::new(None),
        focused_chip: RwSignal::new(None),
        dirty: StoredValue::new(false),
        min_length,
        delay,
        force_selection,
        complete_on_focus,
        auto_option_focus,
        focus_on_hover,
        typeahead,
        add_on_tab,
        add_on_blur,
        separator,
        dropdown_mode,
        disabled: disabled.clone(),
        root_ref: NodeRef::new(),
        input_ref: NodeRef::new(),
        container_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        dropdown_ref: NodeRef::new(),
        listeners: StoredValue::new_local(Vec::new()),
        search_timer: StoredValue::new_local(None),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        on_complete,
        on_select,
        on_unselect,
        on_add,
        on_clear,
        on_dropdown_click,
        on_show,
        on_hide,
        on_focus,
        on_blur,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Значение извне (не набранное) обновляет текст поля.
        let c = core.clone();
        Effect::new(move |_| {
            let v = c.value.get();
            if !c.multiple && v != c.last_typed.get_value() {
                let t = c.options.with(|o| input_text(o, &v));
                if c.text.get_untracked() != t {
                    c.text.set(t);
                }
            }
        });
        // Открытие и закрытие панели: положение, z-index, слушатели.
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
        // Новые варианты после запроса: показать панель или скрыть (`handleSuggestionsChange`).
        let (c, show_empty_message) = (core.clone(), show_empty_message.clone());
        Effect::new(move |_| {
            // Исходный сигнал, а не `rows`: тот же набор вариантов после нового запроса тоже должен показать панель.
            let empty = c.options.with(Vec::is_empty);
            if c.loading.get_untracked() {
                if !empty || show_empty_message.get_untracked() {
                    c.show(false);
                } else {
                    c.hide(false);
                }
                let index = if c.open.get_untracked() && c.auto_option_focus.get_untracked() { c.first_focused_index() } else { None };
                c.focus_index.set(index);
                c.loading.set(false);
                let cc = c.clone();
                set_timeout(move || cc.realign(), Duration::from_millis(1));
            }
        });
    }

    let ctx = Ctx { core: core.clone(), pt: pt.clone(), item, group_view };
    let filled = {
        let core = core.clone();
        move || if core.multiple { core.values.with(|v| !v.is_empty()) } else { core.value.with(|v| !v.is_empty()) }
    };
    let root_class = {
        let (core, pt, style_class, filled, invalid, show_clear, disabled, fluid) = (
            core.clone(),
            pt.clone(),
            style_class.clone(),
            filled.clone(),
            invalid.clone(),
            show_clear.clone(),
            disabled.clone(),
            fluid.clone(),
        );
        move || {
            let (open, focused) = (core.open.get(), core.focused.get());
            let mut cn = ClassNames::new();
            cn.add("p-autocomplete p-component p-inputwrapper")
                .add_if(invalid.get(), "p-invalid")
                .add_if(focused, "p-focus")
                .add_if(filled(), "p-inputwrapper-filled")
                .add_if((focused && !disabled.get()) || open, "p-inputwrapper-focus")
                .add_if(open, "p-autocomplete-open")
                .add_if(show_clear.get() && !disabled.get(), "p-autocomplete-clearable")
                .add_if(fluid.get(), "p-autocomplete-fluid");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (pt, style) = (pt.clone(), style.clone());
        move || {
            let st = style.with(|s| {
                pt.root.merge_style(&format!("position: relative;{}", s.as_deref().map(|s| format!(" {s}")).unwrap_or_default()))
            });
            (!st.is_empty()).then_some(st)
        }
    };
    let input_class = {
        let (pt, filled, invalid, variant, size, fluid, input_style_class) =
            (pt.clone(), filled.clone(), invalid.clone(), variant.clone(), size.clone(), fluid.clone(), input_style_class.clone());
        move || {
            let variant = variant.get().or(use_prime_config().input_variant);
            let mut cn = ClassNames::new();
            input_classes(
                &mut cn,
                &InputTextState { filled: filled(), size: size.get(), invalid: invalid.get(), variant, fluid: fluid.get() },
            );
            cn.add("p-autocomplete-input");
            input_style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.input.extra_class());
            cn.finish_cow()
        }
    };
    let input_attrs = |multi: bool| -> Vec<AnyAttribute> {
        let attr = |name: &'static str, f: Box<dyn Fn() -> Option<String> + Send + Sync>| custom_attribute(name, f).into_any_attr();
        let (core, disabled, readonly, input_id, input_type, tabindex, placeholder, aria_label, aria_labelledby, filled) = (
            core.clone(),
            disabled.clone(),
            readonly.clone(),
            input_id.clone(),
            input_type.clone(),
            tabindex.clone(),
            placeholder.clone(),
            aria_label.clone(),
            aria_labelledby.clone(),
            filled.clone(),
        );
        vec![
            attr("type", Box::new(move || Some(input_type.get().unwrap_or_else(|| "text".to_owned())))),
            attr("id", Box::new(move || input_id.get())),
            attr("placeholder", Box::new(move || if multi && filled() { None } else { placeholder.get() })),
            attr("aria-label", Box::new(move || aria_label.get())),
            attr("aria-labelledby", Box::new(move || aria_labelledby.get())),
            attr("aria-expanded", {
                let core = core.clone();
                Box::new(move || Some(core.open.get().to_string()))
            }),
            attr("aria-controls", {
                let core = core.clone();
                Box::new(move || core.open.get().then(|| format!("{}_list", core.id)))
            }),
            attr("aria-activedescendant", {
                let core = core.clone();
                Box::new(move || if core.focused.get() { core.focus_index.get().map(|i| format!("{}_{i}", core.id)) } else { None })
            }),
            attr("disabled", {
                let disabled = disabled.clone();
                Box::new(move || disabled.get().then(String::new))
            }),
            attr("readonly", Box::new(move || readonly.get().then(String::new))),
            attr("tabindex", Box::new(move || Some(if disabled.get() { -1 } else { tabindex.get().unwrap_or(0) }.to_string()))),
        ]
    };
    let make_input = || {
        let (c1, c2, c3, c4, c5, c6) = (core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
        let text = core.text;
        (
            move |e: web_sys::Event| {
                #[cfg(target_arch = "wasm32")]
                {
                    let query =
                        e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()).map(|i| i.value()).unwrap_or_default();
                    c1.input(query);
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _ = (e, &c1);
            },
            move |e: web_sys::KeyboardEvent| c2.key_down(&e),
            move |e: web_sys::FocusEvent| c3.input_focus(e),
            move |e: web_sys::FocusEvent| c4.input_blur(e),
            move |_: web_sys::Event| c5.force_selection_check(),
            move |e: web_sys::ClipboardEvent| c6.paste(&e),
            move || text.get(),
        )
    };
    let (on_input, on_key, on_focus_, on_blur_, on_change, on_paste, text_value) = make_input();
    let single_input = (!multiple).then(|| {
        let (class, attrs) = (input_class.clone(), input_attrs(false));
        let (input_ref, input_pt_attrs) = (core.input_ref, pt.input.attrs());
        view! {
            <input
                node_ref={input_ref}
                class={class}
                role="combobox"
                aria-autocomplete="list"
                autocomplete="off"
                value={text_value}
                prop:value={text_value}
                on:input={on_input}
                on:keydown={on_key}
                on:focus={on_focus_}
                on:blur={on_blur_}
                on:change={on_change}
                on:paste={on_paste}
                {..attrs}
                {..input_pt_attrs}
            />
        }
    });
    let multi_container = multiple.then(|| {
        let chips = {
            let (core, pt, selected_item) = (core.clone(), pt.clone(), selected_item);
            move || {
                let vals = core.values.get();
                let len = vals.len();
                vals.into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let (core2, core3) = (core.clone(), core.clone());
                        let focused_chip = core.focused_chip;
                        let class = {
                            let pt = pt.clone();
                            move || {
                                let mut cn = ClassNames::new();
                                cn.add("p-autocomplete-chip-item").add_if(focused_chip.get() == Some(i), "p-focus");
                                pt.chip_item.merge_class(&cn.finish_cow()).into_owned()
                            }
                        };
                        let label = core2.options.with(|o| input_text(o, &v));
                        let content = selected_item.map(|cb| cb.run(v.clone()));
                        let disabled = core2.disabled.get_untracked();
                        let chip = match content {
                            Some(view) => view! { <Chip removable=true disabled=disabled style_class="p-autocomplete-chip" on_remove=Callback::new(move |_| core3.remove_chip(i))>{view}</Chip> }.into_any(),
                            None => view! { <Chip label=label removable=true disabled=disabled style_class="p-autocomplete-chip" on_remove=Callback::new(move |_| core3.remove_chip(i)) /> }.into_any(),
                        };
                        view! {
                            <li
                                class={class}
                                id={format!("{}_multiple_option_{i}", core.id)}
                                role="option"
                                aria-setsize={len.to_string()}
                                aria-posinset={(i + 1).to_string()}
                                aria-selected="true"
                                {..pt.chip_item.attrs()}
                            >
                                {chip}
                            </li>
                        }
                    })
                    .collect_view()
            }
        };
        let (class, attrs) = (input_class.clone(), input_attrs(true));
        let (input_ref, input_pt_attrs) = (core.input_ref, pt.input.attrs());
        let (on_input, on_key, on_focus_, on_blur_, on_change, on_paste, text_value) = make_input();
        let container_class = {
            let (pt, disabled, variant) = (pt.clone(), disabled.clone(), variant.clone());
            move || {
                let mut cn = ClassNames::new();
                cn.add("p-autocomplete-input-multiple").add_if(disabled.get(), "p-disabled").add_if(variant.get().or(use_prime_config().input_variant) == Some(Variant::Filled), "p-variant-filled");
                pt.input_multiple.merge_class(&cn.finish_cow()).into_owned()
            }
        };
        let (key_core, container_ref) = (core.clone(), core.container_ref);
        let blur_core = core.clone();
        let activedescendant = {
            let core = core.clone();
            move || core.focused_chip.get().map(|i| format!("{}_multiple_option_{i}", core.id))
        };
        view! {
            <ul
                node_ref={container_ref}
                class={container_class}
                tabindex="-1"
                role="listbox"
                aria-orientation="horizontal"
                aria-activedescendant={activedescendant}
                on:keydown={move |e| key_core.container_key_down(&e)}
                on:blur={move |_| blur_core.focused_chip.set(None)}
                {..pt.input_multiple.attrs()}
            >
                {chips}
                <li class={pt.input_chip.merge_class("p-autocomplete-input-chip").into_owned()} role="option" {..pt.input_chip.attrs()}>
                    <input
                        node_ref={input_ref}
                        class={class}
                        role="combobox"
                        aria-autocomplete="list"
                        autocomplete="off"
                        value={text_value}
                prop:value={text_value}
                        on:input={on_input}
                        on:keydown={on_key}
                        on:focus={on_focus_}
                        on:blur={on_blur_}
                        on:change={on_change}
                        on:paste={on_paste}
                        {..attrs}
                        {..input_pt_attrs}
                    />
                </li>
            </ul>
        }
    });
    let clear = {
        let (core, pt, filled, disabled, show_clear) = (core.clone(), pt.clone(), filled.clone(), disabled.clone(), show_clear.clone());
        move || {
            (filled() && !disabled.get() && show_clear.get() && !core.loading.get()).then(|| {
                let core = core.clone();
                let class = pt.clear_icon.merge_class("p-autocomplete-clear-icon").into_owned();
                view! { <TimesIcon class={class} on:click={move |_| core.clear()} {..pt.clear_icon.attrs()} /> }
            })
        }
    };
    let loader = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            core.loading.get().then(|| view! { <SpinnerIcon spin=true class={pt.loader.merge_class("p-autocomplete-loader").into_owned()} {..pt.loader.attrs()} /> })
        }
    };
    let dropdown_button = {
        let (core, pt, disabled, dropdown) = (core.clone(), pt.clone(), disabled.clone(), dropdown.clone());
        move || {
            dropdown.get().then(|| {
                let core = core.clone();
                let disabled = disabled.clone();
                view! {
                    <button
                        node_ref={core.dropdown_ref}
                        type="button"
                        class={pt.dropdown.merge_class("p-autocomplete-dropdown").into_owned()}
                        disabled={move || disabled.get()}
                        on:click={move |_| core.dropdown_click()}
                        {..pt.dropdown.attrs()}
                    >
                        <ChevronDownIcon />
                    </button>
                }
            })
        }
    };
    let panel = {
        let (ctx, panel_style_class, scroll_height, empty_message) =
            (ctx.clone(), panel_style_class.clone(), scroll_height.clone(), empty_message.clone());
        move || {
            ctx.core.open.get().then(|| panel_view(&ctx, &panel_style_class, &scroll_height, &empty_message, show_empty_message.clone()))
        }
    };
    let click_core = core.clone();
    let (root_ref, root_attrs) = (core.root_ref, pt.root.attrs());
    view! {
        <div node_ref={root_ref} id={root_id} class={root_class} style={root_style} on:click={move |e| click_core.container_click(&e)} {..root_attrs}>
            {single_input}
            {clear}
            {multi_container}
            {loader}
            {dropdown_button}
            {panel}
        </div>
    }
}

/// Панель со списком вариантов.
fn panel_view(
    ctx: &Ctx,
    extra: &OptProp<String>,
    scroll_height: &OptProp<String>,
    empty_message: &OptProp<String>,
    show_empty: Prop<bool>,
) -> impl IntoView + use<> {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let class = {
        let (pt, extra) = (pt.clone(), extra.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-autocomplete-overlay p-component-overlay p-component");
            extra.with(|c| cn.add_user(c.as_deref()));
            pt.overlay.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let container_style = {
        let h = scroll_height.clone();
        move || format!("max-height: {}", h.get().unwrap_or_else(|| "200px".to_owned()))
    };
    let list_id = format!("{}_list", core.id);
    let items = {
        let (ctx, empty_message) = (ctx.clone(), empty_message.clone());
        move || {
            let rows = ctx.core.rows.get();
            let set = set_size(&rows);
            let opts = ctx.core.options.get();
            let mut views: Vec<AnyView> = Vec::with_capacity(rows.len() + 1);
            for (i, row) in rows.iter().enumerate() {
                if row.group {
                    views.push(group_row(&ctx, i, row));
                } else {
                    let item = (ctx.item.is_some()).then(|| find_option(&opts, &row.value)).flatten();
                    views.push(option_row(&ctx, i, row, set, pos_inset(&rows, i), item));
                }
            }
            if rows.is_empty() && show_empty.get() {
                let text = empty_message
                    .get()
                    .filter(|m| !m.is_empty())
                    .unwrap_or_else(|| use_prime_config().locale.text("emptySearchMessage").to_owned());
                views.push(view! { <li class={ctx.pt.empty_message.merge_class("p-autocomplete-empty-message").into_owned()} {..ctx.pt.empty_message.attrs()}>{text}</li> }.into_any());
            }
            views.collect_view()
        }
    };
    let (overlay_ref, overlay_attrs) = (core.overlay_ref, pt.overlay.attrs());
    let (container_class, container_attrs, list_class, list_attrs) = (
        pt.list_container.merge_class("p-autocomplete-list-container").into_owned(),
        pt.list_container.attrs(),
        pt.list.merge_class("p-autocomplete-list").into_owned(),
        pt.list.attrs(),
    );
    view! {
        <div node_ref={overlay_ref} class={class} {..overlay_attrs}>
            <div class={container_class} style={container_style} tabindex="-1" {..container_attrs}>
                <ul id={list_id} class={list_class} role="listbox" {..list_attrs}>{items}</ul>
            </div>
        </div>
    }
}

fn find_option(options: &[AutoCompleteOption], value: &str) -> Option<AutoCompleteOption> {
    for o in options {
        if o.items.is_empty() && o.value_or_label() == value {
            return Some(o.clone());
        }
        if let Some(found) = find_option(&o.items, value) {
            return Some(found);
        }
    }
    None
}

fn group_row(ctx: &Ctx, index: usize, row: &Row) -> AnyView {
    let class = ctx.pt.option_group.merge_class("p-autocomplete-option-group").into_owned();
    let inner = match ctx.group_view {
        Some(cb) => untrack(|| cb.run(row.label.clone())),
        None => row.label.clone().into_any(),
    };
    view! { <li id={format!("{}_{index}", ctx.core.id)} class={class} role="presentation" {..ctx.pt.option_group.attrs()}><span>{inner}</span></li> }.into_any()
}

fn option_row(ctx: &Ctx, index: usize, row: &Row, set: usize, pos: usize, item: Option<AutoCompleteOption>) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let row = row.clone();
    let selected = {
        let (core, row) = (core.clone(), row.clone());
        move || {
            let (value, values) = (core.value.get(), core.values.get());
            is_selected(&row, &value, &values, core.multiple, core.unique.get())
        }
    };
    let focused = {
        let core = core.clone();
        move || core.focus_index.get() == Some(index)
    };
    let class = {
        let (pt, selected, focused, disabled) = (pt.clone(), selected.clone(), focused.clone(), row.disabled);
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-autocomplete-option")
                .add_if(selected(), "p-autocomplete-option-selected")
                .add_if(focused(), "p-focus")
                .add_if(disabled, "p-disabled");
            pt.option.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let content = match (ctx.item, item) {
        (Some(cb), Some(opt)) => untrack(|| cb.run(opt)),
        _ => view! { <span>{row.label.clone()}</span> }.into_any(),
    };
    let (click_core, enter_core) = (core.clone(), core.clone());
    let (sel_attr, focus_attr) = (selected.clone(), focused.clone());
    let disabled = row.disabled;
    view! {
        <li
            id={format!("{}_{index}", core.id)}
            class={class}
            role="option"
            aria-label={row.label.clone()}
            aria-selected={move || selected().to_string()}
            data-p-selected={move || sel_attr().to_string()}
            aria-disabled={disabled.then_some("true")}
            data-p-focused={move || focus_attr().to_string()}
            aria-setsize={set.to_string()}
            aria-posinset={pos.to_string()}
            on:click={move |_| click_core.option_select(index, true)}
            on:mouseenter={move |_| {
                if enter_core.focus_on_hover.get_untracked() {
                    enter_core.change_focus(index);
                }
            }}
            {..pt.option.attrs()}
        >
            {content}
        </li>
    }
    .into_any()
}
