enum _CpuFlag {
    CF,
    ZF,
    SF,
    OF,
    PF,
    DF, // direction
}

#[derive(Default, Debug)]
pub struct CpuContext {
    // General Registers
    _ax: u16,
    _bx: u16,
    _cx: u16,
    _dx: u16,
    _si: u16,
    _di: u16,
    _bp: u16,
    _sp: u16,
    // Segment Registers
    cs: u16,
    _ds: u16,
    _es: u16,
    _ss: u16,
    // Special Purpose Registers
    ip: u16,
    _flag: u16,
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
    pub fn set_ip(&mut self, v: u16) {
        self.ip = v;
    }

    pub fn set_cs(&mut self, v: u16) {
        self.cs = v;
    }
}
