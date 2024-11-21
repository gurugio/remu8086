use crate::memory::Memory;
use crate::parser::Rule;
use crate::{cpucontext::CpuContext, define_handler_one};
use paste::paste;
use pest::iterators::Pair;

/*
inc
1. 1-byte form: inc 16-bit registers
Opcode bit 7-3: 01000
bit 2-0: register table

2. 2-byte form: inc 8-bit registers (If operand is 8-bit reg, mod is always 11 and bit 5-3 is 000)
1-byte Opcode bit 7-1: opcode 1111111
1-byte W-bit 0: 0-operand is 8-bit
2-byte mod-bit 7-6: 11
2-byte Opcode bit 5-3: 000
2-byte bit 2-0: register table

3. 3-byte form: inc memory location
1-byte Opcode bit 7-1: 1111111
1-byte W bit 0: 0-8bit, 1-16bit
2-byte mod-bit 7-6: 01-8bit contents of next byte of instruction sign extended to 16 bits
2-byte Opcode 5-3: 000
2-byte r/m 2-0: (base and index register)
3-byte Displacement 7-0

3. 4-byte form: inc memory location
1-byte Opcode bit 7-1: 1111111
1-byte W bit 0: 0-8bit, 1-16bit
2-byte mod-bit 7-6
  * 10-indirect addressing mode, use base register and 16bit displacement of next two byte of instruction
       (next byte->least significant eight bits, byte after that->most sig bits)
  * 00-direct addressing mode, no use base register, only use 16bit displacement
2-byte Opcode 5-3: 000
2-byte r/m 2-0: (base and index register)
3-byte Displacement 7-0
4-byte Displacement 15-8

register table
000 AX AL
001 CX CL
010 DX DL
011 BX BL
100 SP AH
101 BP CH
110 SI DH
111 DI BH

r/m field | Base register | Index Register
000       | BX            | SI
001       | BX            | DI
010       | BP            | SI
011       | BP            | DI
100       | none          | SI
101       | none          | DI
110       | BP            | none
111       | BX            | none

*/

fn register_table(reg: &str) -> Result<u8, String> {
    match reg {
        "ax" | "al" => Ok(0),
        "cx" | "cl" => Ok(1),
        "dx" | "dl" => Ok(2),
        "bx" | "bl" => Ok(3),
        "sp" | "ah" => Ok(4),
        "bp" | "ch" => Ok(5),
        "si" | "dh" => Ok(6),
        "di" | "bh" => Ok(7),
        _ => Err(format!("{} is not in the register_table", reg)),
    }
}

fn assemble_inc(operand: &Pair<Rule>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    if operand.as_rule() == Rule::reg16 {
        let byte = register_table(operand.as_str()).unwrap();
        // opcode 0100_0000
        v.push(byte | 0x40);
    }
    v
}

define_handler_one!(inc, first, cpu, memory, {
    let code = assemble_inc(&first);
    println!("inc {:?}", code);
    match first.as_rule() {
        Rule::reg16 => {
            let v = cpu.get_register(first.as_str()).unwrap();
            cpu.set_register(first.as_str(), v + 1).unwrap();
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});

#[cfg(test)]
mod tests {
    use crate::parser::AssemblyParser;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pest::Parser;

    #[test]
    fn test_inc_onebyte_form() {
        let parsed = AssemblyParser::parse(Rule::instruction, "inc di")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::inc, parsed.as_rule());
        let operand = parsed.into_inner().next().unwrap();
        let v = assemble_inc(&operand);
        assert_eq!(0x47, v[0]);
    }
}
