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

pub static Route: Atom<AppRoute> = |_| AppRoute::Home;

pub static CurrentUser: Atom<Option<Uuid>> =
    |_| Some(uuid::uuid!("c826b6d2-1e2a-473e-80ac-9d00133d15ad"));

pub static Users: AtomFamily<Uuid, User> = |_| {
    let user_file = std::fs::read_to_string("db/users.json").expect("couldn't find users db");
    serde_json::from_str(&user_file).unwrap()
};

pub static Keywords: AtomFamily<Uuid, KeywordEntry> = |_| {
    let kwf = std::fs::read_to_string("db/keywords.json").expect("couldn't find launch db");
    serde_json::from_str(&kwf).unwrap()
};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeywordEntry {
    pub keyword: String,
    pub creator: Uuid,
    pub products: HashMap<String, ProductListing>,
}

pub fn save() {}

pub fn use_save_keywords(cx: Context) -> impl Fn() {
    let root = use_atom_root(cx);

    || {
        std::fs::write(
            "db/keywords.json",
            serde_json::to_string_pretty("").unwrap(),
            // serde_json::to_string_pretty(&self.keywords).unwrap(),
        )
        .expect("couldn't write launch db");
    }
}

pub(crate) fn use_current_user(cx: Context) -> Option<&User> {
    todo!()
}

pub fn use_keyword_entry(cx: Context, id: Uuid) -> Option<&KeywordEntry> {
    let keywords = use_read(cx, crate::state::Keywords);
    todo!()
}
