//! Страница Stepper: демонстрации `basic`, `linear`, `vertical`, `template`, `stepsonly` из showcase PrimeNG
//! (`apps/showcase/doc/stepper`). Значки PrimeIcons (`pi-user`, `pi-id-card`) заменены ближайшими из `prime-icons`, картинка с CDN
//! в `template` — текстом; `ngModel` полей — сигналами.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity, IconPos};
use prime_icons::{ArrowLeftIcon, ArrowRightIcon, CheckIcon, HomeIcon, StarIcon};
use prime_inputtext::InputText;
use prime_password::Password;
use prime_stepper::{Step, StepContext, StepItem, StepList, StepPanel, StepPanels, Stepper};
use prime_togglebutton::ToggleButton;

const BLOCK: &str = "border: 2px dashed var(--p-content-border-color); border-radius: 6px; background: var(--p-content-hover-background); \
                     height: 12rem; display: flex; align-items: center; justify-content: center; font-weight: 500";

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(f)
}

/// Содержимое панели: пунктирный блок и кнопки перехода («Назад»/«Далее»).
fn panel(text: &'static str, back: Option<u32>, next: Option<u32>, c: StepContext) -> AnyView {
    let activate = c.activate;
    let justify = match (back, next) {
        (Some(_), Some(_)) => "space-between",
        (Some(_), None) => "flex-start",
        _ => "flex-end",
    };
    view! {
        <div style="display: flex; flex-direction: column; height: 12rem"><div style={BLOCK}>{text}</div></div>
        <div style={format!("display: flex; padding-top: 1.5rem; justify-content: {justify}")}>
            {back.map(|b| {
                view! {
                    <Button
                        label="Back"
                        severity=ButtonSeverity::Secondary
                        icon=icon(|| view! { <ArrowLeftIcon /> })
                        on_click=Callback::new(move |_| activate.run(b))
                    />
                }
            })}
            {next.map(|n| {
                view! {
                    <Button
                        label="Next"
                        icon=icon(|| view! { <ArrowRightIcon /> })
                        icon_pos=IconPos::Right
                        on_click=Callback::new(move |_| activate.run(n))
                    />
                }
            })}
        </div>
    }
    .into_any()
}

/// Те же три шага и панели для `basic` и `linear`.
macro_rules! wizard {
    ($value:expr, $linear:expr) => {
        view! {
            <Stepper value=$value linear=$linear style="width: 50rem; max-width: 100%">
                <StepList>
                    <Step value=1u32>"Header I"</Step>
                    <Step value=2u32>"Header II"</Step>
                    <Step value=3u32>"Header III"</Step>
                </StepList>
                <StepPanels>
                    <StepPanel value=1u32 content=Callback::new(|c| panel("Content I", None, Some(2), c)) />
                    <StepPanel value=2u32 content=Callback::new(|c| panel("Content II", Some(1), Some(3), c)) />
                    <StepPanel value=3u32 content=Callback::new(|c| panel("Content III", Some(2), None, c)) />
                </StepPanels>
            </Stepper>
        }
    };
}

/// Круглый значок шага для `template`: закрашен, пока значение не больше активного.
fn round_step(active_step: RwSignal<Option<u32>>, value: u32, icon_view: fn() -> AnyView) -> Callback<StepContext, AnyView> {
    Callback::new(move |c: StepContext| {
        let filled = move || active_step.get().is_some_and(|a| value <= a);
        let style = move || {
            let colors = if filled() {
                "background: var(--p-primary-color); color: var(--p-primary-contrast-color); border-color: var(--p-primary-color)"
            } else {
                "border-color: var(--p-content-border-color)"
            };
            format!(
                "border-width: 2px; border-style: solid; border-radius: 9999px; width: 3rem; height: 3rem; display: inline-flex; align-items: center; justify-content: center; {colors}"
            )
        };
        let activate = c.activate;
        view! {
            <button
                type="button"
                data-testid=format!("round-{value}")
                style="background: transparent; border: 0; display: inline-flex; padding: 0"
                on:click=move |_| activate.run(value)
            >
                <span style={style}>{icon_view()}</span>
            </button>
        }
        .into_any()
    })
}

