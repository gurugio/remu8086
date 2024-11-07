mod common;
mod mov;
mod parser;

use parser::Rule;
use pest::iterators::Pairs;
use pest::Parser;
use std::fs;

call_handler_two!(mov, handle_mov);

#[derive(Default)]
struct CpuContext {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    sp: u16,
    bp: u16,
    si: u16,
    di: u16,
    cs: u16,
    ds: u16,
    es: u16,
    ss: u16,
    ip: u16,
}

impl CpuContext {
    fn new() -> Self {
        Default::default()
    }
    // TODO: set_* functions to set each register16 and register8
    // set_ax, set_al, set_ah, get_ax, get_al, get_ah
    // set_bx, ...
    // For example, "push ax" calls
    // 1. ss = get_ss, sp = get_sp, ax = get_ax
    // 2. set_sp(get_sp() - 2)
    // Initial stack address: ss:sp = 0x0:0x0
    // First input: ss:sp = 0xf:0xfffe
    // Second input: ss:sp = 0xf:0xfffc
    // ....0xf:0x0000
    // ....0xe:0xfffe
    // ....when sp gets underflow, ss is decreased.
    // 3. [ss:sp] = ax
}

fn main() {
    let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
    let file = parser::AssemblyParser::parse(parser::Rule::program, &unparsed_file)
        .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        match line.as_rule() {
            parser::Rule::mov => {
                //println!("mov:{:?}", line);
                let mut inner_rule = line.into_inner();
                handle_mov(&mut inner_rule);
            }
            _ => println!("else:{}", line),
        }
    }
}
