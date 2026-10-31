//! Страница Dialog: демонстрации `basic`, `position`, `modal`, `withoutmodal`, `maximizable`, `responsive`, `longcontent`,
//! `template`, `headless` по `apps/showcase/doc/dialog` PrimeNG.
//!
//! Демонстрация `overlaysinside` не переносится (у `Select` нет `appendTo`, его панель обрежется границами окна). `headless`
//! упрощена: форма входа без логотипа и градиента, а `backdrop-blur` заменён встроенным стилем маски. Значков `external-link`,
//! `user` и `arrow-*-right/left` (диагональных) в `prime-icons` нет, поэтому у кнопок только подписи. Размеры окон задаются
//! встроенными стилями (в PrimeNG — Tailwind).

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_button::{Button, ButtonSeverity};
use prime_dialog::{Dialog, DialogPosition};
use prime_inputtext::InputText;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

const LONG: [&str; 3] = [
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.",
    "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus.",
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.",
];

fn field(label: &'static str, id: &'static str) -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem">
            <label for=id style="font-weight: 600; width: 6rem">{label}</label>
            <InputText id=id style="flex: 1" />
        </div>
    }
}

/// Ширина окна по ширине экрана, как в демонстрациях PrimeNG.
fn breakpoints() -> Vec<(String, String)> {
    vec![("1199px".to_owned(), "75vw".to_owned()), ("575px".to_owned(), "90vw".to_owned())]
}

/// Форма «Edit Profile» из нескольких демонстраций PrimeNG.
fn profile_form(visible: RwSignal<bool>) -> impl IntoView {
    view! {
        <span style="display: block; margin-bottom: 2rem; color: var(--p-text-muted-color)">"Update your information."</span>
        {field("Username", "username")}
        {field("Email", "email")}
        <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem">
            <Button label="Cancel" severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| visible.set(false)) />
            <Button label="Save" on_click=Callback::new(move |_| visible.set(false)) />
        </div>
    }
}

