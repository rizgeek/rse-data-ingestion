use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ProductData {
    pub asin: String,
    pub product_id: String,
    pub name: String,
    pub img_url: String,
    pub product_url: String,
    pub stars: f32,
    pub reviews: i64,
    pub price: Option<f64>,
    pub list_price: Option<f64>,
    pub is_best_seller: Option<i8>,
    pub bought_in_last_month: Option<i64>,
    pub description: String,
    pub stock: i64,
    pub brand: String,
    pub category_ml: String,
    pub creation_date: Option<String>,
    pub last_updated: String,
    pub search_tags: Vec<String>,
}