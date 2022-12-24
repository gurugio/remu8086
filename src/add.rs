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

    // When left is register16
    if let Some(left) = inst.left_operand.as_ref() {
        if inst.right_operand.is_none() {
            panic!("wrong right");
        }

        let target_reg = &left.field;
        let cur_val = context.read_reg(target_reg);
        context.write_reg(
            target_reg,
            cur_val
                + inst
                    .right_operand
                    .as_ref()
                    .map(|opnd| opnd.field.parse::<u16>().unwrap())
                    .unwrap(),
        );
    } else {
        todo!();
    }
}
