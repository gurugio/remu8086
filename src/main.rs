mod cpucontext;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/assembly.pest"] // grammar file
struct AssemblyParser;

fn main() {
    let pairs = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
        .expect("Failed to parse")
        .next()
        .unwrap();

    for pair in pairs.into_inner() {
        println!("Rule: {:?}", pair.as_rule());
        println!("Text: {}", pair.as_str());
    }
}
