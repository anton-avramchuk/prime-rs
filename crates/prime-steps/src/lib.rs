//! Steps — порт `p-steps` из PrimeNG 21.1.10.
//!
//! Нумерованные шаги мастера в ряд: `nav.p-steps` > `ul.p-steps-list` > `li.p-steps-item` со ссылкой, номером и подписью. По
//! умолчанию компонент только отображает текущий шаг (`readonly`); в интерактивном режиме клик и клавиши ←/→/`Home`/`End`/`Enter`/
//! `Space` переключают шаги. Текущий шаг ведёт владелец через `active_index` (значение или сигнал).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_steps::{Steps, StepsItem};
//!
//! let model = vec![StepsItem::new().label("Personal"), StepsItem::new().label("Seat"), StepsItem::new().label("Payment")];
//! let html = Owner::new().with(|| view! { <Steps model=model active_index=1usize /> }.to_html());
//! assert!(html.contains("p-steps-item-active") && html.contains("aria-current=\"step\"") && html.contains(">3<"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{
    Click, aria_disabled, click_outcome, is_activation_key, is_item_disabled, item_classes, key_target, link_tabindex, root_classes,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Событие команды пункта.
#[derive(Clone)]
pub struct StepsCommand {
    /// Исходное событие (клик или клавиша).
    pub event: web_sys::Event,
    /// Пункт.
    pub item: StepsItem,
    /// Индекс пункта.
    pub index: usize,
}

/// Пункт Steps (вместо `MenuItem` PrimeNG, без полей маршрутизатора).
#[derive(Clone, Default)]
pub struct StepsItem {
    /// Атрибут `id` пункта.
    pub id: Option<String>,
    /// Подпись.
    pub label: Option<String>,
    /// Адрес ссылки; без него переход по ссылке отменяется.
    pub url: Option<String>,
    /// `target` ссылки.
    pub target: Option<String>,
    /// Отключён.
    pub disabled: bool,
    /// Свой `tabindex` ссылки.
    pub tabindex: Option<i32>,
    /// Встроенный стиль `li`.
    pub style: Option<String>,
    /// Команда при выборе пункта.
    pub command: Option<Callback<StepsCommand>>,
}

impl StepsItem {
    /// Пустой пункт.
    pub fn new() -> Self {
        Self::default()
    }

    /// Атрибут `id`.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Подпись.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Адрес ссылки.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// `target` ссылки.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Отключён ли пункт.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `tabindex` ссылки.
    pub fn tabindex(mut self, tabindex: i32) -> Self {
        self.tabindex = Some(tabindex);
        self
    }

    /// Встроенный стиль `li`.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Команда.
    pub fn command(mut self, command: impl Into<Callback<StepsCommand>>) -> Self {
        self.command = Some(command.into());
        self
    }
}

/// Pass-through частей [`Steps`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepsPt {
    /// Корень `nav`.
    pub root: Pt,
    /// Список.
    pub list: Pt,
    /// Пункт `li`.
    pub item: Pt,
    /// Ссылка пункта.
    pub item_link: Pt,
    /// Номер.
    pub item_number: Pt,
    /// Подпись.
    pub item_label: Pt,
}

/// Состояние, общее для обработчиков.
struct Core {
    model: Prop<Vec<StepsItem>>,
    readonly: Prop<bool>,
    active: Signal<usize>,
    active_signal: Option<RwSignal<usize>>,
    /// Ссылка с фокусом клавиатуры (roving `tabindex`); `None` — по умолчанию активный пункт.
    focus_index: RwSignal<Option<usize>>,
    list: NodeRef<leptos::html::Ul>,
    on_change: Option<Callback<usize>>,
}

impl Core {
    fn item(&self, index: usize) -> Option<StepsItem> {
        self.model.get_untracked().get(index).cloned()
    }

    /// Клик или `Enter`/`Space` по пункту.
    fn activate(&self, event: web_sys::Event, index: usize) {
        let Some(item) = self.item(index) else { return };
        match click_outcome(item.disabled, self.readonly.get_untracked(), item.url.is_some()) {
            Click::Ignore => event.prevent_default(),
            Click::Activate { prevent_default } => {
                if let Some(sig) = self.active_signal {
                    sig.set(index);
                }
                if let Some(cb) = self.on_change {
                    cb.run(index);
                }
                if prevent_default {
                    event.prevent_default();
                }
                if let Some(cb) = item.command {
                    cb.run(StepsCommand { event, item, index });
                }
            }
        }
    }

    fn focus_link(&self, index: usize) {
        self.focus_index.set(Some(index));
        #[cfg(target_arch = "wasm32")]
        if let Some(list) = self.list.get_untracked()
            && let Some(link) = list
                .children()
                .item(index as u32)
                .and_then(|li| li.first_element_child())
                .and_then(|a| a.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = link.focus();
        }
    }

    fn key_down(&self, e: &web_sys::KeyboardEvent, index: usize) {
        let code = e.code();
        if let Some(target) = key_target(&code, index, self.model.get_untracked().len()) {
            self.focus_link(target);
            e.prevent_default();
        } else if code == "Tab" {
            self.focus_index.set(None);
        } else if is_activation_key(&code) {
            self.activate(e.clone().into(), index);
            e.prevent_default();
        }
    }
}

