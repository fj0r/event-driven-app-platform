use super::{ButtonAttr, CaseAttr, ClassAttr, ImageAttr, JsonComponent, RackAttr, TextAttr};
use regex::Regex;
use std::convert::AsRef;

pub trait Classify {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self;
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self;
}

impl Classify for Option<ButtonAttr> {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        let attr = if let Some(attr) = self {
            attr
        } else {
            &mut ButtonAttr::default()
        };
        let cls = if let Some(cls) = &mut attr.class {
            cls
        } else {
            &mut "".to_owned()
        };
        cls.push_str(class.as_ref());
        self
    }
}

impl Classify for JsonComponent {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        match self {
            JsonComponent::button(c) => c.attrs.add_class(class),
            JsonComponent::case(c) => c.attrs,
            _ => {}
        }
        self
    }
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        if let Some(attr) = &mut self.attrs {
            attr.delete_class(class);
        }
        self
    }
}

impl Position {
    pub fn into_position(&self) -> String {
        let h = match &self.h {
            PosH::right(r) => format!("right: {};", r),
            PosH::left(l) => format!("left: {};", l),
        };
        let v = match &self.v {
            PosV::top(t) => format!("top: {};", t),
            PosV::bottom(b) => format!("bottom: {};", b),
        };
        vec![h, v].join(" ")
    }
}

impl Direction {
    pub fn into_flex(&self) -> String {
        match &self {
            Direction::D => format!("flex-direction: column"),
            Direction::U => format!("flex-direction: column-reverse"),
            Direction::R => format!("flex-direction: row"),
            Direction::L => format!("flex-direction: row-reverse"),
        }
    }
}

impl Size {
    pub fn into_style(&self) -> String {
        let mut s = String::new();
        if let Some(h) = &self.height {
            s.push_str(&format!("height: {};", h));
        };
        if let Some(w) = &self.width {
            s.push_str(&format!("width: {};", w));
        };
        s
    }
}
