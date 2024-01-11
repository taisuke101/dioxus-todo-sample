use dioxus::prelude::*;
use strum::Display;

#[derive(Display)]
pub enum Direction {
    #[strum(serialize = "-row")]
    Row,
    #[strum(serialize = "-column")]
    Column,
}

#[derive(Display)]
pub enum Gap {
    #[strum(serialize = "-gap-4")]
    Four,
    #[strum(serialize = "-gap-8")]
    Eight,
    #[strum(serialize = "-gap-12")]
    Twelve,
    #[strum(serialize = "-gap-16")]
    Sixteen,
    #[strum(serialize = "-gap-20")]
    Twenty,
}

#[component]
pub fn Stack<'a>(cx: Scope, direction: Direction, gap: Gap, children: Element<'a>) -> Element {
    let direction = direction.to_string();
    let gap = gap.to_string();

    cx.render(rsx! {
        style { include_str!("./Stack.scss") }
        div { class: format_args!("stack-container {} {}", direction, gap), children }
    })
}
