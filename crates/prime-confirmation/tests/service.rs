//! Сервис подтверждений: рассылка, порядок, отписка, контекст и строитель `Confirmation`.
#![cfg(not(target_arch = "wasm32"))]

use leptos::prelude::*;
use prime_confirmation::{
    ConfirmButtonProps, ConfirmEvent, ConfirmEventType, ConfirmFocus, Confirmation, ConfirmationService, provide_confirmation_service,
};

fn log() -> StoredValue<Vec<String>> {
    StoredValue::new(Vec::new())
}

fn recorder(name: &'static str, log: StoredValue<Vec<String>>) -> Callback<ConfirmEvent> {
    Callback::new(move |e: ConfirmEvent| {
        let text = match e {
            ConfirmEvent::Confirm(c) => format!("{name}:confirm:{}", c.message.unwrap_or_default()),
            ConfirmEvent::Close => format!("{name}:close"),
        };
        log.update_value(|l| l.push(text));
    })
}

#[test]
fn builder_sets_fields_and_defaults_stay_none() {
    let c = Confirmation::new().message("Sure?").key("a").accept(Callback::new(|()| {}));
    assert_eq!(c.message.as_deref(), Some("Sure?"));
    assert_eq!(c.key.as_deref(), Some("a"));
    assert!(c.accept.is_some() && c.reject.is_none() && c.target.is_none() && c.default_focus.is_none());
    assert!(c.accept_visible() && c.reject_visible(), "кнопки видимы по умолчанию");
    let c = c.accept_visible_flag(false).reject_visible_flag(true).default_focus(ConfirmFocus::Reject).close_on_escape(false);
    assert!(!c.accept_visible() && c.reject_visible());
    assert_eq!((c.default_focus, c.close_on_escape), (Some(ConfirmFocus::Reject), Some(false)));
}

#[test]
fn button_props_builder() {
    let p = ConfirmButtonProps::new().label("Cancel").outlined(true).text(false);
    assert_eq!((p.label.as_deref(), p.outlined, p.text, p.severity), (Some("Cancel"), Some(true), Some(false), None));
    assert_eq!(ConfirmFocus::default().as_str(), "accept");
    assert_eq!(ConfirmFocus::None.as_str(), "none");
}

#[test]
fn confirm_and_close_reach_subscribers_in_order() {
    Owner::new().with(|| {
        let service = provide_confirmation_service();
        let log = log();
        service.subscribe(recorder("a", log));
        service.subscribe(recorder("b", log));
        service.confirm(Confirmation::new().message("Q"));
        service.close();
        assert_eq!(log.get_value(), ["a:confirm:Q", "b:confirm:Q", "a:close", "b:close"]);
    });
}

#[test]
fn unsubscribed_callbacks_are_not_called() {
    Owner::new().with(|| {
        let service = ConfirmationService::new();
        let log = log();
        let a = service.subscribe(recorder("a", log));
        service.subscribe(recorder("b", log));
        service.unsubscribe(a);
        service.close();
        assert_eq!(log.get_value(), ["b:close"]);
        service.unsubscribe(a);
    });
}

#[test]
fn subscriber_may_unsubscribe_during_delivery() {
    Owner::new().with(|| {
        let service = ConfirmationService::new();
        let log = log();
        let id = StoredValue::new(0usize);
        id.set_value(service.subscribe(Callback::new(move |_| {
            service.unsubscribe(id.get_value());
            log.update_value(|l| l.push("once".into()));
        })));
        service.close();
        service.close();
        assert_eq!(log.get_value(), ["once"]);
    });
}

#[test]
fn service_is_found_in_context() {
    Owner::new().with(|| {
        let provided = provide_confirmation_service();
        let found = prime_confirmation::use_confirmation_service();
        let log = log();
        provided.subscribe(recorder("p", log));
        found.close();
        assert_eq!(log.get_value(), ["p:close"]);
    });
}

#[test]
#[should_panic(expected = "ConfirmationService не найден")]
fn missing_service_panics_with_a_hint() {
    Owner::new().with(|| {
        let _ = prime_confirmation::use_confirmation_service();
    });
}

#[test]
fn event_type_values() {
    assert_eq!(ConfirmEventType::Accept.as_str(), "accept");
    assert_eq!(ConfirmEventType::Reject.as_str(), "reject");
    assert_eq!(ConfirmEventType::Cancel.as_str(), "cancel");
}
