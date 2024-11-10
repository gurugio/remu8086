use paste::paste;
use std::fmt;

macro_rules! setter_and_getter_reg {
    ( $($reg:ident),+ ) => {
        paste! {
            $(
            pub fn [<set_ $reg>](&mut self, v: u16) {
                self.$reg = v;
            }
            pub fn [<get_ $reg>](&self) -> u16 {
                self.$reg
            }
            )+
        }
    };
}

enum _CpuFlag {
    CF,
    ZF,
    SF,
    OF,
    PF,
    DF, // direction
}

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
    flag: u16,
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
    setter_and_getter_reg!(cs, ip);
}

impl fmt::Debug for CpuContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CpuContext {{\n\
             \t_ax: 0x{:04X}, _bx: 0x{:04X}, _cx: 0x{:04X}, _dx: 0x{:04X},\n\
             \t_si: 0x{:04X}, _di: 0x{:04X}, _bp: 0x{:04X}, _sp: 0x{:04X},\n\
             \tcs: 0x{:04X}, _ds: 0x{:04X}, _es: 0x{:04X}, _ss: 0x{:04X},\n\
             \tip: 0x{:04X}, _flag: 0x{:04X}\n\
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
            self.flag
        )
    }
}
