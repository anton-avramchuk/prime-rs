//! Разметка Tabs (SSR): части, активность, связь `id`, `hidden`, `disabled`, `lazy`, `scrollable`, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_tabs::{Tab, TabList, TabListPt, TabPanel, TabPanels, Tabs};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

fn render<V: IntoView + 'static>(build: impl FnOnce() -> V) -> String {
    Owner::new().with(|| build().to_html())
}

#[test]
fn structure_and_roles() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="1">
                <TabList>
                    <Tab value="0">"Header I"</Tab>
                    <Tab value="1">"Header II"</Tab>
                </TabList>
                <TabPanels>
                    <TabPanel value="0"><p>"Content I"</p></TabPanel>
                    <TabPanel value="1"><p>"Content II"</p></TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    assert!(html.contains(r#"id="t""#) && html.contains("p-tabs p-component"), "{html}");
    assert!(!html.contains("p-tabs-scrollable"), "{html}");
    for needle in [
        "p-tablist",
        "p-tablist-content p-tablist-viewport",
        "p-tablist-tab-list",
        r#"role="tablist""#,
        "p-tablist-active-bar",
        r#"role="presentation""#,
        r#"data-pc-section="activebar""#,
        "p-tabpanels",
    ] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
    assert_eq!(count(&html, r#"role="tab""#), 2, "{html}");
    assert_eq!(count(&html, r#"role="tabpanel""#), 2, "{html}");
}

#[test]
fn active_tab_and_panel() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="1">
                <TabList>
                    <Tab value="0">"A"</Tab>
                    <Tab value="1">"B"</Tab>
                </TabList>
                <TabPanels>
                    <TabPanel value="0">"first"</TabPanel>
                    <TabPanel value="1">"second"</TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    assert_eq!(count(&html, "p-tab-active"), 1, "{html}");
    assert!(html.contains(r#"aria-selected="true""#) && html.contains(r#"aria-selected="false""#), "{html}");
    assert_eq!(count(&html, "p-tabpanel-active"), 1, "{html}");
    assert_eq!(count(&html, "hidden"), 1, "скрыта только неактивная панель: {html}");
}

#[test]
fn ids_link_tab_and_panel() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="a">
                <TabList><Tab value="a">"A"</Tab></TabList>
                <TabPanels><TabPanel value="a">"x"</TabPanel></TabPanels>
            </Tabs>
        }
    });
    for needle in [r#"id="t_tab_a""#, r#"aria-controls="t_tabpanel_a""#, r#"id="t_tabpanel_a""#, r#"aria-labelledby="t_tab_a""#] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn roving_tabindex() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="1" tabindex=3>
                <TabList>
                    <Tab value="0">"A"</Tab>
                    <Tab value="1">"B"</Tab>
                    <Tab value="2" disabled=true>"C"</Tab>
                </TabList>
            </Tabs>
        }
    });
    assert_eq!(count(&html, r#"tabindex="3""#), 1, "{html}");
    assert_eq!(count(&html, r#"tabindex="-1""#), 2, "{html}");
}

#[test]
fn disabled_tab_attributes() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="0">
                <TabList>
                    <Tab value="0">"A"</Tab>
                    <Tab value="1" disabled=true>"B"</Tab>
                </TabList>
            </Tabs>
        }
    });
    assert!(
        html.contains("p-tab p-disabled") && html.contains(r#"data-p-disabled="true""#) && html.contains(r#"aria-disabled="true""#),
        "{html}"
    );
    assert_eq!(count(&html, "data-p-disabled"), 1, "{html}");
}

#[test]
fn no_value_means_nothing_active() {
    let html = render(|| {
        view! {
            <Tabs id="t">
                <TabList><Tab value="0">"A"</Tab></TabList>
                <TabPanels><TabPanel value="0">"x"</TabPanel></TabPanels>
            </Tabs>
        }
    });
    assert!(!html.contains("p-tab-active") && !html.contains("p-tabpanel-active"), "{html}");
    assert!(html.contains("hidden"), "{html}");
}

#[test]
fn external_value_signal() {
    let value = RwSignal::new(Some("1".to_owned()));
    let html = render(|| {
        view! {
            <Tabs id="t" value=value>
                <TabList>
                    <Tab value="0">"A"</Tab>
                    <Tab value="1">"B"</Tab>
                </TabList>
            </Tabs>
        }
    });
    assert!(html.contains(r#"id="t_tab_1""#) && count(&html, "p-tab-active") == 1, "{html}");
}

#[test]
fn lazy_panels_render_only_when_active() {
    let html = render(|| {
        view! {
            <Tabs id="t" default_value="0" lazy=true>
                <TabPanels>
                    <TabPanel value="0">"visible-content"</TabPanel>
                    <TabPanel value="1">"deferred-content"</TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    assert!(html.contains("visible-content") && !html.contains("deferred-content"), "{html}");
    let own = render(|| {
        view! {
            <Tabs id="t" default_value="0">
                <TabPanels>
                    <TabPanel value="0">"visible-content"</TabPanel>
                    <TabPanel value="1" lazy=true>"deferred-content"</TabPanel>
                    <TabPanel value="2">"eager-content"</TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    assert!(!own.contains("deferred-content") && own.contains("eager-content"), "{own}");
    let eager = render(|| {
        view! {
            <Tabs id="t" default_value="0">
                <TabPanels>
                    <TabPanel value="1">"hidden-but-rendered"</TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    assert!(eager.contains("hidden-but-rendered"), "{eager}");
}

#[test]
fn scrollable_class_and_no_buttons_on_server() {
    let html = render(|| {
        view! {
            <Tabs id="t" scrollable=true>
                <TabList><Tab value="0">"A"</Tab></TabList>
            </Tabs>
        }
    });
    assert!(html.contains("p-tabs-scrollable"), "{html}");
    assert!(!html.contains("p-tablist-nav-button"), "кнопки появляются только на клиенте: {html}");
}

#[test]
fn style_class_and_pt() {
    let list_pt = TabListPt {
        root: Pt::new().class("list-pt"),
        content: Pt::new().class("content-pt"),
        tab_list: Pt::new().class("tabs-pt"),
        active_bar: Pt::new().class("bar-pt"),
        ..Default::default()
    };
    let html = render(|| {
        view! {
            <Tabs id="t" style="width: 20rem" style_class="mine" pt=Pt::new().class("from-pt").attr("data-testid", "root")>
                <TabList pt=list_pt>
                    <Tab value="0" pt=Pt::new().class("tab-pt")>"A"</Tab>
                </TabList>
                <TabPanels pt=Pt::new().class("panels-pt")>
                    <TabPanel value="0" pt=Pt::new().class("panel-pt")>"x"</TabPanel>
                </TabPanels>
            </Tabs>
        }
    });
    for needle in [
        "mine",
        "from-pt",
        "width: 20rem",
        r#"data-testid="root""#,
        "list-pt",
        "content-pt",
        "tabs-pt",
        "bar-pt",
        "tab-pt",
        "panels-pt",
        "panel-pt",
    ] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn default_ids_are_unique() {
    let one = render(|| view! { <Tabs><TabList><Tab value="0">"A"</Tab></TabList></Tabs> });
    let two = render(|| view! { <Tabs><TabList><Tab value="0">"A"</Tab></TabList></Tabs> });
    let id_of = |html: &str| html.split(r#"id="pn_id_"#).nth(1).and_then(|rest| rest.split('"').next()).map(str::to_owned);
    assert!(id_of(&one).is_some() && id_of(&one) != id_of(&two), "{one} / {two}");
}
