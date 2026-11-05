//! Поведение FileUpload в реальном браузере (wasm-bindgen-test): выбор, проверки, лимит, удаление, очистка, перетаскивание,
//! собственная загрузка, отправка по сети, режим `basic`. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_fileupload::{FileUpload, FileUploadMode, FileUploadRef};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{DataTransfer, DragEvent, DragEventInit, Event, EventInit, File, FilePropertyBag, HtmlElement, HtmlInputElement};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    sleep(40).await;
}

fn q(root: &HtmlElement, selector: &str) -> Option<HtmlElement> {
    root.query_selector(selector).unwrap().map(|e| e.dyn_into().unwrap())
}

fn all(root: &HtmlElement, selector: &str) -> Vec<HtmlElement> {
    let list = root.query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn input(host: &HtmlElement) -> HtmlInputElement {
    q(host, "input[type=file]").expect("нет поля выбора").dyn_into().unwrap()
}

fn make_file(name: &str, kind: &str, content: &str) -> File {
    let parts = js_sys::Array::of1(&content.into());
    let bag = FilePropertyBag::new();
    bag.set_type(kind);
    File::new_with_str_sequence_and_options(&parts, name, &bag).unwrap()
}

fn data_transfer(files: &[File]) -> DataTransfer {
    let dt = DataTransfer::new().unwrap();
    for f in files {
        dt.items().add_with_file(f).unwrap();
    }
    dt
}

/// Подставляет файлы в поле и сообщает об изменении.
fn choose(host: &HtmlElement, files: &[File]) {
    let input = input(host);
    input.set_files(data_transfer(files).files().as_ref());
    let init = EventInit::new();
    init.set_bubbles(true);
    input.dispatch_event(&Event::new_with_event_init_dict("change", &init).unwrap()).unwrap();
}

fn drag(target: &HtmlElement, kind: &str, files: &[File]) {
    let init = DragEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_data_transfer(Some(&data_transfer(files)));
    target.dispatch_event(&DragEvent::new_with_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn names(host: &HtmlElement) -> Vec<String> {
    all(host, ".p-fileupload-file-name").iter().map(|e| e.text_content().unwrap_or_default()).collect()
}

fn button(host: &HtmlElement, class: &str) -> HtmlElement {
    q(host, class).unwrap_or_else(|| panic!("нет {class}: {}", host.outer_html()))
}

fn disabled(el: &HtmlElement) -> bool {
    el.has_attribute("disabled")
}

fn texts(host: &HtmlElement, selector: &str) -> String {
    all(host, selector).iter().map(|e| e.text_content().unwrap_or_default()).collect::<Vec<_>>().join("|")
}

async fn selecting_and_removing() {
    let (host, log) = (host(), Log::default());
    let (l_sel, l_rm, l_clear) = (log.clone(), log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <FileUpload
                multiple=true
                url="/nowhere"
                name="files[]"
                on_select=Callback::new(move |files: Vec<File>| l_sel.lock().unwrap().push(format!("select:{}", files.len())))
                on_remove=Callback::new(move |f: File| l_rm.lock().unwrap().push(format!("remove:{}", f.name())))
                on_clear=Callback::new(move |()| l_clear.lock().unwrap().push("clear".into()))
            />
        }
    });
    settle().await;
    assert!(q(&host, ".p-fileupload-file-list").is_none());
    assert!(disabled(&button(&host, ".p-fileupload-upload-button")) && disabled(&button(&host, ".p-fileupload-cancel-button")));

    choose(&host, &[make_file("a.txt", "text/plain", "hello"), make_file("b.png", "image/png", "xx")]);
    settle().await;
    assert_eq!(names(&host), ["a.txt", "b.png"]);
    assert_eq!(texts(&host, ".p-fileupload-file-size"), "5.000 B|2.000 B");
    assert_eq!(texts(&host, ".p-fileupload-file-badge"), "Pending|Pending");
    assert_eq!(all(&host, ".p-fileupload-file-thumbnail").len(), 1, "превью только у изображения");
    assert!(q(&host, ".p-fileupload-file-thumbnail").unwrap().get_attribute("src").unwrap().starts_with("blob:"));
    assert!(q(&host, ".p-progressbar").is_some(), "индикатор виден при наличии файлов");
    assert!(!disabled(&button(&host, ".p-fileupload-upload-button")) && !disabled(&button(&host, ".p-fileupload-cancel-button")));
    assert_eq!(input(&host).value(), "", "поле сброшено");

    // повторный выбор того же файла не дублирует
    choose(&host, &[make_file("a.txt", "text/plain", "hello")]);
    settle().await;
    assert_eq!(names(&host), ["a.txt", "b.png"]);

    all(&host, ".p-fileupload-file-remove-button")[0].click();
    settle().await;
    assert_eq!(names(&host), ["b.png"]);
    button(&host, ".p-fileupload-cancel-button").click();
    settle().await;
    assert!(names(&host).is_empty() && q(&host, ".p-fileupload-file-list").is_none());
    let log = log.lock().unwrap().clone();
    assert_eq!(log, ["select:2", "select:2", "remove:a.txt", "clear"], "{log:?}");
    host.remove();
}

async fn validation_and_limit() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <FileUpload multiple=true accept="image/*" max_file_size=100 file_limit=2 url="/nowhere" /> }
    });
    settle().await;
    choose(&host, &[make_file("a.pdf", "application/pdf", "x")]);
    settle().await;
    assert!(names(&host).is_empty());
    let messages = texts(&host, ".p-message");
    assert!(messages.contains("a.pdf: Invalid file type,") && messages.contains("allowed file types: image/*."), "{messages}");

    choose(&host, &[make_file("big.png", "image/png", &"x".repeat(200))]);
    settle().await;
    let messages = texts(&host, ".p-message");
    assert!(messages.contains("Invalid file size,") && messages.contains("maximum upload size is 100.000 B."), "{messages}");
    assert!(names(&host).is_empty());

    choose(&host, &[make_file("1.png", "image/png", "1"), make_file("2.png", "image/png", "22"), make_file("3.png", "image/png", "333")]);
    settle().await;
    assert_eq!(names(&host).len(), 3, "лишние файлы добавляются, но лимит превышен");
    let messages = texts(&host, ".p-message");
    assert!(messages.contains("Maximum number of files exceeded,") && messages.contains("limit is 2 at most."), "{messages}");
    assert!(disabled(&button(&host, ".p-fileupload-upload-button")), "загрузка недоступна при превышении");
    assert!(disabled(&input(&host).dyn_into::<HtmlElement>().unwrap()), "выбор недоступен при достигнутом лимите");
    all(&host, ".p-fileupload-file-remove-button")[2].click();
    settle().await;
    assert!(!disabled(&button(&host, ".p-fileupload-upload-button")), "после удаления лимит соблюдён");
    assert!(!texts(&host, ".p-message").contains("Maximum number"), "сообщение о лимите убрано");
    host.remove();
}

