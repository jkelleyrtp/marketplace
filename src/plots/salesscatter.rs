use crate::helium10::{calculate_review_velocity, ProductListing};
use crate::state::KeywordEntry;
use atoms::use_read;
use dioxus::prelude::*;
use plotly::{common::Mode, Plot, Scatter};
use plotly::{common::Title, Layout};
use std::collections::HashMap;

#[derive(PartialEq, Props)]
pub struct PlotsProps {
    entry_id: uuid::Uuid,
}

pub fn Plots(cx: Context, props: &PlotsProps) -> Element {
    let keywords = use_read(cx, crate::state::Keywords);
    let data = keywords.get(&props.entry_id)?;

    let velocities = data
        .products
        .iter()
        .map(|(k, p)| (k, calculate_review_velocity(p)))
        .collect::<HashMap<_, _>>();

    cx.render(rsx! {
        div { class: "container flex flex-row md:flex-row py-10 mx-auto"
            "{props.entry_id}",
            "{data.keyword}",
            {ReviewVelocity(cx, &data)}
            {SalesPlot(cx, &data)}
        }
    })
}

fn ReviewVelocity(cx: Context, entry: &KeywordEntry) -> Element {
    let prices = entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let x = (0..prices.len()).collect::<Vec<_>>();

    let mut plot = Plot::new();
    let trace = Scatter::new(x, prices).mode(Mode::Markers);

    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::new("Sales")));

    let name = "SalesPlot";

    let raw = plot.to_inline_html(Some(name));
    let raw_wo_div = &raw[128..raw.len() - 9];

    dbg!(raw_wo_div);

    cx.render(rsx! {
        div {
            div { id: "{name}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}

fn SalesPlot(cx: Context, entry: &KeywordEntry) -> Element {
    let prices = entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    let x = (0..prices.len()).collect::<Vec<_>>();

    let mut plot = Plot::new();
    let trace = Scatter::new(x, prices).mode(Mode::Markers);

    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::new("Sales")));

    let name = "SalesPlot";

    let raw = plot.to_inline_html(Some(name));
    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{name}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}
