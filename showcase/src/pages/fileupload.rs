//! Страница FileUpload: демонстрации `basic`, `advanced`, `auto` и `custom` из showcase PrimeNG
//! (`apps/showcase/doc/fileupload`; формы Angular и шаблоны не переносятся).
//! Сервера загрузки у showcase нет: запрос на `/api/upload` получает ошибку, что видно по строке состояния.

use super::Demo;
use leptos::prelude::*;
use prime_fileupload::{FileUpload, FileUploadMode, FileUploadRef, UploadError, UploadEvent};

fn status_line(id: &'static str, status: RwSignal<String>) -> impl IntoView {
    view! { <p data-testid=id style="margin: 0.5rem 0 0">{move || status.get()}</p> }
}

#[component]
fn Basic() -> impl IntoView {
    let status = RwSignal::new(String::new());
    view! {
        <div id="basic_upload">
            <FileUpload
                mode=FileUploadMode::Basic
                name="demo[]"
                url="/api/upload"
                accept="image/*"
                max_file_size=1_000_000
                auto=true
                choose_label="Browse"
                on_upload=Callback::new(move |e: UploadEvent| status.set(format!("uploaded {} ({})", e.files.len(), e.status)))
                on_error=Callback::new(move |e: UploadError| status.set(format!("error {}", e.status.map_or("network".to_owned(), |s| s.to_string()))))
            />
            {status_line("basic-status", status)}
        </div>
    }
}

#[component]
fn Advanced() -> impl IntoView {
    let status = RwSignal::new(String::new());
    view! {
        <div id="advanced_upload">
            <FileUpload
                name="demo[]"
                url="/api/upload"
                multiple=true
                accept="image/*"
                max_file_size=1_000_000
                empty=ViewFn::from(|| view! { <p style="margin: 0">"Drag and drop files to here to upload."</p> })
                on_upload=Callback::new(move |e: UploadEvent| status.set(format!("uploaded {} ({})", e.files.len(), e.status)))
                on_error=Callback::new(move |e: UploadError| status.set(format!("error {}", e.status.map_or("network".to_owned(), |s| s.to_string()))))
            />
            {status_line("advanced-status", status)}
        </div>
    }
}

#[component]
fn Auto() -> impl IntoView {
    let status = RwSignal::new(String::new());
    view! {
        <div id="auto_upload">
            <FileUpload
                name="demo[]"
                url="/api/upload"
                auto=true
                multiple=true
                accept="image/*"
                max_file_size=1_000_000
                choose_label="Browse"
                on_upload=Callback::new(move |e: UploadEvent| status.set(format!("uploaded {} ({})", e.files.len(), e.status)))
                on_error=Callback::new(move |e: UploadError| status.set(format!("error {}", e.status.map_or("network".to_owned(), |s| s.to_string()))))
            />
            {status_line("auto-status", status)}
        </div>
    }
}

#[component]
fn Custom() -> impl IntoView {
    let status = RwSignal::new(String::new());
    let file_ref = FileUploadRef::new();
    view! {
        <div id="custom_upload">
            <FileUpload
                file_upload_ref=file_ref
                multiple=true
                accept="image/*"
                max_file_size=1_000_000
                custom_upload=true
                upload_label="Custom Upload"
                on_upload_handler=Callback::new(move |files: Vec<web_sys::File>| {
                    status.set(format!("custom {}", files.len()));
                    file_ref.clear();
                })
            />
            {status_line("custom-status", status)}
        </div>
    }
}

/// Страница `/fileupload`.
#[component]
pub fn FileUploadPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="advanced" title="Advanced">
            <Advanced />
        </Demo>
        <Demo name="auto" title="Auto">
            <Auto />
        </Demo>
        <Demo name="custom" title="Custom Upload">
            <Custom />
        </Demo>
    }
}