async fn single_mode_replaces() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <FileUpload url="/nowhere" /> });
    settle().await;
    choose(&host, &[make_file("a.txt", "text/plain", "a")]);
    settle().await;
    choose(&host, &[make_file("b.txt", "text/plain", "b")]);
    settle().await;
    assert_eq!(names(&host), ["b.txt"], "без multiple новый файл заменяет прежний");
    host.remove();
}

async fn drag_and_drop() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <FileUpload multiple=true url="/nowhere" /> });
    settle().await;
    let content = q(&host, ".p-fileupload-content").unwrap();
    drag(&content, "dragover", &[]);
    settle().await;
    assert!(content.class_name().contains("p-fileupload-highlight"));
    assert_eq!(content.get_attribute("data-p-highlight").as_deref(), Some("true"));
    drag(&content, "dragleave", &[]);
    settle().await;
    assert!(!content.class_name().contains("p-fileupload-highlight"));
    drag(&content, "dragover", &[]);
    drag(&content, "drop", &[make_file("dropped.txt", "text/plain", "d")]);
    settle().await;
    assert_eq!(names(&host), ["dropped.txt"]);
    assert!(!content.class_name().contains("p-fileupload-highlight"), "после drop подсветка снята");
    host.remove();

    // без multiple несколько перетащенных файлов отклоняются
    let host = host_again();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <FileUpload url="/nowhere" /> });
    settle().await;
    let content = q(&host, ".p-fileupload-content").unwrap();
    drag(&content, "drop", &[make_file("1.txt", "text/plain", "1"), make_file("2.txt", "text/plain", "2")]);
    settle().await;
    assert!(names(&host).is_empty());
    host.remove();
}

