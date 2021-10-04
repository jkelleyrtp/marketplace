use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationResponse {
    pub hash: String,
    pub left: usize,
    pub plan: String,
    pub errorMsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    status: String,
    data: HashMap<String, ProductResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    status: String,
    data: ProductListing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListing {
    requestId: String,
    asin: String,
    marketplace: String,
    category: CategoryData,
    productData: ProductData,
    salesHistory: SalesHistory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    id: usize,
    name: String,
    isCategoryBsr: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductData {
    sellerType: usize,
    sellersNumber: usize,
    title: String,
    imageUrl: String,
    bsr: usize,
    price: f32,
    fbaFee: usize,
    mfnFee: usize,
    dimensions: serde_json::Value,
    sizeTier: usize,
    numberOfImages: usize,
    numberOfVariations: usize,
    age: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesHistory {
    history: HashMap<String, f32>,
    minDate: String,
    last30DaysSales: isize,
    peak: bool,
    last12MonthsSales: isize,
    last12MonthsChange: isize,
    bestMonth: String,
    last90DaysSalesTrend: isize,
    last90DaysPriceTrend: isize,
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
