//! DatePicker — порт `p-datepicker` из PrimeNG 21.1.10 (этап 1).
//!
//! Выбор даты, нескольких дат и диапазона в календаре: поле ввода с панелью-оверлеем либо встроенный календарь,
//! виды «дни», «месяцы», «годы», ограничения, недоступные даты, форматы и кнопки «Сегодня»/«Очистить». Календарная
//! логика — чистая [`calendar`]. Время и клавиатурная навигация по ячейкам — этап 2.
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_datepicker::{DatePicker, calendar::CalDate};
//!
//! let value = RwSignal::new(vec![CalDate::new(2026, 10, 5)]);
//! let html = view! { <DatePicker value=value /> }.to_html();
//! assert!(html.contains("p-datepicker-input") && html.contains("11/05/2026"));
//! ```

#![recursion_limit = "512"]

pub mod calendar;

use calendar::{
    CalDate, DayCell, Limits, Month, Names, RangeRole, SelectionMode, create_months, decade_years, format_date, is_selected,
    month_disabled, parse_date, range_role, select_date, selection_complete, year_disabled,
};
use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity, ButtonVariant};
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{CalendarIcon, ChevronLeftIcon, ChevronRightIcon, TimesIcon};
use prime_inputtext::{InputTextState, write_classes as input_classes};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::set_timeout_with_handle;
#[cfg(target_arch = "wasm32")]
use prime_core::ZGroup;
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Текущий вид календаря.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DatePickerView {
    /// Дни (по умолчанию).
    #[default]
    Date,
    /// Месяцы.
    Month,
    /// Годы.
    Year,
}

/// Где показывать значок календаря.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IconDisplay {
    /// Кнопка справа от поля (по умолчанию).
    #[default]
    Button,
    /// Значок внутри поля.
    Input,
}

/// Месяц и год (месяц 1–12), как в событиях PrimeNG.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonthYear {
    /// Месяц 1–12.
    pub month: u32,
    /// Год.
    pub year: i32,
}

/// Pass-through частей [`DatePicker`].
#[derive(Clone, Debug, Default)]
pub struct DatePickerPt {
    /// Корень.
    pub root: Pt,
    /// Поле ввода.
    pub input: Pt,
    /// Панель.
    pub panel: Pt,
    /// Контейнер календарей.
    pub calendar_container: Pt,
    /// Календарь одного месяца.
    pub calendar: Pt,
    /// Шапка календаря.
    pub header: Pt,
    /// Таблица дней.
    pub table: Pt,
    /// Ячейка дня.
    pub day_cell: Pt,
    /// День.
    pub day: Pt,
    /// Панель кнопок.
    pub buttonbar: Pt,
}

/// Сегодняшняя дата.
pub fn today_now() -> CalDate {
    #[cfg(target_arch = "wasm32")]
    {
        let now = js_sys::Date::new_0();
        CalDate::new(now.get_full_year() as i32, now.get_month(), now.get_date())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        CalDate::from_days((secs / 86_400) as i64)
    }
}

/// Названия месяцев и дней: английские, с заменой через `Locale` (`monthNames`, `monthNamesShort`, `dayNames`,
/// `dayNamesShort`, `dayNamesMin` — списки через запятую).
pub fn names_from_locale() -> Names {
    let locale = use_prime_config().locale;
    let mut names = Names::default();
    let list = |key: &str| locale.get(key).map(|v| v.split(',').map(|s| s.trim().to_owned()).collect::<Vec<_>>());
    if let Some(v) = list("monthNames").filter(|v| v.len() == 12) {
        names.months = v;
    }
    if let Some(v) = list("monthNamesShort").filter(|v| v.len() == 12) {
        names.months_short = v;
    }
    if let Some(v) = list("dayNames").filter(|v| v.len() == 7) {
        names.days = v;
    }
    if let Some(v) = list("dayNamesShort").filter(|v| v.len() == 7) {
        names.days_short = v;
    }
    if let Some(v) = list("dayNamesMin").filter(|v| v.len() == 7) {
        names.days_min = v;
    }
    names
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    value: RwSignal<Vec<CalDate>>,
    text: RwSignal<String>,
    open: RwSignal<bool>,
    focus: RwSignal<bool>,
    view: RwSignal<DatePickerView>,
    month: RwSignal<u32>,
    year: RwSignal<i32>,
    names: Names,
    mode: Prop<SelectionMode>,
    format: Prop<String>,
    min_date: OptProp<CalDate>,
    max_date: OptProp<CalDate>,
    disabled_dates: Prop<Vec<CalDate>>,
    disabled_days: Prop<Vec<u32>>,
    select_other_months: Prop<bool>,
    show_other_months: Prop<bool>,
    first_day_of_week: Prop<u32>,
    start_week_from_first_day_of_year: Prop<bool>,
    show_week: Prop<bool>,
    number_of_months: Prop<usize>,
    initial_view: Prop<DatePickerView>,
    inline: Prop<bool>,
    disabled: Prop<bool>,
    show_on_focus: Prop<bool>,
    hide_on_date_time_select: Prop<bool>,
    max_date_count: OptProp<usize>,
    default_date: OptProp<CalDate>,
    today: OptProp<CalDate>,
    multiple_separator: Prop<String>,
    range_separator: Prop<String>,
    auto_z_index: Prop<bool>,
    root: NodeRef<leptos::html::Div>,
    panel: NodeRef<leptos::html::Div>,
    /// Значение, с которым текст поля уже согласован: внешнее изменение отличается от него.
    synced: StoredValue<Vec<CalDate>>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    on_select: Option<Callback<CalDate>>,
    on_clear: Option<Callback<()>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_month_change: Option<Callback<MonthYear>>,
    on_year_change: Option<Callback<MonthYear>>,
    on_today_click: Option<Callback<CalDate>>,
    on_clear_click: Option<Callback<()>>,
}

