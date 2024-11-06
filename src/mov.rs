use crate::Rule;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

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

pub fn handle_mov(first: Pair<Rule>, second: Pair<Rule>) {
    println!("first: rule={:?} text={}", first, first.as_str());

    println!("second: rule={:?} text={}", second, second.as_str());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_imm() {}
}
