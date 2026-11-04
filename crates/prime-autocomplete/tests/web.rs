//! Поведение AutoComplete в реальном браузере (wasm-bindgen-test): запрос с задержкой, показ и выбор вариантов, клавиатура,
//! множественный режим с чипами, кнопка-стрелка, очистка, `force_selection`, группы, клик вне.
//!
//! Сценарии идут одним последовательным тестом: слушатели окна и фокус общие на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_autocomplete::{AutoComplete, AutoCompleteCompleteEvent, AutoCompleteOption};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

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

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}: {}", host.outer_html())).dyn_into().unwrap()
}

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

fn count(host: &HtmlElement, sel: &str) -> u32 {
    host.query_selector_all(sel).unwrap().length()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам примениться.
async fn settle() {
    sleep(60).await;
}

fn input(host: &HtmlElement) -> HtmlInputElement {
    q(host, "input.p-autocomplete-input").dyn_into().unwrap()
}

fn type_text(host: &HtmlElement, text: &str) {
    let i = input(host);
    i.set_value(text);
    i.dispatch_event(
        &web_sys::Event::new_with_event_init_dict("input", &{
            let init = web_sys::EventInit::new();
            init.set_bubbles(true);
            init
        })
        .unwrap(),
    )
    .unwrap();
}

fn option_labels(host: &HtmlElement) -> Vec<String> {
    let list = host.query_selector_all("li.p-autocomplete-option").unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().text_content().unwrap_or_default().trim().to_owned()).collect()
}

fn focused_option(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-autocomplete-option.p-focus").unwrap().and_then(|e| e.text_content()).map(|t| t.trim().to_owned())
}

/// Нажатие клавиши и ожидание реактивных обновлений.
async fn key(el: &HtmlElement, code: &str, key: &str, alt: bool) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_alt_key(alt);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    sleep(20).await;
}

fn mouse(el: &web_sys::Element, name: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

fn fruits() -> Vec<AutoCompleteOption> {
    ["Apple", "Apricot", "Avocado", "Banana"].iter().map(|l| AutoCompleteOption::new(*l)).collect()
}

fn completer(suggestions: RwSignal<Vec<AutoCompleteOption>>, log: &Log) -> Callback<AutoCompleteCompleteEvent> {
    let log = log.clone();
    Callback::new(move |e: AutoCompleteCompleteEvent| {
        log.lock().unwrap().push(format!("complete:{}", e.query));
        let q = e.query.to_lowercase();
        suggestions.set(fruits().into_iter().filter(|o| o.label.to_lowercase().contains(&q)).collect());
    })
}

async fn typing_queries_and_selects() {
    let (host, log) = (host(), Log::default());
    let suggestions = RwSignal::new(Vec::<AutoCompleteOption>::new());
    let value = RwSignal::new(String::new());
    let (l1, l2) = (log.clone(), log.clone());
    let cb = completer(suggestions, &log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <AutoComplete
                suggestions=suggestions
                value=value
                delay=20
                on_complete=cb
                on_select=Callback::new(move |v: String| l1.lock().unwrap().push(format!("select:{v}")))
                on_hide=Callback::new(move |_| l2.lock().unwrap().push("hide".into()))
            />
        }
    });
    settle().await;
    input(&host).focus().unwrap();
    type_text(&host, "ap");
    assert_eq!(value.get_untracked(), "ap", "без force_selection набранный текст — значение");
    sleep(15).await;
    assert!(log.lock().unwrap().is_empty(), "запрос ждёт delay");
    sleep(120).await;
    assert_eq!(log.lock().unwrap().as_slice(), ["complete:ap"]);
    assert!(has(&host, ".p-autocomplete-overlay") && q(&host, ".p-autocomplete").class_name().contains("p-autocomplete-open"));
    assert_eq!(option_labels(&host), ["Apple", "Apricot"], "варианты по запросу");
    assert!(!has(&host, ".p-autocomplete-loader"), "спиннер снят после ответа");
    assert_eq!(q(&host, "input").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(!q(&host, ".p-autocomplete-overlay").style().get_property_value("z-index").unwrap().is_empty());

    mouse(&q(&host, "li.p-autocomplete-option:nth-child(2)"), "mouseenter");
    settle().await;
    assert_eq!(focused_option(&host).as_deref(), Some("Apricot"), "наведение переносит фокус");
    q(&host, "li.p-autocomplete-option:nth-child(2)").click();
    settle().await;
    assert_eq!(value.get_untracked(), "Apricot");
    assert_eq!(input(&host).value(), "Apricot");
    assert!(!has(&host, ".p-autocomplete-overlay"), "выбор закрывает панель");
    assert_eq!(log.lock().unwrap().as_slice(), ["complete:ap", "select:Apricot", "hide"]);

    type_text(&host, "zzz");
    sleep(150).await;
    assert!(has(&host, "li.p-autocomplete-empty-message"), "пустой результат — сообщение");
    assert!(q(&host, "li.p-autocomplete-empty-message").text_content().unwrap().contains("No results found"));
    type_text(&host, "");
    sleep(60).await;
    assert!(!has(&host, ".p-autocomplete-overlay"), "очистка поля закрывает панель");
    host.remove();
}