impl Core {
    fn today(&self) -> CalDate {
        self.today.get_untracked().unwrap_or_else(today_now)
    }

    fn limits(&self) -> Limits {
        Limits {
            min: self.min_date.get(),
            max: self.max_date.get(),
            disabled_dates: self.disabled_dates.get(),
            disabled_days: self.disabled_days.get(),
            select_other_months: self.select_other_months.get(),
        }
    }

    fn limits_untracked(&self) -> Limits {
        Limits {
            min: self.min_date.get_untracked(),
            max: self.max_date.get_untracked(),
            disabled_dates: self.disabled_dates.get_untracked(),
            disabled_days: self.disabled_days.get_untracked(),
            select_other_months: self.select_other_months.get_untracked(),
        }
    }

    fn months(&self) -> Vec<Month> {
        let today = self.today.get().unwrap_or_else(today_now);
        create_months(
            self.month.get(),
            self.year.get(),
            self.number_of_months.get().max(1),
            today,
            self.first_day_of_week.get(),
            &self.limits(),
            self.show_week.get(),
            self.start_week_from_first_day_of_year.get(),
        )
    }

    fn format_one(&self, date: CalDate) -> String {
        format_date(date, &self.format.get_untracked(), &self.names)
    }

    /// Текст поля по значению (`updateInputfield`).
    fn format_value(&self, value: &[CalDate]) -> String {
        match self.mode.get_untracked() {
            SelectionMode::Single => value.first().map(|d| self.format_one(*d)).unwrap_or_default(),
            SelectionMode::Multiple => {
                value.iter().map(|d| self.format_one(*d)).collect::<Vec<_>>().join(&format!("{} ", self.multiple_separator.get_untracked()))
            }
            SelectionMode::Range => {
                value.iter().map(|d| self.format_one(*d)).collect::<Vec<_>>().join(&format!(" {} ", self.range_separator.get_untracked()))
            }
        }
    }

    fn sync_text(&self) {
        let value = self.value.get_untracked();
        self.text.set(self.format_value(&value));
        self.synced.set_value(value);
    }

    /// Месяц и год, с которых открывается календарь (`value`, `default_date` или сегодня).
    fn show_position(&self) {
        let base =
            self.value.with_untracked(|v| v.first().copied()).or_else(|| self.default_date.get_untracked()).unwrap_or_else(|| self.today());
        self.month.set(base.month);
        self.year.set(base.year);
        self.view.set(self.initial_view.get_untracked());
    }

    fn show(&self) {
        if self.disabled.get_untracked() || self.open.get_untracked() {
            return;
        }
        self.show_position();
        self.open.set(true);
    }

    fn hide(&self) {
        self.open.try_set(false);
    }

    fn emit_month(&self) {
        if let Some(cb) = self.on_month_change {
            cb.run(MonthYear { month: self.month.get_untracked() + 1, year: self.year.get_untracked() });
        }
    }

    fn emit_year(&self) {
        if let Some(cb) = self.on_year_change {
            cb.run(MonthYear { month: self.month.get_untracked() + 1, year: self.year.get_untracked() });
        }
    }

    fn navigate(&self, forward: bool) {
        if self.disabled.get_untracked() {
            return;
        }
        match self.view.get_untracked() {
            DatePickerView::Month => self.year.update(|y| *y += if forward { 1 } else { -1 }),
            DatePickerView::Year => self.year.update(|y| *y += if forward { 10 } else { -10 }),
            DatePickerView::Date => {
                let (m, y) = (self.month.get_untracked(), self.year.get_untracked());
                let (m, y) = match (forward, m) {
                    (true, 11) => (0, y + 1),
                    (true, m) => (m + 1, y),
                    (false, 0) => (11, y - 1),
                    (false, m) => (m - 1, y),
                };
                self.month.set(m);
                self.year.set(y);
                self.emit_month();
            }
        }
    }

    /// Клик по дню (`onDateSelect`).
    fn select_day(self: &Arc<Self>, date: CalDate, selectable: bool) {
        if self.disabled.get_untracked() || !selectable {
            return;
        }
        let mode = self.mode.get_untracked();
        let next = select_date(mode, &self.value.get_untracked(), date, self.max_date_count.get_untracked());
        self.value.set(next.clone());
        if (mode != SelectionMode::Multiple || next.contains(&date))
            && let Some(cb) = self.on_select
        {
            cb.run(date);
        }
        self.sync_text();
        if self.hide_on_date_time_select.get_untracked() && !self.inline.get_untracked() && selection_complete(mode, &next) {
            #[cfg(target_arch = "wasm32")]
            {
                let this = self.clone();
                let _ = set_timeout_with_handle(
                    move || {
                        if !this.open.is_disposed() {
                            this.hide();
                        }
                    },
                    Duration::from_millis(150),
                );
            }
        }
    }

