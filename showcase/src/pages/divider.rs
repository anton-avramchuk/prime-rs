//! Страница Divider: демонстрации `basic`, `content`, `type`, `vertical`, `login` из showcase PrimeNG (`apps/showcase/doc/divider`).
//!
//! Тексты-заполнители сокращены.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_divider::{Divider, DividerAlign, DividerLayout, DividerType};
use prime_inputtext::InputText;

const LOREM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

/// Страница `/divider`.
#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <h2>"Divider"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <p>{LOREM}</p>
                <Divider />
                <p>{LOREM}</p>
            </div>
        </Demo>
        <Demo name="content" title="Content">
            <div style="width: 100%">
                <p>{LOREM}</p>
                <Divider align=DividerAlign::Left divider_type=DividerType::Solid>
                    <b>"Left"</b>
                </Divider>
                <p>{LOREM}</p>
                <Divider align=DividerAlign::Center divider_type=DividerType::Dotted>
                    <b>"Center"</b>
                </Divider>
                <p>{LOREM}</p>
                <Divider align=DividerAlign::Right divider_type=DividerType::Dashed>
                    <b>"Right"</b>
                </Divider>
            </div>
        </Demo>
        <Demo name="type" title="Type">
            <div style="width: 100%">
                <p>{LOREM}</p>
                <Divider divider_type=DividerType::Solid />
                <p>{LOREM}</p>
                <Divider divider_type=DividerType::Dotted />
                <p>{LOREM}</p>
                <Divider divider_type=DividerType::Dashed />
                <p>{LOREM}</p>
            </div>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <div style="display: flex; width: 100%">
                <p style="flex: 1">{LOREM}</p>
                <Divider layout=DividerLayout::Vertical />
                <p style="flex: 1">{LOREM}</p>
                <Divider layout=DividerLayout::Vertical align=DividerAlign::Top>
                    <b>"OR"</b>
                </Divider>
                <p style="flex: 1">{LOREM}</p>
            </div>
        </Demo>
        <Demo name="login" title="Login">
            <div style="display: flex; width: 100%">
                <div style="flex: 5; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0.75rem; padding: 1.25rem 0">
                    <div style="display: flex; flex-direction: column; gap: 0.5rem">
                        <label for="username">"Username"</label>
                        <InputText id="username" />
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 0.5rem">
                        <label for="password">"Password"</label>
                        <InputText id="password" input_type="password" />
                    </div>
                    <Button label="Login" />
                </div>
                <div style="flex: 2">
                    <Divider layout=DividerLayout::Vertical>
                        <b>"OR"</b>
                    </Divider>
                </div>
                <div style="flex: 5; display: flex; align-items: center; justify-content: center; padding: 1.25rem 0">
                    <Button label="Sign Up" severity=ButtonSeverity::Success />
                </div>
            </div>
        </Demo>
    }
}
