use std::{fs::File, io::BufReader};
use serde_json::{Error, Value};
use crate::odt::ProductData;


fn buffer_reader_from_file(path: String, capacity: usize)-> Result<BufReader<File>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(capacity, file);

    Ok(reader)
}

fn deserialize_json_from_reader(reader: BufReader<File>) -> Result<Value, serde_json::Error> {
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
    let reader = buffer_reader_from_file("sample_augmented_product.json".to_string(), 8 * 1024)?;
    let value = deserialze_json_from_reader(reader)?;

    if let Value::Array(items) = value {
        let products: Result<Vec<ProductData>, Error> = items.into_iter()
            .map(parse_product)
            .collect();

        display_products(&products?);
    }

    Ok(())
}