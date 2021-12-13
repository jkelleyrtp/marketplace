use crate::actions::{fetch_helium_10_from_asins, FetchError};
use crate::api::amazon::{fetch_asins_from_keyword, ScrapedAmazonListing};
use crate::api::helium10::ProductResponse;
use crate::icons;
use crate::state::{KeywordEntry, CURRENT_USER, KEYWORDS, SCRAPER_CFG};

use atoms::{use_read, use_set};
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

const USE_DUMMY_API: bool = true;

pub fn Search(cx: Context, _props: &()) -> Element {
    // Internal state
    let loading_state = use_state(cx, || SearchState::Nothing);
    let keyword_input = use_state(cx, || "".to_string());
    let amazon_results = use_state(cx, || None);
    let selected_keyword = use_state(cx, || None);

    // Global state
    let current_user = use_read(cx, CURRENT_USER);
    let helium_data = use_read(cx, KEYWORDS);
    let set_entries = use_set(cx, KEYWORDS);
    let amazon_cfg = use_read(cx, SCRAPER_CFG).get("amazon_search")?;

    let fetch_amazon_data = use_coroutine(cx, move || {
        let mut loading_state = loading_state.for_async();
        let mut amazon_results = amazon_results.for_async();
        let amazon_cfg = amazon_cfg.to_owned();
        let keyword = keyword_input.get().to_owned();

        async move {
            loading_state.set(SearchState::fetching_from_amazon());

            // we don't want to get throttled by amazon when just testing
            if USE_DUMMY_API {
                let contents = std::fs::read_to_string("data/scrape.html").unwrap();
                let results = crate::api::amazon::parse_document(&contents);
                amazon_results.set(Some(AmazonSearch { keyword, results }));
                return;
            }

            let client = reqwest::Client::builder().build().unwrap();
            let results = fetch_asins_from_keyword(&client, &amazon_cfg, true, &keyword)
                .await
                .unwrap();

            amazon_results.set(Some(AmazonSearch { keyword, results }));
            loading_state.set(SearchState::Nothing);
        }
    });

    let fetch_helium_data = use_coroutine(cx, move || {
        let mut loading_state = loading_state.for_async();
        let mut selected_keyword = selected_keyword.for_async();
        let mut new_entries = helium_data.clone();
        let cur_results = amazon_results.get_rc().clone();
        let creator = current_user.clone();
        let set_entries = set_entries.clone();

        async move {
            let client = reqwest::Client::builder().build().unwrap();

            loading_state.set(SearchState::Loading {
                msg: Some("Fetching data from Helium 10".to_string()),
            });

            if let Some(results) = cur_results.as_ref() {
                let new_asins = results
                    .results
                    .iter()
                    .map(|r| r.asin.clone())
                    .collect::<Vec<_>>();

                let products = match fetch_helium_10_from_asins(&client, &new_asins).await {
                    Ok(products) => products
                        .data
                        .into_iter()
                        .filter_map(|(asin, product)| match product {
                            ProductResponse::Success(product) => Some((asin, product)),
                            ProductResponse::Error(_) => None,
                        })
                        .collect::<HashMap<_, _>>(),

                    Err(e) => {
                        loading_state.set(SearchState::Error { msg: e });
                        return;
                    }
                };

                // Create the new entry and add it to the global state
                let id = Uuid::new_v4();
                new_entries.insert(
                    id,
                    KeywordEntry {
                        creator: creator.unwrap(),
                        keyword: results.keyword.clone(),
                        products,
                    },
                );
                set_entries(new_entries);

                // and default the selection
                loading_state.set(SearchState::Loaded);
                selected_keyword.set(Some(id));
            }
        }
    });

    cx.render(rsx! {
        section { class: "text-gray-600 body-font relative",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-12",
                    h1 { class: "mb-2 text-5xl font-bold font-heading", "Search Amazon for Keywords" },
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base", "Add your keywords here to scrape the amazon search page." }
                }
                div { class: "w-1/3 mx-auto ", SearchBox { keyword: keyword_input } }
                div { class: "w-2/3 mx-auto "
                    div { class: "flex flex-col w-full p-2",
                        div { class: "p-2 w-full",
                            button { class: "flex mx-auto text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Search Amazon"
                                onclick: move |_| fetch_amazon_data.start(),
                            }
                        }
                        div { class: "p-2 w-full",
                            button {
                                class: format_args!("flex mx-auto text-white border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg {}", if amazon_results.is_some() { "bg-indigo-500" } else { "bg-gray-500" }),
                                "Download Helium10"
                                onclick: move |_| fetch_helium_data.start(),
                            }
                        }
                    }
                    LoadingBanner { loading_state: loading_state }
                }

                div { class: "lg:w-2/3 md:w-2/3 mx-auto",
                    div { class: "flex flex-wrap -m-2",
                        ProductTable { amazon_results: amazon_results }
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct ProductTableProps<'a> {
    amazon_results: UseState<'a, Option<AmazonSearch>>,
}

fn ProductTable(cx: Context, props: &ProductTableProps) -> Element {
    if props.amazon_results.get().is_none() {
        return rsx!(cx, div {});
    }

    let rows = props.amazon_results.get().as_ref().and_then(|search| {
        let products = search.results.iter().enumerate().map(|(idx, list)| {
            let ScrapedAmazonListing {
                name,
                asin,
                price,
                rating,
                num_reviews,
                img_url,
                ..
            } = list;

            let is_even = if idx % 2 == 0 { "bg-gray-50" } else { "" };

            rsx!(
                tr { class: "text-xs {is_even}", key: "{asin}"
                    td { class: "py-5 px-6 font-medium",
                        a { "{name}", href: "https://www.amazon.com/dp/{asin}" }
                    }
                    td { class: "font-medium", "{rating}" }
                    td { class: "font-medium", "${price}" }
                    td { class: "font-medium", "{num_reviews}" }
                    td {
                        class: "flex flex-row"
                        img {
                            class: "h-12 rounded-full",
                            src: "{img_url}",
                        }
                        img {
                            class: "h-12 rounded-full",
                            src: "{img_url}",
                        }
                        img {
                            class: "h-12 rounded-full",
                            src: "{img_url}",
                        }
                    }
                }
            )
        });

        cx.render(rsx!({ products }))
    });

    cx.render(rsx!(
        div { class: "p-2 w-full",
            table { class: "table-auto w-full",
                thead {
                    tr { class: "text-xs text-gray-500 text-left",
                        th { class: "pb-3 font-medium", "Product" }
                        th { class: "pb-3 font-medium", "Rating" }
                        th { class: "pb-3 font-medium", "Price" }
                        th { class: "pb-3 font-medium", "Num Reviews" }
                        th { class: "pb-3 font-medium", "Status" }
                    }
                }
                tbody {
                    {rows}
                }
            }
        }
    ))
}

#[derive(Props)]
struct LoadingBannerProps<'a> {
    loading_state: UseState<'a, SearchState>,
}

fn LoadingBanner(cx: Context, props: &LoadingBannerProps) -> Element {
    let text = match props.loading_state.get() {
        SearchState::Nothing => return cx.render(rsx!(div {})),
        SearchState::Loading { msg } => match msg {
            Some(msg) => rsx!({ [format_args!("Loading... {}", msg)] }),
            None => rsx!("Loading..."),
        },
        SearchState::Loaded => rsx!("Loaded!"),
        SearchState::Error { msg } => match msg {
            FetchError::Reqwest(err) => {
                rsx!({ [format_args!("Connecting to server failed. {:}", err)] })
            }
            FetchError::FailedToParse(err) => {
                rsx!({ [format_args!("Failed to parse response from API. {:}", err)] })
            }
            FetchError::OutOfCredits => rsx!("Could not fetch data, out of credits."),
        },
    };

    cx.render(rsx!(
        div { class: "py-8 px-6",
            div { class: "p-6 bg-indigo-50 border-l-4 border-indigo-500 rounded-r-lg",
                div { class: "flex items-center",
                    span { class: "inline-block mr-2", icons::Alert {} }
                    h3 { class: "text-indigo-800 font-medium", {text} }
                    button { class: "ml-auto", icons::Close {} }
                }
            }
        }
    ))
}

#[derive(Props)]
struct SearchBoxProps<'a> {
    keyword: UseState<'a, String>,
}
fn SearchBox(cx: Context, props: &SearchBoxProps) -> Element {
    cx.render(rsx!(
        div { class: "p-2 mx-auto",
            div { class: "relative",
                label { class: "leading-7 text-sm text-gray-600",
                    r#for: "name",
                    "Search keyword"
                }
                input { class: "w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out",
                    id: "name",
                    r#type: "text",
                    name: "name",
                    oninput: move |e| props.keyword.set(e.value.clone()),
                }
            }
        }
    ))
}

#[derive(Debug)]
pub enum SearchState {
    Nothing,
    Loading { msg: Option<String> },
    Error { msg: FetchError },
    Loaded,
}
impl SearchState {
    fn fetching_from_amazon() -> Self {
        SearchState::Loading {
            msg: Some("Fetching products from Amazon".to_string()),
        }
    }
}

struct AmazonSearch {
    keyword: String,
    results: Vec<ScrapedAmazonListing>,
}
