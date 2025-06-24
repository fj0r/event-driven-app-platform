use layout::Layout;

pub fn merge_css_class<'a>(css: &'a mut Vec<&'a str>, layout: &'a Layout) -> &'a mut Vec<&'a str> {
    let mut v = ["box", "case", "rack", "text", "tab", "menu"].contains(&layout.kind.as_str());
    if let Some(a) = layout.attrs.as_ref() {
        if let Some(h) = a.horizontal {
            if h {
                v = false;
            }
        }
        if let Some(cc) = &a.class {
            css.push(cc);
        }
    }
    if v {
        css.push("v");
    }
    css
}
