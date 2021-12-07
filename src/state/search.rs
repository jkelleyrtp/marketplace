use dioxus::hooks::AppModels;
use uuid::Uuid;

use crate::{
    actions::{fetch_asins_from_keyword, fetch_helium_10_from_asins},
    state::{GlobalModel, KeywordEntry},
};
