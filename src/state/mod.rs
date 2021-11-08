use std::collections::HashMap;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{helium10::FlattenedEntry, Routes};

pub struct AppState {
    pub route: Routes,
    pub current_user: Option<Uuid>,
    pub cached_data: CachedData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CachedData {
    pub users: HashMap<Uuid, User>,
    pub keywords: HashMap<Uuid, KeywordEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeywordEntry {
    pub keyword: String,
    pub creator: Uuid,
    pub products: HashMap<String, FlattenedEntry>,
}

impl AppState {
    pub fn new() -> Self {
        let file = std::fs::read_to_string("db/db.json").expect("couldn't find launch db");
        let cached_data: CachedData = serde_json::from_str(&file).unwrap();

        let current_user = if cfg!(debug_assertions) {
            Some(Uuid::parse_str("c826b6d2-1e2a-473e-80ac-9d00133d15ad").unwrap())
        } else {
            None
        };

        let route = if cfg!(debug_assertions) {
            Routes::Home
        } else {
            Routes::Login
        };

        Self {
            route,
            cached_data,
            current_user,
        }
    }
    pub fn save(&mut self) {
        std::fs::write(
            "db/db.json",
            serde_json::to_string_pretty(&self.cached_data).unwrap(),
        )
        .expect("couldn't write launch db");
    }
}

pub fn use_provide_app_state(cx: Context) -> UseSharedState<AppState> {
    use_provide_state(cx, AppState::new);
    use_shared_state(cx).unwrap()
}

pub fn use_app_state(cx: Context) -> Option<UseSharedState<AppState>> {
    use_shared_state::<AppState>(cx)
}
