use crate::assembler::{base_index_table, register_table};
use crate::memory::Memory;
use crate::parser::{self, imm_to_num, mem_to_num, Rule};
use crate::{cpucontext::CpuContext, define_handler_two};
use paste::paste;
use pest::iterators::Pair;

/*
ADD opcode

1. 2-byte form
e.g. add ch, bl
1-byte Opcode bit 7-2: opcode 000000
1-byte D bit 1: 1 (Always 1 in this program)
1-byte W bit 0: 0 (8bit operand)
2-byte mod(bit 7-6): 11 (register operand)
2-byte reg bit 5-3: 101 (ch)
2-byte r/m(2-0): 011 (bl)

4. 4-byte form
e.g. add ax, [000c] 03 06 0c 00
1-byte opcode bit 7-2: 000000
1-byte D bit 1: 1
1-byte W bit 1: 1 (16bit operand)
2-byte mod bit 7-6: 00 (memory)
2-byte reg bit 5-3: 000 (ax)
2-byte r/m bit 2-0: 110
  * When mode is 00, 110 means direct addressing mode.
  * When mode is 10, see base/index register table
  * When mode is 11, see register table

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

reference: https://stevemorse.org/8086/
Reg/memory with register to other: 0000_00dw mod reg r/m
imm to register/memory: 1000_00sw mod 000 r/m
imm to accumulator: 0000_010w data data if w>1
*/

const OPCODE1_SHIFT: u8 = 2;
const DBIT_SHIFT: u8 = 1;
const SBIT_SHIFT: u8 = 1; // signed extended: NOT USED
const WBIT_SHIFT: u8 = 0;
const MOD_SHIFT: u8 = 6;
const OPCODE2_SHIFT: u8 = 3;
const REG_SHIFT: u8 = OPCODE2_SHIFT;
const RM_SHIFT: u8 = 0;

fn assemble_add(first: &Pair<Rule>, second: &Pair<Rule>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    if first.as_str() == "ax" && second.as_rule() == Rule::imm {
        let imm = imm_to_num(&second).unwrap();
        let opcode = 0x04;
        let wbit = 1;
        v.push(opcode | wbit);
        v.push((imm & 0xff) as u8);
        v.push(((imm & 0xff00) >> 8) as u8);
    } else {
        match (first.as_rule(), second.as_rule()) {
            (Rule::reg16, Rule::reg16) => {
                let opcode1 = 0 << OPCODE1_SHIFT;
                let dbit = 1 << DBIT_SHIFT; // regbit=first-operand, rmbit=second-operand
                let wbit = 1 << WBIT_SHIFT;
                v.push(opcode1 | dbit | wbit);

                let modbit = 3 << MOD_SHIFT; // rmbit => register
                                             // no opcode2 but reg
                let regbit = register_table(first.as_str()).unwrap() << REG_SHIFT;
                let rmbit = register_table(second.as_str()).unwrap() << RM_SHIFT;
                v.push(modbit | regbit | rmbit);
            }
            (Rule::reg16, Rule::imm) => {
                let opcode1 = 0x20 << OPCODE1_SHIFT;
                let wbit = 1 << WBIT_SHIFT;
                v.push(opcode1 | wbit);

                let modbit = 2 << MOD_SHIFT; // 2: 16-bit contents of next two bytes are imm
                let rmbit = register_table(first.as_str()).unwrap() << RM_SHIFT;
                v.push(modbit | rmbit);

                let imm = parser::imm_to_num(&second).unwrap();
                v.push((imm & 0xff).try_into().unwrap());
                v.push(((imm & 0xff00) >> 8).try_into().unwrap());
            }
            (Rule::mem16, Rule::imm) => {
                // todo
            }
            (Rule::mem16, Rule::reg16) => {
                // todo
            }
            (Rule::indirect16, Rule::imm) => {
                // todo
            }
            (Rule::indirect16, Rule::reg16) => {
                // todo
            }

            _ => panic!(
                "Unknown format of ADD instruction, add {}, {}",
                first.as_str(),
                second.as_str()
            ),
        }
    }
    // TODO: Other cases
    v
}

fn do_add16(cpu: &mut CpuContext, l: u16, r: u16) -> u16 {
    // work-around the overflow checker of Rust
    let l32: u32 = l as u32;
    let r32: u32 = r as u32 + l32;
    let r_added: u16 = (r32 & 0xffff) as u16;

    // Setting OF cases
    // 1. the sum of two numbers with the sign bit off yields a result number with the sign bit on.
    // 2. the sum of two numbers with the sign bit on yields a result number with the sign bit off.
    if (l & 0x8000) == 0 && (r & 0x8000) == 0 && (r_added & 0x8000) == 0x8000 {
        cpu.set_OF();
    } else if (l & 0x8000) == 0x8000 && (r & 0x8000) == 0x8000 && (r_added & 0x8000) == 0 {
        cpu.set_OF();
    } else {
        cpu.reset_OF();
    }

    // Setting CF cases
    // 1. the addition of two numbers causes a carry out of the most significant (leftmost) bits added.
    // 2. the subtraction of two numbers requires a borrow into the most significant (leftmost) bits subtracted.
    // 2nd case will be implemented in sub instruction handler.
    if (r32 & 0x8000) == 0 && (r32 & 0x10000) != 0 {
        cpu.set_CF();
    } else {
        cpu.reset_CF();
    }

    r_added
}

