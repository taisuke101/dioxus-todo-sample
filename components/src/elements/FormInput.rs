use crate::elements::Stack::{Direction, Gap, Stack};
use dioxus::prelude::*;

#[component]
pub fn FormInput<'a>(
    cx: Scope,
    label: &'a str,
    title: &'a str,
    placeholder: &'a str,
    on_input: EventHandler<'a, FormEvent>,
) -> Element {
    cx.render(rsx! {
        style { include_str!("./FormInput.scss") }
        Stack { direction: Direction::Column, gap: Gap::Four,
            label { r#for: "{label}", class: "form-label", "{title}" }
            input {
                r#type: "text",
                id: "{label}",
                placeholder: "{placeholder}",
                class: "form-input",
                oninput: move |e| on_input.call(e)
            }
        }
    })
}
