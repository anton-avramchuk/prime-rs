//! Оболочка сайта документации: меню по разделам с поиском, страница компонента (описание, импорт, примеры с кодом,
//! справочник API, оформление, заметки) и оглавление страницы. Данные — `crate::generated` (`cargo xtask gen-docs`).

use crate::generated::{CompDoc, DemoDoc, PAGES, PageDoc, ParamDoc};
use leptos::prelude::*;
use leptos_router::components::A;

/// Разделы меню: название и компоненты (путь, подпись).
pub const MENU: &[(&str, &[(&str, &str)])] = &[
    (
        "Form",
        &[
            ("autocomplete", "AutoComplete"),
            ("cascadeselect", "CascadeSelect"),
            ("checkbox", "Checkbox"),
            ("colorpicker", "ColorPicker"),
            ("datepicker", "DatePicker"),
            ("floatlabel", "FloatLabel"),
            ("iftalabel", "IftaLabel"),
            ("iconfield", "IconField"),
            ("inputgroup", "InputGroup"),
            ("inputmask", "InputMask"),
            ("inputnumber", "InputNumber"),
            ("inputotp", "InputOtp"),
            ("inputtext", "InputText"),
            ("knob", "Knob"),
            ("listbox", "Listbox"),
            ("multiselect", "MultiSelect"),
            ("password", "Password"),
            ("radiobutton", "RadioButton"),
            ("rating", "Rating"),
            ("select", "Select"),
            ("selectbutton", "SelectButton"),
            ("slider", "Slider"),
            ("textarea", "Textarea"),
            ("togglebutton", "ToggleButton"),
            ("toggleswitch", "ToggleSwitch"),
            ("treeselect", "TreeSelect"),
        ],
    ),
    ("Button", &[("button", "Button"), ("buttongroup", "ButtonGroup"), ("speeddial", "SpeedDial"), ("splitbutton", "SplitButton")]),
    (
        "Data",
        &[
            ("dataview", "DataView"),
            ("orderlist", "OrderList"),
            ("paginator", "Paginator"),
            ("picklist", "PickList"),
            ("table", "Table"),
            ("timeline", "Timeline"),
            ("tree", "Tree"),
            ("treetable", "TreeTable"),
        ],
    ),
    (
        "Panel",
        &[
            ("accordion", "Accordion"),
            ("card", "Card"),
            ("divider", "Divider"),
            ("fieldset", "Fieldset"),
            ("panel", "Panel"),
            ("scrollpanel", "ScrollPanel"),
            ("splitter", "Splitter"),
            ("stepper", "Stepper"),
            ("tabs", "Tabs"),
            ("toolbar", "Toolbar"),
        ],
    ),
    (
        "Overlay",
        &[
            ("confirmdialog", "ConfirmDialog"),
            ("confirmpopup", "ConfirmPopup"),
            ("dialog", "Dialog"),
            ("drawer", "Drawer"),
            ("popover", "Popover"),
            ("tooltip", "Tooltip"),
        ],
    ),
    ("File", &[("fileupload", "FileUpload")]),
    (
        "Menu",
        &[
            ("breadcrumb", "Breadcrumb"),
            ("contextmenu", "ContextMenu"),
            ("dock", "Dock"),
            ("megamenu", "MegaMenu"),
            ("menu", "Menu"),
            ("menubar", "Menubar"),
            ("panelmenu", "PanelMenu"),
            ("steps", "Steps"),
            ("tieredmenu", "TieredMenu"),
        ],
    ),
    ("Messages", &[("message", "Message"), ("toast", "Toast")]),
    ("Media", &[("carousel", "Carousel"), ("galleria", "Galleria"), ("image", "Image"), ("imagecompare", "ImageCompare")]),
    (
        "Misc",
        &[
            ("avatar", "Avatar"),
            ("badge", "Badge"),
            ("blockui", "BlockUI"),
            ("chip", "Chip"),
            ("fluid", "Fluid"),
            ("inplace", "Inplace"),
            ("metergroup", "MeterGroup"),
            ("overlaybadge", "OverlayBadge"),
            ("progressbar", "ProgressBar"),
            ("progressspinner", "ProgressSpinner"),
            ("scrolltop", "ScrollTop"),
            ("skeleton", "Skeleton"),
            ("tag", "Tag"),
        ],
    ),
];

/// Имя страницы компонента: `Demo` по нему находит код своего примера.
#[derive(Clone, Copy)]
pub struct PageName(pub &'static str);

/// Данные страницы по имени.
pub fn page_doc(name: &str) -> Option<&'static PageDoc> {
    PAGES.iter().find(|p| p.name == name)
}

/// Код примера страницы.
pub fn demo_doc(page: &str, demo: &str) -> Option<&'static DemoDoc> {
    page_doc(page)?.demos.iter().find(|d| d.name == demo)
}

