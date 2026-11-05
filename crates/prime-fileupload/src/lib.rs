//! FileUpload — порт `p-fileupload` из PrimeNG 21.1.10.
//!
//! Загрузка файлов: выбор кнопкой или перетаскиванием, проверка типа, размера и числа, список с превью и значками
//! состояния, индикатор прогресса и отправка на сервер через `XMLHttpRequest` либо собственным обработчиком.
//! Проверки и подписи — чистая логика в [`files`].
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_fileupload::FileUpload;
//!
//! let html = view! { <FileUpload name="demo[]" url="/upload" multiple=true accept="image/*" /> }.to_html();
//! assert!(html.contains("p-fileupload-advanced") && html.contains("p-fileupload-choose-button"));
//! ```

#![recursion_limit = "512"]

pub mod files;

use files::{FileInfo, Rejection, SIZE_UNITS, basic_label, choose_disabled, format_size, is_duplicate, is_image, limit_exceeded, validate};
use leptos::prelude::*;
use prime_badge::{Badge, BadgeSeverity};
use prime_button::{Button, ButtonSeverity};
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{PlusIcon, TimesIcon, UploadIcon};
use prime_message::{Message, MessageSeverity};
use prime_progressbar::ProgressBar;
use std::sync::Arc;
use wasm_bindgen::JsCast;

/// Режим отображения.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileUploadMode {
    /// Панель со списком файлов (по умолчанию).
    #[default]
    Advanced,
    /// Одна кнопка и подпись.
    Basic,
}

impl FileUploadMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Advanced => "advanced",
            Self::Basic => "basic",
        }
    }
}

/// Выбранный файл.
#[derive(Clone)]
pub struct Entry {
    /// Файл браузера.
    pub file: web_sys::File,
    /// Сведения для проверок.
    pub info: FileInfo,
    /// Адрес превью (для изображений).
    pub object_url: Option<String>,
}

impl Entry {
    fn new(file: web_sys::File) -> Self {
        let info = FileInfo::new(file.name(), file.type_(), file.size() as u64);
        let object_url = is_image(&info).then(|| web_sys::Url::create_object_url_with_blob(&file).ok()).flatten();
        Self { file, info, object_url }
    }

    fn revoke(&self) {
        if let Some(url) = &self.object_url {
            let _ = web_sys::Url::revoke_object_url(url);
        }
    }
}

/// Результат успешной загрузки.
#[derive(Clone)]
pub struct UploadEvent {
    /// Загруженные файлы.
    pub files: Vec<web_sys::File>,
    /// Код ответа.
    pub status: u16,
    /// Тело ответа.
    pub response: String,
}

/// Ошибка загрузки.
#[derive(Clone)]
pub struct UploadError {
    /// Файлы, которые не удалось загрузить.
    pub files: Vec<web_sys::File>,
    /// Код ответа (`None` — сетевая ошибка).
    pub status: Option<u16>,
}

/// Pass-through частей [`FileUpload`].
#[derive(Clone, Debug, Default)]
pub struct FileUploadPt {
    /// Корень.
    pub root: Pt,
    /// Шапка с кнопками.
    pub header: Pt,
    /// Содержимое.
    pub content: Pt,
    /// Список файлов.
    pub file_list: Pt,
    /// Файл.
    pub file: Pt,
    /// Миниатюра.
    pub file_thumbnail: Pt,
    /// Блок сведений о файле.
    pub file_info: Pt,
    /// Имя файла.
    pub file_name: Pt,
    /// Размер файла.
    pub file_size: Pt,
    /// Блок действий файла.
    pub file_actions: Pt,
    /// Содержимое режима `basic`.
    pub basic_content: Pt,
}

/// Управляющий объект: выбор, загрузка и очистка из кода.
#[derive(Clone, Copy)]
pub struct FileUploadRef {
    files: RwSignal<Vec<Entry>, LocalStorage>,
    uploaded: RwSignal<Vec<Entry>, LocalStorage>,
    input: NodeRef<leptos::html::Input>,
    upload_hook: StoredValue<Option<Callback<()>>>,
    clear_hook: StoredValue<Option<Callback<()>>>,
}

impl Default for FileUploadRef {
    fn default() -> Self {
        Self::new()
    }
}

