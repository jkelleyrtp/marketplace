use crate::{actions, helium10};
use dioxus::prelude::*;
use std::collections::HashMap;
use tokio::sync::futures;

use crate::{
    helium10::FlattenedEntry,
    state::{use_app_state, KeywordEntry},
    AppRoute,
};

pub fn Search(cx: Context, props: &()) -> Element {
    log::debug!("Rendeirng AddNew {:?}", cx.scope_id());

    let mut state = use_app_state(cx)?;
    let mut kword = use_state(cx, String::new);
    let mut contents = use_state(cx, String::new);
    let cur_user = state.read().current_user?;

    let (asins, set_asins) = use_state(cx, || Vec::<String>::new()).classic();

    log::debug!("new asins are {:?}", asins);

    let search_for_asins = move |_| {
        let set_asins2 = set_asins.clone();
        // let mut asins = asins.for_async();
        // let rt = tokio::runtime::Handle::current();
        let update = cx.schedule_update();

        let word = (*kword).clone();

        cx.push_task(|| async move {
            //
            let new_asins = actions::fetch_asins_from_keyword(&word).await.unwrap();
            dbg!("new asins fetched are {:?}", &new_asins);
            set_asins2(new_asins);
            update();
        });
    };

    let rows = asins.iter().enumerate().map(|(id, asin)| {
        //
        let is_even = if id % 2 == 0 { "bg-gray-50" } else { "" };

        rsx!(
            tr { class: "text-xs {is_even}",
                td { class: "py-5 px-6 font-medium", "{asin}" }
                td { class: "font-medium", "08.04.2021" }
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

    cx.render(rsx!{
        section { class: "text-gray-600 body-font relative",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-12",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900", "Search Amazon for Keywords" }
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base", "Add your keywords here to search for amazon data" }
                }
                div { class: "lg:w-1/2 md:w-2/3 mx-auto",
                    div { class: "flex flex-wrap -m-2",
                        // Keyword entry
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
                                    oninput: move |e| kword.set(e.value.clone()),
                                }
                            }
                        }
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
                                tbody { {rows} }
                            }
                        }

                        // Submit
                        div { class: "p-2 w-full",
                            button { class: "flex mx-auto text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Search Products"
                                onclick: {search_for_asins}
                            }
                        }
                    }
                }
            }
        }
    })
}

// let submit = move |_| {
//     let word = (*kword).clone();
//     let (_, mut set_asins) = asins.split_for_async();
//     let contents = contents.for_async();
//     let h = tokio::task::spawn_local(async move {
//         let asins = actions::fetch_asins_from_keyword(&word).await.unwrap();
//         set_asins.set(asins.clone());

//         let res = actions::fetch_helium_10_from_asins(&asins).await.unwrap();

//         let fname = word.replace(" ", "_");

//         let serialized = serde_json::to_string(&res).unwrap();
//         std::fs::write(format!("./data/{}.json", fname), serialized).unwrap();

//         // let flattned = res.to_flattened();
//         let entries = res
//             .data
//             .into_iter()
//             .map(|(k, v)| (k, v.data.flatten()))
//             .collect::<HashMap<_, _>>();
//     });
// };
