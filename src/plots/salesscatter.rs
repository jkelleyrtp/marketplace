use dioxus::prelude::*;
use plotly::{common::Title, Layout};
use rand::{distributions::Alphanumeric, Rng};

use crate::state::use_app_state;

#[derive(PartialEq, Props)]
pub struct SalesScatterProps {
    id: uuid::Uuid,
    title: &'static str,
    xrow: &'static str,
    yrow: &'static str,
}

pub fn SalesScatter(cx: Context, props: &SalesScatterProps) -> Element {
    log::debug!("Rendeirng sales scatter {:?}", cx.scope_id());

    let state = use_app_state(cx)?;
    let read = state.read();

    let data = read.cached_data.keywords.get(&props.id).unwrap();

    let prices = data
        .products
        .iter()
        .map(|(asin, p)| {
            //
            let (_, without_dollar_sign) = p.Price.split_at(1);
            without_dollar_sign.parse::<f32>().unwrap()
        })
        .collect::<Vec<_>>();

    let x = (0..prices.len()).collect::<Vec<_>>();

    use plotly::{common::Mode, Plot, Scatter};
    let trace = Scatter::new(x, prices).mode(Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::new(props.title)));

    let name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let raw = plot.to_inline_html(Some(name.as_str()));

    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{name}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}
