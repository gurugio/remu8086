use crate::parser::{imm_to_num, Rule};
use crate::{cpucontext::CpuContext, define_handler_two};
use paste::paste;
use pest::iterators::Pair;

/*
/// ADD AL, imm8 $04
/// ADD AX, imm16 $05
/// ADD reg8, r/m8 $02
/// ADD reg16, r/m16 $03
/// ADD r/m8, reg8 $00
/// ADD r/m16, reg16 $01
/// ADD r/m8, imm8 $80 xx000xxx (ModR/M byte)
/// ADD r/m16, imm16 $81 xx000xxx (ModR/M byte)
/// ADD r/m16, imm8 $83 xx000xxx (ModR/M byte)
*/

define_handler_two!(add, first, second, cpu, {
    match (first.as_rule(), second.as_rule()) {
        (Rule::reg16, Rule::reg16) => {
            cpu.set_register(first.as_str(), cpu.get_register(second.as_str()).unwrap())
                .unwrap();
            let l = cpu.get_register(first.as_str()).unwrap();
            let r = cpu.get_register(second.as_str()).unwrap();
            cpu.set_register(first.as_str(), l + r).unwrap();
        }
        (Rule::reg16, Rule::imm) => {
            let v = imm_to_num(&second).unwrap();
            let l = cpu.get_register(first.as_str()).unwrap();
            cpu.set_register(first.as_str(), l + v).unwrap();
        }
        _ => println!("Not supported yet:{:?} {:?}", first, second),
    }
});

// TODO: add unittest for each case
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_add_reg16_imm16() {}
}
