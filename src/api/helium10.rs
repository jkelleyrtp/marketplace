//! Code relevant to the Helium 10 API.
//!
//! The data for the API is obtained through the unofficial API (ie the one your
//! browser hits when you use X-Ray).
//!
//! Note: this API is not quite robust and might fail parsing.

use chrono::{DateTime, NaiveDateTime, Utc};
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

// "Product Details","ASIN","Brand","Price","Sales","Revenue","BSR","FBA Fees","Active Sellers #","Ratings","Review Count","Images","Review velocity","Buy Box","Category","Size Tier","Delivery","Dimensions","Weight","Creation Date"

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProductAnalysis {
    pub sales: isize,
    pub revenue: f64,
    pub review_velocity: f64,
    // pub delivery: String,
    pub creation_date: String,
    pub rating: f64,
    pub num_reviews: f64,
    // pub asin: String,
    // pub brand: String,
    // pub price: f64,
    // pub bsr: isize,
    // pub fba_fees: isize,
    // pub sellers_number: isize,
    // pub ratings: f64,
    // pub review_count: isize,
    // pub images: isize,
    // pub buy_box: bool,
    // pub category: String,
    // pub size_tier: isize,
    // pub dimensions: serde_json::Value,
    // pub weight: f64,
}
impl ProductAnalysis {
    pub fn new(product: &ProductListing) -> Self {
        let sales = product.salesHistory.last30DaysSales;

        let revenue = sales as f64 * product.productData.price;
        let review_velocity = calculate_review_velocity(product);

        let p = product.reviewHistory.values().last().unwrap();

        // Create a NaiveDateTime from the timestamp
        let naive = NaiveDateTime::from_timestamp(product.productData.age as i64, 0);

        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d");

        Self {
            sales,
            revenue,
            review_velocity,
            creation_date: format!("{}", newdate),
            rating: p.rating,
            num_reviews: p.count,
        }
    }
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
    let contents = include_str!("../../responses/invocation.json");
    let g = serde_json::from_str::<InvocationResponse>(contents).unwrap();
    dbg!(g);
}

#[test]
fn parse_product() {
    let contents = include_str!("../../responses/product.json");
    let g = serde_json::from_str::<ProductListResponse>(contents).unwrap();
    dbg!(g);
}
