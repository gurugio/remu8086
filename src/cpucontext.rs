use crate::common::count_bit;
use paste::paste;
use std::fmt;

macro_rules! setter_and_getter_reg {
    ( $($reg:ident),+ ) => {
        paste! {
            $(
                fn [<set_ $reg>](&mut self, v: u16) {
                    self.$reg = v;
                }
                fn [<get_ $reg>](&self) -> u16 {
                    self.$reg
                }
            )+
        }
    };
}

macro_rules! setter_and_resetter_flag {
    ( $($flag:ident),+ ) => {
        paste! {
            $(
                #[allow(non_snake_case)]
                pub fn [<set_ $flag>](&mut self) {
                    self.flags |= [<$flag _MASK>];
                }

                #[allow(non_snake_case)]
                pub fn [<reset_ $flag>](&mut self) {
                    self.flags &= ![<$flag _MASK>];
                }

                #[allow(non_snake_case)]
                pub fn [<get_ $flag>](&mut self) -> u16 {
                    self.flags & [<$flag _MASK>]
                }
            )+
        }
    };
}

/*
reference: https://www.geeksforgeeks.org/flag-register-8086-microprocessor/
*/

const CF: u16 = 0; // Carry: 1=carry, 0=no-carry
const CF_MASK: u16 = 1 << CF;
const PF: u16 = 2; // Parity: 1=even, 0=odd
const PF_MASK: u16 = 1 << PF;
// AC: NOT supported yet
//const _AC: u16 = 4;
//const _AC_MASK: u16 = 1 << _AC;
const ZF: u16 = 6; // Zero: 1=zero, 0=non-zero
const ZF_MASK: u16 = 1 << ZF;
const SF: u16 = 7; // Sign: 1=negative, 0=positive
const SF_MASK: u16 = 1 << SF;
//const IF: u16 = 9;
//const IF_MASK: u16 = 1 << IF;
// DF: NOT supported yet
//const DF: u16 = 10; // direction: 1=down, 0=up (opcode: STD, CLD)
//const DF_MASK: u16 = 1 << DF;
const OF: u16 = 11; // Overflow: 1=overflow, 0=not-overflow
const OF_MASK: u16 = 1 << OF;

#[derive(Default)]
pub struct CpuContext {
    // General Registers
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    si: u16,
    di: u16,
    bp: u16,
    sp: u16,
    // Segment Registers
    cs: u16,
    ds: u16,
    es: u16,
    ss: u16,
    // Special Purpose Registers
    ip: u16,
    flags: u16,
}

impl CpuContext {
    /// Creates a just-booted CPU
    ///
    /// All registers and flags are cleared except cs.
    /// cs:ip=0xffff:0 is a location of BIOS.
    pub fn boot() -> Self {
        CpuContext {
            cs: 0xffff,
            ip: 0,
            ..Default::default()
        }
    }
    // TODO: set_* functions to set each register16 and register8
    // set_ax, set_al, set_ah, get_ax, get_al, get_ah
    // set_bx, ...
    // For example, "push ax" calls
    // 1. ss = get_ss, sp = get_sp, ax = get_ax
    // 2. set_sp(get_sp() - 2)
    // Initial stack address: ss:sp = 0x0:0x0
    // First input: ss:sp = 0xf:0xfffe
    // Second input: ss:sp = 0xf:0xfffc
    // ....0xf:0x0000
    // ....0xe:0xfffe
    // ....when sp gets underflow, ss is decreased.
    // 3. [ss:sp] = ax
    setter_and_getter_reg!(ax, bx, cx, dx, si, di, cs, ip);

    setter_and_resetter_flag!(PF, ZF, SF, OF, CF);

    pub fn get_register(&self, reg: &str) -> Result<u16, String> {
        let r = match reg {
            "ax" => self.get_ax(),
            "bx" => self.get_bx(),
            "cx" => self.get_cx(),
            "dx" => self.get_dx(),
            "si" => self.get_si(),
            "di" => self.get_di(),
            "cs" => self.get_cs(),
            "ip" => self.get_ip(),
            _ => return Err("Wrong register specified for get_register".to_string()),
        };
        Ok(r)
    }

