use dioxus::prelude::*;
use strum::Display;

#[derive(Display)]
pub enum ButtonType {
    #[strum(serialize = "button")]
    Button,
    #[strum(serialize = "submit")]
    Submit,
}

#[derive(Display)]
pub enum ButtonColor {
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
    button_color: ButtonColor,
    title: &'a str,
    on_click: Option<EventHandler<'a, MouseEvent>>,
    class: Option<&'a str>,
    disabled: Option<bool>,
) -> Element {
    let button_type = button_type.to_string();
    let button_color = button_color.to_string();

    cx.render(rsx! {
        style { include_str!("./Button.scss") }
        button {
            r#type: format_args!("{}", button_type),
            disabled: disabled.unwrap_or(false),
            class: format_args!("button-container {} {:?}", button_color, class.unwrap_or("")),
            onclick: move |event| {
                if let Some(on_click) = &on_click {
                    on_click.call(event)
                }
            },
            "{title}"
        }
    })
}
