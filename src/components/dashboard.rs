use dioxus::prelude::*;

use crate::{plots::salesscatter::Plots, state::use_current_user};

pub static Dashboard: Component<()> = |cx, props| {
    let name = &use_current_user(cx)?.name;

    cx.render(rsx!(
        div { class: "py-8 px-6",
            div { class: "container px-4 mx-auto",
                h2 { class: "text-2xl font-bold", "Welcome, {name} 👋" }
            }
        }
        QuickActions {}
        FeaturedSearches {}
    ))
};

static QuickActions: Component<()> = |cx, _| {
    cx.render(rsx!(
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                div { class: "flex flex-wrap items-stretch -m-4",
                    div { class: "w-full lg:w-1/2 p-4",
                        div { class: "bg-white shadow rounded overflow-hidden",
                            div { class: "pt-6 px-6 mb-8",
                                div { class: "flex mb-8 justify-between items-center",
                                    p { class: "text-sm text-gray-500",
                                        "Important information"
                                    }
                                }
                                div {
                                    h3 { class: "mb-2 text-xl font-bold", "Add new products by searching Amazon" }
                                    p { class: "text-sm text-gray-500",
                                        "You have been invited to join this event. It is only for team members."
                                    }
                                }
                            }
                            div { class: "p-6 flex items-center justify-between bg-lightGray-50",
                                div {
                                    p { class: "mb-2 text-xs text-gray-500 font-medium",
                                        "Final Date"
                                    }
                                    span { class: "inline-block py-1 px-2 rounded-full bg-orange-50 text-xs text-red-500",
                                        "20 September 2021"
                                    }
                                }
                                a { class: "py-2 px-3 bg-indigo-500 hover:bg-indigo-600 rounded text-xs text-white",
                                    href: "#",
                                    "View Details"
                                }
                            }
                        }
                    }
                    div { class: "w-full lg:w-1/2 p-4",
                        div { class: "bg-white shadow rounded overflow-hidden",
                            div { class: "pt-6 px-6 mb-8",
                                div { class: "flex mb-8 justify-between items-center",
                                    p { class: "text-sm text-gray-500",
                                        "Weekly schedule"
                                    }
                                }
                                div {
                                    h3 { class: "mb-2 text-xl font-bold", "Review all products" }
                                    p { class: "text-sm text-gray-500",
                                        "You have been invited to join this event. It is only for team members."
                                    }
                                }
                            }
                            div { class: "p-6 flex items-center justify-between bg-lightGray-50",
                                div {
                                    p { class: "mb-2 text-xs text-gray-500 font-medium",
                                        "Place"
                                    }
                                    span { class: "inline-block py-1 px-2 rounded-full bg-blue-50 text-xs text-blue-500",
                                        "Online Meeting"
                                    }
                                }
                                a { class: "py-2 px-3 bg-indigo-500 hover:bg-indigo-600 rounded text-xs text-white",
                                    href: "#",
                                    "View Details"
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
};

static FeaturedSearches: Component<()> = |cx, _| {
    cx.render(rsx!(
        section { class: "py-8",
            div { class: "container px-4 mx-auto",
                h2 { class: "text-2xl font-bold", "Recent Searches" }
                div { class: "flex flex-wrap -m-4",
                    div { class: "w-full lg:w-1/3 p-4",
                        div { class: "p-4 bg-white rounded",
                            div { class: "relative h-40 w-full mb-4",
                                img { class: "w-full h-full object-cover rounded",
                                    src: "https://images.unsplash.com/photo-1506792006437-256b665541e2?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=334&amp;q=80",
                                    alt: "",
                                }
                                span { class: "absolute top-0 right-0 py-1 px-2 mt-2 mr-2 bg-indigo-500 rounded text-xs text-white",
                                    "14 Tasks"
                                }
                            }
                            div { class: "flex mb-6 justify-between items-center",
                                div {
                                    h3 { class: "text-sm font-medium",
                                        "Shuffle - an online editor"
                                    }
                                    span { class: "text-xs text-gray-500",
                                        "Production company"
                                    }
                                }
                                button { class: "ml-auto p-2 bg-indigo-50 rounded",
                                    svg {
                                        view_box: "0 0 16 16",
                                        fill: "none",
                                        width: "16",
                                        height: "16",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path {
                                            d: "M7.99984 9.33335C8.73622 9.33335 9.33317 8.7364 9.33317 8.00002C9.33317 7.26364 8.73622 6.66669 7.99984 6.66669C7.26346 6.66669 6.6665 7.26364 6.6665 8.00002C6.6665 8.7364 7.26346 9.33335 7.99984 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                        path {
                                            fill: "#382CDD",
                                            d: "M3.33333 9.33335C4.06971 9.33335 4.66667 8.7364 4.66667 8.00002C4.66667 7.26364 4.06971 6.66669 3.33333 6.66669C2.59695 6.66669 2 7.26364 2 8.00002C2 8.7364 2.59695 9.33335 3.33333 9.33335Z",
                                        }
                                        path {
                                            d: "M12.6668 9.33335C13.4032 9.33335 14.0002 8.7364 14.0002 8.00002C14.0002 7.26364 13.4032 6.66669 12.6668 6.66669C11.9304 6.66669 11.3335 7.26364 11.3335 8.00002C11.3335 8.7364 11.9304 9.33335 12.6668 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                    }
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Start"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-green-50 text-xs text-green-500",
                                    "08 March 2021"
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Final Date"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-red-50 text-xs text-red-500",
                                    "14 March 2021"
                                }
                            }
                            div { class: "flex mb-5 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Last Change"
                                }
                                span { class: "text-xs text-indigo-500 font-medium",
                                    "6 days ago"
                                }
                            }
                            div { class: "flex items-ceenter justify-between border-t border-gray-50 pt-4",
                                div { class: "flex",
                                    img { class: "w-8 h-8 rounded-full object-cover object-right",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover object-top",
                                        src: "https://images.unsplash.com/photo-1528936466093-76ffdfcf7a40?ixid=MnwxMjA3fDB8MHxwcm9maWxlLXBhZ2V8MXx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=500&amp;q=60",
                                        alt: "",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1564564321837-a57b7070ac4f?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1055&amp;q=80",
                                    }
                                    div { class: "flex items-center justify-center w-8 h-8 -ml-2 rounded-full bg-indigo-50 text-xs text-indigo-500",
                                        "+3"
                                    }
                                }
                                a { class: "py-2 px-3 bg-indigo-500 hover:bg-indigo-600 rounded text-xs text-white transition duration-200",
                                    href: "#",
                                    "See Details"
                                }
                            }
                        }
                    }
                    div { class: "w-full lg:w-1/3 p-4",
                        div { class: "p-4 bg-white rounded",
                            div { class: "relative h-40 w-full mb-4",
                                img { class: "w-full h-full object-cover rounded",
                                    src: "https://images.unsplash.com/photo-1589307004173-3c95204d00ee?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=334&amp;q=80",
                                    alt: "",
                                }
                                span { class: "absolute top-0 right-0 py-1 px-2 mt-2 mr-2 bg-indigo-500 rounded text-xs text-white",
                                    "14 Tasks"
                                }
                            }
                            div { class: "flex mb-6 justify-between items-center",
                                div {
                                    h3 { class: "text-sm font-medium",
                                        "Shuffle - an online editor"
                                    }
                                    span { class: "text-xs text-gray-500",
                                        "Production company"
                                    }
                                }
                                button { class: "ml-auto p-2 bg-indigo-50 rounded",
                                    svg {
                                        width: "16",
                                        view_box: "0 0 16 16",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        height: "16",
                                        fill: "none",
                                        path {
                                            d: "M7.99984 9.33335C8.73622 9.33335 9.33317 8.7364 9.33317 8.00002C9.33317 7.26364 8.73622 6.66669 7.99984 6.66669C7.26346 6.66669 6.6665 7.26364 6.6665 8.00002C6.6665 8.7364 7.26346 9.33335 7.99984 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                        path {
                                            fill: "#382CDD",
                                            d: "M3.33333 9.33335C4.06971 9.33335 4.66667 8.7364 4.66667 8.00002C4.66667 7.26364 4.06971 6.66669 3.33333 6.66669C2.59695 6.66669 2 7.26364 2 8.00002C2 8.7364 2.59695 9.33335 3.33333 9.33335Z",
                                        }
                                        path {
                                            d: "M12.6668 9.33335C13.4032 9.33335 14.0002 8.7364 14.0002 8.00002C14.0002 7.26364 13.4032 6.66669 12.6668 6.66669C11.9304 6.66669 11.3335 7.26364 11.3335 8.00002C11.3335 8.7364 11.9304 9.33335 12.6668 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                    }
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Start"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-green-50 text-xs text-green-500",
                                    "08 March 2021"
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Final Date"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-red-50 text-xs text-red-500",
                                    "14 March 2021"
                                }
                            }
                            div { class: "flex mb-5 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Last Change"
                                }
                                span { class: "text-xs text-indigo-500 font-medium",
                                    "6 days ago"
                                }
                            }
                            div { class: "flex items-ceenter justify-between border-t border-gray-50 pt-4",
                                div { class: "flex",
                                    img { class: "w-8 h-8 rounded-full object-cover object-right",
                                        src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                        alt: "",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover object-top",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1528936466093-76ffdfcf7a40?ixid=MnwxMjA3fDB8MHxwcm9maWxlLXBhZ2V8MXx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=500&amp;q=60",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        src: "https://images.unsplash.com/photo-1564564321837-a57b7070ac4f?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1055&amp;q=80",
                                        alt: "",
                                    }
                                    div { class: "flex items-center justify-center w-8 h-8 -ml-2 rounded-full bg-indigo-50 text-xs text-indigo-500",
                                        "+3"
                                    }
                                }
                                a { class: "py-2 px-3 bg-indigo-500 hover:bg-indigo-600 rounded text-xs text-white transition duration-200",
                                    href: "#",
                                    "See Details"
                                }
                            }
                        }
                    }
                    div { class: "w-full lg:w-1/3 p-4",
                        div { class: "p-4 bg-white rounded",
                            div { class: "relative h-40 w-full mb-4",
                                img { class: "w-full h-full object-cover rounded",
                                    src: "https://images.unsplash.com/photo-1483959651481-dc75b89291f1?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=301&amp;q=80",
                                    alt: "",
                                }
                                span { class: "absolute top-0 right-0 py-1 px-2 mt-2 mr-2 bg-indigo-500 rounded text-xs text-white",
                                    "14 Tasks"
                                }
                            }
                            div { class: "flex mb-6 justify-between items-center",
                                div {
                                    h3 { class: "text-sm font-medium",
                                        "Shuffle - an online editor"
                                    }
                                    span { class: "text-xs text-gray-500",
                                        "Production company"
                                    }
                                }
                                button { class: "ml-auto p-2 bg-indigo-50 rounded",
                                    svg {
                                        height: "16",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 16 16",
                                        fill: "none",
                                        width: "16",
                                        path {
                                            d: "M7.99984 9.33335C8.73622 9.33335 9.33317 8.7364 9.33317 8.00002C9.33317 7.26364 8.73622 6.66669 7.99984 6.66669C7.26346 6.66669 6.6665 7.26364 6.6665 8.00002C6.6665 8.7364 7.26346 9.33335 7.99984 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                        path {
                                            fill: "#382CDD",
                                            d: "M3.33333 9.33335C4.06971 9.33335 4.66667 8.7364 4.66667 8.00002C4.66667 7.26364 4.06971 6.66669 3.33333 6.66669C2.59695 6.66669 2 7.26364 2 8.00002C2 8.7364 2.59695 9.33335 3.33333 9.33335Z",
                                        }
                                        path {
                                            d: "M12.6668 9.33335C13.4032 9.33335 14.0002 8.7364 14.0002 8.00002C14.0002 7.26364 13.4032 6.66669 12.6668 6.66669C11.9304 6.66669 11.3335 7.26364 11.3335 8.00002C11.3335 8.7364 11.9304 9.33335 12.6668 9.33335Z",
                                            fill: "#382CDD",
                                        }
                                    }
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Start"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-green-50 text-xs text-green-500",
                                    "08 March 2021"
                                }
                            }
                            div { class: "flex mb-2 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Final Date"
                                }
                                span { class: "inline-block py-1 px-2 rounded-full bg-red-50 text-xs text-red-500",
                                    "14 March 2021"
                                }
                            }
                            div { class: "flex mb-5 justify-between items-center",
                                h4 { class: "text-xs font-medium",
                                    "Last Change"
                                }
                                span { class: "text-xs text-indigo-500 font-medium",
                                    "6 days ago"
                                }
                            }
                            div { class: "flex items-ceenter justify-between border-t border-gray-50 pt-4",
                                div { class: "flex",
                                    img { class: "w-8 h-8 rounded-full object-cover object-right",
                                        src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                        alt: "",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover object-top",
                                        src: "https://images.unsplash.com/photo-1528936466093-76ffdfcf7a40?ixid=MnwxMjA3fDB8MHxwcm9maWxlLXBhZ2V8MXx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=500&amp;q=60",
                                        alt: "",
                                    }
                                    img { class: "w-8 h-8 -ml-2 rounded-full object-cover",
                                        src: "https://images.unsplash.com/photo-1564564321837-a57b7070ac4f?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;ixlib=rb-1.2.1&amp;auto=format&amp;fit=crop&amp;w=1055&amp;q=80",
                                        alt: "",
                                    }
                                    div { class: "flex items-center justify-center w-8 h-8 -ml-2 rounded-full bg-indigo-50 text-xs text-indigo-500",
                                        "+3"
                                    }
                                }
                                a { class: "py-2 px-3 bg-indigo-500 hover:bg-indigo-600 rounded text-xs text-white transition duration-200",
                                    href: "#",
                                    "See Details"
                                }
                            }
                        }
                    }
                }
            }
        }

    ))
};
