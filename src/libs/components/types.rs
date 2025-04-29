use dioxus::prelude::*;
use std::rc::Rc;

pub type OptionEl = Option<Signal<Option<Rc<MountedData>>>>;
