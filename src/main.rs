use dioxus::prelude::*;
mod libs;
use libs::store::use_store;
use libs::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const DEBUG_CSS: Asset = asset!("/assets/debug.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let url = "ws://localhost:3000/channel";
    let r = use_store(url).expect("connecting failed");
    let x = r.layout;

    let mut count = use_signal(|| 1);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DEBUG_CSS }
        document::Link { rel: "svg", href: HEADER_SVG }
        Frame {
            layout: x()
        }

        div {
            "{count}"
        }
        button {
            onclick: move |_| count += 1,
            "Count"
        }
    }
}

