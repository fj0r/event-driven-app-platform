use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn TABLE(layout: Layout, children: Element) -> Element {
    rsx! {
        table {
            {children}
        }
    }
}

#[component]
pub fn Thead(layout: Layout, children: Element) -> Element {
    rsx! {
        thead {
            {children}
        }
    }
}

#[component]
pub fn Tbody(layout: Layout, children: Element) -> Element {
    rsx! {
        tbody {
            {children}
        }
    }
}

#[component]
pub fn Tr(layout: Layout, children: Element) -> Element {
    rsx! {
        tr {
            {children}
        }
    }
}

#[component]
pub fn Th(layout: Layout, children: Element) -> Element {
    rsx! {
        th {
            {children}
        }
    }
}

#[component]
pub fn Td(layout: Layout, children: Element) -> Element {
    rsx! {
        td {
            {children}
        }
    }
}
