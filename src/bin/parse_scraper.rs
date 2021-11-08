use scraper::{Html, Selector};
fn main() {
    let data = include_str!("../../data/scrape.html");

    let parsed_html = Html::parse_document(&data);

    let selector = Selector::parse("div[data-asin]").unwrap();

    let asins = parsed_html
        .select(&selector)
        .filter_map(|element| {
            let asin = element.value().attr("data-asin").unwrap();
            if asin.is_empty() {
                None
            } else {
                Some(asin)
            }
        })
        .collect::<Vec<_>>();

    dbg!(&asins);
}
