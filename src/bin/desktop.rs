use amazon_isr::app::App;
use dioxus::prelude::*;

fn main() {
    amazon_isr::logging::init(true);
    dioxus::desktop::launch(App, |c| c)
}