fn label_of(name: &str) -> &'static str {
    MENU.iter().flat_map(|(_, items)| items.iter()).find(|(path, _)| *path == name).map_or("", |(_, label)| *label)
}

type Link = (&'static str, &'static str);

/// Предыдущий и следующий компонент в порядке меню.
pub fn neighbours(name: &str) -> (Option<Link>, Option<Link>) {
    let flat: Vec<Link> = MENU.iter().flat_map(|(_, items)| items.iter().copied()).collect();
    let Some(at) = flat.iter().position(|(path, _)| *path == name) else { return (None, None) };
    (at.checked_sub(1).map(|i| flat[i]), flat.get(at + 1).copied())
}

/// Стили оболочки документации.
pub const DOC_CSS: &str = "\
.doc-shell{display:grid;grid-template-columns:15rem minmax(0,1fr);gap:2rem;align-items:start;margin-top:1rem}\
.doc-nav{position:sticky;top:1rem;max-height:calc(100vh - 2rem);overflow:auto;font-size:.9rem;padding-right:.5rem}\
.doc-nav input{width:100%;box-sizing:border-box;margin-bottom:.5rem}\
.doc-nav a{display:block;padding:.25rem .5rem;border-radius:6px;color:var(--p-text-color);text-decoration:none}\
.doc-nav a:hover{background:var(--p-content-hover-background)}\
.doc-nav a[aria-current=page]{background:var(--p-highlight-background);color:var(--p-highlight-color);font-weight:600}\
.doc-nav-title{margin:.75rem .5rem .25rem;font-weight:600;font-size:.75rem;letter-spacing:.04em;text-transform:uppercase;color:var(--p-text-muted-color)}\
.doc-page{display:grid;grid-template-columns:minmax(0,1fr) 11rem;gap:2rem;align-items:start}\
.doc-page h2{margin:0 0 8px;font-size:32px;line-height:40px}\
.doc-desc{margin:0 0 24px;line-height:24px;color:var(--p-text-muted-color)}\
.doc-toc{position:sticky;top:1rem;font-size:.85rem;border-left:1px solid var(--p-content-border-color);padding-left:1rem}\
.doc-toc b{display:block;margin-bottom:.5rem;font-size:.75rem;letter-spacing:.04em;text-transform:uppercase;color:var(--p-text-muted-color)}\
.doc-toc a{display:block;padding:.2rem 0;color:var(--p-text-muted-color);text-decoration:none}\
.doc-toc a:hover{color:var(--p-text-color)}\
.doc-code{margin:-.25rem 0 1rem}\
.doc-code summary{cursor:pointer;width:fit-content;padding:.35rem .75rem;border:1px solid var(--p-content-border-color);border-radius:6px;font-size:.85rem;user-select:none}\
.doc-code summary:hover{background:var(--p-content-hover-background)}\
.doc-code pre,.doc-import pre{overflow:auto;margin:.5rem 0 0;padding:1rem;border-radius:10px;background:var(--p-content-hover-background);font:13px/20px ui-monospace,Menlo,Consolas,monospace}\
.doc-import pre{margin:0}\
.doc-table,.doc-notes table{width:100%;border-collapse:collapse;font-size:.88rem;margin:.5rem 0 1.5rem}\
.doc-table th,.doc-table td,.doc-notes th,.doc-notes td{padding:.5rem .75rem;text-align:left;vertical-align:top;border-bottom:1px solid var(--p-content-border-color)}\
.doc-table th,.doc-notes th{color:var(--p-text-muted-color);font-weight:600}\
.doc-table code,.doc-notes code,.doc-desc code{font:.85em ui-monospace,Menlo,Consolas,monospace;background:var(--p-content-hover-background);padding:.1rem .3rem;border-radius:4px}\
.doc-table td:first-child{white-space:nowrap;font-weight:600}\
.doc-notes pre code{background:none;padding:0}\
.doc-pager{display:flex;justify-content:space-between;margin:2rem 0}\
.doc-pager a{color:var(--p-primary-color);text-decoration:none}\
[data-testid=app]>header{display:flex;align-items:center;justify-content:space-between;padding-bottom:16px;border-bottom:1px solid var(--p-content-border-color)}\n[data-testid=app]>header h1{margin:0;font-size:24px;line-height:32px;letter-spacing:-.02em;color:var(--p-primary-color)}\n[data-testid=app]>header h1::after{content:' / docs';font-weight:400;color:var(--p-text-muted-color)}\n[data-testid=app]>header button{cursor:pointer;padding:8px 14px;font:inherit;font-size:14px;color:var(--p-text-color);background:transparent;border:1px solid var(--p-content-border-color);border-radius:8px}\n[data-testid=app]>header button:hover{background:var(--p-content-hover-background)}\n.doc-page h3{font-size:20px;line-height:28px}\n.doc-page h4{margin:20px 0 4px;font-size:16px;line-height:24px}\n.doc-nav a{transition:background .15s}\n.doc-nav a[aria-current=page]{box-shadow:inset 3px 0 0 var(--p-primary-color);border-radius:0 6px 6px 0}\n.doc-toc a{border-left:2px solid transparent;margin-left:-17px;padding-left:15px}\n.doc-toc a:hover{border-left-color:var(--p-primary-color)}\n[data-demo]{box-shadow:0 1px 2px rgba(0,0,0,.04)}\n.doc-code summary{color:var(--p-primary-color);border-color:var(--p-primary-color);background:transparent;font-weight:600}\n.doc-code summary:hover{background:var(--p-highlight-background)}\n.doc-table tbody tr:hover,.doc-notes tbody tr:hover{background:var(--p-content-hover-background)}\n.doc-table td:first-child{color:var(--p-primary-color)}\n.doc-pager a{padding:8px 12px;border:1px solid var(--p-content-border-color);border-radius:8px}\n.doc-pager a:hover{background:var(--p-content-hover-background)}\n.tk-k{color:#7c3aed}.tk-s{color:#15803d}.tk-c{color:#6b7280}.tk-t{color:#0369a1}.tk-y{color:#b45309}.tk-m{color:#be185d}.tk-n{color:#b91c1c}\
.p-dark .tk-k{color:#c4b5fd}.p-dark .tk-s{color:#86efac}.p-dark .tk-c{color:#9ca3af}.p-dark .tk-t{color:#7dd3fc}.p-dark .tk-y{color:#fcd34d}.p-dark .tk-m{color:#f9a8d4}.p-dark .tk-n{color:#fca5a5}\
@media (max-width:1100px){.doc-page{grid-template-columns:minmax(0,1fr)}.doc-toc{display:none}}\
@media (max-width:800px){.doc-shell{grid-template-columns:minmax(0,1fr)}.doc-nav{position:static;max-height:14rem}}";

