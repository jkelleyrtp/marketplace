//! Code related to Amazon's Search Page.
//!
//!
//! Note: this is only the scraped version of the listing page.

use crate::{actions::FetchError, api::cfg::basic_amazon_search_cfg};
use scraper::{Html, Selector};

use super::cfg::ScraperCfg;

#[derive(Debug, Clone)]
pub struct ScrapedAmazonListing {
    pub name: String,
    pub asin: String,
    pub price: f32,
    pub discount_price: Option<f32>,
    pub rating: f32,
    pub num_reviews: u32,
    pub img_url: String,
}

/// Scrape the amazon search page to find all the asins related to a search term
pub async fn fetch_asins_from_keyword(
    client: &reqwest::Client,
    cfg: &ScraperCfg,
    load_cookies: bool,
    kword: &str,
) -> Result<Vec<ScrapedAmazonListing>, FetchError> {
    let term = kword.replace(" ", "+");

    if load_cookies {
        let jar = reqwest::cookie::Jar::default();
        for cookie in &cfg.cookies {
            jar.add_cookie_str(cookie, &cfg.cookie_domain);
        }
    }

    let endpoint = format!("https://www.amazon.com/s?k={}&ref=nb_sb_noss_1", term);
    let mut get_req = client.get(endpoint);

    for (name, value) in &cfg.headers {
        get_req = get_req.header(name, value);
    }

    let res = get_req.send().await?.text().await?;

    let asins = parse_document(&res);

    Ok(asins)
}

pub fn parse_document(doc: &str) -> Vec<ScrapedAmazonListing> {
    let parsed_html = Html::parse_document(&doc);
    let selector = Selector::parse("div[data-asin]").unwrap();

    let listings = parsed_html
        .select(&selector)
        .filter_map(|element| {
            let asin = element.value().attr("data-asin").unwrap().to_string();
            if asin.is_empty() {
                None
            } else {
                Some(ScrapedAmazonListing {
                    asin,
                    name: "baller".to_string(),
                    price: 10.0,
                    discount_price: Some(9.8),
                    rating: 5.0,
                    num_reviews: 120,
                    img_url: "https://m.media-amazon.com/images/I/41L+FfaM1cL._SX450_.jpg"
                        .to_string(),
                })
            }
        })
        .collect::<Vec<_>>();

    listings
}

#[tokio::test]
async fn fetch_data() {
    //
    let client = reqwest::Client::builder().build().unwrap();
    let cfg = basic_amazon_search_cfg();

    let results = fetch_asins_from_keyword(&client, &cfg, true, "toaster oven")
        .await
        .unwrap();

    dbg!(results);
}

#[test]
fn parse_data() {
    let contents = std::fs::read_to_string("data/scrape.html").unwrap();
    dbg!(contents);
}
