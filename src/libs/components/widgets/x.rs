#![allow(unused_imports)]
use super::super::super::data::Layout;
use super::super::super::store::Store;
use super::super::dynamic::Dynamic;
use super::super::utils::*;
use anyhow::Ok;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    rsx!()
}
