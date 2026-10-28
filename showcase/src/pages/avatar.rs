//! Страница Avatar: демонстрации `label`, `icon`, `image`, `shape`, `size`, `template` из showcase PrimeNG (`apps/showcase/doc/avatar`).
//!
//! Иконка `pi-user` заменена `HomeIcon` из `prime-icons`; изображения берутся с CDN PrimeNG, как в оригинале.

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape, AvatarSize};
use prime_icons::HomeIcon;

fn user() -> ViewFn {
    ViewFn::from(|| view! { <HomeIcon /> })
}

const AVATARS: &str = "https://primefaces.org/cdn/primeng/images/demo/avatar";

/// Страница `/avatar`.
#[component]
pub fn AvatarPage() -> impl IntoView {
    let violet = "background-color: #ece9fc; color: #2a1261; margin-right: 0.5rem";
    let blue = "background-color: #dee9fc; color: #1a2551";
    view! {
        <h2>"Avatar"</h2>
        <Demo name="label" title="Label">
            <Avatar label="P" size=AvatarSize::XLarge style="margin-right: 0.5rem" />
            <Avatar label="V" size=AvatarSize::Large style=violet />
            <Avatar label="U" style=blue />
            <Avatar label="P" size=AvatarSize::XLarge shape=AvatarShape::Circle style="margin-right: 0.5rem" />
            <Avatar label="V" size=AvatarSize::Large shape=AvatarShape::Circle style=violet />
            <Avatar label="U" shape=AvatarShape::Circle style=blue />
        </Demo>
        <Demo name="icon" title="Icon">
            <Avatar icon=user() size=AvatarSize::XLarge style="margin-right: 0.5rem" />
            <Avatar icon=user() size=AvatarSize::Large style=violet />
            <Avatar icon=user() style=blue />
            <Avatar icon=user() size=AvatarSize::XLarge shape=AvatarShape::Circle style="margin-right: 0.5rem" />
            <Avatar icon=user() size=AvatarSize::Large shape=AvatarShape::Circle style=violet />
            <Avatar icon=user() shape=AvatarShape::Circle style=blue />
        </Demo>
        <Demo name="image" title="Image">
            <Avatar image=format!("{AVATARS}/amyelsner.png") size=AvatarSize::XLarge shape=AvatarShape::Circle style="margin-right: 0.5rem" />
            <Avatar image=format!("{AVATARS}/asiyajavayant.png") size=AvatarSize::Large shape=AvatarShape::Circle style="margin-right: 0.5rem" />
            <Avatar image=format!("{AVATARS}/onyamalimba.png") shape=AvatarShape::Circle />
        </Demo>
        <Demo name="shape" title="Shape">
            <Avatar label="P" shape=AvatarShape::Circle />
            <Avatar label="T" />
        </Demo>
        <Demo name="size" title="Size">
            <Avatar label="P" size=AvatarSize::Large />
            <Avatar label="T" size=AvatarSize::XLarge />
        </Demo>
        <Demo name="template" title="Template">
            <Avatar size=AvatarSize::XLarge style="background-color: #2a1261; color: #ece9fc">
                <span style="font-weight: 700">"AB"</span>
            </Avatar>
        </Demo>
    }
}
