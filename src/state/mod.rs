use crate::{
    api::{
        cfg::ScraperCfg,
        helium10::{ProductAnalysis, ProductListing},
    },
    AppRoute,
};
use atoms::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub static ROUTE: Atom<AppRoute> = |_| AppRoute::Home;

pub static CURRENT_USER: Atom<Option<Uuid>> =
    |_| Some(uuid::uuid!("c826b6d2-1e2a-473e-80ac-9d00133d15ad"));

pub static USERS: AtomFamily<Uuid, User> = |_| {
    let user_file = std::fs::read_to_string("db/users.json").expect("couldn't find users db");
    serde_json::from_str(&user_file).unwrap()
};

pub static KEYWORDS: AtomFamily<Uuid, KeywordEntry> = |_| {
    let kwf = std::fs::read_to_string("db/keywords.json").expect("couldn't find launch db");
    serde_json::from_str(&kwf).unwrap()
};

pub static PRODUCT_ANALYSIS: AtomFamily<String, ProductAnalysis> = |_| {
    let kwf = std::fs::read_to_string("db/keywords.json").expect("couldn't find launch db");
    let keywords: HashMap<Uuid, KeywordEntry> = serde_json::from_str(&kwf).unwrap();

    let mut analysis = im_rc::HashMap::new();
    for (id, entry) in keywords {
        for (asin, product) in entry.products {
            let analsysis = ProductAnalysis::new(&product);
            analysis.insert(asin, analsysis);
        }
    }
    analysis
};

pub static SCRAPER_CFG: AtomFamily<String, ScraperCfg> = |_| {
    let cfg_file =
        std::fs::read_to_string("db/scraper_cfg.json").expect("couldn't find scraper cfg");
    serde_json::from_str(&cfg_file).unwrap()
};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub short_name: String,
    pub email: String,
    pub password: String,
    pub credits: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeywordEntry {
    pub keyword: String,
    pub creator: Uuid,
    pub products: HashMap<String, ProductListing>,
}

pub fn save() {}

pub fn use_save_keywords(cx: Scope<()>) -> impl Fn() {
    let root = use_atom_root(&cx);

    || {
        std::fs::write(
            "db/keywords.json",
            serde_json::to_string_pretty("").unwrap(),
            // serde_json::to_string_pretty(&self.keywords).unwrap(),
        )
        .expect("couldn't write launch db");
    }
}

pub(crate) fn use_current_user(cx: &ScopeState) -> Option<&User> {
    let id = use_read(&cx, CURRENT_USER).as_ref()?;
    let users = use_read(&cx, USERS);
    users.get(id)
}

pub fn use_keyword_entry(cx: &ScopeState, id: Uuid) -> Option<&KeywordEntry> {
    let keywords = use_read(&cx, crate::state::KEYWORDS);
    keywords.get(&id)
}
