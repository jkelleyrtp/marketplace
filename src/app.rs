use crate::components;
use atoms::{use_init_atom_root, use_read};
use dioxus::prelude::*;
use uuid::Uuid;

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

pub fn App(cx: Context, _props: &()) -> Element {
    use_init_atom_root(cx);
    let route = use_read(cx, crate::state::ROUTE);

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
                    "currently cannot import csv"
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
