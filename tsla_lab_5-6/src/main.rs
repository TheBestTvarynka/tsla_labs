use tsla_lab_5_6::reader::lexer::parse_file;
use tsla_lab_5_6::parser::parser::build_tree;
// use tsla_lab_5_6::parser::tree::Node;
use tsla_lab_5_6::linter::linter::lint;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify ONE name of the file with the code! Example: `cargo run ex0.txt`");
        return;
    }
    let table = parse_file(args[1].as_str()).unwrap();
    table.print();
    let node = build_tree(table).unwrap();
    match lint(Box::new(node)) {
        Result::Ok((_, _)) => println!("compilation success!"),
        Result::Err(err_msg) => {
            eprintln!("Compilation error:");
            eprintln!("{}", err_msg);
        },
    };
    // Node::print(node);
    // println!("{:?}", node);
}