/// Меню по разделам с поиском по подстроке.
#[component]
pub fn DocNav() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let visible = move |label: &'static str| {
        let q = query.get().trim().to_lowercase();
        q.is_empty() || label.to_lowercase().contains(&q)
    };
    let sections = MENU
        .iter()
        .map(|(title, items)| {
            let any = move || items.iter().any(|(_, label)| visible(label));
            let links = items
                .iter()
                .map(|(path, label)| {
                    let href = format!("/{path}");
                    view! {
                        <div style:display=move || if visible(label) { "block" } else { "none" }>
                            <A href=href>{*label}</A>
                        </div>
                    }
                })
                .collect_view();
            view! {
                <div style:display=move || if any() { "block" } else { "none" }>
                    <div class="doc-nav-title">{*title}</div>
                    {links}
                </div>
            }
        })
        .collect_view();
    view! {
        <nav class="doc-nav" data-testid="nav" aria-label="Components">
            <input
                type="search"
                class="p-inputtext p-component"
                placeholder="Search"
                aria-label="Search components"
                data-testid="nav-search"
                prop:value=move || query.get()
                on:input=move |e| query.set(event_target_value(&e))
            />
            <A href="/">"Home"</A>
            {sections}
        </nav>
    }
}

fn params_table(comp: &'static CompDoc, kind: &'static str) -> Option<impl IntoView + use<>> {
    let rows: Vec<&'static ParamDoc> = comp.params.iter().filter(|p| p.kind == kind).collect();
    if rows.is_empty() {
        return None;
    }
    let body = rows
        .into_iter()
        .map(|p| {
            let default = if p.default.is_empty() { "-".to_owned() } else { p.default.to_owned() };
            let name = if p.required { format!("{} *", p.name) } else { p.name.to_owned() };
            view! {
                <tr>
                    <td>{name}</td>
                    <td><code>{p.ty}</code></td>
                    <td>{default}</td>
                    <td>{p.doc}</td>
                </tr>
            }
        })
        .collect_view();
    Some(view! {
        <table class="doc-table">
            <thead><tr><th>"Name"</th><th>"Type"</th><th>"Default"</th><th>"Description"</th></tr></thead>
            <tbody>{body}</tbody>
        </table>
    })
}

/// Секция со свойствами или событиями: по заголовку на компонент крейта.
fn api_section(page: &'static PageDoc, id: &'static str, title: &'static str, kind: &'static str) -> Option<impl IntoView + use<>> {
    let tables: Vec<_> = page.comps.iter().filter_map(|c| params_table(c, kind).map(|t| view! { <h4>{c.name}</h4> {t} })).collect();
    (!tables.is_empty()).then(|| view! { <section><h3 id=id>{title}</h3>{tables}</section> })
}

