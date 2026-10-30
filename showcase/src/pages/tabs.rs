//! Страница Tabs: демонстрации `basic`, `controlled`, `disabled`, `scrollable`, `dynamic`, `lazy`, `customtemplate`
//! по `apps/showcase/doc/tabs` PrimeNG.
//!
//! Демонстрация `tabmenu` (нужен Router/Menu) не переносится. Значения вкладок — строки; в `customtemplate` значки
//! `pi-minus`/`pi-plus` заменены знаками «−»/«+», утилитные классы Tailwind — встроенными стилями.

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_badge::Badge;
use prime_button::{Button, ButtonVariant};
use prime_tabs::{Tab, TabList, TabPanel, TabPanels, Tabs};

const LOREM: [&str; 3] = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Consectetur, adipisci velit, sed quia non numquam eius modi.",
    "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus.",
];

const TAB_ROW: &str = "display: flex; align-items: center; gap: 0.5rem";

/// Три панели с текстом по значениям `0`, `1`, `2`.
fn lorem_panels() -> impl IntoView {
    view! {
        <TabPanels>
            {LOREM
                .iter()
                .enumerate()
                .map(|(i, text)| view! { <TabPanel value=i.to_string()><p style="margin: 0">{*text}</p></TabPanel> })
                .collect_view()}
        </TabPanels>
    }
}

/// Страница `/tabs`.
#[component]
pub fn TabsPage() -> impl IntoView {
    let controlled = RwSignal::new(Some("0".to_owned()));
    let round = move |label: &'static str, value: &'static str| {
        view! {
            <Button
                label=label
                rounded=true
                variant=Signal::derive(move || (controlled.get().as_deref() != Some(value)).then_some(ButtonVariant::Outlined))
                on_click=Callback::new(move |_| controlled.set(Some(value.to_owned())))
                style="width: 2rem; height: 2rem; padding: 0"
            />
        }
    };
    view! {
        <h2>"Tabs"</h2>
        <Demo name="basic" title="Basic">
            <Tabs default_value="0" style="width: 100%">
                <TabList>
                    <Tab value="0">"Header I"</Tab>
                    <Tab value="1">"Header II"</Tab>
                    <Tab value="2">"Header III"</Tab>
                </TabList>
                {lorem_panels()}
            </Tabs>
        </Demo>
        <Demo name="controlled" title="Controlled">
            <div style="width: 100%">
                <div style="display: flex; margin-bottom: 0.5rem; gap: 0.5rem; justify-content: flex-end">
                    {round("1", "0")} {round("2", "1")} {round("3", "2")}
                </div>
                <Tabs value=controlled>
                    <TabList>
                        <Tab value="0">"Header I"</Tab>
                        <Tab value="1">"Header II"</Tab>
                        <Tab value="2">"Header III"</Tab>
                    </TabList>
                    {lorem_panels()}
                </Tabs>
            </div>
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Tabs default_value="0" style="width: 100%">
                <TabList>
                    <Tab value="0">"Header I"</Tab>
                    <Tab value="1">"Header II"</Tab>
                    <Tab value="2">"Header III"</Tab>
                    <Tab value="3" disabled=true>"Header IV"</Tab>
                </TabList>
                {lorem_panels()}
            </Tabs>
        </Demo>
        <Demo name="scrollable" title="Scrollable">
            <Tabs default_value="0" scrollable=true style="width: 100%">
                <TabList>
                    {(0..50).map(|i| view! { <Tab value=i.to_string()>{format!("Tab {}", i + 1)}</Tab> }).collect_view()}
                </TabList>
                <TabPanels>
                    {(0..50)
                        .map(|i| view! { <TabPanel value=i.to_string()><p style="margin: 0">{format!("Tab {} Content", i + 1)}</p></TabPanel> })
                        .collect_view()}
                </TabPanels>
            </Tabs>
        </Demo>
        <Demo name="dynamic" title="Dynamic">
            <Tabs default_value="0" style="width: 100%">
                <TabList>
                    {(0..3).map(|i| view! { <Tab value=i.to_string()>{format!("Tab {}", i + 1)}</Tab> }).collect_view()}
                </TabList>
                <TabPanels>
                    {(0..3)
                        .map(|i| view! { <TabPanel value=i.to_string()><p style="margin: 0">{format!("Tab {} Content", i + 1)}</p></TabPanel> })
                        .collect_view()}
                </TabPanels>
            </Tabs>
        </Demo>
        <Demo name="lazy" title="Lazy">
            <Tabs default_value="0" lazy=true style="width: 100%">
                <TabList>
                    <Tab value="0">"Header I"</Tab>
                    <Tab value="1">"Header II"</Tab>
                    <Tab value="2">"Header III"</Tab>
                </TabList>
                {lorem_panels()}
            </Tabs>
        </Demo>
        <Demo name="customtemplate" title="Custom Template">
            <Tabs default_value="0" scrollable=true style="width: 100%">
                <TabList
                    prev_icon=ViewFn::from(|| view! { <span>"−"</span> })
                    next_icon=ViewFn::from(|| view! { <span>"+"</span> })
                >
                    <Tab value="0" pt=prime_core::Pt::new().style(TAB_ROW)>
                        <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape=AvatarShape::Circle />
                        <span style="font-weight: 700; white-space: nowrap">"Amy Elsner"</span>
                    </Tab>
                    <Tab value="1" pt=prime_core::Pt::new().style(TAB_ROW)>
                        <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/onyamalimba.png" shape=AvatarShape::Circle />
                        <span style="font-weight: 700; white-space: nowrap">"Onyama Limba"</span>
                    </Tab>
                    <Tab value="2" pt=prime_core::Pt::new().style(TAB_ROW)>
                        <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/ionibowcher.png" shape=AvatarShape::Circle />
                        <span style="font-weight: 700; white-space: nowrap">"Ioni Bowcher"</span>
                        <Badge value="2" />
                    </Tab>
                </TabList>
                {lorem_panels()}
            </Tabs>
        </Demo>
    }
}
