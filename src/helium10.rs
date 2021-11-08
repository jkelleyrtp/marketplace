use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationResponse {
    pub hash: String,
    pub left: usize,
    pub plan: String,
    pub errorMsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub status: String,
    pub left: usize,
    pub total: String,
    pub data: HashMap<String, ProductResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub status: String,
    pub data: ProductListing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListing {
    pub requestId: String,
    pub asin: String,
    pub marketplace: String,
    pub category: CategoryData,
    pub productData: ProductData,
    pub salesHistory: SalesHistory,
}

impl ProductListing {
    pub fn flatten(self) -> FlattenedEntry {
        let ProductListing {
            requestId,
            asin,
            marketplace,
            category,
            productData,
            salesHistory,
        } = self;
        FlattenedEntry {
            Product_Details: todo!(),
            ASIN: asin,
            Brand: todo!(),
            Price: todo!(),
            Sales: todo!(),
            Revenue: todo!(),
            BSR: todo!(),
            FBA_Fees: todo!(),
            Active_Sellers: todo!(),
            Ratings: todo!(),
            Review_Count: todo!(),
            Images: todo!(),
            Review_velocity: todo!(),
            Buy_Box: todo!(),
            Category: todo!(),
            Size_Tier: todo!(),
            Delivery: todo!(),
            Dimensions: todo!(),
            Weight: todo!(),
            Creation_Date: todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    pub id: usize,
    pub name: String,
    pub isCategoryBsr: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductData {
    pub sellerType: usize,
    pub sellersNumber: usize,
    pub title: String,
    pub imageUrl: String,
    pub bsr: usize,
    pub price: f32,
    pub fbaFee: usize,
    pub mfnFee: usize,
    pub dimensions: serde_json::Value,
    pub sizeTier: usize,
    pub numberOfImages: usize,
    pub numberOfVariations: usize,
    pub age: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesHistory {
    pub history: HashMap<String, f32>,
    pub minDate: String,
    pub last30DaysSales: isize,
    pub peak: bool,
    pub last12MonthsSales: isize,
    pub last12MonthsChange: isize,
    pub bestMonth: String,
    pub last90DaysSalesTrend: isize,
    pub last90DaysPriceTrend: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlattenedEntry {
    #[serde(rename = "Product Details")]
    pub Product_Details: String,
    pub ASIN: String,
    pub Brand: String,
    pub Price: String,
    pub Sales: String,
    pub Revenue: String,
    pub BSR: String,

    #[serde(rename = "FBA Fees")]
    pub FBA_Fees: String,

    #[serde(rename = "Active Sellers #")]
    pub Active_Sellers: String,

    pub Ratings: String,

    #[serde(rename = "Review Count")]
    pub Review_Count: String,

    pub Images: String,

    #[serde(rename = "Review velocity")]
    pub Review_velocity: String,

    #[serde(rename = "Buy Box")]
    pub Buy_Box: String,

    pub Category: String,

    #[serde(rename = "Size Tier")]
    pub Size_Tier: String,

    pub Delivery: String,

    pub Dimensions: String,

    pub Weight: String,

    #[serde(rename = "Creation Date")]
    pub Creation_Date: String,
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
