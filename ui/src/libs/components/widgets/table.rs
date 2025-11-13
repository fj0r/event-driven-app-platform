use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn table_(layout: Layout, children: Element) -> Element {
    rsx! {
        table {
            {children}
        }
    }
}

#[component]
pub fn thead_(layout: Layout, children: Element) -> Element {
    rsx! {
        thead {
            {children}
        }
    }
}

#[component]
pub fn tbody_(layout: Layout, children: Element) -> Element {
    rsx! {
        tbody {
            {children}
        }
    }
}

#[component]
pub fn tr_(layout: Layout, children: Element) -> Element {
    rsx! {
        tr {
            {children}
        }
    }
}

#[component]
pub fn th_(layout: Layout, children: Element) -> Element {
    rsx! {
        th {
            {children}
        }
    }
}

#[component]
pub fn td_(layout: Layout, children: Element) -> Element {
    rsx! {
        td {
            {children}
        }
    }
}
