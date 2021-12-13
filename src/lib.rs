#![allow(non_snake_case, non_upper_case_globals)]

pub mod actions;
pub mod app;
pub mod hooks;
pub mod components {
    pub mod dashboard;
    pub mod greeting;
    pub mod jupyter;
    pub mod kwordpage;
    pub mod login;
    pub mod navbar;
    pub mod plots;
    pub mod results;
    pub mod review;
    pub mod search;

    pub use dashboard::*;
    pub use greeting::*;
    pub use jupyter::*;
    pub use kwordpage::*;
    pub use login::*;
    pub use navbar::*;
    pub use plots::*;
    pub use results::*;
    pub use review::*;
    pub use search::*;
}
pub mod api {
    pub mod amazon;
    pub mod cfg;
    pub mod helium10;
}
pub mod icons;
pub mod logging;
pub mod scraper;
pub mod state;

pub use app::AppRoute;
