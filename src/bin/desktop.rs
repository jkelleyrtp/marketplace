use amazon_isr::app::App;
use dioxus::prelude::*;

fn main() {
    simple_logger::init().unwrap();
    dioxus::desktop::launch(App, |c| c)
}
