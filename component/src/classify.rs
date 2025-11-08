use super::{ButtonAttr, CaseAttr, ClassAttr, ImageAttr, JsonComponent, RackAttr, TextAttr};
use std::convert::AsRef;

pub trait Classify {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self;
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self;
}

impl<T: Classify + Default> Classify for Option<T> {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        if let Some(attr) = self {
            attr.add_class(class);
        } else {
            let mut n = T::default();
            n.add_class(class);
            *self = Some(n);
        };
        self
    }
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        if let Some(attr) = self {
            attr.delete_class(class);
        };
        self
    }
}

macro_rules! impl_classify {
    ($($type: ident),*) => {
        $(
            impl Classify for $type {
                fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
                    if let Some(cls) = &mut self.class {
                        cls.push(class.as_ref().to_string());
                    };
                    self
                }
                fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self {
                    if let Some(cls) = &mut self.class {
                        todo!()
                    };
                    self
                }
            }
        )*
    };
}

impl_classify![
    ClassAttr, ButtonAttr, CaseAttr, ImageAttr, RackAttr, TextAttr
];

impl Classify for JsonComponent {
    fn add_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        match self {
            JsonComponent::button(c) => {
                c.attrs.add_class(class);
            }
            JsonComponent::case(c) => {
                c.attrs.add_class(class);
            }
            _ => {}
        }
        self
    }
    fn delete_class(&mut self, class: impl AsRef<str>) -> &mut Self {
        match self {
            JsonComponent::button(c) => {
                c.attrs.delete_class(class);
            }
            JsonComponent::case(c) => {
                c.attrs.delete_class(class);
            }
            _ => {}
        }
        self
    }
}

/*
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
*/
