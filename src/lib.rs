use uuid::Uuid;

pub mod actions;
pub mod app;
pub mod components;
pub mod db;
pub mod helium10;
pub mod icons;
pub mod plots;
pub mod state;

#[derive(Clone, Debug, PartialEq)]
pub enum Routes {
    Home,
    Login,
    AddNew,
    Search,
    Review,
    Jupyter,
    ProductPage { search_id: Uuid },
    NotFound,
}

impl Routes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Routes::Home => "Home",
            Routes::AddNew => "Import CSV",
            Routes::Search => "Keyword Search",
            Routes::Review => "Review Products",
            Routes::Jupyter => "Open in Jupyter",
            Routes::Login => "Login",
            Routes::ProductPage { .. } => "Product Page",
            Routes::NotFound => "Err 404. Not Found.",
        }
    }
}
