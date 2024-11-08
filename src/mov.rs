use crate::define_handler_two;
use crate::parser::Rule;
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

define_handler_two!(handle_mov, first, second, {
    println!("first: rule={:?} text={}", first, first.as_str());
    println!("second: rule={:?} text={}", second, second.as_str());

    match (first.as_rule(), second.as_rule()) {
        (Rule::reg16, Rule::reg16) => {
            //println!("reg16: {:?} reg16:{:?}", first.as_rule(), second.as_rule());
            mov_reg16_reg16(first.as_str(), second.as_str());
        }
        _ => println!("Not supported yet:{:?} {:?}", first, second),
    }
});

fn mov_reg16_reg16(first: &str, second: &str) {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_imm() {}
}
