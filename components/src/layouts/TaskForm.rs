use dioxus::prelude::*;
use models::task::Task;

use crate::elements::{
    CardBase, FormInput,
    Stack::{Direction, Gap, Stack},
};

#[component]
pub fn TaskForm<'a>(cx: Scope, tasks: &'a UseState<Vec<Task>>) -> Element {
    let task = use_state::<Task>(cx, || Task::default());

    cx.render(rsx! {
        CardBase {
            form { onsubmit: move |_| {
                    tasks
                        .modify(|value| {
                            [
                                &value[..],
                                &[
                                    Task {
                                        id: value.len() + 1,
                                        ..task.get().clone()
                                    },
                                ],
                            ]
                                .concat()
                        })
                },
                Stack { direction: Direction::Column, gap: Gap::Sixteen,
                    FormInput {
                        label: "title",
                        title: "Title",
                        placeholder: "Enter a title",
                        on_input: move |e: FormEvent| {
                            task.set(Task {
                                title: e.value.clone(),
                                ..task.get().clone()
                            })
                        }
                    }
                    FormInput {
                        label: "description",
                        title: "Description",
                        placeholder: "Enter a description",
                        on_input: move |e: FormEvent| {
                            task.set(Task {
                                description: e.value.clone(),
                                ..task.get().clone()
                            })
                        }
                    }
                    FormInput {
                        label: "annotation",
                        title: "Annotation",
                        placeholder: "Enter an annotation",
                        on_input: move |e: FormEvent| {
                            task.set(Task {
                                annotation: Some(e.value.clone()),
                                ..task.get().clone()
                            })
                        }
                    }
                    button { r#type: "submit", class: "form-button", "Submit" }
                }
            }
        }
    })
}
