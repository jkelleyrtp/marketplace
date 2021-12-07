use amazon_isr::helium10::{InvocationResponse, ProductListResponse};

fn main() {
    let contents = include_str!("./../data/amazon_response_3.json");
    let response: ProductListResponse = serde_json::from_str(contents).unwrap();
}