/// Индикатор шагов мастера.
///
/// Свойства принимают значение или сигнал. `active_index` ведёт владелец: клик в интерактивном режиме сообщает новый индекс
/// через `on_active_index_change` и обновляет `active_index_signal`, если он задан.
#[component]
pub fn Steps(
    /// Пункты.
    #[prop(optional, into)]
    model: Prop<Vec<StepsItem>>,
    /// Текущий шаг (с нуля).
    #[prop(optional, into)]
    active_index: Prop<usize>,
    /// Двусторонний сигнал текущего шага: клик записывает в него индекс (вместо `active_index`, если задан).
    #[prop(optional)]
    active_index_signal: Option<RwSignal<usize>>,
    /// Только отображение: клик по шагам ничего не делает.
    #[prop(optional, default = Prop::Static(true), into)]
    readonly: Prop<bool>,
    /// Свойство совместимости с PrimeNG (там влияет лишь на ссылки маршрутизатора).
    #[prop(optional, default = Prop::Static(true), into)]
    exact: Prop<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<StepsPt>,
    /// Текущий шаг выбран пользователем.
    #[prop(optional)]
    on_active_index_change: Option<Callback<usize>>,
) -> impl IntoView {
    let _ = exact;
    let pt = Arc::new(pt.unwrap_or_default());
    let active: Signal<usize> = match active_index_signal {
        Some(sig) => sig.into(),
        None => Signal::derive({
            let active_index = active_index.clone();
            move || active_index.get()
        }),
    };
    let core = Arc::new(Core {
        model: model.clone(),
        readonly: readonly.clone(),
        active,
        active_signal: active_index_signal,
        focus_index: RwSignal::new(None),
        list: NodeRef::new(),
        on_change: on_active_index_change,
    });
    // Смена текущего шага возвращает «ролевой» tabindex на него.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            core.active.track();
            core.focus_index.set(None);
        });
    }

    let root_class = {
        let (pt, readonly) = (pt.clone(), readonly.clone());
        move || {
            let extra = style_class.with(|c| c.clone());
            let merged = root_classes(readonly.get(), extra.as_deref());
            pt.root.merge_class(&merged).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let items = {
        let (core, pt) = (core.clone(), pt.clone());
        move || core.model.get().into_iter().enumerate().map(|(index, item)| item_view(&core, &pt, index, item)).collect_view()
    };
    let (root_attrs, list_attrs) = (pt.root.attrs(), pt.list.attrs());
    let list_class = pt.list.merge_class("p-steps-list").into_owned();
    view! {
        <nav class={root_class} style={root_style} data-pc-name="steps" {..root_attrs}>
            <ul node_ref=core.list class={list_class} data-pc-section="menu" {..list_attrs}>
                {items}
            </ul>
        </nav>
    }
}

fn item_view(core: &Arc<Core>, pt: &Arc<StepsPt>, index: usize, item: StepsItem) -> AnyView {
    let (active, readonly, focus_index) = (core.active, core.readonly.clone(), core.focus_index);
    let is_active = move || active.get() == index;
    let disabled = {
        let (item_disabled, readonly) = (item.disabled, readonly.clone());
        move || is_item_disabled(item_disabled, index, active.get(), readonly.get())
    };
    let class = {
        let (pt, disabled, is_active) = (pt.clone(), disabled.clone(), is_active);
        move || pt.item.merge_class(&item_classes(is_active(), disabled())).into_owned()
    };
    let tabindex = {
        let (item_disabled, item_tab) = (item.disabled, item.tabindex);
        move || {
            match focus_index.get() {
                Some(f) => {
                    if f == index {
                        0
                    } else {
                        -1
                    }
                }
                None => link_tabindex(item_disabled, item_tab, index, active.get()),
            }
            .to_string()
        }
    };
    let aria_dis = {
        let (item_disabled, readonly) = (item.disabled, readonly.clone());
        move || aria_disabled(item_disabled, index, active.get(), readonly.get()).to_string()
    };
    let (click_core, key_core) = (core.clone(), core.clone());
    let attrs: Vec<AnyAttribute> = vec![custom_attribute("aria-current", move || is_active().then_some("step")).into_any_attr()];
    let (item_attrs, link_attrs) = (pt.item.attrs(), pt.item_link.attrs());
    let (link_class, number_class, label_class) = (
        pt.item_link.merge_class("p-steps-item-link").into_owned(),
        pt.item_number.merge_class("p-steps-item-number").into_owned(),
        pt.item_label.merge_class("p-steps-item-label").into_owned(),
    );
    let (number_attrs, label_attrs) = (pt.item_number.attrs(), pt.item_label.attrs());
    view! {
        <li class={class} style={item.style.clone()} id={item.id.clone()} data-pc-section="menuitem" {..attrs} {..item_attrs}>
            <a
                role="link"
                class={link_class}
                href={item.url.clone()}
                target={item.target.clone()}
                tabindex={tabindex}
                aria-expanded={move || is_active().to_string()}
                aria-disabled={aria_dis}
                on:click={move |e: web_sys::MouseEvent| click_core.activate(e.into(), index)}
                on:keydown={move |e: web_sys::KeyboardEvent| key_core.key_down(&e, index)}
                {..link_attrs}
            >
                <span class={number_class} {..number_attrs}>{index + 1}</span>
                <span class={label_class} {..label_attrs}>{item.label.clone()}</span>
            </a>
        </li>
    }
    .into_any()
}
