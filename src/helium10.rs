use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationResponse {
    pub hash: String,
    pub left: isize,
    pub plan: String,
    pub errorMsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub status: String,
    pub left: isize,
    pub total: String,
    pub data: HashMap<String, ProductResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]

pub enum ProductResponse {
    #[serde(rename = "success")]
    Success(ProductListing),

    #[serde(rename = "error")]
    Error(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductListing {
    pub requestId: String,
    pub asin: String,
    pub marketplace: String,
    pub category: CategoryData,
    pub productData: ProductData,
    pub bsrHistory: BsrHistory,
    pub salesHistory: SalesHistory,
    pub reviewHistory: HashMap<String, ReviewPoint>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BsrHistory {
    pub history: Option<HashMap<String, usize>>,
    pub minDate: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ReviewPoint {
    pub count: f64,
    pub rating: f64,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CategoryData {
    pub id: isize,
    pub name: String,
    pub isCategoryBsr: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProductData {
    pub sellerType: isize,
    pub sellersNumber: isize,
    pub title: String,
    pub imageUrl: String,
    pub bsr: isize,
    pub price: f64,
    pub fbaFee: isize,
    pub mfnFee: isize,
    pub dimensions: serde_json::Value,
    pub sizeTier: isize,
    pub numberOfImages: isize,
    pub numberOfVariations: isize,
    pub age: isize,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SalesHistory {
    pub history: Option<HashMap<String, f64>>,
    pub minDate: Option<String>,
    pub last30DaysSales: isize,
    pub peak: bool,
    pub last12MonthsSales: isize,
    pub last12MonthsChange: isize,
    pub bestMonth: String,
    pub last90DaysSalesTrend: isize,
    pub last90DaysPriceTrend: isize,
}

pub fn calculate_review_velocity(data: &ProductListing) -> f64 {
    let mut sum = 0.0;
    for (_, point) in &data.reviewHistory {
        sum += point.count;
    }
    sum / data.reviewHistory.len() as f64
}

#[test]
fn parse_invocation() {
    let contents = include_str!("../responses/invocation.json");
    let g = serde_json::from_str::<InvocationResponse>(contents).unwrap();
    dbg!(g);
}

#[test]
fn parse_product() {
    let contents = include_str!("../responses/product.json");
    let g = serde_json::from_str::<ProductListResponse>(contents).unwrap();
    dbg!(g);
}