impl FileUploadRef {
    /// Создаёт объект. Вызывать внутри компонента (владелец сигналов).
    pub fn new() -> Self {
        Self {
            files: RwSignal::new_local(Vec::new()),
            uploaded: RwSignal::new_local(Vec::new()),
            input: NodeRef::new(),
            upload_hook: StoredValue::new(None),
            clear_hook: StoredValue::new(None),
        }
    }

    /// Выбранные файлы (без подписки).
    pub fn files(&self) -> Vec<web_sys::File> {
        self.files.with_untracked(|f| f.iter().map(|e| e.file.clone()).collect())
    }

    /// Загруженные файлы (без подписки).
    pub fn uploaded_files(&self) -> Vec<web_sys::File> {
        self.uploaded.with_untracked(|f| f.iter().map(|e| e.file.clone()).collect())
    }

    /// Открывает диалог выбора файлов.
    pub fn choose(&self) {
        if let Some(input) = self.input.get_untracked() {
            input.click();
        }
    }

    /// Загружает выбранные файлы.
    pub fn upload(&self) {
        if let Some(hook) = self.upload_hook.get_value() {
            hook.run(());
        }
    }

    /// Очищает список выбранных файлов.
    pub fn clear(&self) {
        if let Some(hook) = self.clear_hook.get_value() {
            hook.run(());
        }
    }
}

#[derive(Clone)]
struct Msg {
    text: String,
}

/// Состояние, общее для обработчиков.
struct Core {
    r: FileUploadRef,
    messages: RwSignal<Vec<Msg>>,
    progress: RwSignal<f64>,
    uploading: RwSignal<bool>,
    uploaded_count: RwSignal<usize>,
    highlight: RwSignal<bool>,
    mode: Prop<FileUploadMode>,
    name: OptProp<String>,
    url: OptProp<String>,
    method: Prop<String>,
    headers: Vec<(String, String)>,
    with_credentials: Prop<bool>,
    multiple: Prop<bool>,
    accept: OptProp<String>,
    disabled: Prop<bool>,
    auto: Prop<bool>,
    custom_upload: Prop<bool>,
    max_file_size: OptProp<u64>,
    file_limit: OptProp<usize>,
    size_summary: Prop<String>,
    size_detail: Prop<String>,
    type_summary: Prop<String>,
    type_detail: Prop<String>,
    limit_summary: Prop<String>,
    limit_detail: Prop<String>,
    on_select: Option<Callback<Vec<web_sys::File>>>,
    on_before_upload: Option<Callback<web_sys::FormData>>,
    on_send: Option<Callback<()>>,
    on_progress: Option<Callback<f64>>,
    on_upload: Option<Callback<UploadEvent>>,
    on_error: Option<Callback<UploadError>>,
    on_clear: Option<Callback<()>>,
    on_remove: Option<Callback<web_sys::File>>,
    on_remove_uploaded_file: Option<Callback<web_sys::File>>,
    on_upload_handler: Option<Callback<Vec<web_sys::File>>>,
}

impl Core {
    fn infos(&self) -> Vec<FileInfo> {
        self.r.files.with_untracked(|f| f.iter().map(|e| e.info.clone()).collect())
    }

    fn has_files(&self) -> bool {
        self.r.files.with(|f| !f.is_empty())
    }

    fn has_files_untracked(&self) -> bool {
        self.r.files.with_untracked(|f| !f.is_empty())
    }

    fn limit_exceeded(&self) -> bool {
        limit_exceeded(self.auto.get(), self.r.files.with(Vec::len), self.uploaded_count.get(), self.file_limit.get())
    }

    fn limit_exceeded_untracked(&self) -> bool {
        limit_exceeded(
            self.auto.get_untracked(),
            self.r.files.with_untracked(Vec::len),
            self.uploaded_count.get_untracked(),
            self.file_limit.get_untracked(),
        )
    }

    fn choose_disabled(&self) -> bool {
        self.disabled.get()
            || choose_disabled(self.auto.get(), self.r.files.with(Vec::len), self.uploaded_count.get(), self.file_limit.get())
    }

