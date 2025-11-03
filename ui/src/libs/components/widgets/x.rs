#![allow(unused_imports)]
use super::super::super::store::Status;
use super::super::dynamic::Dynamic;
use crate::libs::hooks::*;
use anyhow::Ok;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use layout::Layout;

#[component]
pub fn X(layout: Layout, children: Element) -> Element {
    rsx!()
}