/// Страница `/dialog`.
#[component]
pub fn DialogPage() -> impl IntoView {
    let basic = RwSignal::new(false);
    let positioned = RwSignal::new(false);
    let position = RwSignal::new(DialogPosition::Center);
    let modal = RwSignal::new(false);
    let nonmodal = RwSignal::new(false);
    let maximizable = RwSignal::new(false);
    let responsive = RwSignal::new(false);
    let long = RwSignal::new(false);
    let template = RwSignal::new(false);
    let headless = RwSignal::new(false);
    let open = |signal: RwSignal<bool>| Callback::new(move |_: web_sys::MouseEvent| signal.set(true));
    let show_at = move |p: DialogPosition| {
        Callback::new(move |_: web_sys::MouseEvent| {
            position.set(p);
            positioned.set(true);
        })
    };
    let secondary = move |label: &'static str, p: DialogPosition| {
        view! { <Button label=label severity=ButtonSeverity::Secondary style="min-width: 10rem" on_click=show_at(p) /> }
    };

    view! {
        <h2>"Dialog"</h2>
        <Demo name="basic" title="Basic">
            <Button label="Show" on_click=open(basic) />
            <Dialog visible=basic header="Edit Profile" modal=true style="width: 25rem">
                {profile_form(basic)}
            </Dialog>
        </Demo>
        <Demo name="position" title="Position">
            <div style="width: 100%">
                <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem; margin-bottom: 0.5rem">
                    {secondary("Left", DialogPosition::Left)} {secondary("Right", DialogPosition::Right)}
                </div>
                <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem; margin-bottom: 0.5rem">
                    {secondary("TopLeft", DialogPosition::TopLeft)} {secondary("Top", DialogPosition::Top)}
                    {secondary("TopRight", DialogPosition::TopRight)}
                </div>
                <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem">
                    {secondary("BottomLeft", DialogPosition::BottomLeft)} {secondary("Bottom", DialogPosition::Bottom)}
                    {secondary("BottomRight", DialogPosition::BottomRight)}
                </div>
            </div>
            <Dialog visible=positioned header="Edit Profile" modal=true position=Signal::derive(move || position.get()) style="width: 25rem">
                {profile_form(positioned)}
            </Dialog>
        </Demo>
        <Demo name="modal" title="Modal">
            <Button label="Show" on_click=open(modal) />
            <Dialog visible=modal header="Edit Profile" modal=true style="width: 25rem">
                {profile_form(modal)}
            </Dialog>
        </Demo>
        <Demo name="withoutmodal" title="Without Modal">
            <Button label="Show" on_click=open(nonmodal) />
            <Dialog visible=nonmodal header="Edit Profile" style="width: 25rem">
                {profile_form(nonmodal)}
            </Dialog>
        </Demo>
        <Demo name="maximizable" title="Maximizable">
            <Button label="Show" on_click=open(maximizable) />
            <Dialog visible=maximizable header="Header" modal=true maximizable=true style="width: 50rem" breakpoints=breakpoints()>
                <p style="margin: 0">{LOREM}</p>
            </Dialog>
        </Demo>
        <Demo name="responsive" title="Responsive">
            <Button label="Show" on_click=open(responsive) />
            <Dialog
                visible=responsive
                header="Header"
                modal=true
                draggable=false
                resizable=false
                style="width: 50vw"
                breakpoints=breakpoints()
            >
                <p style="margin: 0">{LOREM}</p>
            </Dialog>
        </Demo>
        <Demo name="longcontent" title="Long Content">
            <Button label="Show" on_click=open(long) />
            <Dialog visible=long header="Header" modal=true style="width: 50rem" breakpoints=breakpoints()>
                <p style="margin: 0 0 2rem">{LOREM}</p>
                {LONG.iter().chain(LONG.iter()).map(|text| view! { <p style="margin: 0 0 2rem">{*text}</p> }).collect_view()}
                <p style="margin: 0">{LOREM}</p>
            </Dialog>
        </Demo>
        <Demo name="template" title="Template">
            <Button label="Show" on_click=open(template) />
            <Dialog
                visible=template
                modal=true
                style="width: 25rem"
                header_content=ViewFn::from(|| {
                    view! {
                        <div style="display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem">
                            <Avatar label="AE" shape=AvatarShape::Circle />
                            <span style="font-weight: 700; white-space: nowrap">"Amy Elsner"</span>
                        </div>
                    }
                })
                footer=ViewFn::from(move || {
                    view! {
                        <Button label="Cancel" text=true severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| template.set(false)) />
                        <Button label="Save" outlined=true severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| template.set(false)) />
                    }
                })
            >
                <span style="display: block; margin-bottom: 2rem; color: var(--p-text-muted-color)">"Update your information."</span>
                {field("Username", "template-username")}
                {field("Email", "template-email")}
            </Dialog>
        </Demo>
        <Demo name="headless" title="Headless">
            <Button label="Login" on_click=open(headless) />
            <Dialog
                visible=headless
                mask_style="backdrop-filter: blur(4px)"
                style="border: 0; background: transparent"
                headless=ViewFn::from(move || {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 1.5rem; padding: 2rem; border-radius: 12px; background: var(--p-primary-color); color: var(--p-primary-contrast-color)">
                            <span style="font-weight: 600; font-size: 1.5rem; text-align: center">"Your Logo"</span>
                            <InputText placeholder="Username" id="headless-username" />
                            <InputText placeholder="Password" id="headless-password" input_type="password" />
                            <div style="display: flex; align-items: center; gap: 1rem">
                                <Button label="Sign-In" text=true style="flex: 1" on_click=Callback::new(move |_| headless.set(false)) />
                                <Button label="Cancel" text=true style="flex: 1" on_click=Callback::new(move |_| headless.set(false)) />
                            </div>
                        </div>
                    }
                })
            >
                <p>"Unused"</p>
            </Dialog>
        </Demo>
    }
}