    fn select_month(self: &Arc<Self>, index: u32) {
        if self.initial_view.get_untracked() == DatePickerView::Month {
            self.select_day(CalDate::new(self.year.get_untracked(), index, 1), true);
        } else {
            self.month.set(index);
            self.view.set(DatePickerView::Date);
            self.emit_month();
        }
    }

    fn select_year(self: &Arc<Self>, year: i32) {
        if self.initial_view.get_untracked() == DatePickerView::Year {
            self.select_day(CalDate::new(year, 0, 1), true);
        } else {
            self.year.set(year);
            self.view.set(DatePickerView::Month);
            self.emit_year();
        }
    }

    fn today_click(self: &Arc<Self>) {
        let today = self.today();
        self.month.set(today.month);
        self.year.set(today.year);
        self.select_day(today, self.limits_untracked().selectable(today, false));
        if let Some(cb) = self.on_today_click {
            cb.run(today);
        }
    }

    fn clear_click(&self) {
        self.value.set(Vec::new());
        self.sync_text();
        self.hide();
        if let Some(cb) = self.on_clear_click {
            cb.run(());
        }
    }

    fn clear_input(&self) {
        self.value.set(Vec::new());
        self.sync_text();
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }

    /// Ввод текста: корректные доступные даты становятся значением (`onUserInput`).
    fn user_input(&self, text: &str) {
        let trimmed = text.trim();
        let mode = self.mode.get_untracked();
        let parsed: Option<Vec<CalDate>> = if trimmed.is_empty() {
            Some(Vec::new())
        } else {
            let format = self.format.get_untracked();
            let today = self.today();
            let one = |s: &str| parse_date(s.trim(), &format, &self.names, today);
            match mode {
                SelectionMode::Single => one(trimmed).map(|d| vec![d]),
                SelectionMode::Multiple => trimmed.split(self.multiple_separator.get_untracked().as_str()).map(one).collect(),
                SelectionMode::Range => {
                    let sep = format!(" {} ", self.range_separator.get_untracked());
                    let parts: Vec<&str> = trimmed.split(sep.as_str()).collect();
                    if parts.len() > 2 { None } else { parts.into_iter().map(one).collect::<Option<Vec<_>>>() }
                }
            }
        };
        let Some(dates) = parsed else { return };
        let limits = self.limits_untracked();
        if !dates.iter().all(|d| limits.selectable(*d, false)) {
            return;
        }
        if let [start, end] = dates.as_slice()
            && end < start
        {
            return;
        }
        self.synced.set_value(dates.clone());
        if let Some(first) = dates.first() {
            self.month.set(first.month);
            self.year.set(first.year);
        }
        self.value.set(dates);
    }