    fn is_general_reg(&self, reg: &str) -> bool {
        match reg {
            "ax" | "bx" | "cx" | "dx" | "si" | "di" => true,
            _ => false,
        }
    }

    pub fn set_register(&mut self, reg: &str, v: u16) -> Result<(), String> {
        // Flags is changed according to both of the old value of the target register and the new value.
        if self.is_general_reg(reg) {
            self.change_flags(v);
        }

        match reg {
            "ax" => self.set_ax(v),
            "bx" => self.set_bx(v),
            "cx" => self.set_cx(v),
            "dx" => self.set_dx(v),
            "si" => self.set_si(v),
            "di" => self.set_di(v),
            "cs" => self.set_cs(v),
            "ip" => self.set_ip(v),
            _ => return Err("Wrong register specified for set_register".to_string()),
        };
        Ok(())
    }

    /// Change some flags changed by all instruction
    ///
    /// This function is called in set_register function. So this function changes
    /// the flags that can be changed by all instruction.
    /// CF and OF are changed by each instruction handlers: mov, add, sub, stc, clc, cmc and so on.
    /// IF is changed by STI/CLI
    ///
    fn change_flags(&mut self, v: u16) {
        if v & 0x8000 != 0 {
            self.set_SF();
        } else {
            self.reset_SF();
        }

        if count_bit(v) & 0x1 == 0 {
            self.set_PF();
        } else {
            self.reset_PF();
        }

        if v == 0 {
            self.set_ZF();
        } else {
            self.reset_ZF();
        }
    }
}

impl fmt::Debug for CpuContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CpuContext {{\n\
             \t_ax: 0x{:04X}, bx: 0x{:04X}, cx: 0x{:04X}, dx: 0x{:04X},\n\
             \t_si: 0x{:04X}, di: 0x{:04X}, bp: 0x{:04X}, sp: 0x{:04X},\n\
             \tcs: 0x{:04X}, ds: 0x{:04X}, es: 0x{:04X}, ss: 0x{:04X},\n\
             \tip: 0x{:04X}, flags: 0x{:04X}\n\
             }}",
            self.ax,
            self.bx,
            self.cx,
            self.dx,
            self.si,
            self.di,
            self.bp,
            self.sp,
            self.cs,
            self.ds,
            self.es,
            self.ss,
            self.ip,
            self.flags
        )
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_set_reset_flags() {
        let mut cpu = CpuContext::boot();
        cpu.set_ZF();
        assert_eq!(ZF_MASK, cpu.get_ZF());
        cpu.set_PF();
        assert_eq!(ZF_MASK | PF_MASK, cpu.get_ZF() | cpu.get_PF());
        cpu.set_SF();
        assert_eq!(
            ZF_MASK | PF_MASK | SF_MASK,
            cpu.get_ZF() | cpu.get_PF() | cpu.get_SF()
        );
        cpu.reset_SF();
        assert_eq!(ZF_MASK | PF_MASK, cpu.get_ZF() | cpu.get_PF());
        cpu.reset_PF();
        assert_eq!(ZF_MASK, cpu.get_ZF());
        cpu.reset_ZF();
        assert_eq!(0, cpu.flags);
    }

    #[test]
    fn test_zf() {
        let mut cpu = CpuContext::boot();
        cpu.flags = 0;
        cpu.set_register("ax", 0).unwrap();
        assert_eq!(ZF_MASK, cpu.get_ZF());
        cpu.set_register("ax", 1).unwrap();
        assert_eq!(0, cpu.get_ZF());
    }

    #[test]
    fn test_sf() {
        let mut cpu = CpuContext::boot();
        cpu.flags = 0;
        cpu.set_register("ax", 0xffff).unwrap();
        assert_eq!(SF_MASK, cpu.get_SF());
        cpu.set_register("ax", 0x7777).unwrap();
        assert_eq!(0, cpu.get_SF());
    }

    #[test]
    fn test_pf() {
        let mut cpu = CpuContext::boot();
        cpu.flags = 0;
        cpu.set_register("ax", 0x2222).unwrap();
        assert_eq!(PF_MASK, cpu.get_PF());
        cpu.set_register("ax", 0x777).unwrap();
        assert_eq!(0, cpu.get_PF());
    }
}
