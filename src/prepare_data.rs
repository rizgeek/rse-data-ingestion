use std::{fs::File, io::BufReader};
use serde::Deserialize;
use serde_json::{Error, Value};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProductData {
    asin: String,
    product_id: String,
    name: String,
    img_url: String,
    product_url: String,
    stars: f32,
    reviews: i64,
    price: Option<f64>,
    list_price: Option<f64>,
    is_best_seller: Option<i8>,
    bought_in_last_month: Option<i64>,
    description: String,
    stock: i64,
    brand: String,
    category_ml: String,
    creation_date: Option<String>,
    last_updated: String,
    search_tags: Vec<String>,
}


fn buffer_reader_from_file(path: String)-> Result<BufReader<File>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn deserialze_json_from_reader(reader: BufReader<File>) -> Result<Value, serde_json::Error> {
    let value: Value = serde_json::from_reader(reader)?;
    Ok(value)
}

fn parse_product(item: Value) -> Result<ProductData, Error> {
    serde_json::from_value::<ProductData>(item)
}

fn display_products(products: &[ProductData]) {
    for product in products {
        println!("{:?}", product)
    }
}

pub fn print_data() -> Result<(), Box<dyn std::error::Error>> {
    let reader = buffer_reader_from_file("sample_augmented_product.json".to_string())?;
    let value = deserialze_json_from_reader(reader)?;

    if let Value::Array(items) = value {
        let products: Result<Vec<ProductData>, Error> = items.into_iter()
            .map(parse_product)
            .collect();

        display_products(&products?);
    }

    Ok(())
}