    /// Снимает слушатели окна и освобождает z-index.
    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    /// Панель только что показана: z-index, выравнивание по корню, подписки, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(root)) = (self.panel.get_untracked(), self.root.get_untracked()) else { return };
        let (panel, root): (web_sys::HtmlElement, web_sys::HtmlElement) = (panel.unchecked_into(), root.unchecked_into());
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        align_overlay(&panel, &root, AlignMode::Relative, false);
        // Подписки — в следующем тике: клик, открывший панель, ещё всплывает к окну и закрыл бы её сам.
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.open.is_disposed() || !this.open.get_untracked() {
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

    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            let node = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let inside = this.root.get_untracked().is_some_and(|r| r.contains(node.as_ref()));
            // Клик по ячейке, которая уже пересоздана перерисовкой, не считается внешним.
            let detached = node.as_ref().is_some_and(|n| !n.is_connected());
            if !inside && !detached {
                this.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || resize.remove()));
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

/// Поле ввода даты с календарём.
///
/// Каждое свойство принимает значение или сигнал; `value` — двусторонний сигнал. Панель выводится внутри корня
/// (не в `body`), поэтому у предков не должно быть `overflow: hidden`.
#[component]
pub fn DatePicker(
    /// Двусторонний сигнал значения: пусто, одна дата, несколько или `[начало, конец]`.
    #[prop(optional)]
    value: Option<RwSignal<Vec<CalDate>>>,
    /// Режим выбора.
    #[prop(optional, into)]
    selection_mode: Prop<SelectionMode>,
    /// Формат даты (`d dd o oo D DD m mm M MM y yy`).
    #[prop(optional, default = Prop::Static("mm/dd/yy".to_owned()), into)]
    date_format: Prop<String>,
    /// Минимальная дата.
    #[prop(optional, into)]
    min_date: OptProp<CalDate>,
    /// Максимальная дата.
    #[prop(optional, into)]
    max_date: OptProp<CalDate>,
    /// Недоступные даты.
    #[prop(optional, into)]
    disabled_dates: Prop<Vec<CalDate>>,
    /// Недоступные дни недели (0 — воскресенье).
    #[prop(optional, into)]
    disabled_days: Prop<Vec<u32>>,
    /// Первый день недели (0 — воскресенье).
    #[prop(optional, into)]
    first_day_of_week: Prop<u32>,
    /// Показывать дни соседних месяцев.
    #[prop(optional, default = Prop::Static(true), into)]
    show_other_months: Prop<bool>,
    /// Разрешить выбор дней соседних месяцев.
    #[prop(optional, into)]
    select_other_months: Prop<bool>,
    /// Показывать номера недель.
    #[prop(optional, into)]
    show_week: Prop<bool>,
    /// Нумеровать недели от первого дня года.
    #[prop(optional, into)]
    start_week_from_first_day_of_year: Prop<bool>,
    /// Число одновременно показанных месяцев.
    #[prop(optional, default = Prop::Static(1), into)]
    number_of_months: Prop<usize>,
    /// Начальный вид календаря: `Month` и `Year` — выбор месяца и года целиком.
    #[prop(optional, into)]
    view: Prop<DatePickerView>,
    /// Встроенный календарь без поля.
    #[prop(optional, into)]
    inline: Prop<bool>,
    /// Показывать значок календаря.
    #[prop(optional, into)]
    show_icon: Prop<bool>,
    /// Где показывать значок.
    #[prop(optional, into)]
    icon_display: Prop<IconDisplay>,
    /// Показывать кнопки «Сегодня» и «Очистить».
    #[prop(optional, into)]
    show_button_bar: Prop<bool>,
    /// Показывать значок очистки в поле.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Открывать панель при фокусе поля.
    #[prop(optional, default = Prop::Static(true), into)]
    show_on_focus: Prop<bool>,
    /// Закрывать панель после завершения выбора.
    #[prop(optional, default = Prop::Static(true), into)]
    hide_on_date_time_select: Prop<bool>,
    /// Предел числа дат в режиме `Multiple`.
    #[prop(optional, into)]
    max_date_count: OptProp<usize>,
    /// Дата, с которой открывается календарь без значения.
    #[prop(optional, into)]
    default_date: OptProp<CalDate>,
    /// «Сегодня» (по умолчанию — системная дата); нужно для детерминированных тестов.
    #[prop(optional, into)]
    today: OptProp<CalDate>,
    /// Разделитель дат в режиме `Multiple`.
    #[prop(optional, default = Prop::Static(",".to_owned()), into)]
    multiple_separator: Prop<String>,
    /// Разделитель начала и конца диапазона.
    #[prop(optional, default = Prop::Static("-".to_owned()), into)]
    range_separator: Prop<String>,
    /// Подсказка поля.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Поле только для чтения (дата выбирается в календаре).
    #[prop(optional, into)]
    readonly_input: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Вариант поля.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Размер поля.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину контейнера.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Атрибут `id` поля.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Атрибут `tabindex` поля.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` поля.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Автоматически назначать `z-index` панели.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается при выборе даты.
    #[prop(optional, into)]
    on_select: Option<Callback<CalDate>>,
    /// Вызывается при очистке значком в поле.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Вызывается после показа панели.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Вызывается после скрытия панели.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Вызывается при смене месяца.
    #[prop(optional, into)]
    on_month_change: Option<Callback<MonthYear>>,
    /// Вызывается при смене года.
    #[prop(optional, into)]
    on_year_change: Option<Callback<MonthYear>>,
    /// Вызывается по кнопке «Сегодня».
    #[prop(optional, into)]
    on_today_click: Option<Callback<CalDate>>,
    /// Вызывается по кнопке «Очистить».
    #[prop(optional, into)]
    on_clear_click: Option<Callback<()>>,
    /// Вызывается при получении фокуса полем.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса полем.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<DatePickerPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let value = value.unwrap_or_default();
    let initial_view = view.get_untracked();
    let core = Arc::new(Core {
        value,
        text: RwSignal::new(String::new()),
        open: RwSignal::new(false),
        focus: RwSignal::new(false),
        view: RwSignal::new(initial_view),
        month: RwSignal::new(0),
        year: RwSignal::new(0),
        names: names_from_locale(),
        mode: selection_mode,
        format: date_format,
        min_date,
        max_date,
        disabled_dates,
        disabled_days,
        select_other_months,
        show_other_months,
        first_day_of_week,
        start_week_from_first_day_of_year,
        show_week,
        number_of_months,
        initial_view: view,
        inline: inline.clone(),
        disabled: disabled.clone(),
        show_on_focus,
        hide_on_date_time_select,
        max_date_count,
        default_date,
        today,
        multiple_separator,
        range_separator,
        auto_z_index,
        root: NodeRef::new(),
        panel: NodeRef::new(),
        synced: StoredValue::new(Vec::new()),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        on_select,
        on_clear,
        on_show,
        on_hide,
        on_month_change,
        on_year_change,
        on_today_click,
        on_clear_click,
    });
    core.show_position();
    core.sync_text();
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }

    // Внешнее изменение значения обновляет текст; открытие и закрытие панели — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        Effect::new(move |_| {
            let current = c.value.get();
            if current != c.synced.get_value() {
                c.synced.set_value(current.clone());
                c.text.set(c.format_value(&current));
            }
        });
        let c = core.clone();
        Effect::new(move |_| {
            let open = c.open.get();
            let mounted = c.panel.get().is_some();
            let was = c.was_open.get_value();
            if open && mounted && !was {
                c.was_open.set_value(true);
                c.opened();
            } else if !open && was {
                c.was_open.set_value(false);
                c.closed();
            }
        });
    }

    let root_class = {
        let (core, pt) = (core.clone(), pt.clone());
        let (invalid, fluid, variant) = (invalid.clone(), fluid.clone(), variant.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-datepicker p-component p-inputwrapper");
            let fluid = prime_core::fluid::fluid_prop(fluid.clone());
            cn.add_if(invalid.get(), "p-invalid")
                .add_if(fluid.get(), "p-datepicker-fluid")
                .add_if(core.value.with(|v| !v.is_empty()), "p-inputwrapper-filled")
                .add_if(variant.get().or(use_prime_config().input_variant) == Some(Variant::Filled), "p-variant-filled");
            let focused = core.focus.get() || core.open.get();
            cn.add_if(focused, "p-inputwrapper-focus").add_if(focused, "p-focus");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (pt, core) = (pt.clone(), core.clone());
        move || {
            // Панель выравнивается относительно ближайшего предка с позиционированием: им должен быть корень.
            let base = if core.inline.get() { "" } else { "position: relative" };
            let user = style.with(|st| st.clone().unwrap_or_default());
            let merged = pt.root.merge_style_parts(&[base, &user]);
            (!merged.is_empty()).then_some(merged)
        }
    };

    let input_part = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            (!core.inline.get()).then(|| {
                field_view(
                    &core,
                    &pt,
                    &FieldProps {
                        placeholder: placeholder.clone(),
                        readonly_input: readonly_input.clone(),
                        invalid: invalid.clone(),
                        variant: variant.clone(),
                        size: size.clone(),
                        fluid: fluid.clone(),
                        input_id: input_id.clone(),
                        tabindex: tabindex.clone(),
                        aria_label: aria_label.clone(),
                        show_icon: show_icon.clone(),
                        icon_display: icon_display.clone(),
                        show_clear: show_clear.clone(),
                        on_focus,
                        on_blur,
                    },
                )
            })
        }
    };
    let panel = {
        let (core, pt) = (core.clone(), pt.clone());
        let show_button_bar = show_button_bar.clone();
        move || (core.inline.get() || core.open.get()).then(|| panel_view(&core, &pt, show_button_bar.get()))
    };
    let root = core.root;
    view! {
        <div node_ref=root class={root_class} style={root_style} data-pc-name="datepicker" data-pc-section="root" {..pt.root.attrs()}>
            {input_part}
            {panel}
        </div>
    }
}

struct FieldProps {
    placeholder: OptProp<String>,
    readonly_input: Prop<bool>,
    invalid: Prop<bool>,
    variant: OptProp<Variant>,
    size: OptProp<Size>,
    fluid: OptProp<bool>,
    input_id: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    show_icon: Prop<bool>,
    icon_display: Prop<IconDisplay>,
    show_clear: Prop<bool>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
}

fn text_value(key: &str) -> String {
    use_prime_config().locale.text(key).to_owned()
}

/// Поле ввода, значок очистки и значок календаря.
fn field_view(core: &Arc<Core>, pt: &Arc<DatePickerPt>, p: &FieldProps) -> impl IntoView + use<> {
    let class = {
        let (core, pt) = (core.clone(), pt.clone());
        let (invalid, variant, size, fluid) = (p.invalid.clone(), p.variant.clone(), p.size.clone(), p.fluid.clone());
        move || {
            let variant = variant.get().or(use_prime_config().input_variant);
            let fluid = prime_core::fluid::fluid_prop(fluid.clone());
            let mut cn = ClassNames::new();
            input_classes(
                &mut cn,
                &InputTextState {
                    filled: core.value.with(|v| !v.is_empty()),
                    size: size.get(),
                    invalid: invalid.get(),
                    variant,
                    fluid: fluid.get(),
                },
            );
            cn.add("p-datepicker-input");
            cn.add_user(pt.input.extra_class());
            cn.finish_cow()
        }
    };
    let attrs: Vec<AnyAttribute> = {
        let attr = |name: &'static str, f: Box<dyn Fn() -> Option<String> + Send + Sync>| custom_attribute(name, f).into_any_attr();
        let (id, tab, label, ph) = (p.input_id.clone(), p.tabindex.clone(), p.aria_label.clone(), p.placeholder.clone());
        let (dis, ro, c_open, c_ctrl) = (core.disabled.clone(), p.readonly_input.clone(), core.clone(), core.clone());
        let mut v = vec![
            attr("id", Box::new(move || id.get())),
            attr("tabindex", Box::new(move || tab.get().map(|t| t.to_string()))),
            attr("aria-label", Box::new(move || label.get())),
            attr("placeholder", Box::new(move || ph.get())),
            attr("disabled", Box::new(move || dis.get().then(String::new))),
            attr("readonly", Box::new(move || ro.get().then(String::new))),
            attr("aria-expanded", Box::new(move || Some(c_open.open.get().to_string()))),
            attr("aria-controls", Box::new(move || c_ctrl.open.get().then(|| "datepicker-panel".to_owned()))),
        ];
        v.extend(pt.input.attrs());
        v
    };
    let (c_focus, c_click, c_blur, c_key, c_input) = (core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
    let (on_focus, on_blur) = (p.on_focus, p.on_blur);
    let text = core.text;
    let input = view! {
        <input
            type="text"
            role="combobox"
            aria-autocomplete="none"
            aria-haspopup="dialog"
            autocomplete="off"
            class={class}
            value={move || text.get()}
            prop:value={move || text.get()}
            on:focus=move |e: web_sys::FocusEvent| {
                c_focus.focus.set(true);
                if c_focus.show_on_focus.get_untracked() {
                    c_focus.show();
                }
                if let Some(cb) = on_focus {
                    cb.run(e);
                }
            }
            on:click=move |_| {
                if c_click.show_on_focus.get_untracked() {
                    c_click.show();
                }
            }
            on:blur=move |e: web_sys::FocusEvent| {
                c_blur.focus.set(false);
                c_blur.sync_text();
                if let Some(cb) = on_blur {
                    cb.run(e);
                }
            }
            on:keydown=move |e: web_sys::KeyboardEvent| match e.key().as_str() {
                "ArrowDown" => c_key.show(),
                "Escape" | "Tab" => c_key.hide(),
                "Enter" if c_key.open.get_untracked() => {
                    c_key.hide();
                    e.prevent_default();
                }
                _ => {}
            }
            on:input=move |e: web_sys::Event| {
                let value = leptos::prelude::event_target_value(&e);
                text.set(value.clone());
                c_input.user_input(&value);
            }
            {..attrs}
        />
    };
    let clear = {
        let (core, show_clear) = (core.clone(), p.show_clear.clone());
        move || {
            let visible = show_clear.get() && !core.disabled.get() && core.text.with(|t| !t.is_empty());
            let click = core.clone();
            visible.then(|| view! { <TimesIcon class="p-datepicker-clear-icon" on:click=move |_| click.clear_input() /> })
        }
    };
    let icon = {
        let (core, show_icon, display) = (core.clone(), p.show_icon.clone(), p.icon_display.clone());
        move || {
            show_icon.get().then(|| {
                let click = core.clone();
                let toggle = move |_| {
                    if click.disabled.get_untracked() {
                        return;
                    }
                    if click.open.get_untracked() { click.hide() } else { click.show() }
                };
                match display.get() {
                    IconDisplay::Button => {
                        let dis = core.clone();
                        let expanded = core.clone();
                        view! {
                            <button
                                type="button"
                                class="p-datepicker-dropdown"
                                aria-label={text_value("chooseDate")}
                                aria-haspopup="dialog"
                                aria-expanded={move || expanded.open.get().to_string()}
                                disabled={move || dis.disabled.get()}
                                on:click=toggle
                            >
                                <CalendarIcon />
                            </button>
                        }
                        .into_any()
                    }
                    IconDisplay::Input => view! {
                        <span class="p-datepicker-input-icon-container">
                            <CalendarIcon class="p-datepicker-input-icon" on:click=toggle />
                        </span>
                    }
                    .into_any(),
                }
            })
        }
    };
    view! {
        {input}
        {clear}
        {icon}
    }
}

/// Панель с календарями, видами месяцев и лет и панелью кнопок.
fn panel_view(core: &Arc<Core>, pt: &Arc<DatePickerPt>, button_bar: bool) -> impl IntoView + use<> {
    let panel = core.panel;
    let class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-datepicker-panel p-component")
                .add_if(core.inline.get(), "p-datepicker-panel-inline")
                .add_if(core.disabled.get(), "p-disabled");
            cn.add_user(pt.panel.extra_class());
            cn.finish_cow()
        }
    };
    let inline = core.inline.get_untracked();
    let calendars = {
        let (core, pt) = (core.clone(), pt.clone());
        move || calendars_view(&core, &pt)
    };
    let month_view = {
        let core = core.clone();
        move || {
            // год и ограничения читаются внутри `month_picker` без подписки: перестраиваем при их смене
            core.year.track();
            (core.view.get() == DatePickerView::Month).then(|| month_picker(&core))
        }
    };
    let year_view = {
        let core = core.clone();
        move || {
            core.year.track();
            (core.view.get() == DatePickerView::Year).then(|| year_picker(&core))
        }
    };
    let bar = button_bar.then(|| {
        let (today_core, clear_core) = (core.clone(), core.clone());
        let label = |key: &str| text_value(key);
        view! {
            <div class={pt.buttonbar.merge_class("p-datepicker-buttonbar")} {..pt.buttonbar.attrs()}>
                <Button
                    label=Some(label("today"))
                    size=Size::Small
                    severity=ButtonSeverity::Secondary
                    variant=ButtonVariant::Text
                    style_class="p-datepicker-today-button"
                    on_click=Callback::new(move |_| today_core.today_click())
                    pt=ButtonPt::default()
                />
                <Button
                    label=Some(label("clear"))
                    size=Size::Small
                    severity=ButtonSeverity::Secondary
                    variant=ButtonVariant::Text
                    style_class="p-datepicker-clear-button"
                    on_click=Callback::new(move |_| clear_core.clear_click())
                    pt=ButtonPt::default()
                />
            </div>
        }
    });
    view! {
        <div
            node_ref=panel
            id="datepicker-panel"
            class={class}
            aria-label={text_value("chooseDate")}
            style={(!inline).then_some("position: absolute; top: 0; left: 0")}
            role={(!inline).then_some("dialog")}
            aria-modal={(!inline).then_some("true")}
            {..pt.panel.attrs()}
        >
            <div class={pt.calendar_container.merge_class("p-datepicker-calendar-container")} {..pt.calendar_container.attrs()}>
                {calendars}
            </div>
            {month_view}
            {year_view}
            {bar}
        </div>
    }
}

/// Календари месяцев: шапка и таблица дней.
fn calendars_view(core: &Arc<Core>, pt: &Arc<DatePickerPt>) -> impl IntoView + use<> {
    let months = core.months();
    let count = months.len();
    let current_view = core.view.get();
    months
        .into_iter()
        .enumerate()
        .map(|(i, month)| {
            let header = header_view(core, pt, &month, i, count, current_view);
            let table = (current_view == DatePickerView::Date).then(|| table_view(core, pt, &month));
            view! {
                <div class={pt.calendar.merge_class("p-datepicker-calendar")} {..pt.calendar.attrs()}>
                    {header}
                    {table}
                </div>
            }
        })
        .collect_view()
}

fn header_view(
    core: &Arc<Core>,
    pt: &Arc<DatePickerPt>,
    month: &Month,
    index: usize,
    count: usize,
    current_view: DatePickerView,
) -> impl IntoView + use<> {
    let nav = |forward: bool| {
        let core = core.clone();
        let visible = if forward { index + 1 == count } else { index == 0 };
        let key = match (current_view, forward) {
            (DatePickerView::Month, false) => "prevYear",
            (DatePickerView::Month, true) => "nextYear",
            (DatePickerView::Year, false) => "prevDecade",
            (DatePickerView::Year, true) => "nextDecade",
            (DatePickerView::Date, false) => "prevMonth",
            (DatePickerView::Date, true) => "nextMonth",
        };
        let label = text_value(key);
        let icon = if forward { ViewFn::from(|| view! { <ChevronRightIcon /> }) } else { ViewFn::from(|| view! { <ChevronLeftIcon /> }) };
        view! {
            <Button
                rounded=true
                variant=ButtonVariant::Text
                severity=ButtonSeverity::Secondary
                button_type="button"
                icon=icon
                aria_label=Some(label)
                style_class=if forward { "p-datepicker-next-button" } else { "p-datepicker-prev-button" }
                style=if visible { "visibility: visible" } else { "visibility: hidden" }
                on_click=Callback::new(move |_| core.navigate(forward))
            />
        }
    };
    let switch_disabled = core.number_of_months.get_untracked() > 1 || core.disabled.get_untracked();
    let title = match current_view {
        DatePickerView::Date => {
            let (month_core, year_core) = (core.clone(), core.clone());
            view! {
                <button
                    type="button"
                    class="p-datepicker-select-month"
                    aria-label={text_value("chooseMonth")}
                    disabled={switch_disabled}
                    on:click=move |e: web_sys::MouseEvent| {
                        month_core.view.set(DatePickerView::Month);
                        e.prevent_default();
                    }
                >
                    {core.names.months[month.month as usize].clone()}
                </button>
                <button
                    type="button"
                    class="p-datepicker-select-year"
                    aria-label={text_value("chooseYear")}
                    disabled={switch_disabled}
                    on:click=move |e: web_sys::MouseEvent| {
                        year_core.view.set(DatePickerView::Year);
                        e.prevent_default();
                    }
                >
                    {month.year}
                </button>
            }
            .into_any()
        }
        DatePickerView::Month => {
            let year_core = core.clone();
            let year = core.year.get();
            view! {
                <button
                    type="button"
                    class="p-datepicker-select-year"
                    aria-label={text_value("chooseYear")}
                    disabled={switch_disabled}
                    on:click=move |e: web_sys::MouseEvent| {
                        year_core.view.set(DatePickerView::Year);
                        e.prevent_default();
                    }
                >
                    {year}
                </button>
            }
            .into_any()
        }
        DatePickerView::Year => {
            let years = decade_years(core.year.get());
            view! { <span class="p-datepicker-decade">{format!("{} - {}", years[0], years[years.len() - 1])}</span> }.into_any()
        }
    };
    view! {
        <div class={pt.header.merge_class("p-datepicker-header")} {..pt.header.attrs()}>
            {nav(false)}
            <div class="p-datepicker-title">{title}</div>
            {nav(true)}
        </div>
    }
}

/// Классы дня (`day` и `dayCell` из `datepickerstyle.ts`).
fn day_class(mode: SelectionMode, value: &[CalDate], cell: &DayCell, disabled: bool) -> String {
    let mut cn = String::from("p-datepicker-day");
    let date = cell.date();
    if mode == SelectionMode::Range {
        if cell.selectable {
            match range_role(value, date) {
                Some(RangeRole::Edge) => cn.push_str(" p-datepicker-day-selected"),
                Some(RangeRole::Middle) => cn.push_str(" p-datepicker-day-selected-range"),
                None => {}
            }
        }
    } else if is_selected(mode, value, date) && cell.selectable {
        cn.push_str(" p-datepicker-day-selected");
    }
    if disabled || !cell.selectable {
        cn.push_str(" p-disabled");
    }
    cn
}

fn table_view(core: &Arc<Core>, pt: &Arc<DatePickerPt>, month: &Month) -> impl IntoView + use<> {
    let first = core.first_day_of_week.get() as usize;
    let weekdays: Vec<String> = (0..7).map(|i| core.names.days_min[(first + i) % 7].clone()).collect();
    let show_week = core.show_week.get();
    let show_other = core.show_other_months.get();
    let week_header =
        show_week.then(|| view! { <th class="p-datepicker-weekheader p-disabled"><span>{text_value("weekHeader")}</span></th> });
    let head = weekdays
        .into_iter()
        .map(|d| view! { <th class="p-datepicker-weekday-cell" scope="col"><span class="p-datepicker-weekday">{d}</span></th> })
        .collect_view();
    let rows = month
        .weeks
        .iter()
        .enumerate()
        .map(|(j, week)| {
            let number = show_week.then(|| {
                let n = month.week_numbers.get(j).copied().unwrap_or_default();
                view! { <td class="p-datepicker-weeknumber"><span class="p-datepicker-weeklabel-container p-disabled">{n}</span></td> }
            });
            let cells = week.iter().map(|cell| day_cell(core, pt, *cell, show_other)).collect_view();
            view! { <tr>{number}{cells}</tr> }
        })
        .collect_view();
    view! {
        <table class="p-datepicker-day-view" role="grid">
            <thead><tr>{week_header}{head}</tr></thead>
            <tbody>{rows}</tbody>
        </table>
    }
}

fn day_cell(core: &Arc<Core>, pt: &Arc<DatePickerPt>, cell: DayCell, show_other: bool) -> impl IntoView + use<> {
    let mut cell_class = String::from("p-datepicker-day-cell");
    if cell.other_month {
        cell_class.push_str(" p-datepicker-other-month");
    }
    if cell.today {
        cell_class.push_str(" p-datepicker-today");
    }
    let inner = (!cell.other_month || show_other).then(|| {
        let (class_core, click_core, live_core) = (core.clone(), core.clone(), core.clone());
        let date = cell.date();
        let class = {
            let pt = pt.clone();
            move || {
                let mode = class_core.mode.get();
                let disabled = class_core.disabled.get();
                let base = class_core.value.with(|v| day_class(mode, v, &cell, disabled));
                pt.day.merge_class(&base).into_owned()
            }
        };
        let live = move || {
            let mode = live_core.mode.get();
            live_core
                .value
                .with(|v| is_selected(mode, v, date))
                .then(|| view! { <div class="p-hidden-accessible" aria-live="polite">{cell.day}</div> })
        };
        view! {
            <span
                class={class}
                draggable="false"
                data-date={format!("{}-{}-{}", date.year, date.month, date.day)}
                on:click=move |_| click_core.select_day(date, cell.selectable)
                {..pt.day.attrs()}
            >
                {cell.day}
            </span>
            {live}
        }
    });
    view! { <td aria-label={cell.day.to_string()} class={pt.day_cell.merge_class(&cell_class)} {..pt.day_cell.attrs()}>{inner}</td> }
}

fn month_picker(core: &Arc<Core>) -> impl IntoView + use<> {
    let year = core.year.get_untracked();
    let limits = core.limits_untracked();
    let cells = core
        .names
        .months_short
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let index = index as u32;
            let disabled = month_disabled(index, year, &limits);
            let (class_core, click_core) = (core.clone(), core.clone());
            let class = move || {
                let value = class_core.value.get();
                let selected = match class_core.mode.get() {
                    SelectionMode::Multiple => value.iter().any(|d| d.month == index && d.year == year),
                    SelectionMode::Range => match value.as_slice() {
                        [s, e] => (s.year, s.month) <= (year, index) && (year, index) <= (e.year, e.month),
                        [s] => (s.year, s.month) == (year, index),
                        _ => false,
                    },
                    SelectionMode::Single => value.first().is_some_and(|d| d.month == index && d.year == year),
                };
                let mut cn = String::from("p-datepicker-month");
                if selected {
                    cn.push_str(" p-datepicker-month-selected");
                }
                if disabled {
                    cn.push_str(" p-disabled");
                }
                cn
            };
            view! { <span class={class} on:click=move |_| if !disabled { click_core.select_month(index) }>{name.clone()}</span> }
        })
        .collect_view();
    view! { <div class="p-datepicker-month-view">{cells}</div> }
}

fn year_picker(core: &Arc<Core>) -> impl IntoView + use<> {
    let limits = core.limits_untracked();
    let cells = decade_years(core.year.get_untracked())
        .into_iter()
        .map(|year| {
            let disabled = year_disabled(year, &limits);
            let (class_core, click_core) = (core.clone(), core.clone());
            let class = move || {
                let first = class_core.value.with(|v| v.first().copied());
                let selected = class_core.mode.get() != SelectionMode::Multiple && first.is_some_and(|d| d.year == year);
                let mut cn = String::from("p-datepicker-year");
                if selected {
                    cn.push_str(" p-datepicker-year-selected");
                }
                if disabled {
                    cn.push_str(" p-disabled");
                }
                cn
            };
            view! { <span class={class} on:click=move |_| if !disabled { click_core.select_year(year) }>{year}</span> }
        })
        .collect_view();
    view! { <div class="p-datepicker-year-view">{cells}</div> }
}
