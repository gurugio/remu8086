use crate::parser::Rule;
use crate::{cpucontext::CpuContext, define_handler_one};
use pest::iterators::Pair;

/*
org 100h
*/

fn parse_imm(s: &str) -> u16 {
    // if last character is 'h', remove it
    0
}

define_handler_one!(handle_org, first, cpu, {
    println!("current cpu={:?}", cpu);
    match first.as_rule() {
        Rule::imm => {
            println!("handle org {}", first.as_str());
            let ip: u16 = parse_imm(first.as_str());
            cpu.set_cs(0);
            cpu.set_ip(ip);
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_imm() {
        assert_eq!(0x1234, parse_imm("1234h"));
        assert_eq!(0xabcd, parse_imm("0abcdh"));
        assert_eq!(0x1234, parse_imm("0x1234"));
    }
}
