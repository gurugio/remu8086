use crate::memory::Memory;
use crate::parser::Rule;
use crate::{cpucontext::CpuContext, define_handler_one};
use paste::paste;
use pest::iterators::Pair;

/*
INC instruction has 5 forms of code

1. 1-byte form: inc 16-bit registers
e.g. INC DI => 47
Opcode bit 7-3: 01000
bit 2-0: register table

2. 2-byte form: inc 8-bit registers or addressing with only base/index register
e.g. inc byte ptr [bx] => FE 07
e.g. inc word ptr [bx] => FF 07
e.g. INC WORD PTR [SI] => FF 06
e.g. inc dl FE C2
1-byte Opcode bit 7-1: opcode 1111111
1-byte W-bit 0: 0-operand is 8-bit, 1-operand is 16-bit
2-byte mod-bit 7-6
  * 00-addressing with only base/index register
  * 11-increase register value
2-byte Opcode bit 5-3: 000
2-byte bit 2-0
  * base/index register table when mod=00
  * register table when mod=11

3. 3-byte form: inc memory location with 8-bit address (e.g. INC BYTE PTR [BX+10h])
e.g. INC BYTE PTR [BX+10h] => FE 47 10
e.g. INC WORD PTR [BX+SI+10h] => FF 84 10
1-byte Opcode bit 7-1: 1111111
1-byte W bit 0: 0-8bit, 1-16bit
2-byte mod-bit 7-6: 01-use base register and 8-bit displacement of next one byte of instruction
2-byte Opcode 5-3: 000
2-byte r/m 2-0: (base and index register)
3-byte Displacement 7-0

4. 4-byte form: inc memory location with 16-bit address
e.g. INC WORD PTR [BX+SI+1234h] => FF 84 34 12 (ChatGPT shows this code, so it might be wrong)
e.g. INC BYTE PTR [BX+1234h] => FE 87 34 12
e.g. INC WORD PTR [BX+1234h] => FF 87 34 12
e.g. INC WORD PTR [BP+1234h] => FF 86 34 12
e.g. INC WORD PTR [1234h] => FF 06 34 12
e.g. INC WORD PTR [12h] => FF 06 12 00
1-byte Opcode bit 7-1: 1111111
1-byte W bit 0: 0-8bit, 1-16bit
2-byte mod-bit 7-6
  * 00-use only 16-bit displacement of next two bytes of instruction (inc word ptr [1234h])
  * 10-use base register and 16bit displacement of next two bytes of instruction (inc word ptr [bx+si+1234h])
       (next byte->least significant eight bits, byte after that->most sig bits)
2-byte Opcode 5-3: 000
2-byte r/m 2-0: (base and index register)
  * 110-when mode is 00, direct addressing mode
  * base/index register table when mode is 10
3-byte Displacement 7-0
4-byte Displacement 15-8

5. 5-byte form: use ES segment registers
1-byte: 0010_0110 (ES)
2~5-byte: same to 4-byte form

mod table for addressing
00 16-bit displacement (=> 2-byte and 4-byte form)
01 8-bit contents of next byte of instruction sign extended to 16 bits (=> 3-byte form)
10 16-bit contents of next two bytes of instruction (=> 4-byte form)
11 NOT used for memory addressing, increase the value in register

register table
000 AX AL
001 CX CL
010 DX DL
011 BX BL
100 SP AH
101 BP CH
110 SI DH
111 DI BH

base/index table when mod != 11
r/m field | Base register | Index Register | address
000       | BX            | SI             | DS:BX + SI + displacement
001       | BX            | DI             | DS:BX + DI + displacement
010       | BP            | SI             | SP:BP + SI + displacement
011       | BP            | DI             | SP:BP + DI + displacement
100       | none          | SI             | DS:SI + displacement
101       | none          | DI             | DS:DI + displacement
110       | BP            | none           | SP:BP + displacement
111       | BX            | none           | DS:BX + displacement

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

fn base_index_table(base: Option<&str>, index: Option<&str>) -> Result<u8, String> {
    match (base, index) {
        (Some("bx"), Some("si")) => Ok(0),
        _ => Err(format!(
            "{:?} {:?} is not in the base index table",
            base, index
        )),
    }
}

// For 2/3/4 bytes forms
// 1-byte form does not need bit-shift operation.
// 1st byte
const OPCODE1_SHIFT: u8 = 1;
const WBIT_SHIFT: u8 = 0;
// 2nd byte
const MOD_SHIFT: u8 = 6;
const OPCODE2_SHIFT: u8 = 3;
const REG_SHIFT: u8 = 0; // register table or base-index-register table

fn assemble_inc(operand: &Pair<Rule>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    if operand.as_rule() == Rule::reg16 {
        let reg = register_table(operand.as_str()).unwrap();
        let opcode = 0x40;
        v.push(reg | opcode);
    } else if operand.as_rule() == Rule::reg8 {
        let opcode = 0x7f << OPCODE1_SHIFT;
        let wbit = 0x0 << WBIT_SHIFT;
        v.push(opcode | wbit);
        let modbit = 0x3 << MOD_SHIFT;
        let opcode2 = 0x0 << OPCODE2_SHIFT;
        let reg = register_table(operand.as_str()).unwrap() << REG_SHIFT;
        v.push(modbit | opcode2 | reg);
    } else {
        unimplemented!("inc memory not implemented yet")
    }
    v
}

define_handler_one!(inc, first, cpu, memory, {
    match first.as_rule() {
        Rule::reg16 => {
            let code = assemble_inc(&first);
            println!("inc code {:?}", code);
            let v = cpu.get_register(first.as_str()).unwrap();
            cpu.set_register(first.as_str(), v + 1).unwrap();
        }
        Rule::reg8 => {
            let code = assemble_inc(&first);
            println!("inc code {:?}", code);
            let v = cpu.get_register(first.as_str()).unwrap();
            cpu.set_register(first.as_str(), v + 1).unwrap();
        }
        Rule::mem16 => {
            unimplemented!("handle inc memory not implemented yet")
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});

#[cfg(test)]
mod tests {
    use crate::parser::{self, AssemblyParser};

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
        assert_eq!(Rule::reg16, operand.as_rule());
        assert_eq!("di", operand.as_str());
        let v = assemble_inc(&operand);
        assert_eq!(0x47, v[0]);
    }

    #[test]
    fn test_inc_twobyte_form() {
        let parsed = AssemblyParser::parse(Rule::instruction, "inc dl")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::inc, parsed.as_rule());
        let operand = parsed.into_inner().next().unwrap();
        assert_eq!(Rule::reg8, operand.as_rule());
        assert_eq!("dl", operand.as_str());
        let v = assemble_inc(&operand);
        assert_eq!(0xfe, v[0]);
        assert_eq!(0xc2, v[1]);
    }

    #[test]
    fn test_inc_threebyte_form() {
        let parsed = AssemblyParser::parse(Rule::instruction, "inc [1234h]")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::inc, parsed.as_rule());
        let operand = parsed.into_inner().next().unwrap();
        assert_eq!(Rule::mem16, operand.as_rule());
        assert_eq!("[1234h]", operand.as_str());
        assert_eq!(0x1234, parser::mem_to_num(&operand).unwrap());

        let v = assemble_inc(&operand);
    }
}
