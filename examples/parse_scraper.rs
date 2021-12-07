use scraper::{Html, Selector};

#[derive(Debug)]
struct ParsedAmazonListing {
    asin: String,
    rating: String,
    name: String,
    url: String,
    description: String,
    image: String,
    price: String,
    review_count: String,
}

fn main() {
    let data = include_str!("../data/scrape.html");

    let parsed_html = Html::parse_document(&data);

    let selector = Selector::parse(r#"div[data-component-type="s-search-result"]"#).unwrap();
    let select_title = Selector::parse("h2 a.a-link-normal.a-text-normal").unwrap();
    let select_price = Selector::parse("span.a-price:nth-of-type(1) span.a-offscreen").unwrap();

    let asins = parsed_html
        .select(&selector)
        .filter_map(|element| {
            let asin = element.value().attr("data-asin").unwrap();
            if asin.is_empty() {
                None
            } else {
                //
                if let Some(t) = element.select(&select_price).next() {
                    let g = t.value();
                    dbg!(&g.name);
                }
                // if let Some(title) =  {
                //     //
                // }
                // let image = .unwrap();

                Some(ParsedAmazonListing {
                    asin: asin.to_string(),
                    rating: "".to_string(),
                    name: "".to_string(),
                    url: "".to_string(),
                    description: "".to_string(),
                    image: "".to_string(),
                    price: "".to_string(),
                    review_count: "".to_string(),
                })
            }
        })
        .collect::<Vec<_>>();

    dbg!(&asins);
}
