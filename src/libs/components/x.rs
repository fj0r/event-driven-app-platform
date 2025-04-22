#![allow(unused_imports)]
use super::super::data::Layout;
use super::super::store::Store;
use super::dynamic::Dynamic;
use super::utils::*;
use anyhow::Ok;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    rsx!()
}
