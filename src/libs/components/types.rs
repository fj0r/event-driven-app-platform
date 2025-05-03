use dioxus::prelude::*;
use std::rc::Rc;

pub type OptionEl = Option<Signal<Option<Rc<MountedData>>>>;


#[derive(Clone, Copy)]
pub struct ListState {
    pub last: Signal<Option<Rc<MountedData>>>
}
