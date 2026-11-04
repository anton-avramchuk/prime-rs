//! Tree — порт `p-tree` из PrimeNG 21.1.10.
//!
//! Иерархические данные с раскрытием, выбором (`Single`, `Multiple`, `Checkbox`) и фильтром. Узлы — [`TreeNode`] с
//! уникальным `key`: раскрытые и выбранные узлы хранятся как наборы ключей в сигналах ([`Tree`] принимает внешние
//! `expanded_keys` и `selection`), сами узлы неизменяемы. Ленивую загрузку делает пользователь: в `on_node_expand` он
//! обновляет сигнал `value`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_tree::{Tree, TreeNode};
//!
//! let nodes = vec![TreeNode::new("0").label("Documents").expanded(true).children(vec![TreeNode::new("0-0").label("Work")])];
//! let html = Owner::new().with(|| view! { <Tree value=nodes /> }.to_html());
//! assert!(html.contains("p-tree-root-children") && html.contains("role=\"tree\"") && html.contains("Work"));
//! ```

mod state;

use leptos::prelude::*;
use prime_checkbox::CheckboxState;
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{CheckIcon, ChevronDownIcon, ChevronRightIcon, MinusIcon, SearchIcon, SpinnerIcon};
use std::collections::HashSet;
use std::sync::Arc;

pub use state::{
    FilterOpts, Flat, Move, SelectOpts, TreeFilterMode, TreeNode, TreeSelectionMode, arrow_down, arrow_left, arrow_right, arrow_up, end,
    filter_tree, flatten, home, is_partial, propagate_down, propagate_up, seed_expanded, select_click, tab_stop, visible,
};

#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Как показывать загрузку дерева и узлов.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeLoadingMode {
    /// Маска поверх дерева.
    #[default]
    Mask,
    /// Спиннер вместо шеврона загружаемого узла.
    Icon,
}

/// Событие узла: исходное событие и узел.
#[derive(Clone)]
pub struct TreeNodeEvent {
    /// Исходное событие.
    pub original_event: web_sys::Event,
    /// Узел.
    pub node: TreeNode,
}

/// Pass-through частей [`Tree`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TreePt {
    /// Корень.
    pub root: Pt,
    /// Маска загрузки.
    pub mask: Pt,
    /// Значок загрузки.
    pub loading_icon: Pt,
    /// Контейнер поля фильтра.
    pub filter_container: Pt,
    /// Поле фильтра.
    pub filter_input: Pt,
    /// Значок фильтра.
    pub filter_icon: Pt,
    /// Обёртка со списком.
    pub wrapper: Pt,
    /// Корневой список.
    pub root_children: Pt,
    /// Узел `li`.
    pub node: Pt,
    /// Содержимое узла.
    pub node_content: Pt,
    /// Кнопка раскрытия.
    pub node_toggle_button: Pt,
    /// Шеврон кнопки.
    pub node_toggle_icon: Pt,
    /// Флажок узла.
    pub node_checkbox: Pt,
    /// Значок узла.
    pub node_icon: Pt,
    /// Подпись узла.
    pub node_label: Pt,
    /// Вложенный список.
    pub node_children: Pt,
    /// Сообщение о пустом дереве.
    pub empty_message: Pt,
}

/// Состояние, общее для обработчиков.
struct Core {
    value: Prop<Vec<TreeNode>>,
    filter_text: RwSignal<String>,
    shown: Memo<Vec<TreeNode>>,
    flat_full: Memo<Vec<Flat>>,
    flat_shown: Memo<Vec<Flat>>,
    expanded: RwSignal<HashSet<String>>,
    selection: RwSignal<Vec<String>>,
    focus: RwSignal<Option<String>>,
    mode: Option<TreeSelectionMode>,
    meta_key_selection: Prop<bool>,
    propagate_up: Prop<bool>,
    propagate_down: Prop<bool>,
    root_el: NodeRef<leptos::html::Div>,
    filter_by: Prop<String>,
    filter_mode: Prop<TreeFilterMode>,
    on_select: Option<Callback<TreeNodeEvent>>,
    on_unselect: Option<Callback<TreeNodeEvent>>,
    on_expand: Option<Callback<TreeNodeEvent>>,
    on_collapse: Option<Callback<TreeNodeEvent>>,
    on_double_click: Option<Callback<TreeNodeEvent>>,
    on_filter: Option<Callback<String>>,
}

