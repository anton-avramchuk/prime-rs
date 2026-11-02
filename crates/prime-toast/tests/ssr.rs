//! Toast в SSR: пустой контейнер с положением и классами; сервис работает без браузера.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_toast::{MessageService, ServiceEvent, Toast, ToastMessage, ToastPosition, ToastPt, ToastSeverity, provide_message_service};
use std::sync::{Arc, Mutex};

fn render(position: ToastPosition) -> String {
    Owner::new().with(|| {
        provide_message_service();
        view! { <Toast position=position style_class="custom" /> }.to_html()
    })
}

#[test]
fn empty_container_by_default() {
    let html = Owner::new().with(|| {
        provide_message_service();
        view! { <Toast /> }.to_html()
    });
    assert!(html.contains("p-toast p-component p-toast-top-right"), "{html}");
    assert!(html.contains("position: fixed") && html.contains("top: 20px") && html.contains("right: 20px"), "{html}");
    assert!(html.contains(r#"data-p="top-right""#), "{html}");
    assert!(!html.contains("p-toast-message"), "{html}");
}

#[test]
fn position_and_classes_follow_props() {
    let html = render(ToastPosition::BottomCenter);
    assert!(html.contains("p-toast-bottom-center") && html.contains("custom"), "{html}");
    assert!(html.contains("bottom: 20px") && html.contains("left: 50%") && !html.contains("top:"), "{html}");
    let html = render(ToastPosition::Center);
    assert!(html.contains("top: 50%") && html.contains("left: 50%"), "{html}");
}

#[test]
fn pt_is_applied_to_the_container() {
    let html = Owner::new().with(|| {
        provide_message_service();
        view! { <Toast pt=ToastPt { root: Pt::new().class("extra").attr("data-x", "1"), ..Default::default() } /> }.to_html()
    });
    assert!(html.contains("extra") && html.contains(r#"data-x="1""#), "{html}");
}

#[test]
#[should_panic(expected = "MessageService не найден")]
fn toast_without_service_panics_with_a_clear_message() {
    Owner::new().with(|| {
        let _ = view! { <Toast /> }.to_html();
    });
}

#[test]
fn service_delivers_events_in_order_and_stops_after_unsubscribe() {
    Owner::new().with(|| {
        let service = MessageService::new();
        let log: Arc<Mutex<Vec<String>>> = Arc::default();
        let l = log.clone();
        let id = service.subscribe(Callback::new(move |e: ServiceEvent| {
            l.lock().unwrap().push(match e {
                ServiceEvent::Add(m) => format!("add:{}", m.len()),
                ServiceEvent::Clear(k) => format!("clear:{}", k.unwrap_or_else(|| "-".into())),
            });
        }));
        service.add(ToastMessage::new(ToastSeverity::Info, "a", "b"));
        service.add_all(vec![ToastMessage::default(), ToastMessage::default()]);
        service.add_all(vec![]);
        service.clear();
        service.clear_key("k");
        assert_eq!(*log.lock().unwrap(), ["add:1", "add:2", "clear:-", "clear:k"], "пустой add_all ничего не шлёт");
        service.unsubscribe(id);
        service.add(ToastMessage::default());
        service.clear();
        assert_eq!(log.lock().unwrap().len(), 4, "после отписки события не приходят");
    });
}

#[test]
fn every_subscriber_gets_every_event() {
    Owner::new().with(|| {
        let service = MessageService::new();
        let counts: Arc<Mutex<[u32; 2]>> = Arc::default();
        for i in 0..2 {
            let c = counts.clone();
            service.subscribe(Callback::new(move |_| c.lock().unwrap()[i] += 1));
        }
        service.clear();
        service.add(ToastMessage::default());
        assert_eq!(*counts.lock().unwrap(), [2, 2]);
    });
}

#[test]
fn message_builder_sets_every_field() {
    let m = ToastMessage::new(ToastSeverity::Warn, "S", "D")
        .key("k")
        .id("i")
        .life(10)
        .sticky(true)
        .closable(false)
        .icon("pi-x")
        .close_icon("pi-y")
        .style_class("a")
        .content_style_class("b");
    assert_eq!(
        (m.summary.as_deref(), m.detail.as_deref(), m.key.as_deref(), m.id.as_deref()),
        (Some("S"), Some("D"), Some("k"), Some("i"))
    );
    assert_eq!((m.life, m.sticky, m.closable), (Some(10), true, false));
    assert_eq!(
        (m.icon.as_deref(), m.close_icon.as_deref(), m.style_class.as_deref(), m.content_style_class.as_deref()),
        (Some("pi-x"), Some("pi-y"), Some("a"), Some("b"))
    );
    let d = ToastMessage::default();
    assert!(d.closable && !d.sticky && d.severity == ToastSeverity::Info && d.key.is_none(), "по умолчанию: info, закрываемое");
    assert_eq!(ToastSeverity::Custom("custom".into()).as_str(), "custom");
}
