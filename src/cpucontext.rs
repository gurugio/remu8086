enum CpuFlag {
    CF,
    ZF,
    SF,
    OF,
    PF,
    DF, // direction
}

#[derive(Default, Debug)]
pub struct CpuContext {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub ip: u16,
    pub cs: u16,
    pub flag: u16,
    /* more */
}

#[derive(PartialEq, Eq)]

pub enum OperFlag {
    Reg8,
    Reg16,
    Mem8,
    Mem16,
    Imm8,
    Imm16,
}

pub struct Operand {
    pub field: String,
    pub flag: OperFlag,
}

pub struct Instruction {
    pub operation: String,
    pub dest: Option<Operand>,
    pub src: Option<Operand>,
}

impl CpuContext {
    pub fn read_reg(&self, reg: &str) -> u16 {
        match reg {
            "ax" => self.ax,
            _ => panic!("wrong register"),
        }
    }

    pub fn write_reg(&mut self, reg: &str, val: u16) {
        match reg {
            "ax" => self.ax = val,
            _ => panic!("wrong register"),
        }
    }
}
