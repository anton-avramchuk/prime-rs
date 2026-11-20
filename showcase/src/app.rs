//! Page shell, menu and routes.

use crate::doc::{DOC_CSS, DocNav, DocPage, SITE_CSS};
use crate::pages::InstallPage;
use crate::pages::{
    AccordionPage, AutoCompletePage, AvatarPage, BadgePage, BlockUiPage, BreadcrumbPage, ButtonGroupPage, ButtonPage, CardPage,
    CarouselPage, CascadeSelectPage, CheckboxPage, ChipPage, ColorPickerPage, ConfirmDialogPage, ConfirmPopupPage, ContextMenuPage,
    DataViewPage, DatePickerPage, DialogPage, DividerPage, DockPage, DrawerPage, FieldsetPage, FileUploadPage, FloatLabelPage, FluidPage,
    GalleriaPage, HomePage, IconFieldPage, IftaLabelPage, ImageComparePage, ImagePage, InplacePage, InputGroupPage, InputMaskPage,
    InputNumberPage, InputOtpPage, InputTextPage, KnobPage, ListboxPage, MegaMenuPage, MenuPage, MenubarPage, MessagePage, MeterGroupPage,
    MultiSelectPage, OrderListPage, OverlayBadgePage, PaginatorPage, PanelMenuPage, PanelPage, PasswordPage, PickListPage, PopoverPage,
    ProgressBarPage, ProgressSpinnerPage, RadioButtonPage, RatingPage, ScrollPanelPage, ScrollTopPage, SelectButtonPage, SelectPage,
    SkeletonPage, SliderPage, SpeedDialPage, SplitButtonPage, SplitterPage, StepperPage, StepsPage, TablePage, TabsPage, TagPage,
    TextareaPage, TieredMenuPage, TimelinePage, ToastPage, ToggleButtonPage, ToggleSwitchPage, ToolbarPage, TooltipPage, TreePage,
    TreeSelectPage, TreeTablePage,
};
use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
#[cfg(feature = "split")]
use leptos_router::lazy_route;
use leptos_router::{Lazy, LazyRoute, StaticSegment};

/// Declares a lazy route that renders a component page inside [`DocPage`].
///
/// With the `split` feature (`cargo xtask pages`, built with `cargo leptos build --split`) the view is a
/// `#[lazy_route]`, so wasm-split moves the page into its own chunk. `#[lazy]` code only runs after that split step
/// (it imports a placeholder module the step replaces), so without the feature (`cargo leptos serve`, tests) the
/// view is a plain async function and everything stays in one wasm.
macro_rules! lazy_page {
    ($route:ident, $name:literal, $page:ident) => {
        struct $route;

        #[cfg(feature = "split")]
        #[lazy_route]
        impl LazyRoute for $route {
            fn data() -> Self {
                $route
            }

            fn view(_this: Self) -> AnyView {
                view! { <DocPage name=$name><$page /></DocPage> }.into_any()
            }
        }

        #[cfg(not(feature = "split"))]
        impl LazyRoute for $route {
            fn data() -> Self {
                $route
            }

            async fn view(_this: Self) -> AnyView {
                view! { <DocPage name=$name><$page /></DocPage> }.into_any()
            }
        }
    };
}

const DARK_CLASS: &str = "p-dark";

/// Base path the app is served under (set at build time, e.g. `/prime-rs` for GitHub Pages); empty by default.
const ROUTER_BASE: &str = match option_env!("SHOWCASE_BASE") {
    Some(base) => base,
    None => "",
};

/// Prefixes an app path with the router base, so links keep working under the Pages sub-path.
/// `<A href>` only lets the router handle a click when the href already contains the base.
pub(crate) fn route(path: &str) -> String {
    format!("{ROUTER_BASE}{path}")
}

/// Theme components used by the showcase, one per line; also read by `cargo xtask showcase-css` for the static build.
pub const THEME_COMPONENTS: &str = include_str!("../theme-components.txt");

/// Theme CSS: base styles plus the components used by the showcase.
pub fn theme_css() -> String {
    let names: Vec<&str> = THEME_COMPONENTS.lines().filter(|l| !l.is_empty()).collect();
    prime_theme::stylesheet(&names)
}

