use components::pages::TaskPage;
use dioxus::prelude::*;

use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./reset.scss") }
        TaskPage {}
    })
}
