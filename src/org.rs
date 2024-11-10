use crate::parser::{imm_to_num, Rule};
use crate::{cpucontext::CpuContext, define_handler_one};
use pest::iterators::Pair;

/*
org 100h
*/

define_handler_one!(handle_org, first, cpu, {
    println!("current cpu={:?}", cpu);
    match first.as_rule() {
        Rule::imm => {
            println!("handle org {}", first.as_str());
            let ip: u16 = imm_to_num(first).unwrap();
            cpu.set_cs(0);
            cpu.set_ip(ip);
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});
