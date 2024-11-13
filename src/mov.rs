use crate::parser::{imm_to_num, Rule};
use crate::{cpucontext::CpuContext, define_handler_two};
use paste::paste;
use pest::iterators::Pair;

/*
/// MOV r/m8, reg8  $88
/// MOV r/m16, reg16 $89
/// MOV AL, mem8 $A0
/// MOV AX, mem16 $A1
/// MOV mem8, AL $A2
/// MOV mem16, AX $A3
/// MOV reg8, imm8 $B0 + reg8 code
/// MOV reg16,imm16 $B8 + reg16 code
/// MOV r/m8, imm8 $C6, xx000xxx(ModR/M byte)
/// MOV r/m16, imm16 $C7, xx000xxx(ModR/M byte)
/// MOV r/m16,sreg $8C, xx0 sreg xxx(ModR/M byte)
/// MOV sreg, r/m16 $8E, xx0 sreg xxx(ModR/M byte)
*/

define_handler_two!(mov, first, second, cpu, {
    match (first.as_rule(), second.as_rule()) {
        (Rule::reg16, Rule::reg16) => {
            cpu.set_register(first.as_str(), cpu.get_register(second.as_str()).unwrap())
                .unwrap();
        }
        (Rule::reg16, Rule::imm) => {
            let v = imm_to_num(&second).unwrap();
            cpu.set_register(first.as_str(), v).unwrap();
        }
        _ => println!("Not supported yet:{:?} {:?}", first, second),
    }
});

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn test_mov_flags() {}
}
