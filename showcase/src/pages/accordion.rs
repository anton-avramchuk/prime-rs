//! Страница Accordion: демонстрации `basic`, `multiple`, `disabled`, `controlled`, `dynamic` и `template`
//! из showcase PrimeNG (`apps/showcase/doc/accordion`).
//!
//! В `template` иконки «плюс»/«минус» — SVG из `prime-icons` вместо шрифта PrimeIcons.

use super::Demo;
use leptos::prelude::*;
use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel};
use prime_avatar::{Avatar, AvatarShape};
use prime_badge::Badge;
use prime_button::{Button, ButtonVariant};
use prime_icons::{MinusIcon, PlusIcon};

const LOREM_1: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
const LOREM_2: &str = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Consectetur, adipisci velit, sed quia non numquam eius modi.";

fn v(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Три панели с заголовками `Header I…III`.
#[component]
fn Three(#[prop(optional)] disabled_third: bool) -> impl IntoView {
    view! {
        <AccordionPanel value="0">
            <AccordionHeader>"Header I"</AccordionHeader>
            <AccordionContent>
                <p style="margin: 0">{LOREM_1}</p>
            </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="1">
            <AccordionHeader>"Header II"</AccordionHeader>
            <AccordionContent>
                <p style="margin: 0">{LOREM_2}</p>
            </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="2">
            <AccordionHeader>"Header III"</AccordionHeader>
            <AccordionContent>
                <p style="margin: 0">{LOREM_2}</p>
            </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="3" disabled=disabled_third>
            <AccordionHeader>"Header IV"</AccordionHeader>
            <AccordionContent>
                <p style="margin: 0">{LOREM_1}</p>
            </AccordionContent>
        </AccordionPanel>
    }
}

/// Страница `/accordion`.
#[component]
pub fn AccordionPage() -> impl IntoView {
    let active = RwSignal::new(v(&["0"]));
    let tabs = [("Title 1", "Content 1", "0"), ("Title 2", "Content 2", "1"), ("Title 3", "Content 3", "2")];
    let toggle_icon =
        Callback::new(|active: bool| if active { view! { <MinusIcon /> }.into_any() } else { view! { <PlusIcon /> }.into_any() });
    view! {
        <h2>"Accordion"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <Accordion default_value=v(&["0"])>
                    <Three />
                </Accordion>
            </div>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <div style="width: 100%">
                <Accordion multiple=true default_value=v(&["0"])>
                    <Three />
                </Accordion>
            </div>
        </Demo>
        <Demo name="disabled" title="Disabled">
            <div style="width: 100%">
                <Accordion multiple=true default_value=v(&["0"])>
                    <Three disabled_third=true />
                </Accordion>
            </div>
        </Demo>
        <Demo name="controlled" title="Controlled">
            <div style="width: 100%">
                <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-bottom: 1rem">
                    {["0", "1", "2"]
                        .into_iter()
                        .map(|key| {
                            let outlined = move || active.with(|a| a.iter().all(|x| x != key));
                            let variant = Signal::derive(move || outlined().then_some(ButtonVariant::Outlined));
                            view! {
                                <Button
                                    label=(key.parse::<usize>().unwrap_or(0) + 1).to_string()
                                    rounded=true
                                    variant=variant
                                    style="width: 2rem; height: 2rem; padding: 0"
                                    on_click=Callback::new(move |_| active.set(vec![key.to_owned()]))
                                />
                            }
                        })
                        .collect_view()}
                </div>
                <Accordion value=active>
                    <Three />
                </Accordion>
            </div>
        </Demo>
        <Demo name="dynamic" title="Dynamic">
            <div style="width: 100%">
                <Accordion default_value=v(&["0"])>
                    <For each=move || tabs.to_vec() key=|t| t.2 children=|(title, content, value)| {
                        view! {
                            <AccordionPanel value=value>
                                <AccordionHeader>{title}</AccordionHeader>
                                <AccordionContent>
                                    <p style="margin: 0">{content}</p>
                                </AccordionContent>
                            </AccordionPanel>
                        }
                    } />
                </Accordion>
            </div>
        </Demo>
        <Demo name="template" title="Template">
            <div style="width: 100%">
                <Accordion default_value=v(&["0"])>
                    <AccordionPanel value="0">
                        <AccordionHeader toggle_icon_template=toggle_icon>
                            <span style="display: flex; align-items: center; gap: 0.5rem; width: 100%">
                                <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape=AvatarShape::Circle />
                                <span style="font-weight: bold; white-space: nowrap">"Amy Elsner"</span>
                                <span style="margin-left: auto; margin-right: 0.5rem"><Badge value="3" /></span>
                            </span>
                        </AccordionHeader>
                        <AccordionContent>
                            <p style="margin: 0">{LOREM_1}</p>
                        </AccordionContent>
                    </AccordionPanel>
                    <AccordionPanel value="1">
                        <AccordionHeader toggle_icon_template=toggle_icon>
                            <span style="display: flex; align-items: center; gap: 0.5rem; width: 100%">
                                <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/onyamalimba.png" shape=AvatarShape::Circle />
                                <span style="font-weight: bold; white-space: nowrap">"Onyama Limba"</span>
                                <span style="margin-left: auto; margin-right: 0.5rem"><Badge value="4" /></span>
                            </span>
                        </AccordionHeader>
                        <AccordionContent>
                            <p style="margin: 0">{LOREM_2}</p>
                        </AccordionContent>
                    </AccordionPanel>
                </Accordion>
            </div>
        </Demo>
    }
}
