use amazon_isr::app::App;
use dioxus::prelude::*;

fn main() {
    amazon_isr::logging::init(true);
    dioxus::desktop::launch(App, |c| {
        c.with_window(|f| f.with_maximized(true).with_title("Helium10 Research Tool"))
    })
}
