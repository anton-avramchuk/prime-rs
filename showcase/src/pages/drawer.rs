//! Страница Drawer: демонстрации `basic`, `position`, `size`, `fullscreen`, `template`, `headless` по `apps/showcase/doc/drawer` PrimeNG.
//!
//! Значков `plus`-аватара, `user`, `sign-out` и логотипа в `prime-icons` нет, поэтому кнопки подвала в `template` без значков, а
//! `headless` упрощена: заголовок с кнопкой закрытия и список ссылок вместо меню профиля с `Ripple` и `StyleClass`. Размер в `size`
//! задаётся встроенным стилем (в PrimeNG — утилиты Tailwind).

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_button::{Button, ButtonSeverity};
use prime_drawer::{Drawer, DrawerPosition};
use prime_icons::{ArrowDownIcon, ArrowLeftIcon, ArrowRightIcon, ArrowUpIcon, PlusIcon, TimesIcon, WindowMaximizeIcon};

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

/// Страница `/drawer`.
#[component]
pub fn DrawerPage() -> impl IntoView {
    let basic = RwSignal::new(false);
    let (left, right, top, bottom) = (RwSignal::new(false), RwSignal::new(false), RwSignal::new(false), RwSignal::new(false));
    let size = RwSignal::new(false);
    let full = RwSignal::new(false);
    let template = RwSignal::new(false);
    let headless = RwSignal::new(false);
    let open = |signal: RwSignal<bool>| Callback::new(move |_: web_sys::MouseEvent| signal.set(true));

    view! {
        <h2>"Drawer"</h2>
        <Demo name="basic" title="Basic">
            <Drawer visible=basic header="Drawer">
                <p>{LOREM}</p>
            </Drawer>
            <Button icon=ViewFn::from(|| view! { <ArrowRightIcon /> }) on_click=open(basic) />
        </Demo>
        <Demo name="position" title="Position">
            <Drawer visible=left header="Left Drawer" position=DrawerPosition::Left>
                <p>{LOREM}</p>
            </Drawer>
            <Drawer visible=right header="Right Drawer" position=DrawerPosition::Right>
                <p>{LOREM}</p>
            </Drawer>
            <Drawer visible=top header="Top Drawer" position=DrawerPosition::Top style="height: auto">
                <p>{LOREM}</p>
            </Drawer>
            <Drawer visible=bottom header="Bottom Drawer" position=DrawerPosition::Bottom style="height: auto">
                <p>{LOREM}</p>
            </Drawer>
            <div style="display: flex; gap: 0.5rem; justify-content: center">
                <Button button_type="button" icon=ViewFn::from(|| view! { <ArrowRightIcon /> }) on_click=open(left) />
                <Button button_type="button" icon=ViewFn::from(|| view! { <ArrowLeftIcon /> }) on_click=open(right) />
                <Button button_type="button" icon=ViewFn::from(|| view! { <ArrowDownIcon /> }) on_click=open(top) />
                <Button button_type="button" icon=ViewFn::from(|| view! { <ArrowUpIcon /> }) on_click=open(bottom) />
            </div>
        </Demo>
        <Demo name="size" title="Size">
            <Drawer visible=size header="Drawer" style="width: min(100%, 30rem)">
                <p>{LOREM}</p>
            </Drawer>
            <Button icon=ViewFn::from(|| view! { <ArrowRightIcon /> }) on_click=open(size) />
        </Demo>
        <Demo name="fullscreen" title="Full Screen">
            <Drawer visible=full header="Drawer" full_screen=true>
                <p>{LOREM}</p>
            </Drawer>
            <Button icon=ViewFn::from(|| view! { <WindowMaximizeIcon /> }) on_click=open(full) />
        </Demo>
        <Demo name="template" title="Template">
            <Drawer
                visible=template
                closable=false
                header_content=ViewFn::from(|| {
                    view! {
                        <div style="display: flex; align-items: center; gap: 0.5rem">
                            <Avatar label="AE" shape=AvatarShape::Circle />
                            <span style="font-weight: 700">"Amy Elsner"</span>
                        </div>
                    }
                })
                footer=ViewFn::from(move || {
                    view! {
                        <div style="display: flex; align-items: center; gap: 0.5rem">
                            <Button label="Account" outlined=true style="width: 100%" />
                            <Button label="Logout" severity=ButtonSeverity::Danger text=true style="width: 100%" />
                        </div>
                    }
                })
            >
                <p>{LOREM}</p>
            </Drawer>
            <Button icon=ViewFn::from(|| view! { <PlusIcon /> }) on_click=open(template) />
        </Demo>
        <Demo name="headless" title="Headless">
            <Drawer
                visible=headless
                headless=ViewFn::from(move || {
                    view! {
                        <div style="display: flex; flex-direction: column; height: 100%">
                            <div style="display: flex; align-items: center; justify-content: space-between; padding: 1rem 1.5rem 0">
                                <span style="font-weight: 600; font-size: 1.5rem; color: var(--p-primary-color)">"Your Logo"</span>
                                <Button
                                    button_type="button"
                                    rounded=true
                                    outlined=true
                                    icon=ViewFn::from(|| view! { <TimesIcon /> })
                                    on_click=Callback::new(move |_: web_sys::MouseEvent| headless.set(false))
                                />
                            </div>
                            <ul style="list-style: none; margin: 0; padding: 1rem 1.5rem; flex: 1">
                                <li style="padding: 0.75rem 0">"Dashboard"</li>
                                <li style="padding: 0.75rem 0">"Bookmarks"</li>
                                <li style="padding: 0.75rem 0">"Settings"</li>
                            </ul>
                            <div style="display: flex; align-items: center; gap: 0.5rem; padding: 1rem 1.5rem; border-top: 1px solid var(--p-content-border-color)">
                                <Avatar label="AE" shape=AvatarShape::Circle />
                                <span style="font-weight: 700">"Amy Elsner"</span>
                            </div>
                        </div>
                    }
                })
            >
                <p>"Unused"</p>
            </Drawer>
            <Button label="Show" on_click=open(headless) />
        </Demo>
    }
}
