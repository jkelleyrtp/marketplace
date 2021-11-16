use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    helium10::FlattenedEntry,
    state::{use_app_state, KeywordEntry},
    AppRoute,
};

use csv;

pub static AddNew: FC<()> = |cx, props| {
    log::debug!("Rendeirng AddNew {:?}", cx.scope_id());

    let mut state = use_app_state(cx)?;
    let mut name = use_state(cx, String::new);
    let mut contents = use_state(cx, String::new);
    let cur_user = state.read().current_user?;

    let submit = move |_| {
        let mut rdr = csv::Reader::from_reader(contents.as_bytes());

        let mut entries = HashMap::new();
        for result in rdr.deserialize() {
            let record: FlattenedEntry = result.unwrap();
            entries.insert(record.ASIN.clone(), record);
        }

        let mut st = state.write();
        st.route = AppRoute::Home;
        st.cached_data.keywords.insert(
            uuid::Uuid::new_v4(),
            KeywordEntry {
                creator: cur_user,
                products: entries,
                keyword: name.to_string(),
            },
        );
    };

    cx.render(rsx!{
        section { class: "text-gray-600 body-font relative",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-12",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900", "Add dataset from CSV" }
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base", "Paste the CSV from Helium10 to enter the search into the database" }
                }
                div { class: "lg:w-1/2 md:w-2/3 mx-auto",
                    div { class: "flex flex-wrap -m-2",
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
                                    oninput: move |e| name.set(e.value.clone()),
                                }
                            }
                        }
                        div { class: "p-2 w-full",
                            div { class: "relative",
                                label { class: "leading-7 text-sm text-gray-600",
                                    r#for: "message",
                                    "Dataset (csv)"
                                }
                                textarea { class: "w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 h-32 text-base outline-none text-gray-700 py-1 px-3 resize-none leading-6 transition-colors duration-200 ease-in-out",
                                    id: "message",
                                    name: "message",
                                    ""
                                    oninput: move |e| contents.set(e.value.clone()),
                                }
                            }
                        }
                        div { class: "p-2 w-full",
                            button { class: "flex mx-auto text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Add Dataset"
                                onclick: {submit}
                            }
                        }
                    }
                }
            }
        }
    })
};
