use super::{ButtonAttr, CaseAttr, ClassAttr, ImageAttr, JsonComponent, RackAttr, TextAttr};
use std::convert::AsRef;

pub trait Classify {
    fn get_class(&self) -> &Option<Vec<String>>;
    fn add_class(&mut self, class: &str);
    fn delete_class(&mut self, class: &str);
    fn is_horizontal(&self) -> bool;
}

impl<T: Classify + Default> Classify for Option<T> {
    fn get_class(&self) -> &Option<Vec<String>> {
        if let Some(attr) = self {
            attr.get_class()
        } else {
            &None
        }
    }
    fn add_class(&mut self, class: &str) {
        if let Some(attr) = self {
            attr.add_class(class);
        } else {
            let mut n = T::default();
            n.add_class(class);
            *self = Some(n);
        };
    }
    fn delete_class(&mut self, class: &str) {
        if let Some(attr) = self {
            attr.delete_class(class);
        };
    }
    fn is_horizontal(&self) -> bool {
        if let Some(attr) = self {
            return attr.is_horizontal();
        }
        false
    }
}

macro_rules! impl_classify {
    ($($type: ident),*) => {
        $(
            impl Classify for $type {
                fn get_class(&self) -> &Option<Vec<String>> {
                    &self.class
                }
                fn add_class(&mut self, class: &str) {
                    if let Some(cls) = &mut self.class {
                        cls.push(class.to_string());
                    } else {
                        self.class = Some(vec![class.to_string()]);
                    };
                }
                fn delete_class(&mut self, class: &str) {
                    if let Some(cls) = &mut self.class
                        && cls.contains(&class.to_string()) {
                        let ix = cls.iter().position(|x| x == &class.as_ref()).unwrap();
                        cls.remove(ix);
                    };
                }
                fn is_horizontal(&self) -> bool {
                    false
                }
            }
        )*
    };
}

impl_classify![
    ClassAttr, ButtonAttr, CaseAttr, ImageAttr, RackAttr, TextAttr
];

impl Classify for JsonComponent {
    fn get_class(&self) -> &Option<Vec<String>> {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) =>  c.attrs.get_class(),)*
                    _ => &None
                }
            }
        }
        m![ self =>
            button, case, placeholder, chart, diagram, float, fold,
            form, popup, svg, rack, image, input, select, text,
        ]
    }
    fn add_class(&mut self, class: &str) {
        macro_rules! m {
            ($s:ident , $cls:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(
                        JsonComponent::$c(c) => {
                            c.attrs.add_class($cls);
                        }
                    )*
                    _ => {}
                }
            };
        }
        m![ self, class =>
            button, case, placeholder, chart, diagram, float, fold,
            form, popup, svg, rack, image, input, select, text,
        ];
    }
    fn delete_class(&mut self, class: &str) {
        macro_rules! m {
            ($s:ident , $cls:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(
                        JsonComponent::$c(c) => {
                            c.attrs.delete_class($cls);
                        }
                    )*
                    _ => {}
                }
            };
        }
        m! [self, class =>
            button, case, placeholder, chart, diagram, float, fold,
            form, popup, svg, rack, image, input, select, text,
        ];
    }
    fn is_horizontal(&self) -> bool {
        macro_rules! m {
            ($s:ident => $($c: ident),* $(,)?) => {
                match $s {
                    $(JsonComponent::$c(c) =>  c.attrs.is_horizontal(),)*
                    _ => false
                }
            }
        }
        m![ self =>
            button, case, placeholder, chart, diagram, float, fold,
            form, popup, svg, rack, image, input, select, text,
        ]
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
