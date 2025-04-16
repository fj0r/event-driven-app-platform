use dioxus::prelude::*;
mod libs;
use libs::store::{use_store, Store};
use libs::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const DEBUG_CSS: Asset = asset!("/assets/debug.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

static STORE: GlobalSignal<Store> = Global::new(|| {
    let url = "ws://localhost:3000/channel";
    let r = use_store(url).expect("connecting failed");
    r
});

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| STORE());
    let layout = STORE().layout;

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DEBUG_CSS }
        document::Link { rel: "svg", href: HEADER_SVG }
        Frame {
            layout: layout()
        }
    }
}

