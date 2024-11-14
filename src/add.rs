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

fn do_add(cpu: &mut CpuContext, reg: &str, l: u16, r: u16) {
    // work-around the overflow checker of Rust
    let l32: u32 = l as u32;
    let r32: u32 = r as u32 + l32;
    let r_added = r32 as u16;

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
    if (r32 & 0x8000) == 0 && (r32 & 0x10000) == 1 {
        cpu.set_CF();
    } else {
        cpu.reset_CF();
    }

    cpu.set_register(reg, r_added).unwrap();
}

define_handler_two!(add, first, second, cpu, {
    match (first.as_rule(), second.as_rule()) {
        (Rule::reg16, Rule::reg16) => {
            cpu.set_register(first.as_str(), cpu.get_register(second.as_str()).unwrap())
                .unwrap();
            let l: u16 = cpu.get_register(first.as_str()).unwrap();
            let r: u16 = cpu.get_register(second.as_str()).unwrap();
            do_add(cpu, first.as_str(), l, r);
        }
        (Rule::reg16, Rule::imm) => {
            let r = imm_to_num(&second).unwrap();
            let l = cpu.get_register(first.as_str()).unwrap();
            do_add(cpu, first.as_str(), l, r);
        }
        _ => println!("Not supported yet:{:?} {:?}", first, second),
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_overflow() {
        let mut cpu: CpuContext = CpuContext::boot();

        // Plus + Plus = Minus => Overflow error!
        do_add(&mut cpu, "ax", u16::MAX / 2, u16::MAX / 2);
        assert_ne!(0, cpu.get_OF());

        // 0xffff + 1 = 0x10000 => 0x0 as u16.
        // There is no overflow because -1 + 1 = 0.
        // But there is a carry.
        do_add(&mut cpu, "ax", u16::MAX, 1);
        assert_eq!(0, cpu.get_OF());

        // Plus + Plus = Minus => Overflow error!
        do_add(&mut cpu, "ax", 0x7fff, 1);
        assert_ne!(0, cpu.get_OF());
    }
}
