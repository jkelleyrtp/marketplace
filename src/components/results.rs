use std::collections::HashSet;

use crate::{
    api::helium10::{calculate_review_velocity, ProductAnalysis, ProductListing},
    components::plots,
    icons,
    state::use_keyword_entry,
};

use crate::state::KeywordEntry;
use atoms::{use_read, use_set};
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct ResultsPageProps {
    id: Uuid,
}

pub fn ResultsPage(cx: Scope<ResultsPageProps>) -> Element {
    let keywords = use_read(&cx, crate::state::KEYWORDS);
    let data = keywords.get(&cx.props.id)?;
    let user = use_read(&cx, crate::state::USERS).get(&data.creator)?;

    cx.render(rsx! (
        div { class: "py-4 px-6",
            div { class: "container px-4 mx-auto",
                h1 { class: "mb-2 text-5xl font-bold font-heading", "Search: \"{data.keyword}\"" }
                h1 { class: "mb-2 text-3xl font-bold font-heading" "searched by: {user.short_name}" }
            }
        }

        /*
        We care about whether or not a category is considered "fresh". IE is there a major owner of the market or is there good dispersion of revenue?

        - A histogram of revenue
        - A histogram of revenue divided by review velocity (is there a newcomer on the market?)


        */
        PlotContainer { entry: data }
        section { class: "text-gray-500 bg-white body-font mx-auto px-12 pt-12"
            ListingTable { id: cx.props.id }
        }
    ))
}

#[derive(PartialEq, Props)]
struct ListingTableProps {
    id: Uuid,
}

#[derive(PartialEq)]
enum Sortby {
    ReviewVelocity,
    NumReviews,
    Rating,
    Revenue,
    Sales,
    Price,
}

fn ListingTable(cx: Scope<ListingTableProps>) -> Element {
    let current_keyword = use_keyword_entry(&cx, cx.props.id)?;
    let show_more = use_ref(&cx, || HashSet::new());
    let sort_by = use_state(&cx, || Sortby::NumReviews);
    let reverse_sort = use_state(&cx, || false);

    let analsysis = use_read(&cx, crate::state::PRODUCT_ANALYSIS);

    let mut sorted_ids = current_keyword
        .products
        .iter()
        .map(|(id, k)| (k, analsysis.get(id).unwrap()))
        .collect::<Vec<_>>();

    match *sort_by {
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
        Sortby::Price => {
            sorted_ids.sort_by(|(_, a), (_, b)| a.price.partial_cmp(&b.price).unwrap())
        }
        Sortby::Sales => {
            sorted_ids.sort_by(|(_, a), (_, b)| a.sales.partial_cmp(&b.sales).unwrap())
        }
    }

    let mut rows = sorted_ids
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, (product, analysis))| {
            rsx!(TableRow {
                // todo: dioxus is broken around list re-ordring. Need to fix this.
                // key: "{product.asin}"
                is_gray: idx % 2 == 0,
                product: product,
                show_more: show_more,
                analysis: Some(*analysis)
            })
        })
        .collect::<Vec<_>>();

    if *reverse_sort {
        rows.reverse();
    }

    let page_header = current_keyword.products.values().next().map(|first| {
        rsx!(
            div { class: "container px-4 mx-auto",
                h2 { class: "text-2xl font-bold", "Viewing search: {current_keyword.keyword}" }
                h3 { class: "text-xl", "Category: {first.category.name}" }
            }
        )
    });

    let apply_sort = move |new: Sortby| {
        if *sort_by == new {
            reverse_sort.set(!*reverse_sort);
        } else {
            sort_by.set(new);
            reverse_sort.set(false);
        }
    };

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
                            thead {
                                tr { class: "text-xs text-gray-500 text-left",
                                    th { class: "pb-3 font-medium", "Image" }
                                    th { class: "pb-3 font-medium", "Name" }
                                    th { class: "pb-3 font-medium", "Price", icons::ChevronUpDown {}, onclick: move |_| apply_sort(Sortby::Price) }
                                    th { class: "pb-3 font-medium", "BSR", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::ReviewVelocity) }
                                    th { class: "pb-3 font-medium", "Created", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::ReviewVelocity) }
                                    th { class: "pb-3 font-medium", "Reviews", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::NumReviews) }
                                    th { class: "pb-3 font-medium", "Rating", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::Rating) }
                                    th { class: "pb-3 font-medium", "Sales", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::Sales) }
                                    th { class: "pb-3 font-medium", "Review Velocity", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::ReviewVelocity) }
                                    th { class: "pb-3 font-medium", "Revenue", icons::ChevronUpDown {} , onclick: move |_| apply_sort(Sortby::Revenue) }
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

