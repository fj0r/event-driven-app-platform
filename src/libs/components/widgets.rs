use dioxus::prelude::*;

#[component]
pub fn Input(children: Element) -> Element {
    rsx!{
        div {
            class: "input",
            {children}
        }
    }
}

#[component]
pub fn Text(children: Element) -> Element {
    rsx!{
        div {
            class: "text",
            {children}
        }
    }
}


#[component]
pub fn Card(children: Element) -> Element {
    rsx!{
        div {
            class: "card",
            {children}
        }
    }
}


#[component]
pub fn Button(children: Element) -> Element {
    rsx!{
        div {
            class: "button",
            {children}
        }
    }
}
