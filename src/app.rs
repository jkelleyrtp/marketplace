use crate::components;
use dioxus::prelude::*;
use dioxus::router::{use_router, Routable};
use uuid::Uuid;

use crate::state::use_provide_app_state;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/new")]
    ImportCsv,
    #[at("/search")]
    Search,
    #[at("/review")]
    Review,
    #[at("/jupyter")]
    Jupyter,
    #[at("/product/{search_id}")]
    ProductPage { search_id: Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn App(cx: Context, props: &()) -> Element {
    let st = use_provide_app_state(cx);
    let s = st.read();
    let route = &s.route;

    // let route = use_router(cx, |f| AppRoute::Home)?;

    cx.render(rsx! {
        div { class: "mx-auto lg:ml-80",
            {match route {
                AppRoute::Login => rsx! {
                    components::Login {}
                },
                AppRoute::Home => rsx! {
                    components::NavBar {}
                    components::Dashboard {}
                },
                AppRoute::ImportCsv => rsx! {
                    components::NavBar {}
                    components::AddNew {}
                },
                AppRoute::Search => rsx! {
                    components::NavBar {}
                    components::Search {}
                },
                AppRoute::ProductPage { search_id } => rsx!(
                    components::NavBar {}
                    components::ResultsPage { id: *search_id }
                ),
                AppRoute::Review => rsx!(
                    components::NavBar {}
                    components::Review {}
                ),
                AppRoute::Jupyter => rsx!(
                    components::NavBar {}
                    components::Jupyter {}
                ),
                AppRoute::NotFound => rsx!(
                    components::NavBar {}
                    "not found :("
                ),
            }}
        }
    })
}
