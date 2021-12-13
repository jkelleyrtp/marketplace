//! Show all the products and the appropriate plots

use crate::{icons, state};
use atoms::use_read;
use dioxus::prelude::*;
use uuid::Uuid;

pub static Review: Component<()> = |cx, _| {
    cx.render(rsx!(
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                div { class: "pt-4 bg-white shadow rounded",
                    ManageKeywords {}
                    ReviewTable { }
                }
            }
        }
    ))
};

fn ReviewTable(cx: Context, _: &()) -> Element {
    let keywords = use_read(cx, state::KEYWORDS);

    let rows = keywords.keys().copied().map(|id| {
        rsx!(Row {
            key: "{id}",
            id: id,
        })
    });

    cx.render(rsx!(
        table { class: "table-auto w-full",
            thead { class: "bg-gray-50",
                tr { class: "text-xs text-gray-500 text-left",
                    th { class: "flex items-center pl-6 py-4 font-medium",
                        input { class: "mr-3",
                            id: "",
                            r#type: "checkbox",
                            name: "",
                        }
                        span { "Information" }
                    }
                    th { class: "py-4 font-medium", "Project Name" }
                    th { class: "py-4 font-medium", "Progress" }
                }
            }
            tbody {
                {rows}
            }
        }
    ))
}

#[derive(Props, PartialEq)]
struct RowProps {
    id: Uuid,
}

fn Row(cx: Context, props: &RowProps) -> Element {
    let state::KeywordEntry {
        keyword,
        creator,
        products,
        ..
    } = state::use_keyword_entry(cx, props.id)?;

    let img_url = &products.values().next().unwrap().productData.imageUrl;

    cx.render(rsx!(
        tr { class: "border-b border-blue-50",
            td { class: "flex items-center py-4 px-6 font-medium",
                input { class: "mr-3", id: "", name: "", r#type: "checkbox", }
                div { class: "flex px-4 py-3",
                    img { class: "w-8 h-8 mr-4 object-cover rounded-md", alt: "", src: "{img_url}", }
                    div {
                        p { class: "text-sm font-medium", "{keyword}" }
                        p { class: "text-xs text-gray-500", "{creator}" }
                    }
                }
            }
            td { class: "font-medium",
                p { class: "text-sm", "Example of Project" }
                p { class: "text-xs text-gray-500", "New development" }
            }
            td { class: "pr-6",
                p { class: "mb-1 text-xs text-indigo-500 font-medium", "65%" }
                div { class: "flex",
                    div { class: "relative h-1 w-48 bg-indigo-50 rounded-full",
                        div { class: "absolute top-0 left-0 h-full w-2/6 bg-indigo-500 rounded-full", }
                    }
                    a { class: "ml-auto", href: "#", icons::IconTripleDots {} }
                }
            }
        }
    ))
}

fn ManageKeywords(cx: Context, _: &()) -> Element {
    cx.render(rsx!(
        div { class: "px-6 border-b border-blue-50",
            div { class: "flex flex-wrap items-center mb-4",
                div {
                    h3 { class: "text-xl font-bold", "Project Progress Data" }
                    p { class: "text-sm text-gray-500 font-medium", "List of recent contracts and freelancers" }
                }
                a { class: "ml-auto flex items-center py-2 px-3 text-xs text-white bg-indigo-500 hover:bg-indigo-600 rounded",
                    href: "#",
                    icons::IconUpload {}
                    span { "Report" }
                }
            }
            div {
                a { class: "inline-block px-4 pb-2 text-sm font-medium text-indigo-500 border-b-2 border-indigo-500",
                    href: "#",
                    "Progress"
                }
                a { class: "inline-block px-4 pb-2 text-sm font-medium text-gray-300 border-b-2 border-transparent hover:border-gray-300",
                    href: "#",
                    "Completed"
                }
                a { class: "inline-block px-4 pb-2 text-sm font-medium text-gray-300 border-b-2 border-transparent hover:border-gray-300",
                    href: "#",
                    "Invoices"
                }
            }
        }
    ))
}

fn see_more(cx: Context) -> Element {
    cx.render(rsx!(
        div { class: "py-4 text-center",
            a { class: "inline-flex items-center text-xs text-indigo-500 hover:text-blue-600 font-medium",
                href: "#",
                span { class: "mr-1", icons::IconCharts {} }
                span { "See more Projects" }
            }
        }
    ))
}
