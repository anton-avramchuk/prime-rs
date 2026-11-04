//! TreeSelect — порт `p-treeselect` из PrimeNG 21.1.10.
//!
//! Поле выбора узла (или узлов) дерева: метка с подписями через запятую либо чипами, значок очистки, кнопка раскрытия и
//! панель внутри корня с готовым [`prime_tree::Tree`]. Значение — двусторонний сигнал ключей выбранных узлов
//! (`RwSignal<Vec<String>>`; в режиме `Single` — 0 или 1 ключ). Режимы выбора, фильтр и сообщения передаются в `Tree`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_tree::TreeNode;
//! use prime_treeselect::TreeSelect;
//!
//! let options = vec![TreeNode::new("0").label("Documents").children(vec![TreeNode::new("0-0").label("Work")])];
//! let value = RwSignal::new(vec!["0-0".to_owned()]);
//! let html = Owner::new().with(|| view! { <TreeSelect options=options value=value placeholder="Select" /> }.to_html());
//! assert!(html.contains("p-treeselect-label") && html.contains("Work") && !html.contains("p-treeselect-overlay"));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::leptos_dom::helpers::WindowListenerHandle;
use leptos::prelude::*;
use prime_chip::Chip;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{ChevronDownIcon, TimesIcon};
use prime_tree::{Tree, TreeFilterMode, TreeNode, TreeNodeEvent, TreeSelectionMode, flatten};
use std::collections::HashSet;
use std::sync::Arc;

pub use state::{ancestors_of, labels_of};

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

/// Как показывать выбранные узлы в метке.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeSelectDisplay {
    /// Подписи через запятую.
    #[default]
    Comma,
    /// Чипы.
    Chip,
}

/// Pass-through частей [`TreeSelect`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TreeSelectPt {
    /// Корень.
    pub root: Pt,
    /// Контейнер скрытого поля.
    pub hidden_input_container: Pt,
    /// Скрытое поле.
    pub hidden_input: Pt,
    /// Контейнер метки.
    pub label_container: Pt,
    /// Метка.
    pub label: Pt,
    /// Элемент чипа.
    pub chip_item: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Кнопка раскрытия.
    pub dropdown: Pt,
    /// Стрелка кнопки.
    pub dropdown_icon: Pt,
    /// Панель.
    pub panel: Pt,
    /// Контейнер дерева.
    pub tree_container: Pt,
}

