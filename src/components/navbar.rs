use crate::icons;
use crate::AppRoute;
use atoms::use_read;
use atoms::use_set;
use dioxus::prelude::*;

const ROUTES: &[AppRoute] = &[
    AppRoute::Home,
    AppRoute::Search,
    AppRoute::ImportCsv,
    AppRoute::Review,
    AppRoute::Jupyter,
];

pub fn NavBar(cx: Context, _props: &()) -> Element {
    let set_route = use_set(cx, crate::state::ROUTE);
    let keywords = use_read(cx, crate::state::KEYWORDS);

    let primaries = ROUTES.iter().map(|route| {
        let text = match route {
            AppRoute::Home => "Home",
            AppRoute::ImportCsv => "Import CSV",
            AppRoute::Search => "Keyword Search",
            AppRoute::Review => "Review Products",
            AppRoute::Jupyter => "Open in Jupyter",
            AppRoute::Login => "Login",
            AppRoute::ProductPage { .. } => "Product Page",
            AppRoute::NotFound => "Err 404. Not Found.",
        };
        rsx!(
            li { key: "{text}"
                a { class: "flex items-center pl-3 py-3 pr-4 text-gray-50 hover:bg-gray-900 rounded",
                    onclick: move |_| set_route(route.clone()),
                    href: "#",
                    span { "{text}" }
                }
            }
        )
    });

    let kwords = keywords.iter().map(|(k, v)| {
        rsx!(
            li { key: "{k}"
                a { class: "flex items-center pl-3 py-3 pr-2 text-gray-50 hover:bg-gray-900 rounded",
                    onclick: move |_| set_route(AppRoute::ProductPage { search_id: *k }),
                    href: "#",
                    span { class: "inline-block mr-3", }
                    span { "{v.keyword}" }
                }
            }
        )
    });

    cx.render(rsx!{
        nav { class: "lg:hidden py-6 px-6 bg-gray-800 resize-x",
            div { class: "flex items-center justify-between",
                a { class: "text-2xl text-white font-semibold", href: "#", }
                button { class: "navbar-burger flex items-center rounded focus:outline-none", icons::Mobilemenu {} }
            }
        }
        div { class: "hidden lg:block navbar-menu relative z-50",
            div { class: "navbar-backdrop fixed lg:hidden inset-0 bg-gray-800 opacity-10", }
            nav { class: "fixed top-0 left-0 bottom-0 flex flex-col w-3/4 lg:w-80 sm:max-w-xs pt-6 pb-8 bg-gray-800 overflow-y-auto",
                div { class: "flex w-full items-center px-6 pb-6 mb-6 lg:border-b border-gray-700",
                    a { class: "text-xl text-white font-semibold", href: "#", }
                }
                div { class: "px-4 pb-6",
                    h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Main" }
                    ul { class: "mb-8 text-sm font-medium", {primaries} }
                    h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Searches" }
                    ul {
                        class: "text-sm font-medium",
                        {kwords}
                    }
                    div { class: "pt-8",
                        a { class: "flex items-center pl-3 py-3 pr-2 text-gray-50 hover:bg-gray-900 rounded",
                            href: "#",
                            span { class: "inline-block mr-4", icons::Settings {} }
                            span { "Settings" }
                        }
                        a { class: "flex items-center pl-3 py-3 pr-2 text-gray-50 hover:bg-gray-900 rounded",
                            href: "#",
                            span { class: "inline-block mr-4", icons::Logout {} }
                            span { "Log Out" }
                        }
                    }
                }
            }
        }
  })
}