/// Site stylesheet: the theme, the doc page styles and the site layout; built once.
fn site_css() -> &'static str {
    static CSS: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CSS.get_or_init(|| {
        let mut css = theme_css();
        css.push_str(DOC_CSS);
        css.push_str(SITE_CSS);
        css
    })
}

/// HTML shell. The styles are inlined ahead of the hydration scripts: a linked stylesheet would compete with the
/// wasm preloads for bandwidth and delay the first paint of the pre-rendered page (measured about 4.8 s instead of
/// 0.55 s on a slow connection).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"prime-rs"</title>
                <style>{site_css()}</style>
                <AutoReload options=options.clone() />
                <HydrationScripts options root=ROUTER_BASE />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Toggles the dark scheme class on `<html>`; does nothing on the server.
fn toggle_dark() {
    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = root.class_list().toggle(DARK_CLASS);
    }
}

/// Root component of the showcase.
#[component]
pub fn App() -> impl IntoView {
    view! {
            <Router base=ROUTER_BASE>
                <div data-testid="app" style="font-family: sans-serif; padding: 1.5rem; background: var(--p-content-background); color: var(--p-text-color); min-height: 100vh">
                    <header style="display: flex; align-items: center; justify-content: space-between">
                        <h1>"prime-rs"</h1>
                                            <button type="button" data-testid="toggle-dark" on:click=|_| toggle_dark()>
                            "Light / dark scheme"
                        </button>
                    </header>
                    <div class="doc-shell">
    <DocNav />
    <main>
                        <Routes fallback=|| "Page not found">
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=StaticSegment("install") view=InstallPage />
                            <Route path=StaticSegment("button") view={Lazy::<ButtonRoute>::new()} />
                            <Route path=StaticSegment("badge") view={Lazy::<BadgeRoute>::new()} />
                            <Route path=StaticSegment("tag") view={Lazy::<TagRoute>::new()} />
                            <Route path=StaticSegment("chip") view={Lazy::<ChipRoute>::new()} />
                            <Route path=StaticSegment("avatar") view={Lazy::<AvatarRoute>::new()} />
                            <Route path=StaticSegment("divider") view={Lazy::<DividerRoute>::new()} />
                            <Route path=StaticSegment("card") view={Lazy::<CardRoute>::new()} />
                            <Route path=StaticSegment("inputtext") view={Lazy::<InputTextRoute>::new()} />
                            <Route path=StaticSegment("checkbox") view={Lazy::<CheckboxRoute>::new()} />
                            <Route path=StaticSegment("radiobutton") view={Lazy::<RadioButtonRoute>::new()} />
                            <Route path=StaticSegment("togglebutton") view={Lazy::<ToggleButtonRoute>::new()} />
                            <Route path=StaticSegment("selectbutton") view={Lazy::<SelectButtonRoute>::new()} />
                            <Route path=StaticSegment("rating") view={Lazy::<RatingRoute>::new()} />
                            <Route path=StaticSegment("slider") view={Lazy::<SliderRoute>::new()} />
                            <Route path=StaticSegment("knob") view={Lazy::<KnobRoute>::new()} />
                            <Route path=StaticSegment("inputnumber") view={Lazy::<InputNumberRoute>::new()} />
                            <Route path=StaticSegment("select") view={Lazy::<SelectRoute>::new()} />
                            <Route path=StaticSegment("toggleswitch") view={Lazy::<ToggleSwitchRoute>::new()} />
                            <Route path=StaticSegment("message") view={Lazy::<MessageRoute>::new()} />
                            <Route path=StaticSegment("progressbar") view={Lazy::<ProgressBarRoute>::new()} />
                            <Route path=StaticSegment("skeleton") view={Lazy::<SkeletonRoute>::new()} />
                            <Route path=StaticSegment("textarea") view={Lazy::<TextareaRoute>::new()} />
                            <Route path=StaticSegment("panel") view={Lazy::<PanelRoute>::new()} />
                            <Route path=StaticSegment("fieldset") view={Lazy::<FieldsetRoute>::new()} />
                            <Route path=StaticSegment("accordion") view={Lazy::<AccordionRoute>::new()} />
                            <Route path=StaticSegment("toolbar") view={Lazy::<ToolbarRoute>::new()} />
                            <Route path=StaticSegment("scrollpanel") view={Lazy::<ScrollPanelRoute>::new()} />
                            <Route path=StaticSegment("progressspinner") view={Lazy::<ProgressSpinnerRoute>::new()} />
                            <Route path=StaticSegment("inputgroup") view={Lazy::<InputGroupRoute>::new()} />
                            <Route path=StaticSegment("buttongroup") view={Lazy::<ButtonGroupRoute>::new()} />
                            <Route path=StaticSegment("fluid") view={Lazy::<FluidRoute>::new()} />
                            <Route path=StaticSegment("iconfield") view={Lazy::<IconFieldRoute>::new()} />
                            <Route path=StaticSegment("floatlabel") view={Lazy::<FloatLabelRoute>::new()} />
                            <Route path=StaticSegment("iftalabel") view={Lazy::<IftaLabelRoute>::new()} />
                            <Route path=StaticSegment("overlaybadge") view={Lazy::<OverlayBadgeRoute>::new()} />
                            <Route path=StaticSegment("timeline") view={Lazy::<TimelineRoute>::new()} />
                            <Route path=StaticSegment("metergroup") view={Lazy::<MeterGroupRoute>::new()} />
                            <Route path=StaticSegment("breadcrumb") view={Lazy::<BreadcrumbRoute>::new()} />
                            <Route path=StaticSegment("inputotp") view={Lazy::<InputOtpRoute>::new()} />
                            <Route path=StaticSegment("scrolltop") view={Lazy::<ScrollTopRoute>::new()} />
                            <Route path=StaticSegment("password") view={Lazy::<PasswordRoute>::new()} />
                            <Route path=StaticSegment("inplace") view={Lazy::<InplaceRoute>::new()} />
                            <Route path=StaticSegment("splitter") view={Lazy::<SplitterRoute>::new()} />
                            <Route path=StaticSegment("tabs") view={Lazy::<TabsRoute>::new()} />
                            <Route path=StaticSegment("popover") view={Lazy::<PopoverRoute>::new()} />
                            <Route path=StaticSegment("drawer") view={Lazy::<DrawerRoute>::new()} />
                            <Route path=StaticSegment("dialog") view={Lazy::<DialogRoute>::new()} />
                            <Route path=StaticSegment("tooltip") view={Lazy::<TooltipRoute>::new()} />
                            <Route path=StaticSegment("toast") view={Lazy::<ToastRoute>::new()} />
                            <Route path=StaticSegment("menu") view={Lazy::<MenuRoute>::new()} />
                            <Route path=StaticSegment("listbox") view={Lazy::<ListboxRoute>::new()} />
                            <Route path=StaticSegment("multiselect") view={Lazy::<MultiSelectRoute>::new()} />
                            <Route path=StaticSegment("paginator") view={Lazy::<PaginatorRoute>::new()} />
                            <Route path=StaticSegment("stepper") view={Lazy::<StepperRoute>::new()} />
                            <Route path=StaticSegment("imagecompare") view={Lazy::<ImageCompareRoute>::new()} />
                            <Route path=StaticSegment("blockui") view={Lazy::<BlockUiRoute>::new()} />
                            <Route path=StaticSegment("splitbutton") view={Lazy::<SplitButtonRoute>::new()} />
                            <Route path=StaticSegment("dock") view={Lazy::<DockRoute>::new()} />
                            <Route path=StaticSegment("image") view={Lazy::<ImageRoute>::new()} />
                            <Route path=StaticSegment("confirmpopup") view={Lazy::<ConfirmPopupRoute>::new()} />
                            <Route path=StaticSegment("confirmdialog") view={Lazy::<ConfirmDialogRoute>::new()} />
                            <Route path=StaticSegment("speeddial") view={Lazy::<SpeedDialRoute>::new()} />
                            <Route path=StaticSegment("carousel") view={Lazy::<CarouselRoute>::new()} />
                            <Route path=StaticSegment("tieredmenu") view={Lazy::<TieredMenuRoute>::new()} />
                            <Route path=StaticSegment("steps") view={Lazy::<StepsRoute>::new()} />
                            <Route path=StaticSegment("contextmenu") view={Lazy::<ContextMenuRoute>::new()} />
                            <Route path=StaticSegment("menubar") view={Lazy::<MenubarRoute>::new()} />
                            <Route path=StaticSegment("panelmenu") view={Lazy::<PanelMenuRoute>::new()} />
                            <Route path=StaticSegment("megamenu") view={Lazy::<MegaMenuRoute>::new()} />
                            <Route path=StaticSegment("tree") view={Lazy::<TreeRoute>::new()} />
                            <Route path=StaticSegment("treeselect") view={Lazy::<TreeSelectRoute>::new()} />
                            <Route path=StaticSegment("cascadeselect") view={Lazy::<CascadeSelectRoute>::new()} />
                            <Route path=StaticSegment("autocomplete") view={Lazy::<AutoCompleteRoute>::new()} />
                            <Route path=StaticSegment("inputmask") view={Lazy::<InputMaskRoute>::new()} />
                            <Route path=StaticSegment("colorpicker") view={Lazy::<ColorPickerRoute>::new()} />
                            <Route path=StaticSegment("galleria") view={Lazy::<GalleriaRoute>::new()} />
                            <Route path=StaticSegment("fileupload") view={Lazy::<FileUploadRoute>::new()} />
                            <Route path=StaticSegment("orderlist") view={Lazy::<OrderListRoute>::new()} />
                            <Route path=StaticSegment("picklist") view={Lazy::<PickListRoute>::new()} />
                            <Route path=StaticSegment("datepicker") view={Lazy::<DatePickerRoute>::new()} />
                            <Route path=StaticSegment("dataview") view={Lazy::<DataViewRoute>::new()} />
                            <Route path=StaticSegment("table") view={Lazy::<TableRoute>::new()} />
                            <Route path=StaticSegment("treetable") view={Lazy::<TreeTableRoute>::new()} />
                        </Routes>
                    </main>
    </div>
                </div>
            </Router>
        }
}