fn find_node(nodes: &[TreeNode], key: &str) -> Option<TreeNode> {
    for n in nodes {
        if n.key == key {
            return Some(n.clone());
        }
        if let Some(found) = find_node(&n.children, key) {
            return Some(found);
        }
    }
    None
}

impl Core {
    fn node(&self, key: &str) -> Option<TreeNode> {
        self.value.with(|v| find_node(v, key))
    }

    fn event(&self, e: &web_sys::Event, key: &str) -> Option<TreeNodeEvent> {
        self.node(key).map(|node| TreeNodeEvent { original_event: e.clone(), node })
    }

    fn with_visible<R>(&self, f: impl FnOnce(&[&Flat]) -> R) -> R {
        let expanded = self.expanded.get_untracked();
        self.flat_shown.with_untracked(|flat| f(&visible(flat, &expanded)))
    }

    /// Фокус на `li` по ключу (только браузер).
    fn focus_key(&self, key: &str) {
        #[cfg(target_arch = "wasm32")]
        if let Some(root) = self.root_el.get_untracked() {
            let escaped = key.replace('\\', "\\\\").replace('"', "\\\"");
            if let Ok(Some(el)) = root.query_selector(&format!("li[data-id=\"{escaped}\"]"))
                && let Ok(el) = el.dyn_into::<web_sys::HtmlElement>()
            {
                let _ = el.focus();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = key;
    }

    fn set_expanded(&self, e: &web_sys::Event, key: &str, open: bool) {
        self.expanded.update(|s| {
            if open {
                s.insert(key.to_owned());
            } else {
                s.remove(key);
            }
        });
        let cb = if open { self.on_expand } else { self.on_collapse };
        if let (Some(cb), Some(ev)) = (cb, self.event(e, key)) {
            cb.run(ev);
        }
    }

    /// Клик по кнопке раскрытия.
    fn toggle(&self, e: web_sys::MouseEvent, key: &str) {
        e.stop_propagation();
        let open = !self.expanded.with_untracked(|s| s.contains(key));
        self.set_expanded(&e, key, open);
    }

    /// Клик по узлу или `Enter`/`Space`: выбор.
    fn select(&self, e: &web_sys::Event, key: &str, meta_key: bool) {
        let Some(mode) = self.mode else { return };
        let opts = SelectOpts {
            mode,
            meta_selection: self.meta_key_selection.get_untracked(),
            meta_key,
            up: self.propagate_up.get_untracked(),
            down: self.propagate_down.get_untracked(),
        };
        let mut sel = self.selection.get_untracked();
        let outcome = self.flat_full.with_untracked(|flat| select_click(flat, &mut sel, key, opts));
        let Some(select) = outcome else { return };
        self.selection.set(sel);
        let cb = if select { self.on_select } else { self.on_unselect };
        if let (Some(cb), Some(ev)) = (cb, self.event(e, key)) {
            cb.run(ev);
        }
    }

    fn apply_move(&self, e: &web_sys::Event, mv: Move) {
        match mv {
            Move::Focus(key) => {
                self.focus.set(Some(key.clone()));
                self.focus_key(&key);
            }
            Move::Collapse(key) => self.set_expanded(e, &key, false),
            Move::ExpandAndEnter(key) => {
                self.set_expanded(e, &key, true);
                let first = self.flat_shown.with_untracked(|f| f.iter().find(|n| n.key == key).and_then(|n| n.children.first().cloned()));
                if let Some(child) = first {
                    self.focus.set(Some(child.clone()));
                    #[cfg(target_arch = "wasm32")]
                    {
                        let (root, id) = (self.root_el, child);
                        set_timeout(
                            move || {
                                if let Some(root) = root.try_get_untracked().flatten() {
                                    let escaped = id.replace('\\', "\\\\").replace('"', "\\\"");
                                    if let Ok(Some(el)) = root.query_selector(&format!("li[data-id=\"{escaped}\"]"))
                                        && let Ok(el) = el.dyn_into::<web_sys::HtmlElement>()
                                    {
                                        let _ = el.focus();
                                    }
                                }
                            },
                            Duration::from_millis(1),
                        );
                    }
                }
            }
            Move::Stay => {}
        }
    }

    /// Клавиши на узле (только если событие пришло на сам `li`).
    fn key_down(&self, e: &web_sys::KeyboardEvent, key: &str) {
        if e.target() != e.current_target() {
            return;
        }
        let ev: &web_sys::Event = e;
        match e.code().as_str() {
            "ArrowDown" => {
                let mv = self.with_visible(|vis| arrow_down(vis, key));
                self.apply_move(ev, mv);
                e.prevent_default();
            }
            "ArrowUp" => {
                let mv = self.with_visible(|vis| arrow_up(vis, key));
                self.apply_move(ev, mv);
                e.prevent_default();
            }
            "ArrowRight" => {
                let mv = self.flat_shown.with_untracked(|flat| arrow_right(flat, &self.expanded.get_untracked(), key));
                self.apply_move(ev, mv);
                e.prevent_default();
            }
            "ArrowLeft" => {
                let mv = self.flat_shown.with_untracked(|flat| arrow_left(flat, &self.expanded.get_untracked(), key));
                if mv != Move::Stay {
                    self.apply_move(ev, mv);
                    e.prevent_default();
                }
            }
            "Home" => {
                let mv = self.with_visible(home);
                self.apply_move(ev, mv);
                e.prevent_default();
            }
            "End" => {
                let mv = self.with_visible(end);
                self.apply_move(ev, mv);
                e.prevent_default();
            }
            "Enter" | "NumpadEnter" | "Space" => {
                self.select(ev, key, e.ctrl_key() || e.meta_key());
                e.prevent_default();
            }
            _ => {}
        }
    }

    /// Ввод в поле фильтра.
    fn filter_input(&self, e: &web_sys::Event) {
        #[cfg(target_arch = "wasm32")]
        let text = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()).map(|i| i.value()).unwrap_or_default();
        #[cfg(not(target_arch = "wasm32"))]
        let text = {
            let _ = e;
            String::new()
        };
        self.apply_filter(text);
    }

    fn apply_filter(&self, text: String) {
        let by = self.filter_by.get_untracked();
        let fields: Vec<&str> = by.split(',').map(str::trim).collect();
        let mode = self.filter_mode.get_untracked();
        if let Some((_, keys)) = self.value.with(|v| filter_tree(v, &text.to_lowercase(), &FilterOpts { fields: &fields, mode })) {
            self.expanded.update(|s| s.extend(keys));
        }
        self.filter_text.set(text.clone());
        if let Some(cb) = self.on_filter {
            cb.run(text);
        }
    }
}

/// Всё, что нужно для вывода узлов.
#[derive(Clone)]
struct Ctx {
    core: Arc<Core>,
    pt: Arc<TreePt>,
    node: Option<Callback<TreeNode, AnyView>>,
    toggler_icon: Option<ViewFn>,
    indentation: Prop<f64>,
    highlight_on_select: Prop<bool>,
    loading_mode: Prop<TreeLoadingMode>,
}

/// Дерево с раскрытием, выбором и фильтром.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn Tree(
    /// Узлы.
    #[prop(optional, into)]
    value: Prop<Vec<TreeNode>>,
    /// Режим выбора.
    #[prop(optional)]
    selection_mode: Option<TreeSelectionMode>,
    /// Выбранные ключи (двусторонний сигнал; в `Single` — 0 или 1 ключ).
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Раскрытые ключи (двусторонний сигнал).
    #[prop(optional)]
    expanded_keys: Option<RwSignal<HashSet<String>>>,
    /// Выбор с Ctrl/Meta (иначе клик заменяет выбор).
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Распространять выбор на предков (`Checkbox`).
    #[prop(optional, default = Prop::Static(true), into)]
    propagate_selection_up: Prop<bool>,
    /// Распространять выбор на потомков (`Checkbox`).
    #[prop(optional, default = Prop::Static(true), into)]
    propagate_selection_down: Prop<bool>,
    /// Подсвечивать выбранные узлы в режиме `Checkbox`.
    #[prop(optional, into)]
    highlight_on_select: Prop<bool>,
    /// Идёт загрузка дерева.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Как показывать загрузку.
    #[prop(optional, into)]
    loading_mode: Prop<TreeLoadingMode>,
    /// Сообщение о пустом дереве (по умолчанию — из `locale`).
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Атрибут `aria-label` списка.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` списка.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Показывать поле фильтра.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Поля фильтра через запятую (`label`, `key`).
    #[prop(optional, default = Prop::Static("label".to_owned()), into)]
    filter_by: Prop<String>,
    /// Режим фильтра.
    #[prop(optional, into)]
    filter_mode: Prop<TreeFilterMode>,
    /// Подсказка поля фильтра.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Высота обёртки, после которой список прокручивается (`flex` — по высоте родителя).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Отступ уровня в rem.
    #[prop(optional, default = Prop::Static(1.5), into)]
    indentation: Prop<f64>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<TreePt>,
    /// Своё содержимое подписи узла (по `type_` можно выбирать вид).
    #[prop(optional)]
    node: Option<Callback<TreeNode, AnyView>>,
    /// Свой шеврон.
    #[prop(optional, into)]
    toggler_icon: Option<ViewFn>,
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
    /// Двойной клик по узлу.
    #[prop(optional)]
    on_node_double_click: Option<Callback<TreeNodeEvent>>,
    /// Изменён текст фильтра.
    #[prop(optional)]
    on_filter: Option<Callback<String>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let tree_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", prime_core::ids::next_unique()));
    let filter_text = RwSignal::new(String::new());
    let shown = Memo::new_with_compare(
        {
            let (value, filter_by, filter_mode) = (value.clone(), filter_by.clone(), filter_mode.clone());
            move |_| {
                let text = filter_text.get();
                value.with(|v| {
                    let by = filter_by.get();
                    let fields: Vec<&str> = by.split(',').map(str::trim).collect();
                    match filter_tree(v, &text.to_lowercase(), &FilterOpts { fields: &fields, mode: filter_mode.get() }) {
                        Some((nodes, _)) => nodes,
                        None => v.clone(),
                    }
                })
            }
        },
        |_, _| true,
    );
    let flat_full = Memo::new({
        let value = value.clone();
        move |_| value.with(|v| flatten(v))
    });
    let flat_shown = Memo::new(move |_| shown.with(|v| flatten(v)));
    let expanded = expanded_keys.unwrap_or_else(|| RwSignal::new(HashSet::new()));
    let seed = value.with(|v| seed_expanded(v));
    if !seed.is_empty() {
        expanded.update(|s| s.extend(seed));
    }
    let selection = selection.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let core = Arc::new(Core {
        value: value.clone(),
        filter_text,
        shown,
        flat_full,
        flat_shown,
        expanded,
        selection,
        focus: RwSignal::new(None),
        mode: selection_mode,
        meta_key_selection,
        propagate_up: propagate_selection_up,
        propagate_down: propagate_selection_down,
        root_el: NodeRef::new(),
        filter_by,
        filter_mode,
        on_select: on_node_select,
        on_unselect: on_node_unselect,
        on_expand: on_node_expand,
        on_collapse: on_node_collapse,
        on_double_click: on_node_double_click,
        on_filter,
    });
    let ctx = Ctx { core: core.clone(), pt: pt.clone(), node, toggler_icon, indentation, highlight_on_select, loading_mode };

    let class = {
        let (pt, loading) = (pt.clone(), loading.clone());
        let (style_class, scroll_height) = (style_class.clone(), scroll_height.clone());
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-tree p-component")
                .add_if(selection_mode.is_some(), "p-tree-selectable")
                .add_if(loading.get(), "p-tree-loading")
                .add_if(scroll_height.with(|h| h.as_deref() == Some("flex")), "p-tree-flex-scrollable");
            let extra = style_class.with(|c| c.clone());
            cn.add_user(extra.as_deref());
            pt.root.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let root_style = {
        let (pt, style) = (pt.clone(), style.clone());
        move || {
            let own = style.with(|s| s.clone()).unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&own]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let mask = {
        let (pt, ctx, loading) = (pt.clone(), ctx.clone(), loading.clone());
        move || {
            (loading.get() && ctx.loading_mode.get() == TreeLoadingMode::Mask).then(|| {
                view! {
                    <div class={pt.mask.merge_class("p-tree-mask p-overlay-mask").into_owned()} {..pt.mask.attrs()}>
                        <SpinnerIcon spin=true class={pt.loading_icon.merge_class("p-tree-loading-icon").into_owned()} />
                    </div>
                }
            })
        }
    };
    let filter_view = {
        let (core, pt, placeholder) = (core.clone(), pt.clone(), filter_placeholder.clone());
        move || {
            filter.get().then(|| {
                let (core, placeholder) = (core.clone(), placeholder.clone());
                let text = core.filter_text;
                view! {
                    <span class={pt.filter_container.merge_class("p-iconfield p-tree-filter-container").into_owned()} {..pt.filter_container.attrs()}>
                        <input
                            type="search"
                            autocomplete="off"
                            class={pt.filter_input.merge_class("p-inputtext p-component p-tree-filter-input").into_owned()}
                            placeholder={move || placeholder.get()}
                            prop:value={move || text.get()}
                            on:keydown={|e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" {
                                    e.prevent_default();
                                }
                            }}
                            on:input={move |e| core.filter_input(&e)}
                            {..pt.filter_input.attrs()}
                        />
                        <span class="p-inputicon"><SearchIcon class={pt.filter_icon.merge_class("p-tree-filter-icon").into_owned()} /></span>
                    </span>
                }
            })
        }
    };
    let body = {
        let (ctx, scroll_height, aria_label, aria_labelledby) =
            (ctx.clone(), scroll_height.clone(), aria_label.clone(), aria_labelledby.clone());
        move || {
            let has_nodes = ctx.core.shown.with(|n| !n.is_empty());
            has_nodes.then(|| {
                let (wrapper_class, wrapper_attrs) = (ctx.pt.wrapper.merge_class("p-tree-root").into_owned(), ctx.pt.wrapper.attrs());
                let (list_class, list_attrs) = (ctx.pt.root_children.merge_class("p-tree-root-children").into_owned(), ctx.pt.root_children.attrs());
                let style = {
                    let scroll_height = scroll_height.clone();
                    move || scroll_height.with(|h| h.clone()).filter(|h| h != "flex").map(|h| format!("max-height: {h}"))
                };
                let nodes = ctx.core.shown.get_untracked();
                let children = nodes_view(&ctx, &nodes, 0);
                let (aria_label, aria_labelledby) = (aria_label.clone(), aria_labelledby.clone());
                view! {
                    <div class={wrapper_class} style={style} {..wrapper_attrs}>
                        <ul class={list_class} role="tree" aria-label={move || aria_label.get()} aria-labelledby={move || aria_labelledby.get()} {..list_attrs}>
                            {children}
                        </ul>
                    </div>
                }
            })
        }
    };
    let empty = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let none = core.shown.with(|n| n.is_empty());
            (!loading.get() && none).then(|| {
                let text = empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| prime_core::use_prime_config().locale.text("emptyMessage").to_owned());
                view! { <div class={pt.empty_message.merge_class("p-tree-empty-message").into_owned()} {..pt.empty_message.attrs()}>{text}</div> }
            })
        }
    };
    let root_attrs = pt.root.attrs();
    view! {
        <div node_ref=core.root_el id={tree_id} class={class} style={root_style} {..root_attrs}>
            {mask}
            {filter_view}
            {body}
            {empty}
        </div>
    }
}

fn join(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(e) => format!("{base} {e}"),
        None => base.to_owned(),
    }
}

/// Узлы одного уровня. Весь вид строится один раз на смену дерева: состояние читается из замыканий в атрибутах.
fn nodes_view(ctx: &Ctx, nodes: &[TreeNode], level: usize) -> AnyView {
    nodes.iter().enumerate().map(|(index, node)| node_view(ctx, node, level, index)).collect_view().into_any()
}

#[allow(clippy::clone_on_copy)] // замыкание над `Memo` — `Copy`, `clone()` оставлен для единообразия с остальными замыканиями
fn node_view(ctx: &Ctx, node: &TreeNode, level: usize, index: usize) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let leaf = node.is_leaf();
    let mode = core.mode;
    let (expanded, selection, focus) = (core.expanded, core.selection, core.focus);
    // Memo: поддерево перестраивается только при смене раскрытия этого узла, а не любого другого.
    let is_expanded = {
        let key = key.clone();
        let memo = Memo::new(move |_| expanded.with(|s| s.contains(&key)));
        move || memo.get()
    };
    let is_selected = {
        let key = key.clone();
        move || selection.with(|s| s.contains(&key))
    };
    let partial = {
        let (core, key) = (core.clone(), key.clone());
        move || {
            mode == Some(TreeSelectionMode::Checkbox)
                && core.propagate_up.get()
                && selection.with(|s| core.flat_full.with(|f| is_partial(f, s, &key)))
        }
    };
    let tab_stop_key = {
        let (core, key) = (core.clone(), key.clone());
        move || {
            let expanded = expanded.get();
            let sel = selection.get();
            let focus = focus.get();
            core.flat_shown.with(|f| tab_stop(&visible(f, &expanded), &sel, focus.as_deref()).as_deref() == Some(key.as_str()))
        }
    };
    let li_class = join(
        &pt.node.merge_class(&if leaf { "p-tree-node p-tree-node-leaf".to_owned() } else { "p-tree-node".to_owned() }),
        node.style_class.as_deref(),
    );
    let content_class = {
        let (pt, is_selected, highlight) = (pt.clone(), is_selected.clone(), ctx.highlight_on_select.clone());
        let selectable = mode.is_some() && node.selectable;
        move || {
            let mut cn = prime_core::ClassNames::new();
            let selected = if mode == Some(TreeSelectionMode::Checkbox) { highlight.get() && is_selected() } else { is_selected() };
            cn.add("p-tree-node-content").add_if(selectable, "p-tree-node-selectable").add_if(selected, "p-tree-node-selected");
            pt.node_content.merge_class(&cn.finish_cow()).into_owned()
        }
    };
    let padding = {
        let indentation = ctx.indentation.clone();
        move || format!("padding-left: {}rem", f64::from(level as u32) * indentation.get())
    };
    let toggle = toggle_view(ctx, node, is_expanded.clone());
    let checkbox =
        (mode == Some(TreeSelectionMode::Checkbox)).then(|| checkbox_view(ctx, is_selected.clone(), partial.clone(), !node.selectable));
    let icon = icon_view(ctx, node, is_expanded.clone());
    let label = match ctx.node {
        Some(template) => {
            let node = node.clone();
            view! { <span class={pt.node_label.merge_class("p-tree-node-label").into_owned()} {..pt.node_label.attrs()}><span>{untrack(|| template.run(node))}</span></span> }.into_any()
        }
        None => view! { <span class={pt.node_label.merge_class("p-tree-node-label").into_owned()} {..pt.node_label.attrs()}><span>{node.label.clone()}</span></span> }.into_any(),
    };
    let children = (!node.children.is_empty()).then(|| {
        let (ctx, kids, is_expanded) = (ctx.clone(), node.children.clone(), is_expanded.clone());
        let (class, attrs) = (ctx.pt.node_children.merge_class("p-tree-node-children").into_owned(), ctx.pt.node_children.attrs());
        move || {
            is_expanded().then(|| {
                let items = untrack(|| nodes_view(&ctx, &kids, level + 1));
                view! { <ul class={class.clone()} role="group" {..attrs.clone()}>{items}</ul> }
            })
        }
    });
    let (click_core, dbl_core, key_core, focus_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let (click_key, dbl_key, key_key, focus_key) = (key.clone(), key.clone(), key.clone(), key.clone());
    let selected_attr = {
        let is_selected = is_selected.clone();
        move || matches!(mode, Some(TreeSelectionMode::Single | TreeSelectionMode::Multiple)).then(|| is_selected().to_string())
    };
    let checked_attr = {
        let is_selected = is_selected.clone();
        move || (mode == Some(TreeSelectionMode::Checkbox)).then(|| is_selected().to_string())
    };
    let expanded_attr = {
        let is_expanded = is_expanded.clone();
        move || (!leaf).then(|| is_expanded().to_string())
    };
    let mut li_attrs = pt.node.attrs();
    li_attrs.push(leptos::attr::custom::custom_attribute("aria-level", (level + 1).to_string()).into_any_attr());
    let content_attrs = pt.node_content.attrs();
    view! {
        <li
            class={li_class}
            style={node.style.clone()}
            role="treeitem"
            aria-label={node.label.clone()}
            aria-checked={checked_attr}
            aria-selected={selected_attr}
            aria-expanded={expanded_attr}
            aria-setsize={node.children.len().to_string()}
            aria-posinset={(index + 1).to_string()}
            tabindex={move || if tab_stop_key() { "0" } else { "-1" }}
            data-id={key.clone()}
            on:keydown={move |e| key_core.key_down(&e, &key_key)}
            on:focus={move |_| focus_core.focus.set(Some(focus_key.clone()))}
            {..li_attrs}
        >
            <div
                class={content_class}
                style={padding}
                on:click={move |e: web_sys::MouseEvent| click_core.select(&e, &click_key, e.ctrl_key() || e.meta_key())}
                on:dblclick={move |e: web_sys::MouseEvent| {
                    if let (Some(cb), Some(ev)) = (dbl_core.on_double_click, dbl_core.event(&e, &dbl_key)) {
                        cb.run(ev);
                    }
                }}
                {..content_attrs}
            >
                {toggle}
                {checkbox}
                {icon}
                {label}
            </div>
            {children}
        </li>
    }
    .into_any()
}

/// Кнопка раскрытия: шеврон или спиннер загружаемого узла.
fn toggle_view(ctx: &Ctx, node: &TreeNode, is_expanded: impl Fn() -> bool + Clone + Send + Sync + 'static) -> AnyView {
    let (core, pt) = (ctx.core.clone(), ctx.pt.clone());
    let key = node.key.clone();
    let loading = node.loading;
    let loading_mode = ctx.loading_mode.clone();
    let custom = ctx.toggler_icon.clone();
    let icon_class = pt.node_toggle_icon.merge_class("p-tree-node-toggle-icon").into_owned();
    let inner = move || -> AnyView {
        if let Some(custom) = custom.clone() {
            return view! { <span class={icon_class.clone()}>{custom.run()}</span> }.into_any();
        }
        if loading && loading_mode.get() == TreeLoadingMode::Icon {
            return view! { <SpinnerIcon spin=true class={icon_class.clone()} /> }.into_any();
        }
        if is_expanded() {
            view! { <ChevronDownIcon class={icon_class.clone()} /> }.into_any()
        } else {
            view! { <ChevronRightIcon class={icon_class.clone()} /> }.into_any()
        }
    };
    let visibility = (node.is_leaf() && !loading).then_some("visibility: hidden");
    view! {
        <button
            type="button"
            tabindex="-1"
            class={pt.node_toggle_button.merge_class("p-tree-node-toggle-button").into_owned()}
            style={visibility}
            on:click={move |e| core.toggle(e, &key)}
            {..pt.node_toggle_button.attrs()}
        >
            {inner}
        </button>
    }
    .into_any()
}

/// Значок узла: собственный либо раскрытый/свёрнутый.
fn icon_view(ctx: &Ctx, node: &TreeNode, is_expanded: impl Fn() -> bool + Clone + Send + Sync + 'static) -> Option<AnyView> {
    let class = ctx.pt.node_icon.merge_class("p-tree-node-icon").into_owned();
    if let Some(icon) = node.icon.clone() {
        return Some(view! { <span class={class}>{icon.run()}</span> }.into_any());
    }
    if node.expanded_icon.is_none() && node.collapsed_icon.is_none() {
        return None;
    }
    let (open, closed, has_children) = (node.expanded_icon.clone(), node.collapsed_icon.clone(), !node.children.is_empty());
    Some(
        view! {
            <span class={class}>
                {move || {
                    let pick = if is_expanded() && has_children { open.clone() } else { closed.clone() };
                    pick.map(|i| i.run())
                }}
            </span>
        }
        .into_any(),
    )
}

/// Флажок узла: разметка `Checkbox`, состояние из замыканий; частичный выбор — минус.
fn checkbox_view(
    ctx: &Ctx,
    checked: impl Fn() -> bool + Clone + Send + Sync + 'static,
    partial: impl Fn() -> bool + Clone + Send + Sync + 'static,
    disabled: bool,
) -> AnyView {
    let pt = &ctx.pt;
    let state = {
        let (checked, partial) = (checked.clone(), partial.clone());
        move || CheckboxState { checked: checked() || partial(), disabled, invalid: false, variant: None, size: None }
    };
    let root_class = {
        let (state, extra) = (state.clone(), pt.node_checkbox.merge_class("p-tree-node-checkbox").into_owned());
        move || {
            let mut cn = prime_core::ClassNames::new();
            prime_checkbox::write_classes(&mut cn, &state());
            format!("{} {extra}", cn.finish())
        }
    };
    let data = {
        let state = state.clone();
        move || prime_checkbox::data_p(&state())
    };
    let data_box = data.clone();
    let (a, b) = (checked.clone(), checked.clone());
    let p = partial.clone();
    let icon = {
        let (checked, partial) = (checked.clone(), partial.clone());
        move || -> AnyView {
            if partial() {
                view! { <MinusIcon class="p-checkbox-icon".to_owned() /> }.into_any()
            } else if checked() {
                view! { <CheckIcon class="p-checkbox-icon".to_owned() /> }.into_any()
            } else {
                ().into_any()
            }
        }
    };
    let is_checked = checked.clone();
    view! {
        <div
            class={root_class}
            data-p-highlight={move || a().to_string()}
            data-p-checked={move || b().to_string()}
            data-p-partialchecked={move || p().to_string()}
            data-p-disabled={disabled.to_string()}
            data-p={data}
            {..pt.node_checkbox.attrs()}
        >
            <input
                type="checkbox"
                class="p-checkbox-input"
                tabindex="-1"
                prop:checked={move || is_checked()}
                disabled={disabled}
                on:click={|e: web_sys::MouseEvent| e.prevent_default()}
            />
            <div class="p-checkbox-box" data-p={data_box}>{icon}</div>
        </div>
    }
    .into_any()
}
