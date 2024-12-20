use crate::memory::Memory;
use crate::parser::{imm_to_num, Rule};
use crate::{cpucontext::CpuContext, define_handler_one};
use paste::paste;
use pest::iterators::Pair;

/*
org 100h
*/

define_handler_one!(org, first, cpu, _memory, {
    match first.as_rule() {
        Rule::imm => {
            let ip: u16 = imm_to_num(&first).unwrap();
            cpu.set_register16("cs", 0);
            cpu.set_register16("ip", ip);
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});