    fn units(&self) -> Vec<&'static str> {
        SIZE_UNITS.to_vec()
    }

    fn clear_input(&self) {
        if let Some(input) = self.r.input.get_untracked() {
            input.set_value("");
        }
    }

    fn clear(&self) {
        self.r.files.update(|files| {
            files.iter().for_each(Entry::revoke);
            files.clear();
        });
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
        self.clear_input();
        self.messages.set(Vec::new());
    }

    fn remove(&self, index: usize) {
        self.clear_input();
        let removed = self.r.files.try_update(|files| (index < files.len()).then(|| files.remove(index))).flatten();
        if let Some(entry) = removed {
            entry.revoke();
            if let Some(cb) = self.on_remove {
                cb.run(entry.file.clone());
            }
        }
        let len = self.r.files.with_untracked(Vec::len);
        self.check_file_limit(len);
    }

    fn remove_uploaded(&self, index: usize) {
        let removed = self.r.uploaded.try_update(|files| (index < files.len()).then(|| files.remove(index))).flatten();
        if let Some(entry) = removed {
            entry.revoke();
            if let Some(cb) = self.on_remove_uploaded_file {
                cb.run(entry.file.clone());
            }
        }
    }

    fn check_file_limit(&self, selected: usize) {
        let limit = self.file_limit.get_untracked();
        let existing = self.messages.with_untracked(|m| !m.is_empty()) && limit.is_some_and(|l| l < selected);
        if self.limit_exceeded_untracked() || existing {
            let limit = limit.unwrap_or_default().to_string();
            let text = format!(
                "{} {}",
                self.limit_summary.get_untracked().replace("{0}", &limit),
                self.limit_detail.get_untracked().replace("{0}", &limit)
            );
            self.messages.update(|m| m.push(Msg { text }));
        } else {
            let summary = self.limit_summary.get_untracked();
            self.messages.update(|m| m.retain(|msg| !msg.text.contains(&summary)));
        }
    }

    fn rejection_text(&self, kind: &Rejection, file: &FileInfo) -> String {
        match kind {
            Rejection::Type => format!(
                "{} {}",
                self.type_summary.get_untracked().replace("{0}", &file.name),
                self.type_detail.get_untracked().replace("{0}", &self.accept.get_untracked().unwrap_or_default())
            ),
            Rejection::Size => format!(
                "{} {}",
                self.size_summary.get_untracked().replace("{0}", &file.name),
                self.size_detail
                    .get_untracked()
                    .replace("{0}", &format_size(self.max_file_size.get_untracked().unwrap_or_default(), &self.units()))
            ),
        }
    }

    /// Обработка выбранных или перетащенных файлов (`onFileSelect`).
    fn select(self: &Arc<Self>, list: &web_sys::FileList) {
        if !self.multiple.get_untracked() {
            self.r.files.update(|files| {
                files.iter().for_each(Entry::revoke);
                files.clear();
            });
        }
        self.messages.set(Vec::new());
        let (accept, max) = (self.accept.get_untracked(), self.max_file_size.get_untracked());
        for i in 0..list.length() {
            let Some(file) = list.item(i) else { continue };
            let info = FileInfo::new(file.name(), file.type_(), file.size() as u64);
            if is_duplicate(&self.infos(), &info) {
                continue;
            }
            match validate(&info, accept.as_deref(), max) {
                Ok(()) => self.r.files.update(|files| files.push(Entry::new(file))),
                Err(kind) => {
                    let text = self.rejection_text(&kind, &info);
                    self.messages.update(|m| m.push(Msg { text }));
                }
            }
        }
        if let Some(cb) = self.on_select {
            cb.run(self.r.files());
        }
        self.check_file_limit(list.length() as usize);
        if self.has_files_untracked()
            && self.auto.get_untracked()
            && (self.mode.get_untracked() != FileUploadMode::Advanced || !self.limit_exceeded_untracked())
        {
            self.upload();
        }
        self.clear_input();
    }

    fn upload(self: &Arc<Self>) {
        if !self.has_files_untracked() {
            return;
        }
        let files = self.r.files();
        if self.custom_upload.get_untracked() {
            if self.file_limit.get_untracked().is_some() {
                self.uploaded_count.update(|c| *c += files.len());
            }
            if let Some(cb) = self.on_upload_handler {
                cb.run(files);
            }
            return;
        }
        self.send(files);
    }

    /// Отправка через `XMLHttpRequest` с прогрессом.
    fn send(self: &Arc<Self>, files: Vec<web_sys::File>) {
        use wasm_bindgen::closure::Closure;
        self.uploading.set(true);
        self.messages.set(Vec::new());
        let Ok(form) = web_sys::FormData::new() else { return };
        if let Some(cb) = self.on_before_upload {
            cb.run(form.clone());
        }
        let name = self.name.get_untracked().unwrap_or_default();
        for file in &files {
            let _ = form.append_with_blob_and_filename(&name, file, &file.name());
        }
        let fail = {
            let this = self.clone();
            let files = files.clone();
            move |status: Option<u16>| {
                if this.uploading.is_disposed() {
                    return;
                }
                this.uploading.set(false);
                if let Some(cb) = this.on_error {
                    cb.run(UploadError { files: files.clone(), status });
                }
            }
        };
        let Ok(xhr) = web_sys::XmlHttpRequest::new() else {
            fail(None);
            return;
        };
        let url = self.url.get_untracked().unwrap_or_default();
        if xhr.open(&self.method.get_untracked().to_uppercase(), &url).is_err() {
            fail(None);
            return;
        }
        for (key, value) in &self.headers {
            let _ = xhr.set_request_header(key, value);
        }
        xhr.set_with_credentials(self.with_credentials.get_untracked());

        if let Ok(upload) = xhr.upload() {
            let this = self.clone();
            let on_progress = Closure::<dyn FnMut(web_sys::ProgressEvent)>::new(move |e: web_sys::ProgressEvent| {
                if this.progress.is_disposed() || e.total() <= 0.0 {
                    return;
                }
                let progress = (e.loaded() * 100.0 / e.total()).round();
                this.progress.set(progress);
                if let Some(cb) = this.on_progress {
                    cb.run(progress);
                }
            });
            upload.set_onprogress(Some(on_progress.as_ref().unchecked_ref()));
            on_progress.forget();
        }
        let this = self.clone();
        let xhr_load = xhr.clone();
        let fail_load = fail.clone();
        let files_load = files.clone();
        let on_load = Closure::<dyn FnMut()>::new(move || {
            if this.uploading.is_disposed() {
                return;
            }
            let status = xhr_load.status().unwrap_or(0);
            if (200..300).contains(&status) {
                this.uploading.set(false);
                this.progress.set(0.0);
                if this.file_limit.get_untracked().is_some() {
                    this.uploaded_count.update(|c| *c += files_load.len());
                }
                if let Some(cb) = this.on_upload {
                    cb.run(UploadEvent {
                        files: files_load.clone(),
                        status,
                        response: xhr_load.response_text().ok().flatten().unwrap_or_default(),
                    });
                }
                let moved = this.r.files.try_update(std::mem::take).unwrap_or_default();
                this.r.uploaded.update(|u| u.extend(moved));
                this.clear_after_upload();
            } else {
                fail_load(Some(status));
            }
        });
        xhr.set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();
        let on_error = Closure::<dyn FnMut()>::new(move || fail(None));
        xhr.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();

        if xhr.send_with_opt_form_data(Some(&form)).is_ok()
            && let Some(cb) = self.on_send
        {
            cb.run(());
        }
    }

    /// После успешной загрузки список ожидающих уже передан в загруженные: только сообщения и поле.
    fn clear_after_upload(&self) {
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
        self.clear_input();
        self.messages.set(Vec::new());
    }
}