fn host_again() -> HtmlElement {
    host()
}

async fn custom_upload_and_auto() {
    let (host, log) = (host(), Log::default());
    let l = log.clone();
    let r = std::cell::Cell::new(None::<FileUploadRef>);
    let _h = leptos::mount::mount_to(host.clone(), {
        let r = &r;
        let file_ref = FileUploadRef::new();
        r.set(Some(file_ref));
        move || {
            view! {
                <FileUpload
                    file_upload_ref=file_ref
                    multiple=true
                    custom_upload=true
                    on_upload_handler=Callback::new(move |files: Vec<File>| l.lock().unwrap().push(format!("handler:{}", files.len())))
                />
            }
        }
    });
    settle().await;
    choose(&host, &[make_file("a.txt", "text/plain", "a"), make_file("b.txt", "text/plain", "b")]);
    settle().await;
    button(&host, ".p-fileupload-upload-button").click();
    settle().await;
    assert_eq!(log.lock().unwrap().as_slice(), ["handler:2"]);
    assert_eq!(names(&host).len(), 2, "при собственной загрузке список очищает приложение");
    let file_ref = r.get().unwrap();
    assert_eq!(file_ref.files().len(), 2);
    file_ref.clear();
    settle().await;
    assert!(names(&host).is_empty());
    host.remove();

    // auto: загрузка сразу после выбора
    let (host, log) = (host_again(), Log::default());
    let l = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <FileUpload
                auto=true
                custom_upload=true
                on_upload_handler=Callback::new(move |files: Vec<File>| l.lock().unwrap().push(format!("auto:{}", files.len())))
            />
        }
    });
    settle().await;
    assert!(q(&host, ".p-fileupload-upload-button").is_none());
    choose(&host, &[make_file("a.txt", "text/plain", "a")]);
    settle().await;
    assert_eq!(log.lock().unwrap().as_slice(), ["auto:1"]);
    host.remove();
}

async fn network_error_keeps_files() {
    let (host, log) = (host(), Log::default());
    let (l_before, l_err) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <FileUpload
                url="/definitely-not-an-upload-endpoint"
                name="f"
                on_before_upload=Callback::new(move |_| l_before.lock().unwrap().push("before".into()))
                on_error=Callback::new(move |e: prime_fileupload::UploadError| {
                    l_err.lock().unwrap().push(format!("error:{}:{}", e.files.len(), e.status.is_some()))
                })
            />
        }
    });
    settle().await;
    choose(&host, &[make_file("a.txt", "text/plain", "a")]);
    settle().await;
    button(&host, ".p-fileupload-upload-button").click();
    sleep(400).await;
    let log = log.lock().unwrap().clone();
    assert_eq!(log.first().map(String::as_str), Some("before"), "{log:?}");
    assert!(log.iter().any(|l| l.starts_with("error:1:")), "{log:?}");
    assert_eq!(names(&host), ["a.txt"], "файл остался в списке");
    assert!(!disabled(&button(&host, ".p-fileupload-cancel-button")), "после ошибки отмена снова доступна");
    host.remove();
}

async fn basic_mode() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <FileUpload mode=FileUploadMode::Basic multiple=true url="/nowhere" /> }
    });
    settle().await;
    assert!(q(&host, ".p-fileupload-basic-content").is_some() && q(&host, ".p-fileupload-header").is_none());
    assert_eq!(q(&host, ".p-fileupload-basic-content > span").unwrap().text_content().as_deref(), Some("No file chosen"));
    choose(&host, &[make_file("only.txt", "text/plain", "o")]);
    settle().await;
    assert_eq!(texts(&host, ".p-fileupload-basic-content > span"), "only.txt");
    choose(&host, &[make_file("two.txt", "text/plain", "t")]);
    settle().await;
    assert_eq!(texts(&host, ".p-fileupload-basic-content > span"), "Files");
    host.remove();
}

#[wasm_bindgen_test]
async fn fileupload_in_the_browser() {
    selecting_and_removing().await;
    validation_and_limit().await;
    single_mode_replaces().await;
    drag_and_drop().await;
    custom_upload_and_auto().await;
    network_error_keeps_files().await;
    basic_mode().await;
}
