use crate::icons;
use crate::state::use_current_user;
use crate::AppRoute;
use atoms::use_read;
use atoms::use_set;
use dioxus::prelude::*;

const ROUTES: &[AppRoute] = &[
    AppRoute::Home,
    AppRoute::Search,
    AppRoute::Review,
    AppRoute::Analyze,
];

pub fn VerticalNav(cx: Scope<()>) -> Element {
    let set_route = use_set(&cx, crate::state::ROUTE);
    let keywords = use_read(&cx, crate::state::KEYWORDS);

    let primaries = ROUTES.iter().map(|route| {
        let text = match route {
            AppRoute::Home => "Home",
            AppRoute::ImportCsv => "Import CSV",
            AppRoute::Search => "Keyword Search",
            AppRoute::Review => "Mange Searches",
            AppRoute::Analyze => "Open in Jupyter",
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
                    a { class: "text-xl text-white font-semibold", href: "#", "Marketplace Analyzer"}
                }
                div { class: "px-4 pb-6",
                    h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Main" }
                    ul { class: "mb-8 text-sm font-medium", {primaries} }
                    h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Searches" }
                    ul { class: "text-sm font-medium", {kwords} }
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

pub fn TopNav(cx: Scope<()>) -> Element {
    cx.render(rsx!(
        section { class: "py-5 px-6 bg-white shadow",
            nav { class: "relative",
                div { class: "flex items-center",
                    div { class: "flex items-center mr-auto",
                        button { class: "flex items-center",
                            span { class: "flex justify-center items-center mr-3 w-10 h-10 bg-indigo-500 text-sm text-white rounded-full",
                                "EP"
                            }
                            p { class: "text-sm font-medium mr-2", "Fall 2021 ISR" }
                            span { class: "inline-block -mb-px", icons::ChevronUpDown {} }
                        }
                    }
                    div { class: "ml-auto lg:hidden",
                        button { class: "flex items-center",
                            icons::Empty {}
                        }
                    }
                    QuickIcons {}
                    UserCard {}
                }
            }
        }
    ))
}

fn UserCard(cx: Scope<()>) -> Element {
    let user = use_current_user(&cx)?;

    cx.render(rsx!(
        div { class: "hidden lg:block",
            button { class: "flex items-center",
                img { class: "w-10 h-10 mr-2 rounded-full object-cover object-right",
                    alt: "",
                    src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                }
                p { class: "text-sm mr-3", "{user.full_name}" }
                span {  icons::ChevronUpDown {} }
            }
        }
    ))
}

fn QuickIcons(cx: Scope<()>) -> Element {
    cx.render(rsx!(
        ul { class: "hidden lg:flex items-center space-x-6 mr-6",
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::SearchGlass {}
                }
            }
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::ChatMessage {}
                }
            }
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::Bell {}
                }
            }
        }
    ))
}
