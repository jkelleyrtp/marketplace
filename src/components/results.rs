use std::collections::HashSet;

use crate::{
    helium10::{calculate_review_velocity, ProductListing},
    plots::salesscatter::Plots,
    state::use_keyword_entry,
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
    let current_product = use_keyword_entry(cx, props.id)?;
    let show_more = use_ref(cx, || HashSet::new());

    let rows = current_product
        .products
        .values()
        .enumerate()
        .map(|(idx, product)| {
            rsx!(TableRow {
                is_gray: idx % 2 == 0,
                product: product,
                show_more: show_more,
            })
        });

    let page_header = current_product.products.values().next().map(|first| {
        rsx!(
            div { class: "container px-4 mx-auto",
                h2 { class: "text-2xl font-bold", "Viewing search: {current_product.keyword}" }
                h3 { class: "text-xl", "Category: {first.category.name}" }
            }
        )
    });

    cx.render(rsx! {
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                {page_header}
                div { class: "pt-4 bg-white shadow rounded",
                    div { class: "flex px-6 pb-4 border-b",
                        h3 { class: "text-xl font-bold", "Recent Transactions" }
                    }
                    div { class: "p-4",
                        table { class: "table-fixed w-full",
                            {table_header(cx)}
                            tbody { {rows} }
                        }
                    }
                    // load_more(cx)
                }
            }
        }
    })
}

#[derive(Props)]
struct TableRowProps<'a> {
    is_gray: bool,
    product: &'a ProductListing,
    show_more: UseRef<'a, HashSet<String>>,
}

fn TableRow(cx: Context, props: &TableRowProps) -> Element {
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
    } = props.product;

    let len = productData.title.len();
    let trim_len = if len > 100 { 100 } else { len };
    let title = &productData.title[..trim_len];

    // this is pretty heavy, we should move it into a selector
    // Fortunately, our parent is memoized, so it's very unlikely that we will update
    let velocity = calculate_review_velocity(props.product);

    cx.render(rsx!(
        tr {
            class: format_args!("text-xs {}", if props.is_gray { "bg-gray-50" } else { "" }),
            onclick: move |_| { props.show_more.write().insert(props.product.asin.clone()); },

            td { class: "py-5 pl-6 font-medium", "{title}..." }
            td { class: "font-medium", "{productData.price}" }
            td { class: "font-medium", "{productData.bsr}" }
            td { class: "font-medium", "{velocity}" }
            td { class: "font-medium", "{productData.bsr}" }
            // td { span { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full", "Completed" } }
        }
    ))
}

fn table_header(cx: Context) -> Element {
    cx.render(rsx!(
        thead {
            tr { class: "text-xs text-gray-500 text-left",
                th { class: "pb-3 font-medium", "Name" }
                th { class: "pb-3 font-medium", "Price" }
                th { class: "pb-3 font-medium", "Rating" }
                th { class: "pb-3 font-medium", "Review Velocity" }
                th { class: "pb-3 font-medium", "BSR" }
            }
        }
    ))
}

fn load_more(cx: Context) -> Element {
    cx.render(rsx!(
        div { class: "text-center mt-5",
            a { class: "inline-flex items-center text-xs text-indigo-500 hover:text-blue-600 font-medium",
                href: "#",
                span { class: "inline-block mr-2", crate::icons::IconCopy {} }
                span { "Load more transactions" }
            }
        }
    ))
}
