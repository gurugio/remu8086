mod common;
mod cpucontext;
mod mov;
mod org;
mod parser;

use cpucontext::CpuContext;
use parser::Rule;
use paste::paste;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;

define_caller_two!(mov, cpu);
define_caller_one!(org, cpu);

fn main() {
    let mut cpu: cpucontext::CpuContext = cpucontext::CpuContext::boot();

    let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
    let file = parser::AssemblyParser::parse(parser::Rule::program, &unparsed_file)
        .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        match line.as_rule() {
            parser::Rule::mov => {
                let mut inner_rule = line.into_inner();
                caller_mov(&mut cpu, &mut inner_rule);
            }
            parser::Rule::org => {
                let mut inner_rule = line.into_inner();
                caller_org(&mut cpu, &mut inner_rule);
            }
            _ => println!("else:{}", line),
        }
    }
}