/// Maybe do_add16 and do_add8 could be unified as do_add(cpu, num_bit, l, r)
fn _do_add8(cpu: &mut CpuContext, l: u8, r: u8) -> u8 {
    // work-around the overflow checker of Rust
    let l32: u32 = l as u32;
    let r32: u32 = r as u32 + l32;
    let r_added: u8 = (r32 & 0xff) as u8;

    // Setting OF cases
    // 1. the sum of two numbers with the sign bit off yields a result number with the sign bit on.
    // 2. the sum of two numbers with the sign bit on yields a result number with the sign bit off.
    if (l & 0x80) == 0 && (r & 0x80) == 0 && (r_added & 0x80) == 0x80 {
        cpu.set_OF();
    } else if (l & 0x80) == 0x80 && (r & 0x80) == 0x80 && (r_added & 0x80) == 0 {
        cpu.set_OF();
    } else {
        cpu.reset_OF();
    }

    // Setting CF cases
    // 1. the addition of two numbers causes a carry out of the most significant (leftmost) bits added.
    // 2. the subtraction of two numbers requires a borrow into the most significant (leftmost) bits subtracted.
    // 2nd case will be implemented in sub instruction handler.
    if (r32 & 0x80) == 0 && (r32 & 0x100) == 1 {
        cpu.set_CF();
    } else {
        cpu.reset_CF();
    }

    r_added
}

define_handler_two!(add, first, second, cpu, memory, {
    if first.as_str() == "ax" && second.as_rule() == Rule::imm {
        let l: u16 = cpu.get_register16(first.as_str());
        let r = imm_to_num(&second).unwrap();
        let v = do_add16(cpu, l, r);
        cpu.set_register16(first.as_str(), v);
        let code = assemble_add(&first, &second);
        println!("Code for add ax, {} = {:?}", r, code);
    } else if first.as_str() == "al" && second.as_rule() == Rule::imm {
        // TODO
    } else {
        match (first.as_rule(), second.as_rule()) {
            // There is no mem-mem operation for ALL instruction.
            // 8/16-bit each has 5 operations. Total 10 operations.
            // reg-reg
            // reg-mem
            // reg-imm
            // mem-reg
            // mem-imm
            (Rule::reg16, Rule::reg16) => {
                let l: u16 = cpu.get_register16(first.as_str());
                let r: u16 = cpu.get_register16(second.as_str());
                let v = do_add16(cpu, l, r);
                cpu.set_register16(first.as_str(), v);
            }
            (Rule::reg16, Rule::imm) => {
                let l = cpu.get_register16(first.as_str());
                let r = imm_to_num(&second).unwrap();
                let v = do_add16(cpu, l, r);
                let _ = cpu.set_register16(first.as_str(), v);
            }
            (Rule::reg16, Rule::mem16) => {
                let address = mem_to_num(&second).unwrap();
                let l = memory.read16(address);
                let r = cpu.get_register16(first.as_str());
                let v = do_add16(cpu, l, r);
                let _ = cpu.set_register16(first.as_str(), v);
            }
            (Rule::reg8, Rule::reg8) => {
                // TODO
            }
            (Rule::reg8, Rule::imm) => {
                // TODO
            }
            (Rule::reg8, Rule::mem8) => {
                // Todo
            }
            (Rule::mem16, Rule::reg16) => {
                let address = mem_to_num(&first).unwrap();
                let l = memory.read16(address);
                let r = cpu.get_register16(first.as_str());
                let v = do_add16(cpu, l, r);
                memory.write16(address, v);
            }
            (Rule::mem16, Rule::imm) => {
                // Todo
                let address = mem_to_num(&first).unwrap();
                let l = memory.read16(address);
                let r = imm_to_num(&second).unwrap();
                let v = do_add16(cpu, l, r);
                memory.write16(address, v);
            }
            (Rule::mem8, Rule::reg8) => {
                // Todo
            }
            (Rule::mem8, Rule::imm) => {
                // Todo
            }
            _ => println!("Not supported yet:{:?} {:?}", first, second),
        }
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_overflow() {
        let mut cpu = CpuContext::boot();

        // Plus + Plus = Minus => Overflow error!
        do_add16(&mut cpu, u16::MAX / 2, u16::MAX / 2);
        assert_ne!(0, cpu.get_OF());

        // 0xffff + 1 = 0x10000 => 0x0 as u16.
        // There is no overflow because -1 + 1 = 0.
        // But there is a carry.
        do_add16(&mut cpu, u16::MAX, 1);
        assert_eq!(0, cpu.get_OF());

        // Plus + Plus = Minus => Overflow error!
        do_add16(&mut cpu, 0x7fff, 1);
        assert_ne!(0, cpu.get_OF());
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = CpuContext::boot();

        // 0xffff + 1 = 0x10000 => 0x0 as u16.
        // There is no overflow because -1 + 1 = 0.
        // But there is a carry.
        do_add16(&mut cpu, u16::MAX, 1);
        assert_eq!(0, cpu.get_OF());
        assert_ne!(0, cpu.get_CF());

        do_add16(&mut cpu, 1, 1);
        assert_eq!(0, cpu.get_OF());
        assert_eq!(0, cpu.get_CF());
    }

    #[test]
    fn test_add_reg_reg() {
        // TODO
    }

    #[test]
    fn test_add_mem_reg() {
        // TODO
    }

    #[test]
    fn test_add_reg_imm() {
        // todo
    }

    #[test]
    fn test_add_mem_imm() {
        // todo
    }

    #[test]
    fn test_add_indirect_imm() {
        // todo
    }

    #[test]
    fn test_add_indirect_reg() {
        // todo
    }
}