async fn keyboard_navigation() {
    let (host, log) = (host(), Log::default());
    let suggestions = RwSignal::new(Vec::<AutoCompleteOption>::new());
    let value = RwSignal::new(String::new());
    let cb = completer(suggestions, &log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <AutoComplete suggestions=suggestions value=value delay=10 on_complete=cb /> }
    });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    type_text(&host, "a");
    sleep(100).await;
    assert_eq!(option_labels(&host).len(), 4, "Apple, Apricot, Avocado, Banana");
    assert!(q(&host, "input").get_attribute("aria-controls").is_some());
    key(&i, "ArrowDown", "ArrowDown", false).await;
    assert_eq!(focused_option(&host).as_deref(), Some("Apple"));
    key(&i, "ArrowDown", "ArrowDown", false).await;
    key(&i, "ArrowDown", "ArrowDown", false).await;
    assert_eq!(focused_option(&host).as_deref(), Some("Avocado"));
    assert!(q(&host, "input").get_attribute("aria-activedescendant").is_some());
    key(&i, "ArrowUp", "ArrowUp", false).await;
    assert_eq!(focused_option(&host).as_deref(), Some("Apricot"));
    key(&i, "Enter", "Enter", false).await;
    settle().await;
    assert_eq!(value.get_untracked(), "Apricot", "Enter выбирает");
    assert!(!has(&host, ".p-autocomplete-overlay"));

    type_text(&host, "b");
    sleep(100).await;
    key(&i, "ArrowDown", "ArrowDown", false).await;
    key(&i, "ArrowUp", "ArrowUp", true).await;
    settle().await;
    assert_eq!(value.get_untracked(), "Banana", "Alt+↑ выбирает и закрывает");
    type_text(&host, "av");
    sleep(100).await;
    key(&i, "Escape", "Escape", false).await;
    settle().await;
    assert!(!has(&host, ".p-autocomplete-overlay"), "Escape закрывает");
    type_text(&host, "av");
    sleep(100).await;
    key(&i, "ArrowDown", "ArrowDown", false).await;
    key(&i, "Tab", "Tab", false).await;
    settle().await;
    assert_eq!(value.get_untracked(), "Avocado", "Tab выбирает сфокусированный");
    host.remove();
}

async fn dropdown_clear_and_force_selection() {
    let (host, log) = (host(), Log::default());
    let suggestions = RwSignal::new(Vec::<AutoCompleteOption>::new());
    let value = RwSignal::new(String::new());
    let cb = completer(suggestions, &log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <AutoComplete suggestions=suggestions value=value dropdown=true show_clear=true force_selection=true delay=10 on_complete=cb /> }
    });
    settle().await;
    q(&host, ".p-autocomplete-dropdown").click();
    sleep(100).await;
    assert_eq!(log.lock().unwrap().as_slice(), ["complete:"], "кнопка отправляет пустой запрос");
    assert_eq!(option_labels(&host).len(), 4);
    q(&host, ".p-autocomplete-dropdown").click();
    settle().await;
    assert!(!has(&host, ".p-autocomplete-overlay"), "повторный клик закрывает панель");

    q(&host, ".p-autocomplete-dropdown").click();
    sleep(100).await;
    q(&host, "li.p-autocomplete-option:nth-child(4)").click();
    settle().await;
    assert_eq!(value.get_untracked(), "Banana");
    assert!(has(&host, ".p-autocomplete-clear-icon"));
    mouse(&host.query_selector(".p-autocomplete-clear-icon").unwrap().unwrap(), "click");
    settle().await;
    assert!(value.get_untracked().is_empty() && input(&host).value().is_empty(), "очистка");
    assert!(!has(&host, ".p-autocomplete-clear-icon"));

    // force_selection: не совпавший с вариантом ввод очищается по change.
    input(&host).focus().unwrap();
    type_text(&host, "zzz");
    assert!(value.get_untracked().is_empty(), "при force_selection текст значением не становится");
    sleep(100).await;
    input(&host).dispatch_event(&web_sys::Event::new("change").unwrap()).unwrap();
    key(&input(&host), "Escape", "Escape", false).await;
    settle().await;
    assert!(input(&host).value().is_empty(), "несовпавший ввод очищен");
    host.remove();
}