/// Страница `/stepper`.
#[component]
pub fn StepperPage() -> impl IntoView {
    let basic = RwSignal::new(Some(1u32));
    let linear = RwSignal::new(Some(1u32));
    let vertical = RwSignal::new(Some(1u32));
    let active_step = RwSignal::new(Some(1u32));
    let password = RwSignal::new(String::new());
    let interests: Vec<(&'static str, RwSignal<bool>)> =
        ["Nature", "Art", "Music", "Design", "Photography", "Movies", "Sports", "Gaming", "Traveling", "Dancing"]
            .into_iter()
            .map(|label| (label, RwSignal::new(false)))
            .collect();
    view! {
        <Demo name="basic" title="Basic">
            {wizard!(basic, false)}
            <span data-testid="echo">{move || basic.get().map(|v| v.to_string()).unwrap_or_default()}</span>
        </Demo>
        <Demo name="linear" title="Linear">
            {wizard!(linear, true)}
        </Demo>
        <Demo name="vertical" title="Vertical">
            <Stepper value=vertical style="width: 100%">
                <StepItem value=1u32>
                    <Step>"Header I"</Step>
                    <StepPanel content=Callback::new(|c| panel("Content I", None, Some(2), c)) />
                </StepItem>
                <StepItem value=2u32>
                    <Step>"Header II"</Step>
                    <StepPanel content=Callback::new(|c| panel("Content II", Some(1), Some(3), c)) />
                </StepItem>
                <StepItem value=3u32>
                    <Step>"Header III"</Step>
                    <StepPanel content=Callback::new(|c| panel("Content III", Some(2), None, c)) />
                </StepItem>
            </Stepper>
        </Demo>
        <Demo name="template" title="Template">
            <Stepper value=active_step style="width: 40rem; max-width: 100%">
                <StepList>
                    <Step value=1u32 content=round_step(active_step, 1, || view! { <HomeIcon /> }.into_any()) />
                    <Step value=2u32 content=round_step(active_step, 2, || view! { <StarIcon /> }.into_any()) />
                    <Step value=3u32 content=round_step(active_step, 3, || view! { <CheckIcon /> }.into_any()) />
                </StepList>
                <StepPanels>
                    <StepPanel
                        value=1u32
                        content=Callback::new(move |c: StepContext| {
                            let activate = c.activate;
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 0.5rem; margin: 0 auto; min-height: 16rem; max-width: 20rem">
                                    <div style="text-align: center; margin: 1rem 0; font-size: 1.25rem; font-weight: 600">"Create your account"</div>
                                    <InputText placeholder="Name" fluid=true />
                                    <InputText placeholder="Email" fluid=true />
                                    <Password value=password placeholder="Password" fluid=true />
                                </div>
                                <div style="display: flex; padding-top: 1.5rem; justify-content: flex-end">
                                    <Button
                                        label="Next"
                                        icon=icon(|| view! { <ArrowRightIcon /> })
                                        icon_pos=IconPos::Right
                                        on_click=Callback::new(move |_| activate.run(2))
                                    />
                                </div>
                            }
                            .into_any()
                        })
                    />
                    <StepPanel
                        value=2u32
                        content=Callback::new({
                            let interests = interests.clone();
                            move |c: StepContext| {
                                let activate = c.activate;
                                let toggles = interests
                                    .iter()
                                    .map(|(label, checked)| view! { <ToggleButton checked=*checked on_label=*label off_label=*label /> })
                                    .collect_view();
                                view! {
                                    <div style="display: flex; flex-direction: column; gap: 0.5rem; margin: 0 auto; min-height: 16rem; max-width: 24rem">
                                        <div style="text-align: center; margin: 1rem 0; font-size: 1.25rem; font-weight: 600">"Choose your interests"</div>
                                        <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 1rem">{toggles}</div>
                                    </div>
                                    <div style="display: flex; padding-top: 1.5rem; justify-content: space-between">
                                        <Button
                                            label="Back"
                                            severity=ButtonSeverity::Secondary
                                            icon=icon(|| view! { <ArrowLeftIcon /> })
                                            on_click=Callback::new(move |_| activate.run(1))
                                        />
                                        <Button
                                            label="Next"
                                            icon=icon(|| view! { <ArrowRightIcon /> })
                                            icon_pos=IconPos::Right
                                            on_click=Callback::new(move |_| activate.run(3))
                                        />
                                    </div>
                                }
                                .into_any()
                            }
                        })
                    />
                    <StepPanel
                        value=3u32
                        content=Callback::new(move |c: StepContext| {
                            let activate = c.activate;
                            view! {
                                <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem; margin: 0 auto; min-height: 16rem; max-width: 24rem">
                                    <div style="text-align: center; margin: 1rem 0; font-size: 1.25rem; font-weight: 600">"Account created successfully"</div>
                                    <div style="font-size: 4rem">"🎉"</div>
                                </div>
                                <div style="display: flex; padding-top: 1.5rem; justify-content: flex-start">
                                    <Button
                                        label="Back"
                                        severity=ButtonSeverity::Secondary
                                        icon=icon(|| view! { <ArrowLeftIcon /> })
                                        on_click=Callback::new(move |_| activate.run(2))
                                    />
                                </div>
                            }
                            .into_any()
                        })
                    />
                </StepPanels>
            </Stepper>
        </Demo>
        <Demo name="stepsonly" title="Steps Only">
            <Stepper default_value=1u32 style="width: 50rem; max-width: 100%">
                <StepList>
                    <Step value=1u32>"Design"</Step>
                    <Step value=2u32>"Development"</Step>
                    <Step value=3u32>"QA"</Step>
                </StepList>
            </Stepper>
        </Demo>
    }
}
