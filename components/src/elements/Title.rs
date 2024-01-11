use dioxus::prelude::*;
use strum::Display;

#[derive(Display)]
pub enum TextSize {
    #[strum(serialize = "-text-small")]
    Small,
    #[strum(serialize = "-text-medium")]
    Medium,
    #[strum(serialize = "-text-large")]
    Large,
    #[strum(serialize = "-text-exlarge")]
    ExtraLarge,
}

#[derive(Display)]
pub enum TextAlign {
    #[strum(serialize = "-text-left")]
    Left,
    #[strum(serialize = "-text-center")]
    Center,
    #[strum(serialize = "-text-right")]
    Right,
}

pub enum HeadingType {
    H1,
    H2,
    H3,
    H4,
}

#[component]
pub fn Title<'a>(
    cx: Scope,
    title: &'a str,
    size: TextSize,
    align: Option<TextAlign>,
    heading_type: HeadingType,
) -> Element {
    let text_size = size.to_string();
    let text_align = match align {
        Some(align) => align.to_string(),
        None => "".to_string(),
    };
    let style_sheet = rsx! {
        style { include_str!("./Title.scss") }
    };

    cx.render(match heading_type {
        HeadingType::H1 => {
            rsx! {
                style_sheet,
                h1 { class: format_args!("text-container {} {}", text_size, text_align), "{title}" }
            }
        }
        HeadingType::H2 => {
            rsx! {
                style_sheet,
                h2 { class: format_args!("text-container {} {}", text_size, text_align), "{title}" }
            }
        }
        HeadingType::H3 => {
            rsx! {
                style_sheet,
                h3 { class: format_args!("text-container {} {}", text_size, text_align), "{title}" }
            }
        }
        HeadingType::H4 => {
            rsx! {
                style_sheet,
                h4 { class: format_args!("text-container {} {}", text_size, text_align), "{title}" }
            }
        }
    })
}
