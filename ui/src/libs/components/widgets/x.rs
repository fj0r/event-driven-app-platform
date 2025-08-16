#![allow(unused_imports)]
use super::super::super::store::Store;
use super::super::dynamic::Dynamic;
use super::super::utils::*;
use anyhow::Ok;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    rsx!()
}
