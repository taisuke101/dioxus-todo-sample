use dioxus::prelude::*;
use models::task::Task;

use crate::elements::{
    Button::{Button, ButtonColor, ButtonType},
    CardBase, FormInput,
    Stack::{Direction, Gap, Stack},
};

enum Field {
    Title,
    Description,
    Annotation,
}

#[component]
pub fn TaskForm<'a>(cx: Scope, on_submit: EventHandler<'a, &'a Task>) -> Element {
    let task = use_state::<Task>(cx, || Task::default());

    let on_input = |field: Field, e: FormEvent| {
        let current_task = task.get().clone();
        let new_value = e.value.clone();

        task.set(match field {
            Field::Title => Task {
                title: new_value,
                ..current_task
            },
            Field::Description => Task {
                description: new_value,
                ..current_task
            },
            Field::Annotation => Task {
                annotation: Some(new_value),
                ..current_task
            },
        })
    };

    cx.render(rsx! {
        CardBase {
            form { onsubmit: move |_| {
                    on_submit.call(task.get());
                    task.set(Task::default())
                },
                Stack { direction: Direction::Column, gap: Gap::Sixteen,
                    FormInput {
                        label: "title",
                        title: "Title",
                        placeholder: "Enter a title",
                        value: &task.get().title,
                        on_input: move |e| on_input(Field::Title, e)
                    }
                    FormInput {
                        label: "description",
                        title: "Description",
                        placeholder: "Enter a description",
                        value: &task.get().description,
                        on_input: move |e| on_input(Field::Description, e)
                    }
                    FormInput {
                        label: "annotation",
                        title: "Annotation",
                        placeholder: "Enter an annotation",
                        value: task.get().annotation.as_deref().unwrap_or(""),
                        on_input: move |e| on_input(Field::Annotation, e)
                    }
                    Button {
                        button_type: ButtonType::Submit,
                        button_color: ButtonColor::Primary,
                        title: "Submit",
                        class: "form-button",
                        disabled: task.get() == &Task::default()
                    }
                }
            }
        }
    })
}