/// Загрузка файлов.
///
/// Каждое свойство принимает значение или сигнал. Выбранные и загруженные файлы доступны через [`FileUploadRef`].
#[component]
pub fn FileUpload(
    /// Управляющий объект.
    #[prop(optional)]
    file_upload_ref: Option<FileUploadRef>,
    /// Режим.
    #[prop(optional, into)]
    mode: Prop<FileUploadMode>,
    /// Имя поля формы с файлами.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Адрес загрузки.
    #[prop(optional, into)]
    url: OptProp<String>,
    /// HTTP-метод.
    #[prop(optional, default = Prop::Static("post".to_owned()), into)]
    method: Prop<String>,
    /// Заголовки запроса.
    #[prop(optional, into)]
    headers: Vec<(String, String)>,
    /// Передавать учётные данные при кросс-доменном запросе.
    #[prop(optional, into)]
    with_credentials: Prop<bool>,
    /// Разрешить несколько файлов.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Допустимые типы (список через запятую).
    #[prop(optional, into)]
    accept: OptProp<String>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Загружать сразу после выбора.
    #[prop(optional, into)]
    auto: Prop<bool>,
    /// Не отправлять запрос, а вызвать `on_upload_handler`.
    #[prop(optional, into)]
    custom_upload: Prop<bool>,
    /// Максимальный размер файла, байты.
    #[prop(optional, into)]
    max_file_size: OptProp<u64>,
    /// Максимальное число файлов.
    #[prop(optional, into)]
    file_limit: OptProp<usize>,
    /// Начало сообщения о неверном размере.
    #[prop(optional, default = Prop::Static("{0}: Invalid file size, ".to_owned()), into)]
    invalid_file_size_message_summary: Prop<String>,
    /// Окончание сообщения о неверном размере.
    #[prop(optional, default = Prop::Static("maximum upload size is {0}.".to_owned()), into)]
    invalid_file_size_message_detail: Prop<String>,
    /// Начало сообщения о неверном типе.
    #[prop(optional, default = Prop::Static("{0}: Invalid file type, ".to_owned()), into)]
    invalid_file_type_message_summary: Prop<String>,
    /// Окончание сообщения о неверном типе.
    #[prop(optional, default = Prop::Static("allowed file types: {0}.".to_owned()), into)]
    invalid_file_type_message_detail: Prop<String>,
    /// Начало сообщения о превышении лимита.
    #[prop(optional, default = Prop::Static("Maximum number of files exceeded, ".to_owned()), into)]
    invalid_file_limit_message_summary: Prop<String>,
    /// Окончание сообщения о превышении лимита.
    #[prop(optional, default = Prop::Static("limit is {0} at most.".to_owned()), into)]
    invalid_file_limit_message_detail: Prop<String>,
    /// Ширина превью, px.
    #[prop(optional, default = Prop::Static(50), into)]
    preview_width: Prop<u32>,
    /// Метка кнопки выбора.
    #[prop(optional, into)]
    choose_label: OptProp<String>,
    /// Метка кнопки загрузки.
    #[prop(optional, into)]
    upload_label: OptProp<String>,
    /// Метка кнопки отмены.
    #[prop(optional, into)]
    cancel_label: OptProp<String>,
    /// Показывать кнопку загрузки.
    #[prop(optional, default = Prop::Static(true), into)]
    show_upload_button: Prop<bool>,
    /// Показывать кнопку отмены.
    #[prop(optional, default = Prop::Static(true), into)]
    show_cancel_button: Prop<bool>,
    /// Класс кнопки выбора.
    #[prop(optional, into)]
    choose_style_class: OptProp<String>,
    /// Класс кнопки загрузки.
    #[prop(optional, into)]
    upload_style_class: OptProp<String>,
    /// Класс кнопки отмены.
    #[prop(optional, into)]
    cancel_style_class: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Дополнительное содержимое шапки (после кнопок).
    #[prop(optional, into)]
    header: Option<ViewFn>,
    /// Содержимое для пустого списка.
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    /// Вызывается после выбора файлов.
    #[prop(optional, into)]
    on_select: Option<Callback<Vec<web_sys::File>>>,
    /// Вызывается перед отправкой; позволяет дополнить `FormData`.
    #[prop(optional, into)]
    on_before_upload: Option<Callback<web_sys::FormData>>,
    /// Вызывается после отправки запроса.
    #[prop(optional, into)]
    on_send: Option<Callback<()>>,
    /// Вызывается при изменении прогресса (0–100).
    #[prop(optional, into)]
    on_progress: Option<Callback<f64>>,
    /// Вызывается после успешной загрузки.
    #[prop(optional, into)]
    on_upload: Option<Callback<UploadEvent>>,
    /// Вызывается при ошибке загрузки.
    #[prop(optional, into)]
    on_error: Option<Callback<UploadError>>,
    /// Вызывается при очистке списка.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Вызывается при удалении выбранного файла.
    #[prop(optional, into)]
    on_remove: Option<Callback<web_sys::File>>,
    /// Вызывается при удалении загруженного файла.
    #[prop(optional, into)]
    on_remove_uploaded_file: Option<Callback<web_sys::File>>,
    /// Обработчик собственной загрузки (`custom_upload`).
    #[prop(optional, into)]
    on_upload_handler: Option<Callback<Vec<web_sys::File>>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<FileUploadPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let r = file_upload_ref.unwrap_or_default();
    let core = Arc::new(Core {
        r,
        messages: RwSignal::new(Vec::new()),
        progress: RwSignal::new(0.0),
        uploading: RwSignal::new(false),
        uploaded_count: RwSignal::new(0),
        highlight: RwSignal::new(false),
        mode,
        name,
        url,
        method,
        headers,
        with_credentials,
        multiple,
        accept,
        disabled,
        auto,
        custom_upload,
        max_file_size,
        file_limit,
        size_summary: invalid_file_size_message_summary,
        size_detail: invalid_file_size_message_detail,
        type_summary: invalid_file_type_message_summary,
        type_detail: invalid_file_type_message_detail,
        limit_summary: invalid_file_limit_message_summary,
        limit_detail: invalid_file_limit_message_detail,
        on_select,
        on_before_upload,
        on_send,
        on_progress,
        on_upload,
        on_error,
        on_clear,
        on_remove,
        on_remove_uploaded_file,
        on_upload_handler,
    });
    {
        let c = core.clone();
        r.upload_hook.set_value(Some(Callback::new(move |()| c.upload())));
        let c = core.clone();
        r.clear_hook.set_value(Some(Callback::new(move |()| c.clear())));
        let c = core.clone();
        on_cleanup(move || {
            c.r.files.try_update(|files| files.iter().for_each(Entry::revoke));
            c.r.uploaded.try_update(|files| files.iter().for_each(Entry::revoke));
        });
    }

    let labels = Arc::new(Labels { choose: choose_label, upload: upload_label, cancel: cancel_label });
    let extras = Arc::new(Extras {
        show_upload_button,
        show_cancel_button,
        choose_style_class,
        upload_style_class,
        cancel_style_class,
        preview_width,
        header,
        empty,
    });
    let root_class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let mut cn = format!("p-fileupload p-fileupload-{} p-component", core.mode.get().as_str());
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let body = {
        let (core, pt, labels, extras) = (core.clone(), pt.clone(), labels.clone(), extras.clone());
        move || match core.mode.get() {
            FileUploadMode::Advanced => advanced_view(&core, &pt, &labels, &extras).into_any(),
            FileUploadMode::Basic => basic_view(&core, &pt, &labels).into_any(),
        }
    };
    view! {
        <div class={root_class} style={root_style} data-pc-name="fileupload" data-pc-section="root" {..pt.root.attrs()}>
            {body}
        </div>
    }
}

