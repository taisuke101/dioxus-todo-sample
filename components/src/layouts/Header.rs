use crate::elements::{
    Stack::{Direction, Gap, Stack},
    Title::{HeadingType, TextSize, Title},
};
use dioxus::prelude::*;

#[component]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./Header.scss") }
        header { class: "header-container",
            Title {
                title: "Dioxus TodoList",
                size: TextSize::ExtraLarge,
                heading_type: HeadingType::H1
            }
            Stack { direction: Direction::Row, gap: Gap::Four,
                div { "sample" }
                div { "sample2" }
            }
        }
    })
}
