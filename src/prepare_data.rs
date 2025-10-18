use std::{fs::File, io::{BufReader}};
use serde_json::Deserializer;
use crate::odt::ProductData;


const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn make_reader(path: &str, capacity: usize)-> Result<BufReader<File>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(capacity, file);

    Ok(reader)
}

fn read_ndjson (reader: BufReader<File>) -> impl Iterator<Item = Result<ProductData, serde_json::Error>> {
    let stream = Deserializer::from_reader(reader).into_iter::<ProductData>();
    stream
}

fn display_products(product: &ProductData) {
    println!("{:?}", product)
}

pub fn print_data(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let reader = make_reader(path, DEFAULT_BUF_SIZE)?;
    let stream = read_ndjson(reader);
    

    for (i, value) in stream.enumerate() {
        match value {
            Ok(product) => display_products(&product),
            Err(e) => eprintln!("Error di baris {}: {:?}", i + 1, e),
        }
    }

    Ok(())
}