use tsla_lab4::reader::lexer::parse_file;

fn main() {
    let table = parse_file("e1.rs").unwrap();
    table.print();
}
