use super::{Attrs, DirH, DirV, Direction, Layout};
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

impl Classify for Layout {
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

impl Direction {
    pub fn into_position(&self) -> String {
        let h = match &self.h {
            DirH::right(r) => format!("right: {};", r),
            DirH::left(l) => format!("left: {};", l),
        };
        let v = match &self.v {
            DirV::top(t) => format!("top: {};", t),
            DirV::bottom(b) => format!("bottom: {};", b),
        };
        vec![h, v].join(" ")
    }
}
