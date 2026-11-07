//! Страница TreeTable (этап 1): демонстрации `basic`, `controlled`, `sortsinglecolumn`, `paginatorbasic`,
//! `selectionsingle`, `selectionmultiple`, `selectioncheckbox`, `gridlines`/`size` и `loadingmask` из showcase PrimeNG
//! (`apps/showcase/doc/treetable`; фильтры, правка, изменение и перестановка столбцов, заморозка и ленивая загрузка —
//! следующие этапы).

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_core::Size;
use prime_table::Column;
use prime_treetable::{TreeNode, TreeSelectionMode, TreeTable};

#[derive(Clone, Debug)]
struct File {
    name: &'static str,
    size: &'static str,
    kind: &'static str,
}

fn file(name: &'static str, size: &'static str, kind: &'static str) -> File {
    File { name, size, kind }
}

fn nodes() -> Vec<TreeNode<File>> {
    vec![
        TreeNode::new("0", file("Applications", "100kb", "Folder")).children(vec![
            TreeNode::new("0-0", file("React", "25kb", "Folder"))
                .children(vec![TreeNode::new("0-0-0", file("react.app", "10kb", "Application"))]),
            TreeNode::new("0-1", file("editor.app", "25kb", "Application")),
            TreeNode::new("0-2", file("settings.app", "50kb", "Application")),
        ]),
        TreeNode::new("1", file("Cloud", "20kb", "Folder")).children(vec![
            TreeNode::new("1-0", file("backup-1.zip", "10kb", "Zip")),
            TreeNode::new("1-1", file("backup-2.zip", "10kb", "Zip")),
        ]),
        TreeNode::new("2", file("Desktop", "150kb", "Folder")).children(vec![
            TreeNode::new("2-0", file("note-meeting.txt", "50kb", "Text")),
            TreeNode::new("2-1", file("note-todo.txt", "100kb", "Text")),
        ]),
        TreeNode::new("3", file("Documents", "75kb", "Folder")).children(vec![TreeNode::new("3-0", file("Work", "55kb", "Folder"))]),
        TreeNode::new("4", file("Downloads", "25kb", "Folder")),
    ]
}

fn columns() -> Vec<Column<File>> {
    vec![
        Column::text("name", "Name", |f: &File| f.name.to_owned()),
        Column::text("size", "Size", |f: &File| f.size.to_owned()),
        Column::text("kind", "Type", |f: &File| f.kind.to_owned()),
    ]
}

fn sortable_columns() -> Vec<Column<File>> {
    vec![
        Column::text("name", "Name", |f: &File| f.name.to_owned()).sortable(|a, b| a.name.cmp(b.name)),
        Column::text("size", "Size", |f: &File| f.size.to_owned()).sortable(|a, b| {
            let kb = |f: &File| f.size.trim_end_matches("kb").parse::<u32>().unwrap_or(0);
            kb(a).cmp(&kb(b))
        }),
        Column::text("kind", "Type", |f: &File| f.kind.to_owned()).sortable(|a, b| a.kind.cmp(b.kind)),
    ]
}

const TABLE_STYLE: &str = "min-width: 40rem";

#[component]
fn Basic() -> impl IntoView {
    view! {
        <div id="basic_treetable">
            <TreeTable value=nodes() columns=columns() table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn Controlled() -> impl IntoView {
    let expanded = RwSignal::new(vec!["0".to_owned()]);
    let all = ["0", "0-0", "1", "2", "3", "3-0"].map(str::to_owned).to_vec();
    view! {
        <div id="controlled_treetable">
            <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem">
                <Button label="Expand all" on_click=Callback::new(move |_| expanded.set(all.clone())) />
                <Button label="Collapse all" on_click=Callback::new(move |_| expanded.set(Vec::new())) />
            </div>
            <TreeTable value=nodes() columns=columns() expanded_keys=expanded table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn Sorting() -> impl IntoView {
    view! {
        <div id="sort_treetable">
            <TreeTable value=nodes() columns=sortable_columns() table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn Paginated() -> impl IntoView {
    view! {
        <div id="paginator_treetable">
            <TreeTable value=nodes() columns=sortable_columns() paginator=true default_rows=2usize table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn SingleSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    view! {
        <div id="single_treetable">
            <TreeTable value=nodes() columns=columns() selection_mode=Some(TreeSelectionMode::Single) selection=selection table_style=TABLE_STYLE />
            <p data-testid="single-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn MultipleSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    view! {
        <div id="multiple_treetable">
            <TreeTable
                value=nodes()
                columns=columns()
                selection_mode=Some(TreeSelectionMode::Multiple)
                meta_key_selection=true
                selection=selection
                table_style=TABLE_STYLE
            />
            <p data-testid="multiple-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn CheckboxSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    let expanded = RwSignal::new(vec!["0".to_owned(), "0-0".to_owned()]);
    view! {
        <div id="checkbox_treetable">
            <TreeTable
                value=nodes()
                columns=columns()
                selection_mode=Some(TreeSelectionMode::Checkbox)
                selection=selection
                expanded_keys=expanded
                table_style=TABLE_STYLE
            />
            <p data-testid="checkbox-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn Looks() -> impl IntoView {
    view! {
        <div id="looks_treetable" style="display: flex; flex-direction: column; gap: 1.5rem">
            <TreeTable value=nodes() columns=columns() show_gridlines=true table_style=TABLE_STYLE />
            <TreeTable value=nodes() columns=columns() size=Size::Small table_style=TABLE_STYLE />
            <TreeTable value=nodes() columns=columns() size=Size::Large table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! {
        <div id="loading_treetable">
            <TreeTable value=nodes() columns=columns() loading=true table_style=TABLE_STYLE />
        </div>
    }
}

/// Страница `/treetable`.
#[component]
pub fn TreeTablePage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="controlled" title="Controlled">
            <Controlled />
        </Demo>
        <Demo name="sortsinglecolumn" title="Sort">
            <Sorting />
        </Demo>
        <Demo name="paginatorbasic" title="Pagination">
            <Paginated />
        </Demo>
        <Demo name="selectionsingle" title="Single Selection">
            <SingleSelection />
        </Demo>
        <Demo name="selectionmultiple" title="Multiple Selection">
            <MultipleSelection />
        </Demo>
        <Demo name="selectioncheckbox" title="Checkbox Selection">
            <CheckboxSelection />
        </Demo>
        <Demo name="looks" title="Gridlines and Size">
            <Looks />
        </Demo>
        <Demo name="loadingmask" title="Loading Mask">
            <Loading />
        </Demo>
    }
}