// One lazily loaded route per component page: with `cargo leptos build --split` each page becomes its own wasm chunk.
lazy_page!(ButtonRoute, "button", ButtonPage);
lazy_page!(BadgeRoute, "badge", BadgePage);
lazy_page!(TagRoute, "tag", TagPage);
lazy_page!(ChipRoute, "chip", ChipPage);
lazy_page!(AvatarRoute, "avatar", AvatarPage);
lazy_page!(DividerRoute, "divider", DividerPage);
lazy_page!(CardRoute, "card", CardPage);
lazy_page!(InputTextRoute, "inputtext", InputTextPage);
lazy_page!(CheckboxRoute, "checkbox", CheckboxPage);
lazy_page!(RadioButtonRoute, "radiobutton", RadioButtonPage);
lazy_page!(ToggleButtonRoute, "togglebutton", ToggleButtonPage);
lazy_page!(SelectButtonRoute, "selectbutton", SelectButtonPage);
lazy_page!(RatingRoute, "rating", RatingPage);
lazy_page!(SliderRoute, "slider", SliderPage);
lazy_page!(KnobRoute, "knob", KnobPage);
lazy_page!(InputNumberRoute, "inputnumber", InputNumberPage);
lazy_page!(SelectRoute, "select", SelectPage);
lazy_page!(ToggleSwitchRoute, "toggleswitch", ToggleSwitchPage);
lazy_page!(MessageRoute, "message", MessagePage);
lazy_page!(ProgressBarRoute, "progressbar", ProgressBarPage);
lazy_page!(SkeletonRoute, "skeleton", SkeletonPage);
lazy_page!(TextareaRoute, "textarea", TextareaPage);
lazy_page!(PanelRoute, "panel", PanelPage);
lazy_page!(FieldsetRoute, "fieldset", FieldsetPage);
lazy_page!(AccordionRoute, "accordion", AccordionPage);
lazy_page!(ToolbarRoute, "toolbar", ToolbarPage);
lazy_page!(ScrollPanelRoute, "scrollpanel", ScrollPanelPage);
lazy_page!(ProgressSpinnerRoute, "progressspinner", ProgressSpinnerPage);
lazy_page!(InputGroupRoute, "inputgroup", InputGroupPage);
lazy_page!(ButtonGroupRoute, "buttongroup", ButtonGroupPage);
lazy_page!(FluidRoute, "fluid", FluidPage);
lazy_page!(IconFieldRoute, "iconfield", IconFieldPage);
lazy_page!(FloatLabelRoute, "floatlabel", FloatLabelPage);
lazy_page!(IftaLabelRoute, "iftalabel", IftaLabelPage);
lazy_page!(OverlayBadgeRoute, "overlaybadge", OverlayBadgePage);
lazy_page!(TimelineRoute, "timeline", TimelinePage);
lazy_page!(MeterGroupRoute, "metergroup", MeterGroupPage);
lazy_page!(BreadcrumbRoute, "breadcrumb", BreadcrumbPage);
lazy_page!(InputOtpRoute, "inputotp", InputOtpPage);
lazy_page!(ScrollTopRoute, "scrolltop", ScrollTopPage);
lazy_page!(PasswordRoute, "password", PasswordPage);
lazy_page!(InplaceRoute, "inplace", InplacePage);
lazy_page!(SplitterRoute, "splitter", SplitterPage);
lazy_page!(TabsRoute, "tabs", TabsPage);
lazy_page!(PopoverRoute, "popover", PopoverPage);
lazy_page!(DrawerRoute, "drawer", DrawerPage);
lazy_page!(DialogRoute, "dialog", DialogPage);
lazy_page!(TooltipRoute, "tooltip", TooltipPage);
lazy_page!(ToastRoute, "toast", ToastPage);
lazy_page!(MenuRoute, "menu", MenuPage);
lazy_page!(ListboxRoute, "listbox", ListboxPage);
lazy_page!(MultiSelectRoute, "multiselect", MultiSelectPage);
lazy_page!(PaginatorRoute, "paginator", PaginatorPage);
lazy_page!(StepperRoute, "stepper", StepperPage);
lazy_page!(ImageCompareRoute, "imagecompare", ImageComparePage);
lazy_page!(BlockUiRoute, "blockui", BlockUiPage);
lazy_page!(SplitButtonRoute, "splitbutton", SplitButtonPage);
lazy_page!(DockRoute, "dock", DockPage);
lazy_page!(ImageRoute, "image", ImagePage);
lazy_page!(ConfirmPopupRoute, "confirmpopup", ConfirmPopupPage);
lazy_page!(ConfirmDialogRoute, "confirmdialog", ConfirmDialogPage);
lazy_page!(SpeedDialRoute, "speeddial", SpeedDialPage);
lazy_page!(CarouselRoute, "carousel", CarouselPage);
lazy_page!(TieredMenuRoute, "tieredmenu", TieredMenuPage);
lazy_page!(StepsRoute, "steps", StepsPage);
lazy_page!(ContextMenuRoute, "contextmenu", ContextMenuPage);
lazy_page!(MenubarRoute, "menubar", MenubarPage);
lazy_page!(PanelMenuRoute, "panelmenu", PanelMenuPage);
lazy_page!(MegaMenuRoute, "megamenu", MegaMenuPage);
lazy_page!(TreeRoute, "tree", TreePage);
lazy_page!(TreeSelectRoute, "treeselect", TreeSelectPage);
lazy_page!(CascadeSelectRoute, "cascadeselect", CascadeSelectPage);
lazy_page!(AutoCompleteRoute, "autocomplete", AutoCompletePage);
lazy_page!(InputMaskRoute, "inputmask", InputMaskPage);
lazy_page!(ColorPickerRoute, "colorpicker", ColorPickerPage);
lazy_page!(GalleriaRoute, "galleria", GalleriaPage);
lazy_page!(FileUploadRoute, "fileupload", FileUploadPage);
lazy_page!(OrderListRoute, "orderlist", OrderListPage);
lazy_page!(PickListRoute, "picklist", PickListPage);
lazy_page!(DatePickerRoute, "datepicker", DatePickerPage);
lazy_page!(DataViewRoute, "dataview", DataViewPage);
lazy_page!(TableRoute, "table", TablePage);
lazy_page!(TreeTableRoute, "treetable", TreeTablePage);
