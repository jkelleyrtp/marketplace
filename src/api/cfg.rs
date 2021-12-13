use std::collections::HashMap;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScraperCfgFile(pub HashMap<String, ScraperCfg>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScraperCfg {
    pub headers: HashMap<String, String>,
    pub cookies: Vec<String>,
    pub cookie_domain: Url,
}

pub fn basic_amazon_search_cfg() -> ScraperCfg {
    let p = include_str!("../../db/scraper_cfg.json");
    let mut scrapers: ScraperCfgFile = serde_json::from_str(p).unwrap();
    scrapers.0.remove("amazon_search").unwrap()
}
