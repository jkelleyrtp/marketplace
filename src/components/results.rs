use std::collections::HashSet;

use crate::{
    api::helium10::{calculate_review_velocity, ProductAnalysis, ProductListing},
    components::plots,
    state::use_keyword_entry,
};

use crate::state::KeywordEntry;
use atoms::{use_read, use_set};
use dioxus::prelude::*;
use plotly::{common::Mode, Plot, Scatter};
use plotly::{common::Title, Layout};
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct ResultsPageProps {
    id: Uuid,
}

pub fn ResultsPage(cx: &Scope, props: &ResultsPageProps) -> Element {
    let keywords = use_read(cx, crate::state::KEYWORDS);
    let data = keywords.get(&props.id)?;
    let user = use_read(cx, crate::state::USERS).get(&data.creator)?;

    cx.render(rsx! (
        div { class: "py-4 px-6",
            div { class: "container px-4 mx-auto",
                h1 { class: "mb-2 text-5xl font-bold font-heading", "Search: \"{data.keyword}\"" }
                h1 { class: "mb-2 text-3xl font-bold font-heading" "searched by: {user.short_name}" }
            }
        }
        div { class: "container flex flex-row md:flex-row mx-auto"
            ReviewVelocity { entry: data, divid: "r1" }
            SalesPlot { entry: data, divid: "r2" }
        }
        section { class: "text-gray-500 bg-white body-font mx-auto px-12 pt-12"
            ListingTable { id: props.id }
        }
    ))
}

#[derive(PartialEq, Props)]
struct ListingTableProps {
    id: Uuid,
}

enum Sortby {
    Alphabetical,
    ReviewVelocity,
    NumReviews,
    Rating,
    Revenue,
    Sales,
}

fn ListingTable(cx: &Scope, props: &ListingTableProps) -> Element {
    let current_keyword = use_keyword_entry(cx, props.id)?;
    let show_more = use_ref(cx, || HashSet::new());
    let sort_by = use_state(cx, || Sortby::NumReviews);

    let analsysis = use_read(cx, crate::state::PRODUCT_ANALYSIS);

    let mut sorted_ids = current_keyword
        .products
        .iter()
        .map(|(id, k)| (k, analsysis.get(id).unwrap()))
        .collect::<Vec<_>>();

    match *sort_by {
        Sortby::Alphabetical => todo!(),
        Sortby::ReviewVelocity => sorted_ids
            .sort_by(|(_, a), (_, b)| a.review_velocity.partial_cmp(&b.review_velocity).unwrap()),
        Sortby::NumReviews => {
            sorted_ids.sort_by(|(_, a), (_, b)| a.num_reviews.partial_cmp(&b.num_reviews).unwrap())
        }
        Sortby::Rating => {
            sorted_ids.sort_by(|(_, a), (_, b)| a.rating.partial_cmp(&b.rating).unwrap())
        }
        Sortby::Revenue => {
            sorted_ids.sort_by(|(_, a), (_, b)| a.revenue.partial_cmp(&b.revenue).unwrap())
        }
        Sortby::Sales => todo!(),
    }

    let rows = sorted_ids
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, (product, analysis))| {
            let analysis = analsysis.get(&product.asin);
            rsx!(TableRow {
                key: "{product.asin}"
                is_gray: idx % 2 == 0,
                product: product,
                show_more: show_more,
                analysis: analysis
            })
        });

    let page_header = current_keyword.products.values().next().map(|first| {
        rsx!(
            div { class: "container px-4 mx-auto",
                h2 { class: "text-2xl font-bold", "Viewing search: {current_keyword.keyword}" }
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
                            // todo: sort by column
                            // todo: show image
                            thead {
                                tr { class: "text-xs text-gray-500 text-left",
                                    th { class: "pb-3 font-medium", "Image" }
                                    th { class: "pb-3 font-medium", "Name" }
                                    th { class: "pb-3 font-medium", "Price" }
                                    th { class: "pb-3 font-medium", "BSR" }
                                    th { class: "pb-3 font-medium", "Created" }
                                    th { class: "pb-3 font-medium", "Reviews" }
                                    th { class: "pb-3 font-medium", "Rating" }
                                    th { class: "pb-3 font-medium", "Sales" }
                                    th { class: "pb-3 font-medium", "Review Velocity" }
                                    th { class: "pb-3 font-medium", "Revenue" }
                                }
                            }
                            tbody { {rows} }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct TableRowProps<'a> {
    is_gray: bool,
    product: &'a ProductListing,
    analysis: Option<&'a ProductAnalysis>,
    show_more: UseRef<'a, HashSet<String>>,
}

fn TableRow(cx: Context, props: &TableRowProps) -> Element {
    let ProductListing { productData, .. } = props.product;

    let len = productData.title.len();
    let trim_len = if len > 100 { 100 } else { len };
    let title = &productData.title[..trim_len];

    let ProductAnalysis {
        sales,
        revenue,
        review_velocity,
        creation_date,
        rating,
        num_reviews,
    } = match props.analysis {
        Some(a) => a.clone(),
        None => ProductAnalysis::new(props.product),
    };

    let img_url = &productData.imageUrl;
    cx.render(rsx!(
        tr { class: format_args!("text-xs {}", if props.is_gray { "bg-gray-50" } else { "" }),
            onclick: move |_| { props.show_more.write().insert(props.product.asin.clone()); },
            td { class: "flex flex-row"
                img { class: "h-24 rounded", src: "{img_url}", }
            }
            td { class: "py-5 pl-6 font-medium", "{title}..." }
            td { class: "font-medium", "{productData.price}" }
            td { class: "font-medium", "{productData.bsr}" }
            td { class: "font-medium", "{creation_date}" }
            td { class: "font-medium", "{num_reviews}" }
            td { class: "font-medium", "{rating}" }
            td { class: "font-medium", "{sales}" }
            td { class: "font-medium", "{review_velocity}" }
            td { class: "font-medium", "{revenue}" }
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

#[derive(Props)]
pub struct PlotsProps<'a> {
    entry: &'a KeywordEntry,
    divid: &'static str,
}

pub fn ReviewVelocity(cx: Context, props: &PlotsProps) -> Element {
    let prices = props
        .entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let x = (0..prices.len()).collect::<Vec<_>>();

    let mut plot = Plot::new();
    let trace = Scatter::new(x, prices).mode(Mode::Markers);

    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::new("Sales")));

    let name = props.divid;

    let raw = plot.to_inline_html(Some(name));
    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{name}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}

pub fn SalesPlot(cx: Context, props: &PlotsProps) -> Element {
    let prices = props
        .entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let x = (0..prices.len()).collect::<Vec<_>>();

    let mut plot = Plot::new();
    let trace = Scatter::new(x, prices).mode(Mode::Markers);

    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::new("Sales")));

    let name = props.divid;

    let raw = plot.to_inline_html(Some(name));
    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{name}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}