struct Labels {
    choose: OptProp<String>,
    upload: OptProp<String>,
    cancel: OptProp<String>,
}

struct Extras {
    show_upload_button: Prop<bool>,
    show_cancel_button: Prop<bool>,
    choose_style_class: OptProp<String>,
    upload_style_class: OptProp<String>,
    cancel_style_class: OptProp<String>,
    preview_width: Prop<u32>,
    header: Option<ViewFn>,
    empty: Option<ViewFn>,
}

fn text(key: &str) -> String {
    prime_core::use_prime_config().locale.text(key).to_owned()
}

fn class_with(base: &str, extra: &OptProp<String>) -> String {
    match extra.get() {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_owned(),
    }
}

/// Скрытое поле выбора файлов.
fn file_input(core: &Arc<Core>) -> impl IntoView + use<> {
    let (c_dis, c_change, c_mult, c_accept) = (core.clone(), core.clone(), core.clone(), core.clone());
    let input = core.r.input;
    view! {
        <input
            node_ref=input
            type="file"
            title=""
            aria-label={text("aria.browseFiles")}
            multiple={move || c_mult.multiple.get()}
            accept={move || c_accept.accept.get()}
            disabled={move || c_dis.choose_disabled()}
            on:change=move |e: web_sys::Event| {
                let list = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()).and_then(|i| i.files());
                if let Some(list) = list {
                    c_change.select(&list);
                }
            }
        />
    }
}

