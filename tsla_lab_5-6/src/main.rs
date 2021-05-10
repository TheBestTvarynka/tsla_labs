use tsla_lab_5_6::reader::lexer::parse_file;
use tsla_lab_5_6::parser::parser::build_tree;

fn main() {
    let table = parse_file("ex2.txt").unwrap();
    table.print();
    let node = build_tree(table).unwrap();
    println!("{:?}", node);
}
