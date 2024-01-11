use dioxus::prelude::*;

use crate::elements::{
    CardBase,
    Stack::{Direction, Gap, Stack},
};
use models::task::Task;

#[component]
pub fn TaskCard(cx: Scope, task: Task) -> Element {
    cx.render(rsx! {
        style { include_str!("./TaskCard.scss") }
        CardBase {
            Stack { direction: Direction::Column, gap: Gap::Eight,
                div {
                    strong { "Title:" }
                    p { "{task.title}" }
                }
                div {
                    strong { "Description:" }
                    p { "{task.description}" }
                }
                task.annotation.as_ref().and_then(|annotation| {
                    Some(rsx!(
                        div {
                            strong { "Annotation:" }
                            p { "{annotation}" }
                        }))
                }),
                div {
                    Stack { direction: Direction::Row, gap: Gap::Eight,
                        strong { "Is Complete:" }
                        input { r#type: "checkbox" }
                    }
                }
            }
        }
    })
}