fn advanced_view(core: &Arc<Core>, pt: &Arc<FileUploadPt>, labels: &Arc<Labels>, extras: &Arc<Extras>) -> impl IntoView + use<> {
    let input = file_input(core);

    let choose = {
        let (core, labels, extras) = (core.clone(), labels.clone(), extras.clone());
        let disabled = Signal::derive({
            let core = core.clone();
            move || core.choose_disabled()
        });
        let choose_core = core.clone();
        view! {
            <Button
                label=Signal::derive(move || Some(labels.choose.get().unwrap_or_else(|| text("choose"))))
                icon=ViewFn::from(|| view! { <PlusIcon /> })
                disabled=disabled
                style_class=Signal::derive(move || Some(class_with("p-fileupload-choose-button", &extras.choose_style_class)))
                on_click=Callback::new(move |_| choose_core.r.choose())
            />
        }
    };
    let upload = {
        let (core, labels, extras) = (core.clone(), labels.clone(), extras.clone());
        move || {
            let show = !core.auto.get() && extras.show_upload_button.get();
            show.then(|| {
                let (dis_core, click_core, extras, labels) = (core.clone(), core.clone(), extras.clone(), labels.clone());
                view! {
                    <Button
                        label=Signal::derive(move || Some(labels.upload.get().unwrap_or_else(|| text("upload"))))
                        icon=ViewFn::from(|| view! { <UploadIcon /> })
                        severity=ButtonSeverity::Secondary
                        disabled=Signal::derive(move || !dis_core.has_files() || dis_core.limit_exceeded())
                        style_class=Signal::derive(move || Some(class_with("p-fileupload-upload-button", &extras.upload_style_class)))
                        on_click=Callback::new(move |_| click_core.upload())
                    />
                }
            })
        }
    };
    let cancel = {
        let (core, labels, extras) = (core.clone(), labels.clone(), extras.clone());
        move || {
            let show = !core.auto.get() && extras.show_cancel_button.get();
            show.then(|| {
                let (dis_core, click_core, extras, labels) = (core.clone(), core.clone(), extras.clone(), labels.clone());
                view! {
                    <Button
                        label=Signal::derive(move || Some(labels.cancel.get().unwrap_or_else(|| text("cancel"))))
                        icon=ViewFn::from(|| view! { <TimesIcon /> })
                        severity=ButtonSeverity::Secondary
                        disabled=Signal::derive(move || !dis_core.has_files() || dis_core.uploading.get())
                        style_class=Signal::derive(move || Some(class_with("p-fileupload-cancel-button", &extras.cancel_style_class)))
                        on_click=Callback::new(move |_| click_core.clear())
                    />
                }
            })
        }
    };
    let header_extra = extras.header.clone().map(|h| h.run());

    let content_class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let base = if core.highlight.get() { "p-fileupload-content p-fileupload-highlight" } else { "p-fileupload-content" };
            pt.content.merge_class(base).into_owned()
        }
    };
    let highlight_attr = {
        let core = core.clone();
        move || core.highlight.get().to_string()
    };
    let (enter, over, leave, drop_core) = (core.clone(), core.clone(), core.clone(), core.clone());
    let progress = {
        let core = core.clone();
        move || {
            core.has_files().then(|| {
                let progress = core.progress;
                view! { <ProgressBar value=Signal::derive(move || Some(progress.get())) show_value=false /> }
            })
        }
    };
    let messages = {
        let core = core.clone();
        move || core.messages.get().into_iter().map(|m| view! { <Message severity=MessageSeverity::Error text=m.text /> }).collect_view()
    };
    let pending = file_list(core, pt, extras, false);
    let done = file_list(core, pt, extras, true);
    let empty = {
        let (core, extras) = (core.clone(), extras.clone());
        move || {
            let none = !core.has_files() && core.r.uploaded.with(Vec::is_empty);
            none.then(|| extras.empty.clone().map(|e| e.run())).flatten()
        }
    };
    view! {
        {input}
        <div class={pt.header.merge_class("p-fileupload-header")} {..pt.header.attrs()}>
            {choose}
            {upload}
            {cancel}
            {header_extra}
        </div>
        <div
            class={content_class}
            data-p-highlight={highlight_attr}
            on:dragenter=move |e: web_sys::DragEvent| {
                if !enter.disabled.get_untracked() {
                    e.stop_propagation();
                    e.prevent_default();
                }
            }
            on:dragover=move |e: web_sys::DragEvent| {
                if !over.disabled.get_untracked() {
                    over.highlight.set(true);
                    e.stop_propagation();
                    e.prevent_default();
                }
            }
            on:dragleave=move |_| {
                if !leave.disabled.get_untracked() {
                    leave.highlight.set(false);
                }
            }
            on:drop=move |e: web_sys::DragEvent| {
                if drop_core.disabled.get_untracked() {
                    return;
                }
                drop_core.highlight.set(false);
                e.stop_propagation();
                e.prevent_default();
                if let Some(list) = e.data_transfer().and_then(|d| d.files())
                    && (drop_core.multiple.get_untracked() || list.length() == 1)
                {
                    drop_core.select(&list);
                }
            }
            {..pt.content.attrs()}
        >
            {progress}
            {messages}
            {pending}
            {done}
            {empty}
        </div>
    }
}

