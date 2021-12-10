use std::collections::HashMap;

use crate::actions::{fetch_asins_from_keyword, fetch_helium_10_from_asins, FetchError};
use crate::helium10::ProductResponse;
use crate::state::{use_keyword_entry, KeywordEntry, CURRENT_USER, KEYWORDS};
use atoms::{use_read, use_set};
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub enum SearchState {
    Nothing,
    Loading { msg: Option<String> },
    Error { msg: FetchError },
    Loaded,
}

pub fn Search(cx: Context, _props: &()) -> Element {
    let loading_state = use_state(cx, || SearchState::Nothing);
    let keyword = use_state(cx, || "".to_string());
    let current_result_entry = use_state(cx, || None);

    let current_user = use_read(cx, CURRENT_USER);
    let keywords_entries = use_read(cx, KEYWORDS);
    let set_entries = use_set(cx, KEYWORDS);

    let fetch_task = use_coroutine(cx, move || {
        // The task will persist between renders, so we need to make sure our values
        // and handles are used by ref
        let mut loading_state = loading_state.for_async();
        let mut selected_product = current_result_entry.for_async();

        let creator = current_user.clone();
        let keyword = keyword.inner();
        let set_entries = set_entries.clone();
        let mut new_entries = keywords_entries.clone();

        async move {
            loading_state.set(SearchState::Loading {
                msg: Some("Fetching products from Amazon".to_string()),
            });

            // Fetch the data
            let client = reqwest::Client::builder().build().unwrap();

            let new_asins = fetch_asins_from_keyword(&client, &keyword).await.unwrap();

            log::debug!("Fetched asins {:?}", new_asins);

            loading_state.set(SearchState::Loading {
                msg: Some("Fetching data from Helium 10".to_string()),
            });

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
                    keyword,
                    products,
                },
            );
            set_entries(new_entries);

            // and default the selection
            loading_state.set(SearchState::Loaded);
            selected_product.set(Some(id));
        }
    });

    let loading_msg = match loading_state.get() {
        SearchState::Nothing => rsx!(""),
        SearchState::Loading { msg } => rsx!({ [format_args!("Loading... {:?}", msg)] }),
        SearchState::Loaded => rsx!("Loaded!"),
        SearchState::Error { msg } => match msg {
            FetchError::Reqwest(err) => todo!(),
            FetchError::FailedToParse(err) => todo!(),
            FetchError::OutOfCredits => rsx!("Could not fetch data, out of credits."),
        },
    };

    cx.render(rsx! {
        section { class: "text-gray-600 body-font relative",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-12",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900", "Search Amazon for Keywords" }
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base", "Add your keywords here to search for amazon data" }
                }
                div { class: "lg:w-1/2 md:w-2/3 mx-auto",
                    div { class: "flex flex-wrap -m-2",
                        SearchBox { keyword: keyword }
                        ProductTable { cur_product: current_result_entry }
                        {loading_msg}
                        div { class: "p-2 w-full",
                            button { class: "flex mx-auto text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Download Helium10 Data"
                                onclick: move |_| fetch_task.start(),
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct ProductTableProps<'a> {
    cur_product: UseState<'a, Option<Uuid>>,
}

fn ProductTable(cx: Context, props: &ProductTableProps) -> Element {
    let product = use_keyword_entry(cx, *props.cur_product.get().as_ref()?);

    let rows = product.and_then(|entry| {
        let products = entry.products.values().enumerate().map(|(idx, product)| {
            let is_even = if idx % 2 == 0 { "bg-gray-50" } else { "" };
            let asin = &product.asin;
            let Product_Details = "";
            rsx!(
                tr { class: "text-xs {is_even}", key: "{asin}"
                    td { class: "py-5 px-6 font-medium", "{asin}" }
                    td { class: "font-medium", "{Product_Details}" }
                    td { class: "font-medium", "name@shuffle.dev" }
                    td { class: "font-medium", "Monthly" }
                    td {
                        span { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full",
                            "Completed"
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
                        th { class: "pb-3 font-medium", "Transaction ID" }
                        th { class: "pb-3 font-medium", "Date" }
                        th { class: "pb-3 font-medium", "E-mail" }
                        th { class: "pb-3 font-medium", "Subscription" }
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
struct SearchBoxProps<'a> {
    keyword: UseState<'a, String>,
}
fn SearchBox(cx: Context, props: &SearchBoxProps) -> Element {
    cx.render(rsx!(
        div { class: "p-2 w-1/2", 
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
