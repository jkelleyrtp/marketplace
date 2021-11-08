use dioxus::prelude::*;

use crate::state::use_app_state;
pub static Login: FC<()> = |(cx, props)| {
    let mut state = use_app_state(cx)?;

    let mut username = use_state(cx, String::new);
    let mut password = use_state(cx, String::new);
    let submit = |_| {
        //
    };

    cx.render(rsx!{
        section { class: "text-gray-600 body-font relative",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-12",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900", "Log into your account" }
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base", "Don't use a strong password. We have no security (yet)." }
                }
                div { class: "lg:w-1/2 md:w-2/3 mx-auto",
                    div { class: "flex flex-wrap -m-2",
                        div { class: "p-2 w-1/2",
                            div { class: "relative",
                                label { class: "leading-7 text-sm text-gray-600",
                                    r#for: "username",
                                    "Username"
                                }
                                input { class: "w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out",
                                    id: "username",
                                    r#type: "text",
                                    name: "username",
                                }
                            }
                        }
                        div { class: "p-2 w-1/2",
                            div { class: "relative",
                                label { class: "leading-7 text-sm text-gray-600",
                                    r#for: "password",
                                    "Password"
                                }
                                input { class: "w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out",
                                    id: "password",
                                    name: "password",
                                    r#type: "password",
                                }
                            }
                        }
                        div { class: "p-2 w-full",
                            div { class: "relative",
                                label { class: "leading-7 text-sm text-gray-600",
                                    r#for: "message",
                                    "Message"
                                }
                                textarea { class: "w-full bg-gray-100 bg-opacity-50 rounded border border-gray-300 focus:border-indigo-500 focus:bg-white focus:ring-2 focus:ring-indigo-200 h-32 text-base outline-none text-gray-700 py-1 px-3 resize-none leading-6 transition-colors duration-200 ease-in-out",
                                    id: "message",
                                    name: "message",
                                    ""
                                }
                            }
                        }
                        div { class: "p-2 w-full",
                            button { class: "flex mx-auto text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Log In"
                                onclick: {submit}
                            }
                        }
                    }
                }
            }
        }
    })
};
