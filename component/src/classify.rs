use super::*;
use std::convert::AsRef;

pub trait Classify {
    fn get_class(&self) -> &Option<Vec<String>>;
    fn add_class(&mut self, class: &str);
    fn delete_class(&mut self, class: &str);
    fn is_horizontal(&self) -> bool;
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
