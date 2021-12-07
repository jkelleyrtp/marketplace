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

impl ProductListing {
    pub fn flatten(self) -> FlattenedEntry {
        let ProductListing {
            requestId,
            asin,
            marketplace,
            category:
                CategoryData {
                    id,
                    name: category_name,
                    isCategoryBsr,
                },
            productData:
                ProductData {
                    sellerType,
                    sellersNumber,
                    title,
                    imageUrl,
                    bsr,
                    price,
                    fbaFee,
                    mfnFee,
                    dimensions,
                    sizeTier,
                    numberOfImages,
                    numberOfVariations,
                    age,
                },
            // salesHistory:
            //     SalesHistory {
            //         bestMonth,
            //         // history,
            //         last12MonthsChange,
            //         last12MonthsSales,
            //         last30DaysSales,
            //         last90DaysPriceTrend,
            //         last90DaysSalesTrend,
            //         minDate,
            //         peak,
            //     },
        } = self;

        FlattenedEntry {
            ASIN: asin,
            Price: price.to_string(),
            Product_Details: String::from("todo"),
            FBA_Fees: fbaFee.to_string(),
            Active_Sellers: sellersNumber.to_string(),
            BSR: bsr.to_string(),
            Images: numberOfImages.to_string(),
            Category: category_name,

            Brand: String::from("todo"),
            Sales: String::from("todo"),
            Revenue: String::from("todo"),
            Ratings: String::from("todo"),
            Review_Count: String::from("todo"),
            Review_velocity: String::from("todo"),
            Buy_Box: String::from("todo"),
            Size_Tier: String::from("todo"),
            Delivery: String::from("todo"),
            Dimensions: String::from("todo"),
            Weight: String::from("todo"),
            Creation_Date: String::from("todo"),
        }
    }
}
