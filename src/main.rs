use dioxus::prelude::*;
mod libs;
use dioxus_logger::tracing;
use libs::components::*;
use libs::store::{use_store, Store};
use tracing_wasm::WASMLayerConfigBuilder;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const APEXCHART_JS: Asset = asset!("/assets/apexcharts.min.js");

static STORE: GlobalSignal<Store> = Global::new(|| {
    let d = web_sys::window().unwrap().document().unwrap();
    let url = d
        .query_selector("#main")
        .ok()
        .and_then(|x| x)
        .and_then(|u| u.get_attribute("data-host"))
        .unwrap_or_else(|| d.location().unwrap().host().unwrap());
    let url = format!("ws://{}/channel", url);
    use_store(&url).expect("connecting failed")
});

fn main() {
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::INFO)
            .build(),
    );
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| STORE());
    let layout = STORE().layout;

    rsx! {
        document::Style { href: MAIN_CSS }
        document::Script { src: APEXCHART_JS }
        Frame {
            layout: layout()
        }
    }
}
