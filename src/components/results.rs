use crate::plots::salesscatter::SalesScatter;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct ResultsPageProps {
    id: Uuid,
}

pub fn ResultsPage(cx: &Scope, props: &ResultsPageProps) -> Element {
    cx.render(rsx! (
        section { class: "text-gray-500 bg-white body-font mx-auto px-12 pt-12"
            div { class: "container flex flex-row md:flex-row py-10 mx-auto"
                SalesScatter {
                    xrow: "",
                    yrow: "",
                    title: "Revenue vs Review Count"
                    id: props.id,
                }
                SalesScatter {
                    xrow: "",
                    yrow: "",
                    title: "Revenue vs Review Count"
                    id: props.id,
                }
            }
            ListingTable {
                id: props.id
            }
        }
    ))
}

#[derive(PartialEq, Props)]
struct ListingTableProps {
    id: Uuid,
}

fn ListingTable(cx: &Scope, _props: &ListingTableProps) -> Element {
    let rows = ["r1", "r2", "r3", "r1", "r2", "r3"]
        .iter()
        .cycle()
        .take(20)
        .enumerate()
        .map(|(id, b)| {
            rsx!(
                tr { class: format_args!("text-xs {}", if id % 2 == 0 { "bg-gray-50" } else { "" }),
                    td { class: "py-5 px-6 font-medium", "SR2451EW32" }
                    td { class: "font-medium", "08.04.2021" }
                    td { class: "font-medium", "name@shuffle.dev" }
                    td { class: "font-medium", "Monthly" }
                    td { span { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full", "Completed" } }
                }
            )
        });

    cx.render(rsx!{
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                div { class: "pt-4 bg-white shadow rounded",
                    div { class: "flex px-6 pb-4 border-b", h3 { class: "text-xl font-bold", "Recent Transactions" } }
                    div { class: "p-4",
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
