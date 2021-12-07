use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};
use uuid::Uuid;

use crate::{helium10::ProductListing, AppRoute};
use atoms::prelude::*;

pub static Route: Atom<AppRoute> = || AppRoute::Home;
pub static CurrentUser: Atom<Option<Uuid>> = || None;
pub static Users: AtomFamily<Uuid, User> = |_| Default::default();
pub static Keywords: AtomFamily<Uuid, KeywordEntry> = |_| Default::default();
pub static Products: AtomFamily<String, ProductListing> = |_| Default::default();

pub struct GlobalModel {
    pub route: AppRoute,
    pub current_user: Option<Uuid>,
    pub users: HashMap<Uuid, User>,
    pub keywords: HashMap<Uuid, KeywordEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeywordEntry {
    pub keyword: String,
    pub creator: Uuid,
    pub products: HashMap<String, ProductListing>,
}

impl GlobalModel {
    pub fn new() -> Self {
        let kwf = std::fs::read_to_string("db/keywords.json").expect("couldn't find launch db");
        let keywords = serde_json::from_str(&kwf).unwrap();

        let user_file = std::fs::read_to_string("db/users.json").expect("couldn't find users db");
        let users = serde_json::from_str(&user_file).unwrap();

        let current_user = Some(Uuid::parse_str("c826b6d2-1e2a-473e-80ac-9d00133d15ad").unwrap());

        Self {
            route: AppRoute::Home,
            keywords,
            current_user,
            users,
        }
    }

    pub fn save(&self) {
        std::fs::write(
            "db/keywords.json",
            serde_json::to_string_pretty(&self.keywords).unwrap(),
        )
        .expect("couldn't write launch db");
    }
}

pub fn use_provide_app_state(cx: Context) -> UseSharedState<GlobalModel> {
    use_provide_state(cx, GlobalModel::new);
    use_shared_state(cx).unwrap()
}

pub fn use_app_state(cx: Context) -> Option<UseSharedState<GlobalModel>> {
    use_shared_state::<GlobalModel>(cx)
}

pub fn use_current_user(cx: Context) -> Option<Uuid> {
    let c = use_app_state(cx)?;
    let p = c.read();
    p.current_user.clone()
}
