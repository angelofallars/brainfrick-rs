mod interpreter;
mod utils;

mod test;

fn main() {
    let mut byte_cells = interpreter::create_byte_cells();

    let snippet = String::from("+++>+++++<[->+<]");

    interpreter::process_commands(&mut byte_cells, snippet);
}
