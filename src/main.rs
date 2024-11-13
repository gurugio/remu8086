mod add;
mod common;
mod cpucontext;
mod mov;
mod org;
mod parser;

use paste::paste;
use pest::Parser;
use std::fs;

fn main() {
    let mut cpu: cpucontext::CpuContext = cpucontext::CpuContext::boot();

    let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
    let file = parser::AssemblyParser::parse(parser::Rule::program, &unparsed_file)
        .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        println!("Execute:{}", line.as_str());
        match line.as_rule() {
            parser::Rule::mov => {
                caller_two!(mov, cpu, line);
            }
            parser::Rule::org => {
                caller_one!(org, cpu, line);
            }
            parser::Rule::add => {
                caller_two!(add, cpu, line);
            }
            _ => println!("NOT implemented yet:{}", line),
        }
        println!("{:?}", cpu);
    }
}
