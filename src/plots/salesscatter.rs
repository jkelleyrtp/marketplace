use crate::helium10::{calculate_review_velocity, ProductListing};
use crate::state::{use_app_state, KeywordEntry};
use dioxus::prelude::*;
use plotly::{common::Mode, Plot, Scatter};
use plotly::{common::Title, Layout};
use std::collections::HashMap;

#[derive(PartialEq, Props)]
pub struct PlotsProps {
    entry_id: uuid::Uuid,
}

pub fn Plots(cx: Context, props: &PlotsProps) -> Element {
    let state = use_app_state(cx)?;
    let read = state.read();
    let data = read.keywords.get(&props.entry_id).unwrap();

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
            // {SalesPlot(cx, &data)}
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
    // let prices = entry
    //     .products
    //     .iter()
    //     .map(|(_asin, listing)| listing.productData.price)
    //     .collect::<Vec<_>>();

    // let x = (0..prices.len()).collect::<Vec<_>>();

    // let mut plot = Plot::new();
    // let trace = Scatter::new(x, prices).mode(Mode::Markers);

    // plot.add_trace(trace);
    // plot.set_layout(Layout::new().title(Title::new("Sales")));

    // let name = "ReviewVelocity";

    // let raw = plot.to_inline_html(Some(name));
    // let raw_wo_div = &raw[128..raw.len() - 9];

    // cx.render(rsx! {
    //     div {
    //         div { id: "{name}", class: "" }
    //         script { "{raw_wo_div}" }
    //     }
    // })
}

// fn SalesPlot(cx: Context, entry: &KeywordEntry) -> Element {
//     let prices = entry
//         .products
//         .iter()
//         .map(|(_asin, listing)| listing.productData.price)
//         .collect::<Vec<_>>();

//     let x = (0..prices.len()).collect::<Vec<_>>();

//     let mut plot = Plot::new();
//     let trace = Scatter::new(x, prices).mode(Mode::Markers);

//     plot.add_trace(trace);
//     plot.set_layout(Layout::new().title(Title::new("Sales")));

//     let name = "SalesPlot";

//     let raw = plot.to_inline_html(Some(name));
//     let raw_wo_div = &raw[128..raw.len() - 9];

//     cx.render(rsx! {
//         div {
//             div { id: "{name}", class: "" }
//             script { "{raw_wo_div}" }
//         }
//     })
// }

// "PLOTLYENV=window.PLOTLYENV || {};\n    if (document.getElementById(\"SalesPlot\")) {\n        var d3 = Plotly.d3;\n        var image_element= d3.select('#image-export');\n        var trace_0 = {\"type\":\"scatter\",\"mode\":\"markers\",\"x\":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60],\"y\":[37.99,18.27,21.59,22.97,41.99,29.99,34.99,44.99,38.99,46.98,23.46,36.39,29.99,94.99,72.98,52.99,34.57,25.5,39.99,76.48,25.88,33.79,-1.0,39.99,26.99,21.99,49.79,42.55,45.49,39.99,38.99,64.98,29.99,41.99,47.98,47.08,29.99,54.88,17.98,43.99,17.98,47.99,19.98,69.98,39.99,36.99,11.99,29.99,39.99,109.99,29.99,35.99,39.99,46.99,47.99,199.99,59.99,39.99,43.96,45.96,44.99]};\nvar data = [trace_0];\nvar layout = {\"title\":{\"text\":\"Sales\"}};\n        Plotly.newPlot('SalesPlot', data, layout, {\"responsive\": true});\n    };\n"

// [src/plots/salesscatter.rs:54] raw_wo_div = "PLOTLYENV=window.PLOTLYENV || {};\n    if (document.getElementById(\"SalesPlot\")) {\n        var d3 = Plotly.d3;\n        var image_element= d3.select('#image-export');\n        var trace_0 = {\"type\":\"scatter\",\"mode\":\"markers\",\"x\":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49],\"y\":[6.99,24.99,19.89,13.95,17.98,14.97,12.99,9.99,11.99,22.99,8.99,28.99,9.94,9.99,17.98,9.97,6.91,12.8,16.98,7.99,22.95,9.99,9.99,14.99,12.99,6.99,14.99,8.99,12.99,5.99,11.95,9.39,8.99,16.69,8.99,12.99,9.89,12.99,7.99,6.99,4.99,6.99,9.99,4.12,6.99,9.99,14.66,14.98,11.89,5.78]};\nvar data = [trace_0];\nvar layout = {\"title\":{\"text\":\"Sales\"}};\n        Plotly.newPlot('SalesPlot', data, layout, {\"responsive\": true});\n    };\n"
// [src/plots/salesscatter.rs:54] raw_wo_div = "PLOTLYENV=window.PLOTLYENV || {};\n    if (document.getElementById(\"SalesPlot\")) {\n        var d3 = Plotly.d3;\n        var image_element= d3.select('#image-export');\n        var trace_0 = {\"type\":\"scatter\",\"mode\":\"markers\",\"x\":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66],\"y\":[30.99,34.99,21.99,31.85,26.99,23.99,32.99,18.23,51.99,26.99,15.99,38.99,46.98,17.98,79.98,28.04,24.99,36.99,33.85,32.99,25.99,39.99,21.99,35.99,37.99,42.0,-1.0,84.99,49.99,15.99,33.99,25.99,69.98,19.98,39.99,19.98,39.99,19.98,35.99,31.44,27.99,26.99,25.0,17.98,44.99,33.99,26.99,32.99,18.23,23.99,19.98,32.99,49.99,23.99,32.81,19.98,22.49,18.69,59.99,37.99,19.98,34.99,26.99,60.79,49.97,32.99,25.99]};\nvar data = [trace_0];\nvar layout = {\"title\":{\"text\":\"Sales\"}};\n        Plotly.newPlot('SalesPlot', data, layout, {\"responsive\": true});\n    };\n"