async fn multiple_chips() {
    let (host, log) = (host(), Log::default());
    let suggestions = RwSignal::new(Vec::<AutoCompleteOption>::new());
    let values = RwSignal::new(Vec::<String>::new());
    let (l1, l2) = (log.clone(), log.clone());
    let cb = completer(suggestions, &log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <AutoComplete
                suggestions=suggestions
                values=values
                multiple=true
                delay=10
                on_complete=cb
                on_unselect=Callback::new(move |v: String| l1.lock().unwrap().push(format!("unselect:{v}")))
                on_select=Callback::new(move |v: String| l2.lock().unwrap().push(format!("select:{v}")))
            />
        }
    });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    type_text(&host, "ap");
    sleep(100).await;
    q(&host, "li.p-autocomplete-option:nth-child(1)").click();
    settle().await;
    assert_eq!(values.get_untracked(), ["Apple"]);
    assert_eq!(count(&host, ".p-autocomplete-chip-item"), 1);
    assert!(input(&host).value().is_empty(), "после выбора поле очищается");
    type_text(&host, "ap");
    sleep(100).await;
    q(&host, "li.p-autocomplete-option:nth-child(1)").click();
    settle().await;
    assert_eq!(values.get_untracked(), ["Apple"], "unique: повтор не добавляется");
    type_text(&host, "ban");
    sleep(100).await;
    q(&host, "li.p-autocomplete-option:nth-child(1)").click();
    settle().await;
    assert_eq!(values.get_untracked(), ["Apple", "Banana"]);
    // Backspace в пустом поле удаляет последний чип.
    input(&host).focus().unwrap();
    key(&input(&host), "Backspace", "Backspace", false).await;
    settle().await;
    assert_eq!(values.get_untracked(), ["Apple"]);
    // ← на пустом поле переводит на чипы, Backspace удаляет сфокусированный.
    key(&input(&host), "ArrowLeft", "ArrowLeft", false).await;
    settle().await;
    assert!(has(&host, "li.p-autocomplete-chip-item.p-focus"), "чип сфокусирован");
    let ul = q(&host, "ul.p-autocomplete-input-multiple");
    key(&ul, "Backspace", "Backspace", false).await;
    settle().await;
    assert!(values.get_untracked().is_empty());
    assert_eq!(
        log.lock().unwrap().as_slice(),
        [
            "complete:ap",
            "select:Apple",
            "complete:ap",
            "select:Apple",
            "complete:ban",
            "select:Banana",
            "unselect:Banana",
            "unselect:Apple"
        ]
    );
    host.remove();
}

async fn typeahead_off_adds_typed_text() {
    let host = host();
    let values = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <AutoComplete suggestions={Vec::<AutoCompleteOption>::new()} values=values multiple=true typeahead=false separator="," add_on_blur=true /> }
    });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    type_text(&host, "one");
    key(&i, "Enter", "Enter", false).await;
    assert_eq!(values.get_untracked(), ["one"], "Enter добавляет набранное");
    type_text(&host, "two");
    key(&i, "Comma", ",", false).await;
    assert_eq!(values.get_untracked(), ["one", "two"], "разделитель добавляет набранное");
    type_text(&host, "three");
    i.blur().unwrap();
    settle().await;
    assert_eq!(values.get_untracked(), ["one", "two", "three"], "add_on_blur");
    host.remove();
}

async fn groups_and_outside_click() {
    let host = host();
    let suggestions = RwSignal::new(vec![
        AutoCompleteOption::new("Fruits").items(vec![AutoCompleteOption::new("Apple"), AutoCompleteOption::new("Pear").disabled(true)]),
        AutoCompleteOption::new("Veg").items(vec![AutoCompleteOption::new("Kale")]),
    ]);
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <AutoComplete suggestions=suggestions value=value group=true dropdown=true on_complete=Callback::new(move |_| suggestions.update(|_| {})) /> }
    });
    settle().await;
    q(&host, ".p-autocomplete-dropdown").click();
    sleep(80).await;
    assert!(has(&host, ".p-autocomplete-overlay"), "кнопка показывает панель при ответе");
    assert_eq!(count(&host, "li.p-autocomplete-option-group"), 2, "заголовки групп");
    assert_eq!(count(&host, "li.p-autocomplete-option"), 3);
    assert_eq!(q(&host, "li.p-autocomplete-option:nth-of-type(3)").get_attribute("aria-disabled").as_deref(), Some("true"));
    assert_eq!(q(&host, "li.p-autocomplete-option").get_attribute("aria-posinset").as_deref(), Some("1"));
    assert_eq!(q(&host, "li.p-autocomplete-option").get_attribute("aria-setsize").as_deref(), Some("3"));
    let i = input(&host);
    key(&i, "ArrowDown", "ArrowDown", false).await;
    key(&i, "ArrowDown", "ArrowDown", false).await;
    assert_eq!(focused_option(&host).as_deref(), Some("Kale"), "отключённый и заголовки пропущены");
    host.remove();

    let host = self::host();
    let suggestions = RwSignal::new(Vec::<AutoCompleteOption>::new());
    let value = RwSignal::new(String::new());
    let log = Log::default();
    let cb = completer(suggestions, &log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <AutoComplete suggestions=suggestions value=value group=false delay=10 on_complete=cb /> }
    });
    settle().await;
    input(&host).focus().unwrap();
    type_text(&host, "b");
    sleep(100).await;
    assert!(has(&host, ".p-autocomplete-overlay"));
    mouse(&doc().body().unwrap().into(), "click");
    settle().await;
    assert!(!has(&host, ".p-autocomplete-overlay"), "клик вне закрывает");
    host.remove();
}

#[wasm_bindgen_test]
async fn autocomplete_in_the_browser() {
    typing_queries_and_selects().await;
    keyboard_navigation().await;
    dropdown_clear_and_force_selection().await;
    multiple_chips().await;
    typeahead_off_adds_typed_text().await;
    groups_and_outside_click().await;
}
