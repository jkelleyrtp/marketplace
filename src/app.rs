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
    #[at("/analyze")]
    Analyze,
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
                    components::VerticalNav {}
                    components::TopNav {}
                    components::Greeting {}
                    components::Dashboard {}
                    // components::Review {}
                },
                AppRoute::ImportCsv => rsx! {
                    components::VerticalNav {}
                    components::TopNav {}
                    "currently cannot import csv"
                },
                AppRoute::Search => rsx! {
                    components::VerticalNav {}
                    components::TopNav {}
                    components::Search {}
                },
                AppRoute::ProductPage { search_id } => rsx!(
                    components::VerticalNav {}
                    components::TopNav {}
                    components::ResultsPage { id: *search_id }
                ),
                AppRoute::Review => rsx!(
                    components::VerticalNav {}
                    components::TopNav {}
                    components::Review {}
                ),
                AppRoute::Analyze => rsx!(
                    components::VerticalNav {}
                    components::TopNav {}
                    components::Jupyter {}
                ),
                AppRoute::NotFound => rsx!(
                    components::VerticalNav {}
                    components::TopNav {}
                    "not found :("
                ),
            }}
        }
    })
}