/// Состояние, общее для обработчиков.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: String,
    options: Prop<Vec<TreeNode>>,
    value: RwSignal<Vec<String>>,
    expanded: RwSignal<HashSet<String>>,
    open: RwSignal<bool>,
    focused: RwSignal<bool>,
    mode: TreeSelectionMode,
    disabled: Prop<bool>,
    root_ref: NodeRef<html::Div>,
    overlay_ref: NodeRef<html::Div>,
    input_ref: NodeRef<html::Input>,
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_open: StoredValue<bool>,
    on_node_expand: Option<Callback<TreeNodeEvent>>,
    on_node_collapse: Option<Callback<TreeNodeEvent>>,
    on_node_select: Option<Callback<TreeNodeEvent>>,
    on_node_unselect: Option<Callback<TreeNodeEvent>>,
    on_clear: Option<Callback<()>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Core {
    fn focus_input(&self) {
        if let Some(input) = self.input_ref.get_untracked() {
            let _ = input.focus();
        }
    }

    /// `show()`: раскрывает предков выбранных узлов и открывает панель.
    fn show(&self) {
        if self.open.get_untracked() {
            return;
        }
        let keys = self.value.get_untracked();
        let ancestors = self.options.with(|o| ancestors_of(o, &keys));
        self.expanded.update(|s| s.extend(ancestors));
        self.open.set(true);
    }

    /// `hide()`: закрывает панель.
    fn hide(&self) {
        self.open.try_set(false);
    }

    fn toggle(&self) {
        if self.open.get_untracked() { self.hide() } else { self.show() }
    }

    fn clear(&self, e: web_sys::MouseEvent) {
        e.stop_propagation();
        self.value.set(Vec::new());
        self.expanded.set(HashSet::new());
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }

    /// Клик по корню: открывает или закрывает панель, кроме кликов внутри панели.
    fn root_click(&self, e: &web_sys::MouseEvent) {
        if self.disabled.get_untracked() {
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
        self.toggle();
        self.focus_input();
    }

    /// Первый узел дерева в панели получает фокус (в следующем такте, пока панель рисуется).
    fn focus_first_node(self: &Arc<Self>) {
        #[cfg(target_arch = "wasm32")]
        {
            let this = self.clone();
            set_timeout(
                move || {
                    if let Some(overlay) = this.overlay_ref.try_get_untracked().flatten()
                        && let Ok(Some(el)) = overlay.query_selector("li.p-tree-node[tabindex=\"0\"], li.p-tree-node")
                        && let Ok(el) = el.dyn_into::<web_sys::HtmlElement>()
                    {
                        let _ = el.focus();
                    }
                },
                Duration::ZERO,
            );
        }
    }

    fn key_down(self: &Arc<Self>, e: &web_sys::KeyboardEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        match e.code().as_str() {
            "ArrowDown" => {
                self.show();
                self.focus_first_node();
                e.prevent_default();
            }
            "Space" | "Enter" | "NumpadEnter" => {
                if !self.open.get_untracked() {
                    self.show();
                    e.prevent_default();
                }
            }
            "Escape" => {
                if self.open.get_untracked() {
                    self.hide();
                    self.focus_input();
                    e.prevent_default();
                }
            }
            "Tab" => self.hide(),
            _ => {}
        }
    }

    /// Выбор узла: в `Single` панель закрывается, фокус возвращается на поле.
    fn node_select(&self, e: TreeNodeEvent) {
        if let Some(cb) = self.on_node_select {
            cb.run(e);
        }
        if self.mode == TreeSelectionMode::Single {
            self.hide();
            self.focus_input();
        }
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
                this.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(ev::resize, move |_| {
            if this.open.get_untracked() {
                this.hide();
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

    /// Пересчитывает положение панели после раскрытия узла.
    fn realign(self: &Arc<Self>) {
        #[cfg(target_arch = "wasm32")]
        {
            let this = self.clone();
            set_timeout(
                move || {
                    if this.open.is_disposed() {
                        return;
                    }
                    if let (Some(overlay), Some(root)) = (this.overlay_ref.get_untracked(), this.root_ref.get_untracked()) {
                        align_overlay(
                            &overlay.unchecked_into::<web_sys::HtmlElement>(),
                            &root.unchecked_into::<web_sys::HtmlElement>(),
                            AlignMode::Relative,
                            true,
                        );
                    }
                },
                Duration::ZERO,
            );
        }
    }
}

/// Поле выбора узлов дерева в выпадающей панели.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn TreeSelect(
    /// Узлы дерева.
    #[prop(optional, into)]
    options: Prop<Vec<TreeNode>>,
    /// Выбранные ключи (двусторонний сигнал).
    #[prop(optional)]
    value: Option<RwSignal<Vec<String>>>,
    /// Режим выбора (по умолчанию `Single`).
    #[prop(optional, default = TreeSelectionMode::Single)]
    selection_mode: TreeSelectionMode,
    /// Вид метки.
    #[prop(optional, into)]
    display: Prop<TreeSelectDisplay>,
    /// Подсказка пустого поля.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Высота контейнера дерева (по умолчанию `400px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Выбор с Ctrl/Meta (`Multiple`).
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Распространять выбор на предков (`Checkbox`).
    #[prop(optional, default = Prop::Static(true), into)]
    propagate_selection_up: Prop<bool>,
    /// Распространять выбор на потомков (`Checkbox`).
    #[prop(optional, default = Prop::Static(true), into)]
    propagate_selection_down: Prop<bool>,
    /// Показывать значок очистки.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
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
    /// Поле фильтра в панели.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Поля фильтра через запятую.
    #[prop(optional, default = Prop::Static("label".to_owned()), into)]
    filter_by: Prop<String>,
    /// Режим фильтра.
    #[prop(optional, into)]
    filter_mode: Prop<TreeFilterMode>,
    /// Подсказка поля фильтра.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Сообщение о пустом дереве.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Идёт загрузка дерева.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Атрибут `id` корня (основа идентификатора панели).
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
    pt: Option<TreeSelectPt>,
    /// Своё представление выбранного (вместо метки).
    #[prop(optional)]
    value_view: Option<Callback<Vec<TreeNode>, AnyView>>,
    /// Своё содержимое подписи узла дерева.
    #[prop(optional)]
    node: Option<Callback<TreeNode, AnyView>>,
    /// Узел выбран.
    #[prop(optional)]
    on_node_select: Option<Callback<TreeNodeEvent>>,
    /// Выбор узла снят.
    #[prop(optional)]
    on_node_unselect: Option<Callback<TreeNodeEvent>>,
    /// Узел раскрыт.
    #[prop(optional)]
    on_node_expand: Option<Callback<TreeNodeEvent>>,
    /// Узел свёрнут.
    #[prop(optional)]
    on_node_collapse: Option<Callback<TreeNodeEvent>>,
    /// Значение очищено.
    #[prop(optional)]
    on_clear: Option<Callback<()>>,
    /// Панель показана.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Панель скрыта.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
    /// Скрытое поле получило фокус.
    #[prop(optional)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Скрытое поле потеряло фокус.
    #[prop(optional)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let value = value.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let core = Arc::new(Core {
        id: id.clone(),
        options: options.clone(),
        value,
        expanded: RwSignal::new(HashSet::new()),
        open: RwSignal::new(false),
        focused: RwSignal::new(false),
        mode: selection_mode,
        disabled: disabled.clone(),
        root_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        input_ref: NodeRef::new(),
        listeners: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
        on_node_expand,
        on_node_collapse,
        on_node_select,
        on_node_unselect,
        on_clear,
        on_show,
        on_hide,
        on_focus,
        on_blur,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // Побочные эффекты открытия и закрытия панели — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let is_open = core.open.get();
            if is_open != core.was_open.get_value() {
                core.was_open.set_value(is_open);
                if is_open {
                    core.opened();
                } else {
                    core.closed();
                }
            }
        });
    }

    // Выбранные узлы по ключам, в порядке выбора.
    let selected_labels = {
        let (options, value) = (options.clone(), core.value);
        move || -> Vec<(String, String)> { options.with(|o| value.with(|v| labels_of(&flatten(o), v))) }
    };
    let filled = {
        let sel = selected_labels.clone();
        move || !sel().is_empty()
    };
    let label_text = {
        let (sel, placeholder) = (selected_labels.clone(), placeholder.clone());
        move || -> Option<String> {
            let labels = sel();
            if labels.is_empty() { placeholder.get() } else { Some(labels.into_iter().map(|(_, l)| l).collect::<Vec<_>>().join(", ")) }
        }
    };

    let root_class = {
        let (core, pt, style_class, filled, display) = (core.clone(), pt.clone(), style_class.clone(), filled.clone(), display.clone());
        let (disabled, invalid, variant, size, fluid, show_clear) =
            (disabled.clone(), invalid.clone(), variant.clone(), size.clone(), fluid.clone(), show_clear.clone());
        move || {
            let config_variant = use_prime_config().input_variant;
            let filled_variant = variant.get().or(config_variant) == Some(Variant::Filled);
            let open = core.open.get();
            let focused = core.focused.get();
            let mut cn = ClassNames::new();
            cn.add("p-treeselect p-component p-inputwrapper")
                .add_if(display.get() == TreeSelectDisplay::Chip, "p-treeselect-display-chip")
                .add_if(disabled.get(), "p-disabled")
                .add_if(invalid.get(), "p-invalid")
                .add_if(focused, "p-focus")
                .add_if(filled_variant, "p-variant-filled")
                .add_if(filled(), "p-inputwrapper-filled")
                .add_if(focused || open, "p-inputwrapper-focus")
                .add_if(open, "p-treeselect-open")
                .add_if(show_clear.get(), "p-treeselect-clearable")
                .add_if(fluid.get(), "p-treeselect-fluid")
                .add_if(size.get() == Some(Size::Small), "p-treeselect-sm p-inputfield-sm")
                .add_if(size.get() == Some(Size::Large), "p-treeselect-lg p-inputfield-lg");
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
        let (pt, filled, placeholder) = (pt.clone(), filled.clone(), placeholder.clone());
        move || {
            let empty = !filled();
            let has_placeholder = placeholder.with(|p| p.as_deref().is_some_and(|p| !p.is_empty()));
            let mut cn = ClassNames::new();
            cn.add("p-treeselect-label")
                .add_if(empty && has_placeholder, "p-placeholder")
                .add_if(empty && !has_placeholder, "p-treeselect-label-empty");
            cn.add_user(pt.label.extra_class());
            cn.finish_cow()
        }
    };
    let label_content = {
        let (sel, label_text, display, options, value) =
            (selected_labels.clone(), label_text.clone(), display.clone(), options.clone(), core.value);
        move || -> AnyView {
            if let Some(cb) = value_view {
                let nodes: Vec<TreeNode> = options.with(|o| {
                    value.with(|v| {
                        let mut found = Vec::new();
                        collect_nodes(o, v, &mut found);
                        found
                    })
                });
                return cb.run(nodes);
            }
            if display.get() == TreeSelectDisplay::Chip {
                let labels = sel();
                if labels.is_empty() {
                    return label_text().unwrap_or_else(|| "empty".to_owned()).into_any();
                }
                return labels
                    .into_iter()
                    .map(|(_, label)| view! { <div class="p-treeselect-chip-item"><Chip label=label style_class="p-treeselect-chip" /></div> })
                    .collect_view()
                    .into_any();
            }
            label_text().unwrap_or_else(|| "empty".to_owned()).into_any()
        }
    };
    let clear = {
        let (core, pt, filled, disabled, show_clear) = (core.clone(), pt.clone(), filled.clone(), disabled.clone(), show_clear.clone());
        move || {
            (filled() && !disabled.get() && show_clear.get()).then(|| {
                let core = core.clone();
                let class = pt.clear_icon.merge_class("p-treeselect-clear-icon").into_owned();
                view! { <TimesIcon class={class} on:click={move |e: web_sys::MouseEvent| core.clear(e)} {..pt.clear_icon.attrs()} /> }
            })
        }
    };
    let panel = {
        let (core, pt) = (core.clone(), pt.clone());
        let props = PanelProps {
            scroll_height,
            meta_key_selection,
            propagate_selection_up,
            propagate_selection_down,
            filter,
            filter_by,
            filter_mode,
            filter_placeholder,
            empty_message,
            loading,
            panel_style_class,
            node,
        };
        move || core.open.get().then(|| panel_view(&core, &pt, &props))
    };
    let input_attrs: Vec<AnyAttribute> = {
        let (core, disabled, input_id, tabindex, aria_label, aria_labelledby, label_text) = (
            core.clone(),
            disabled.clone(),
            input_id.clone(),
            tabindex.clone(),
            aria_label.clone(),
            aria_labelledby.clone(),
            label_text.clone(),
        );
        let attr = |name: &'static str, f: Box<dyn Fn() -> Option<String> + Send + Sync>| custom_attribute(name, f).into_any_attr();
        vec![
            attr("id", Box::new(move || input_id.get())),
            attr("aria-label", Box::new(move || aria_label.get().or_else(&label_text))),
            attr("aria-labelledby", Box::new(move || aria_labelledby.get())),
            attr("aria-expanded", {
                let core = core.clone();
                Box::new(move || Some(core.open.get().to_string()))
            }),
            attr("aria-controls", {
                let core = core.clone();
                Box::new(move || core.open.get().then(|| format!("{}_list", core.id)))
            }),
            attr("disabled", {
                let disabled = disabled.clone();
                Box::new(move || disabled.get().then(String::new))
            }),
            attr("tabindex", Box::new(move || Some(if disabled.get() { -1 } else { tabindex.get().unwrap_or(0) }.to_string()))),
        ]
    };
    let (focus_core, blur_core, key_core, click_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let (root_attrs, hidden_container_attrs, hidden_attrs, label_container_attrs, label_attrs, dropdown_attrs) = (
        pt.root.attrs(),
        pt.hidden_input_container.attrs(),
        pt.hidden_input.attrs(),
        pt.label_container.attrs(),
        pt.label.attrs(),
        pt.dropdown.attrs(),
    );
    let (hidden_container_class, label_container_class, dropdown_class) = (
        pt.hidden_input_container.merge_class("p-hidden-accessible").into_owned(),
        pt.label_container.merge_class("p-treeselect-label-container").into_owned(),
        pt.dropdown.merge_class("p-treeselect-dropdown").into_owned(),
    );
    let dropdown_expanded = {
        let core = core.clone();
        move || core.open.get().to_string()
    };
    let dropdown_icon_class = pt.dropdown_icon.merge_class("p-treeselect-dropdown-icon").into_owned();
    let (root_ref, input_ref) = (core.root_ref, core.input_ref);
    view! {
        <div node_ref={root_ref} id={id} class={root_class} style={root_style} on:click={move |e| click_core.root_click(&e)} {..root_attrs}>
            <div class={hidden_container_class} data-p-hidden-accessible="true" {..hidden_container_attrs}>
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
                        if let Some(cb) = blur_core.on_blur {
                            cb.run(e);
                        }
                    }}
                    on:keydown={move |e| key_core.key_down(&e)}
                    {..input_attrs}
                    {..hidden_attrs}
                />
            </div>
            <div class={label_container_class} {..label_container_attrs}>
                <div class={label_class} {..label_attrs}>{label_content}</div>
            </div>
            {clear}
            <div class={dropdown_class} role="button" aria-haspopup="tree" aria-expanded={dropdown_expanded} aria-label="treeselect trigger" {..dropdown_attrs}>
                <ChevronDownIcon class={dropdown_icon_class} />
            </div>
            {panel}
        </div>
    }
}

fn collect_nodes(nodes: &[TreeNode], keys: &[String], out: &mut Vec<TreeNode>) {
    for n in nodes {
        if keys.contains(&n.key) {
            out.push(n.clone());
        }
        collect_nodes(&n.children, keys, out);
    }
}

/// Свойства, которые панель передаёт в дерево.
struct PanelProps {
    scroll_height: OptProp<String>,
    meta_key_selection: Prop<bool>,
    propagate_selection_up: Prop<bool>,
    propagate_selection_down: Prop<bool>,
    filter: Prop<bool>,
    filter_by: Prop<String>,
    filter_mode: Prop<TreeFilterMode>,
    filter_placeholder: OptProp<String>,
    empty_message: OptProp<String>,
    loading: Prop<bool>,
    panel_style_class: OptProp<String>,
    node: Option<Callback<TreeNode, AnyView>>,
}

/// Панель с деревом.
fn panel_view(core: &Arc<Core>, pt: &Arc<TreeSelectPt>, p: &PanelProps) -> impl IntoView + use<> {
    let class = {
        let (pt, extra) = (pt.clone(), p.panel_style_class.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-treeselect-overlay p-component-overlay p-component");
            extra.with(|c| cn.add_user(c.as_deref()));
            pt.panel.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let container_style = {
        let h = p.scroll_height.clone();
        move || format!("max-height: {}", h.get().unwrap_or_else(|| "400px".to_owned()))
    };
    let (select_core, unselect_core, expand_core, collapse_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let on_select = Callback::new(move |e: TreeNodeEvent| select_core.node_select(e));
    let on_unselect = Callback::new(move |e: TreeNodeEvent| {
        if let Some(cb) = unselect_core.on_node_unselect {
            cb.run(e);
        }
    });
    let on_expand = Callback::new(move |e: TreeNodeEvent| {
        if let Some(cb) = expand_core.on_node_expand {
            cb.run(e);
        }
        expand_core.realign();
    });
    let on_collapse = Callback::new(move |e: TreeNodeEvent| {
        if let Some(cb) = collapse_core.on_node_collapse {
            cb.run(e);
        }
        collapse_core.realign();
    });
    let list_id = format!("{}_list", core.id);
    let (container_class, container_attrs, panel_attrs) =
        (pt.tree_container.merge_class("p-treeselect-tree-container").into_owned(), pt.tree_container.attrs(), pt.panel.attrs());
    let overlay_ref = core.overlay_ref;
    let (value, expanded, options, mode) = (core.value, core.expanded, core.options.clone(), core.mode);
    let (meta, up, down, filter, filter_by, filter_mode, filter_placeholder, empty_message, loading, node) = (
        p.meta_key_selection.clone(),
        p.propagate_selection_up.clone(),
        p.propagate_selection_down.clone(),
        p.filter.clone(),
        p.filter_by.clone(),
        p.filter_mode.clone(),
        p.filter_placeholder.clone(),
        p.empty_message.clone(),
        p.loading.clone(),
        p.node,
    );
    let tree = move || -> AnyView {
        match node {
            Some(node) => view! {
                <Tree
                    value=options.clone()
                    selection_mode=mode
                    selection=value
                    expanded_keys=expanded
                    meta_key_selection=meta.clone()
                    propagate_selection_up=up.clone()
                    propagate_selection_down=down.clone()
                    filter=filter.clone()
                    filter_by=filter_by.clone()
                    filter_mode=filter_mode.clone()
                    filter_placeholder=filter_placeholder.clone()
                    empty_message=empty_message.clone()
                    loading=loading.clone()
                    node=node
                    on_node_select=on_select
                    on_node_unselect=on_unselect
                    on_node_expand=on_expand
                    on_node_collapse=on_collapse
                />
            }
            .into_any(),
            None => view! {
                <Tree
                    value=options.clone()
                    selection_mode=mode
                    selection=value
                    expanded_keys=expanded
                    meta_key_selection=meta.clone()
                    propagate_selection_up=up.clone()
                    propagate_selection_down=down.clone()
                    filter=filter.clone()
                    filter_by=filter_by.clone()
                    filter_mode=filter_mode.clone()
                    filter_placeholder=filter_placeholder.clone()
                    empty_message=empty_message.clone()
                    loading=loading.clone()
                    on_node_select=on_select
                    on_node_unselect=on_unselect
                    on_node_expand=on_expand
                    on_node_collapse=on_collapse
                />
            }
            .into_any(),
        }
    };
    view! {
        <div node_ref={overlay_ref} id={list_id} class={class} {..panel_attrs}>
            <div class={container_class} style={container_style} {..container_attrs}>{tree}</div>
        </div>
    }
}
