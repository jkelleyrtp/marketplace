use std::collections::HashSet;

use crate::{
    helium10::{calculate_review_velocity, ProductListing},
    plots::salesscatter::Plots,
    state::use_app_state,
};
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct ResultsPageProps {
    id: Uuid,
}

pub fn ResultsPage(cx: &Scope, props: &ResultsPageProps) -> Element {
    cx.render(rsx! (
        section { class: "text-gray-500 bg-white body-font mx-auto px-12 pt-12"
            Plots { entry_id: props.id, }
            ListingTable { id: props.id }
        }
    ))
}

#[derive(PartialEq, Props)]
struct ListingTableProps {
    id: Uuid,
}

fn ListingTable(cx: &Scope, props: &ListingTableProps) -> Element {
    let state = use_app_state(cx)?;
    let state_read = state.read();
    let current_product = state_read.keywords.get(&props.id)?;

    let show_more = use_ref(cx, || HashSet::new());

    let cur_name = &current_product.keyword;

    let thead = rsx!(
        thead {
            tr { class: "text-xs text-gray-500 text-left",
                th { class: "pb-3 font-medium", "Name" }
                th { class: "pb-3 font-medium", "Price" }
                th { class: "pb-3 font-medium", "Rating" }
                th { class: "pb-3 font-medium", "Review Velocity" }
                th { class: "pb-3 font-medium", "BSR" }
            }
        }
    );

    let rows = current_product
        .products
        .values()
        .enumerate()
        .map(|(id, product)| {
            let ProductListing {
                asin,
                category,
                marketplace,
                productData,
                bsrHistory,
                requestId,
                reviewHistory,
                salesHistory,
                ..
            } = product;

            let len = productData.title.len();
            let title = &productData.title[..{
                if len > 100 {
                    100
                } else {
                    len
                }
            }];

            let velocity = calculate_review_velocity(product);

            rsx!(
                tr {
                    key: "{asin}"
                    class: format_args!("text-xs {}", if id % 2 == 0 { "bg-gray-50" } else { "" }),
                    onclick: move |_| { show_more.write().insert(asin.clone()); },

                    td { class: "py-5 pl-6 font-medium", "{title}..." }
                    td { class: "font-medium", "{productData.price}" }
                    td { class: "font-medium", "{productData.bsr}" }
                    td { class: "font-medium", "{velocity}" }
                    td { class: "font-medium", "{productData.bsr}" }
                    // td { span { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full", "Completed" } }
                }
            )
        });

    let first = current_product.products.values().next().unwrap();
    let header = rsx!(
        div { class: "container px-4 mx-auto",
            h2 { class: "text-2xl font-bold",
                "Viewing search: {cur_name} "
            }
            h3 { "Category: {first.category.name}" }
        }
    );

    cx.render(rsx!{
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                {header}
                div { class: "pt-4 bg-white shadow rounded",
                    div { class: "flex px-6 pb-4 border-b", h3 { class: "text-xl font-bold", "Recent Transactions" } }
                    div { class: "p-4",
                        table { class: "table-fixed w-full",
                        // table { class: "table-auto w-full",
                            {thead}
                            tbody { {rows} }
                        }
                        div { class: "text-center mt-5",
                            a { class: "inline-flex items-center text-xs text-indigo-500 hover:text-blue-600 font-medium",
                                href: "#",
                                span { class: "inline-block mr-2", crate::icons::IconCopy {} }
                                span { "Load more transactions" }
                            }
                        }
                    }
                }
            }
        }
    })
}
