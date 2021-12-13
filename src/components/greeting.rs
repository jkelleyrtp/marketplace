use crate::state::use_current_user;
use dioxus::prelude::*;

pub static Greeting: Component<()> = |cx, _| {
    let user = use_current_user(cx)?;

    cx.render(rsx!(
        div { class: "py-8 px-6",
            div { class: "container px-4 mx-auto",
                h1 { class: "mb-2 text-5xl font-bold font-heading", "Welcome back, {user.short_name} 👋" }
                h1 { class: "mb-2 text-3xl font-bold font-heading" "You have {user.credits} x-ray credits left" }

                p { class: "mb-2 text-base text-gray-500",
                    "This marketplace tool is useful for finding product ideas from data on Amazon."
                    "Start by searching for new products or analyzing existing keywords."
                }
            }
        }
    ))
};
