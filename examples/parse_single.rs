use amazon_isr::helium10::*;

fn main() {
    let contents = include_str!("../data/amazon_response.json");

    let v: ProductListResponse = serde_json::from_str(contents).unwrap();
}
