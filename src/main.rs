mod prepare_data;

fn main() {
    if let Err(e) = prepare_data::print_data() {
        eprint!("Error {}", e);
    }
}
