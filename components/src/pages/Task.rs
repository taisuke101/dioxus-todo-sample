use dioxus::prelude::*;

use crate::elements::Button::{Button, ButtonType};
use crate::elements::Stack::{Direction, Gap, Stack};
use crate::elements::Title::{HeadingType, TextAlign, TextSize, Title};
use crate::layouts::{Header, TaskCard, TaskForm};
use models::task::Task;

#[component]
pub fn TaskPage(cx: Scope) -> Element {
    let tasks = use_state::<Vec<Task>>(cx, || Vec::new());
    let is_display_form = use_state(cx, || false);

    cx.render(rsx! {
        Header {}
        Title {
            title: "TodoList",
            size: TextSize::ExtraLarge,
            heading_type: HeadingType::H1,
            align: TextAlign::Center
        }

        match tasks.get().len() {
            0 => {
                rsx!(
                    Stack {
                        direction: Direction::Column,
                        gap: Gap::Twelve,
                        p {
                            text_align: "center",
                            "there is no tasks,"
                        }
                    }
                )
            },
            _ => {
                rsx!(
                    Stack {
                        direction: Direction::Column,
                        gap: Gap::Eight,
                        tasks.iter().map(|task| {
                                rsx! {
                                    TaskCard { task: task.clone()}
                                }
                        })
                    }
                )
            }
        },
        match is_display_form.get() {
            true => {
                rsx!(
                    TaskForm { tasks: tasks }
                )
            },
            false => {
                rsx!(
                    Button {
                        button_type: ButtonType::Secondary,
                        title: "Add Task",
                        on_click: move |_| is_display_form.set(true)
                    }
                )
            }
        }
    })
}
