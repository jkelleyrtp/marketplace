use crate::state::KeywordEntry;
use atoms::use_read;
use dioxus::prelude::*;
use plotly::{common::Mode, Plot, Scatter};
use plotly::{common::Title, Layout};

#[derive(Props)]
pub struct PlotsProps<'a> {
    entry: &'a KeywordEntry,
}

pub fn ReviewVelocity(cx: Context, props: &PlotsProps) -> Element {
    let prices = props
        .entry
        .products
        .iter()
        .map(|(_asin, listing)| listing.productData.price)
        .collect::<Vec<_>>();

    // let velocities = data
    //     .products
    //     .iter()
    //     .map(|(k, p)| (k, calculate_review_velocity(p)))
    //     .collect::<HashMap<_, _>>();

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

pub fn SalesPlot(cx: Context, props: &PlotsProps) -> Element {
    let prices = props
        .entry
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
