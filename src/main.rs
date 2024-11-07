mod common;
mod mov;
mod parser;

use parser::Rule;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;

call_handler_two!(mov, handle_mov);

fn main() {
    let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
    let file = parser::AssemblyParser::parse(parser::Rule::program, &unparsed_file)
        .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        match line.as_rule() {
            parser::Rule::mov => {
                println!("mov:{:?}", line);
                let mut inner_rule = line.into_inner();
                handle_mov(&mut inner_rule);
            }
            _ => println!("else:{}", line),
        }
    }
}
