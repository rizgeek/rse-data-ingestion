use data_ingestion::prepare_data;

fn main() {
    if let Err(e) = prepare_data::print_data("fulldata_augmented_product_nd.ndjson") {
        eprint!("Error {}", e);
    }
}
