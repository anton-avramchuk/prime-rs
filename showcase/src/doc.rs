//! Docs site shell: a menu by section with search, a component page (description, import, examples with code,
//! API reference, styling, notes) and the page table of contents. Data comes from `crate::generated` (`cargo xtask gen-docs`).

use crate::generated::{CompDoc, DemoDoc, PAGES, PageDoc, ParamDoc};
use crate::highlight::highlight;
use leptos::prelude::*;
use leptos_router::components::A;

/// Menu sections: name and components (path, label).
pub const MENU: &[(&str, &[(&str, &str)])] = &[
    ("Start", &[("install", "Install")]),
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

/// Component page name: `Demo` uses it to find the code of its example.
#[derive(Clone, Copy)]
pub struct PageName(pub &'static str);

/// Page data by name.
pub fn page_doc(name: &str) -> Option<&'static PageDoc> {
    PAGES.iter().find(|p| p.name == name)
}

/// Page example code.
pub fn demo_doc(page: &str, demo: &str) -> Option<&'static DemoDoc> {
    page_doc(page)?.demos.iter().find(|d| d.name == demo)
}

fn label_of(name: &str) -> &'static str {
    MENU.iter().flat_map(|(_, items)| items.iter()).find(|(path, _)| *path == name).map_or("", |(_, label)| *label)
}

type Link = (&'static str, &'static str);

/// Previous and next component in menu order.
pub fn neighbours(name: &str) -> (Option<Link>, Option<Link>) {
    let flat: Vec<Link> = MENU.iter().flat_map(|(_, items)| items.iter().copied()).collect();
    let Some(at) = flat.iter().position(|(path, _)| *path == name) else { return (None, None) };
    (at.checked_sub(1).map(|i| flat[i]), flat.get(at + 1).copied())
}

/// Styles of the Home and Install pages.
pub const SITE_CSS: &str = include_str!("site.css");

/// Docs shell styles.
pub const DOC_CSS: &str = include_str!("doc.css");

/// Menu by section with substring search.
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
                    let href = crate::app::route(&format!("/{path}"));
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
            <A href=crate::app::route("/") exact=true>"Home"</A>
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

/// Section with properties or events: one heading per crate component.
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

/// Component page: a shell around the demos (`children`).
#[component]
pub fn DocPage(
    /// Page name (as in the route).
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
            <span>{prev.map(|(path, label)| view! { <A href=crate::app::route(&format!("/{path}"))>{format!("← {label}")}</A> })}</span>
            <span>{next.map(|(path, label)| view! { <A href=crate::app::route(&format!("/{path}"))>{format!("{label} →")}</A> })}</span>
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

/// The "Code" block under an example; takes the page from the `DocPage` context.
#[component]
pub fn DemoCode(
    /// Example name.
    demo: &'static str,
) -> impl IntoView {
    let page = use_context::<PageName>()?.0;
    demo_doc(page, demo).filter(|d| !d.code.is_empty()).map(|d| {
        view! {
            <details class="doc-code" data-testid=format!("code-{demo}")>
                <summary>"Code"</summary>
                <pre><code inner_html=highlight(d.code)></code></pre>
            </details>
        }
    })
}
