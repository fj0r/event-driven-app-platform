use super::{Attrs, Component, Direction, PosH, PosV, Position, Size};
use regex::Regex;
use std::convert::AsRef;

pub trait Classify {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self;
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self;
}

impl Attrs {
    fn split(&self) -> Vec<&str> {
        if let Some(cls) = &self.class {
            let sep = Regex::new(r"\s+").unwrap();
            let v: Vec<&str> = sep.split(cls).collect();
            v
        } else {
            Vec::new()
        }
    }
}

impl Classify for Attrs {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        let mut cls = self.split();
        if !cls.contains(&class.as_ref()) {
            cls.push(class.as_ref());
        }
        self.class = Some(cls.join(" "));
        self
    }
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        let mut cls = self.split();
        if cls.contains(&class.as_ref()) {
            let ix = cls.iter().position(|x| x == &class.as_ref()).unwrap();
            cls.remove(ix);
        }
        self.class = Some(cls.join(" "));
        self
    }
}

impl Classify for Component {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        if let Some(attr) = &mut self.attrs {
            attr.add_class(class);
        } else {
            let mut attr = Attrs::default();
            attr.add_class(class);
            self.attrs = Some(attr);
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