fn TableRow<'a>(cx: Scope<'a, TableRowProps<'a>>) -> Element {
    let ProductListing {
        productData, asin, ..
    } = cx.props.product;

    let len = productData.title.len();
    let trim_len = if len > 25 { 25 } else { len };
    let title = &productData.title[..trim_len];

    let ProductAnalysis {
        sales,
        revenue,
        review_velocity,
        creation_date,
        rating,
        num_reviews,
        price,
    } = match cx.props.analysis {
        Some(a) => a.clone(),
        None => ProductAnalysis::new(cx.props.product),
    };

    let img_url = &productData.imageUrl;

    use num_format::{Locale, ToFormattedString};
    let loc = Locale::en;

    cx.render(rsx!(
        tr { class: format_args!("text-xs {}", if cx.props.is_gray { "bg-gray-50" } else { "" }),
            // onclick: move |_| { props.show_more.write().insert(props.product.asin.clone()); },
            // href: "#"
            td { class: "flex flex-row"
                img { class: "h-24 rounded", src: "{img_url}", }
            }
            td { class: "font-medium", a { "{title}...", href: "https://www.amazon.com/dp/{asin}" } }

            // td { class: "py-5 pl-6 font-medium", a { href: "{}", } }
            td { class: "font-medium", "${productData.price}" }
            td { class: "font-medium", "{productData.bsr}" }
            td { class: "font-medium", "{creation_date}" }
            td { class: "font-medium", {[format_args!("{}", (num_reviews as i64).to_formatted_string(&loc))]} }
            td { class: "font-medium", {[format_args!("{}", rating)]} }
            td { class: "font-medium", {[format_args!("${}", sales.to_formatted_string(&loc))]} }
            td { class: "font-medium", {[format_args!("{}", (review_velocity as i64).to_formatted_string(&loc))]} }
            td { class: "font-medium", {[format_args!("${}", (revenue as i64).to_formatted_string(&loc))]} }
        }
    ))
}

fn load_more(cx: Scope<()>) -> Element {
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
pub struct PlotContainerProps<'a> {
    entry: &'a KeywordEntry,
}
fn PlotContainer<'a>(cx: Scope<'a, PlotContainerProps<'a>>) -> Element {
    let regen = use_state(&cx, || false);

    if *regen {
        cx.push_task(move || async move {
            regen.set(false);
        });
        cx.render(rsx!("regenerating..."))
    } else {
        cx.render(rsx!(
            div { class: "container flex flex-row md:flex-row mx-auto"
                button {
                    onclick: move |_| regen.set(true),
                    "Regenerate Plots"
                }
                ReviewVelocity { entry: cx.props.entry, divid: "r1" }
                SalesPlot { entry: cx.props.entry, divid: "r2" }
            }
        ))
    }
}

#[derive(Props)]
pub struct PlotsProps<'a> {
    entry: &'a KeywordEntry,
    divid: &'static str,
}

/// Generate a revenue vs review plot
pub fn ReviewVelocity<'a>(cx: Scope<'a, PlotsProps<'a>>) -> Element {
    use plotly::{common::Mode, Plot, Scatter};
    use plotly::{common::Title, Layout};

    let prices = cx
        .props
        .entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(0..prices.len(), prices).mode(Mode::Markers));
    plot.set_layout(Layout::new().title(Title::new("Distribution of Revenue")));

    build_plotly(&cx, plot, cx.props.divid)
}

pub fn SalesPlot<'a>(cx: Scope<'a, PlotsProps<'a>>) -> Element {
    use plotly::{common::Mode, Plot, Scatter};
    use plotly::{common::Title, Layout};

    let prices = cx
        .props
        .entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let mut plot = Plot::new();

    plot.add_trace(
        Scatter::new(0..prices.len(), prices)
            .mode(Mode::Markers)
            .legend_group("review"),
    );
    plot.set_layout(Layout::new().title(Title::new(
        "Distribution of Revenue normalized by Review Velocity",
    )));

    build_plotly(&cx, plot, cx.props.divid)
}

fn build_plotly<'a>(cx: &'a ScopeState, plot: plotly::Plot, id: &str) -> Element<'a> {
    // need to trim out the div
    let raw = plot.to_inline_html(Some(id));
    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{id}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}
