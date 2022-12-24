use crate::cpucontext::{CpuContext, Instruction, OperFlag};

/// add series
/// ADD r/m8, reg8 $00
/// ADD r/m16, reg16 $01
/// ADD reg8, r/m8 $02
/// ADD reg16, r/m16 $03
/// ADD AL, imm8 $04
/// ADD AX, imm16 $05
/// ADD r/m8, imm8 $80 xx000xxx (ModR/M byte)
/// ADD r/m16, imm16 $81 xx000xxx (ModR/M byte)
/// ADD r/m16, imm8 $83 xx000xxx (ModR/M byte)
/// ex) add
///

pub fn do_add(context: &mut CpuContext, inst: &Instruction) {
    // We already know that inst.operation is "ADD".
    // ADD operation always has dest and src operands.

    if inst.dest.is_none() || inst.src.is_none() {
        // TODO: return error with anyhow
        panic!("wrong operands: Add requires both operands");
    }

    let dest = inst.dest.as_ref().unwrap();
    let src = inst.src.as_ref().unwrap();

    // TODO: check type of dest and src
    add_reg16_imm16(context, &dest.field, &src.field);
}

fn add_reg16_imm16(context: &mut CpuContext, reg: &str, imm: &str) {
    // if dest is reg16 and src is imm16
    let cur_val = context.read_reg(reg);
    context.write_reg(reg, cur_val + imm.parse::<u16>().unwrap());
    // TODO: update context.flag
}

// TODO: add unittest for each case
