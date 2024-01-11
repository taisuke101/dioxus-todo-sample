use dioxus::prelude::*;

#[component]
pub fn CardBase<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        style { include_str!("./CardBase.scss") }
        div { class: "card-container", children }
    })
}
