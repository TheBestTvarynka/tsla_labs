use tsla_lab4::reader::lexer::parse_file;

// main function for testing out lexer
fn main() {
    let table = parse_file("e2.rs").unwrap();
    table.print();
}
