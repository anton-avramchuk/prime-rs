//! Разметка FileUpload (SSR): режимы, кнопки и их состояния, поле выбора, подписи, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::Pt;
use prime_fileupload::{FileUpload, FileUploadMode, FileUploadPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

/// Открывающий тег элемента, в котором встречается класс.
fn tag_of<'a>(html: &'a str, class: &str) -> &'a str {
    let at = html.find(class).unwrap_or_else(|| panic!("нет {class}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

#[test]
fn advanced_markup() {
    let html = render(|| view! { <FileUpload name="demo[]" url="/upload" multiple=true accept="image/*" /> }.to_html());
    assert!(html.contains("p-fileupload p-fileupload-advanced p-component"), "{html}");
    assert!(html.contains("type=\"file\"") && html.contains("multiple") && html.contains("accept=\"image/*\""), "{html}");
    assert!(html.contains("aria-label=\"Browse Files\""), "{html}");
    assert!(html.contains("p-fileupload-header") && html.contains("p-fileupload-content"), "{html}");
    for button in ["p-fileupload-choose-button", "p-fileupload-upload-button", "p-fileupload-cancel-button"] {
        assert!(html.contains(button), "нет {button}: {html}");
    }
    for label in ["Choose", "Upload", "Cancel"] {
        assert!(html.contains(label), "нет метки {label}: {html}");
    }
    assert!(!html.contains("p-fileupload-file-list") && !html.contains("p-progressbar"), "пустой: {html}");
}

#[test]
fn upload_and_cancel_are_disabled_without_files() {
    let html = render(|| view! { <FileUpload url="/upload" /> }.to_html());
    assert!(tag_of(&html, "p-fileupload-upload-button").contains("disabled"), "{html}");
    assert!(tag_of(&html, "p-fileupload-cancel-button").contains("disabled"), "{html}");
    assert!(!tag_of(&html, "p-fileupload-choose-button").contains("disabled"), "выбор доступен: {html}");
}

#[test]
fn auto_hides_upload_and_cancel() {
    let html = render(|| view! { <FileUpload url="/upload" auto=true /> }.to_html());
    assert!(html.contains("p-fileupload-choose-button"), "{html}");
    assert!(!html.contains("p-fileupload-upload-button") && !html.contains("p-fileupload-cancel-button"), "{html}");
}

#[test]
fn buttons_can_be_hidden_and_labelled() {
    let html = render(|| {
        view! { <FileUpload show_upload_button=false show_cancel_button=false choose_label="Pick" choose_style_class="mine" /> }.to_html()
    });
    assert!(!html.contains("p-fileupload-upload-button") && !html.contains("p-fileupload-cancel-button"), "{html}");
    assert!(html.contains("Pick") && html.contains("p-fileupload-choose-button mine"), "{html}");
}

#[test]
fn disabled_disables_choose_and_input() {
    let html = render(|| view! { <FileUpload disabled=true /> }.to_html());
    assert!(tag_of(&html, "p-fileupload-choose-button").contains("disabled"), "{html}");
    assert!(tag_of(&html, "type=\"file\"").contains("disabled"), "{html}");
}

#[test]
fn basic_markup_and_label() {
    let html = render(|| view! { <FileUpload mode=FileUploadMode::Basic url="/upload" /> }.to_html());
    assert!(html.contains("p-fileupload p-fileupload-basic p-component") && html.contains("p-fileupload-basic-content"), "{html}");
    assert!(html.contains("p-fileupload-choose-button") && html.contains("No file chosen"), "{html}");
    assert!(!html.contains("p-fileupload-header") && !html.contains("p-fileupload-upload-button"), "{html}");
    let html = render(|| view! { <FileUpload mode=FileUploadMode::Basic auto=true /> }.to_html());
    assert!(!html.contains("No file chosen"), "при auto подписи нет: {html}");
}

#[test]
fn empty_and_header_slots() {
    let html = render(|| {
        view! {
            <FileUpload
                header=ViewFn::from(|| view! { <i>"Extra"</i> })
                empty=ViewFn::from(|| view! { <p>"Drag and drop files here."</p> })
            />
        }
        .to_html()
    });
    assert!(html.contains("<i>Extra</i>") && html.contains("Drag and drop files here."), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = FileUploadPt { root: Pt::new().class("root-x"), header: Pt::new().class("header-x"), ..Default::default() };
    let html = render(|| view! { <FileUpload style="width: 20rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("width: 20rem"), "{html}");
    assert!(html.contains("p-fileupload-header header-x"), "{html}");
}