/// Список ожидающих или загруженных файлов.
fn file_list(core: &Arc<Core>, pt: &Arc<FileUploadPt>, extras: &Arc<Extras>, uploaded: bool) -> impl IntoView + use<> {
    let (core, pt, extras) = (core.clone(), pt.clone(), extras.clone());
    move || {
        let signal = if uploaded { core.r.uploaded } else { core.r.files };
        let items: Vec<(String, String, Option<String>)> = signal.with(|files| {
            files.iter().map(|e| (e.info.name.clone(), format_size(e.info.size, &SIZE_UNITS), e.object_url.clone())).collect()
        });
        (!items.is_empty()).then(|| {
            let rows = items
                .into_iter()
                .enumerate()
                .map(|(index, (name, size, url))| file_row(&core, &pt, &extras, uploaded, index, name, size, url))
                .collect_view();
            view! { <div class={pt.file_list.merge_class("p-fileupload-file-list")} {..pt.file_list.attrs()}>{rows}</div> }
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn file_row(
    core: &Arc<Core>,
    pt: &Arc<FileUploadPt>,
    extras: &Arc<Extras>,
    uploaded: bool,
    index: usize,
    name: String,
    size: String,
    url: Option<String>,
) -> impl IntoView + use<> {
    let thumbnail = url.map(|src| {
        view! {
            <img
                role="presentation"
                class={pt.file_thumbnail.merge_class("p-fileupload-file-thumbnail")}
                alt={name.clone()}
                src={src}
                width={extras.preview_width.get_untracked()}
                {..pt.file_thumbnail.attrs()}
            />
        }
    });
    let (badge, severity) = if uploaded { (text("completed"), BadgeSeverity::Success) } else { (text("pending"), BadgeSeverity::Warn) };
    let click_core = core.clone();
    view! {
        <div class={pt.file.merge_class("p-fileupload-file")} {..pt.file.attrs()}>
            {thumbnail}
            <div class={pt.file_info.merge_class("p-fileupload-file-info")} {..pt.file_info.attrs()}>
                <div class={pt.file_name.merge_class("p-fileupload-file-name")} {..pt.file_name.attrs()}>{name.clone()}</div>
                <span class={pt.file_size.merge_class("p-fileupload-file-size")} {..pt.file_size.attrs()}>{size}</span>
            </div>
            <Badge value=badge severity=severity style_class="p-fileupload-file-badge" />
            <div class={pt.file_actions.merge_class("p-fileupload-file-actions")} {..pt.file_actions.attrs()}>
                <Button
                    text=true
                    rounded=true
                    severity=ButtonSeverity::Danger
                    icon=ViewFn::from(|| view! { <TimesIcon /> })
                    style_class="p-fileupload-file-remove-button"
                    on_click=Callback::new(move |_| {
                        if uploaded {
                            click_core.remove_uploaded(index);
                        } else {
                            click_core.remove(index);
                        }
                    })
                />
            </div>
        </div>
    }
}

fn basic_view(core: &Arc<Core>, pt: &Arc<FileUploadPt>, labels: &Arc<Labels>) -> impl IntoView + use<> {
    let messages = {
        let core = core.clone();
        move || core.messages.get().into_iter().map(|m| view! { <Message severity=MessageSeverity::Error text=m.text /> }).collect_view()
    };
    let input = file_input(core);
    let choose = {
        let (core, labels) = (core.clone(), labels.clone());
        let (icon_core, dis_core, click_core) = (core.clone(), core.clone(), core.clone());
        view! {
            <Button
                label=Signal::derive(move || Some(labels.choose.get().unwrap_or_else(|| text("choose"))))
                icon=ViewFn::from(move || {
                    if icon_core.has_files() && !icon_core.auto.get() { view! { <UploadIcon /> }.into_any() } else { view! { <PlusIcon /> }.into_any() }
                })
                disabled=Signal::derive(move || dis_core.disabled.get())
                style_class="p-fileupload-choose-button"
                on_click=Callback::new(move |_| click_core.r.choose())
            />
        }
    };
    let label = {
        let (core, labels) = (core.clone(), labels.clone());
        move || {
            (!core.auto.get()).then(|| {
                let names: Vec<String> = core.r.files.with(|f| f.iter().map(|e| e.info.name.clone()).collect());
                let choose_label = labels.choose.get().unwrap_or_else(|| text("choose"));
                view! { <span>{basic_label(false, &choose_label, &names, &text("fileChosenMessage"), &text("noFileChosenMessage"))}</span> }
            })
        }
    };
    view! {
        {messages}
        <div class={pt.basic_content.merge_class("p-fileupload-basic-content")} {..pt.basic_content.attrs()}>
            {choose}
            {input}
            {label}
        </div>
    }
}
