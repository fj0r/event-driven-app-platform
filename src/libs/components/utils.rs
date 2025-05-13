use super::super::data::Layout;

pub fn merge_css_class<'a>(css: &'a mut Vec<&'a str>, layout: &'a Layout) -> &'a mut Vec<&'a str> {
    let mut ho = false;
    if let Some(a) = layout.attrs.as_ref() {
        if let Some(h) = a.horizontal {
            if h {
                ho = true;
            }
        }
        if let Some(cc) = &a.class {
            css.push(cc);
        }
    }
    if !ho { css.push("v"); }
    css
}