fn pt_section(page: &'static PageDoc) -> Option<impl IntoView + use<>> {
    if page.pt.is_empty() {
        return None;
    }
    let blocks = page
        .pt
        .iter()
        .map(|s| {
            let rows = s.fields.iter().map(|(n, d)| view! { <tr><td>{*n}</td><td>{*d}</td></tr> }).collect_view();
            view! {
                <h4>{s.name}</h4>
                <table class="doc-table">
                    <thead><tr><th>"Part"</th><th>"Description"</th></tr></thead>
                    <tbody>{rows}</tbody>
                </table>
            }
        })
        .collect_view();
    Some(view! { <section><h3 id="pt">"Pass Through"</h3>{blocks}</section> })
}

fn theming_section(page: &'static PageDoc) -> Option<impl IntoView + use<>> {
    if page.classes.is_empty() {
        return None;
    }
    let rows = page.classes.iter().map(|c| view! { <tr><td><code>{format!(".{c}")}</code></td></tr> }).collect_view();
    Some(view! {
        <section>
            <h3 id="theming">"Theming"</h3>
            <table class="doc-table">
                <thead><tr><th>"CSS class"</th></tr></thead>
                <tbody>{rows}</tbody>
            </table>
        </section>
    })
}

fn toc_entries(page: Option<&'static PageDoc>) -> Vec<Link> {
    let mut toc: Vec<Link> = Vec::new();
    let Some(p) = page else { return toc };
    if !p.imports.is_empty() {
        toc.push(("import", "Import"));
    }
    toc.extend(p.demos.iter().map(|d| (d.name, d.title)));
    let has = |kind: &str| p.comps.iter().any(|c| c.params.iter().any(|a| a.kind == kind));
    for (id, title, shown) in [
        ("props", "Props", has("prop")),
        ("events", "Events", has("event")),
        ("pt", "Pass Through", !p.pt.is_empty()),
        ("theming", "Theming", !p.classes.is_empty()),
        ("accessibility", "Accessibility", !p.a11y.is_empty()),
        ("notes", "Notes", !p.notes.is_empty()),
    ] {
        if shown {
            toc.push((id, title));
        }
    }
    toc
}

/// Страница компонента: оболочка вокруг демонстраций (`children`).
#[component]
pub fn DocPage(
    /// Имя страницы (как в маршруте).
    name: &'static str,
    children: Children,
) -> impl IntoView {
    provide_context(PageName(name));
    let page = page_doc(name);
    let (prev, next) = neighbours(name);
    let toc_links = toc_entries(page).into_iter().map(|(id, title)| view! { <a href=format!("#{id}")>{title}</a> }).collect_view();
    let imports = page.filter(|p| !p.imports.is_empty()).map(|p| {
        view! {
            <section class="doc-import">
                <h3 id="import">"Import"</h3>
                <pre><code>{p.imports}</code></pre>
            </section>
        }
    });
    let api = page.map(|p| {
        let a11y = (!p.a11y.is_empty()).then(
            || view! { <section class="doc-notes"><h3 id="accessibility">"Accessibility"</h3><div inner_html=p.a11y></div></section> },
        );
        let notes = (!p.notes.is_empty())
            .then(|| view! { <section class="doc-notes"><h3 id="notes">"Notes"</h3><div inner_html=p.notes></div></section> });
        view! {
            {api_section(p, "props", "Props", "prop")}
            {api_section(p, "events", "Events", "event")}
            {pt_section(p)}
            {theming_section(p)}
            {a11y}
            {notes}
        }
    });
    let pager = view! {
        <div class="doc-pager">
            <span>{prev.map(|(path, label)| view! { <A href=format!("/{path}")>{format!("← {label}")}</A> })}</span>
            <span>{next.map(|(path, label)| view! { <A href=format!("/{path}")>{format!("{label} →")}</A> })}</span>
        </div>
    };
    view! {
        <div class="doc-page">
            <article>
                <h2>{label_of(name)}</h2>
                <p class="doc-desc">{page.map(|p| p.description).unwrap_or_default()}</p>
                {imports}
                {children()}
                {api}
                {pager}
            </article>
            <aside class="doc-toc" aria-label="On this page">
                <b>"On this page"</b>
                {toc_links}
            </aside>
        </div>
    }
}

/// Блок «Code» под примером; страницу берёт из контекста `DocPage`.
#[component]
pub fn DemoCode(
    /// Имя примера.
    demo: &'static str,
) -> impl IntoView {
    let page = use_context::<PageName>()?.0;
    demo_doc(page, demo).filter(|d| !d.code.is_empty()).map(|d| {
        view! {
            <details class="doc-code" data-testid=format!("code-{demo}")>
                <summary>"Code"</summary>
                <pre><code inner_html=d.html></code></pre>
            </details>
        }
    })
}
