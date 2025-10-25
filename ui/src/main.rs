mod libs;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use libs::components::*;
use libs::store::{Store, use_store};
use tracing_wasm::WASMLayerConfigBuilder;

static STORE: GlobalSignal<Store> = Global::new(|| {
    let doc = web_sys::window().unwrap().document().unwrap();
    let loc = doc.location().unwrap();
    let mut host = "".to_owned();
    let mut token = None;
    if let Ok(Some(ele)) = doc.query_selector("#main") {
        if let Some(h) = ele.get_attribute("data-host") {
            host = h;
        } else {
            host = loc.host().unwrap();
        };

        if let Ok(href) = loc.href()
            && let Ok(href) = web_sys::Url::new(&href)
            && let Some(t) = href.search_params().get("token")
        {
            token = Some(t);
        } else if let Some(t) = ele.get_attribute("data-token") {
            token = Some(t);
        };
    };
    let query = if let Some(token) = token {
        format!("?token={}", &token)
    } else {
        "".to_owned()
    };
    let url = format!("ws://{}/channel{}", host, query);
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
        document::Style { href: asset!("/assets/main.css") }
        document::Style { href: asset!("/assets/custom.css") }
        // document::Script { src: asset!("/assets/apexcharts.min.js") }
        // document::Script { src: asset!("/assets/mermaid.min.js") }
        Frame {
            layout: layout()
        }
    }
}
