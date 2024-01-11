use dioxus::prelude::*;
use strum::Display;

#[derive(Display)]
pub enum ButtonType {
    #[strum(serialize = "-primary")]
    Primary,
    #[strum(serialize = "-secondary")]
    Secondary,
    #[strum(serialize = "-outline")]
    OutLine,
}

#[component]
pub fn Button<'a>(
    cx: Scope,
    button_type: ButtonType,
    title: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
) -> Element {
    let button_type = button_type.to_string();

    cx.render(rsx! {
        style { include_str!("./Button.scss") }
        button { class: format_args!("button-container {}", button_type), onclick: move |event| on_click.call(event), "{title}" }
    })
}
