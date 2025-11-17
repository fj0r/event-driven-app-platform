use super::*;

pub trait Classify {
    fn get_class(&self) -> &Option<Vec<String>>;
    fn add_class(&mut self, class: &str);
    fn get_selector(&self) -> &Option<String>;
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
    fn get_selector(&self) -> &Option<String> {
        if let Some(attr) = self {
            attr.get_selector()
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

impl SizeAttr {
    pub fn size_style(&self) -> String {
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

impl ImageAttr {
    pub fn size_style(&self) -> String {
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

impl PositionAttr {
    pub fn into_style(&self) -> String {
        let h = match &self.h {
            Some(PosH::right(r)) => format!("right: {};", r),
            Some(PosH::left(l)) => format!("left: {};", l),
            None => "".to_string(),
        };
        let v = match &self.v {
            Some(PosV::top(t)) => format!("top: {};", t),
            Some(PosV::bottom(b)) => format!("bottom: {};", b),
            None => "".to_string(),
        };
        vec![h, v].join(" ")
    }
}

impl DirectionAttr {
    pub fn into_style(&self) -> String {
        match &self.direction {
            Some(Direction::D) => format!("flex-direction: column"),
            Some(Direction::U) => format!("flex-direction: column-reverse"),
            Some(Direction::R) => format!("flex-direction: row"),
            Some(Direction::L) => format!("flex-direction: row-reverse"),
            None => "".to_string(),
        }
    }
}
