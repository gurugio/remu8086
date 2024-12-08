use crate::memory::Memory;
use crate::parser::Rule;
use crate::{cpucontext::CpuContext, define_handler_one};
use paste::paste;
use pest::iterators::Pair;

define_handler_one!(jmp, first, _cpu, _memory, {
    println!("jmp {:?}", first);
});
