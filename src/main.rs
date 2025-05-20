use dioxus::prelude::*;
mod libs;
use dioxus_logger::tracing;
use libs::components::*;
use libs::store::{use_store, Store};
use tracing_wasm::WASMLayerConfigBuilder;

const MAIN_CSS: Asset = asset!("/assets/main.css");

static STORE: GlobalSignal<Store> = Global::new(|| {
    let url = "ws://localhost:3000/channel";
    use_store(url).expect("connecting failed")
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Frame {
            layout: layout()
        }
    }
}
