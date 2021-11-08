use crate::{components, Routes};
use dioxus::prelude::*;

use crate::state::use_provide_app_state;

// head {
//     link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
//     script {  src: "https://cdn.plot.ly/plotly-1.52.3.min.js" }
// }
// body {
// }

pub static App: FC<()> = |(cx, props)| {
    //
    let mut state_ = use_provide_app_state(cx);
    let state = state_.read();

    let inner = match state.route {
        Routes::Login => rsx! {
            components::Login {}
        },
        Routes::Home => rsx! {
            components::NavBar {}
            components::Dashboard {}
        },
        Routes::AddNew => rsx! {
            components::NavBar {}
            components::AddNew {}
        },
        Routes::Search => rsx! {
            components::NavBar {}
            components::Search {}
        },
        Routes::ProductPage { search_id } => rsx!(
            components::NavBar {}
            components::ResultsPage { id: search_id }
        ),
        Routes::Review => rsx!(
            components::NavBar {}
            components::Review {}
        ),
        Routes::Jupyter => rsx!(
            components::NavBar {}
            components::Jupyter {}
        ),
        Routes::NotFound => rsx!(
            components::NavBar {}
            "not found :("
        ),
    };

    cx.render(rsx! {
        div { class: "mx-auto lg:ml-80",
            {inner}
        }
    })
};
