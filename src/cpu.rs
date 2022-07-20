use super::register::Register;
use super::instruction as inst;

pub struct CPU {
    pub regs: Register,
    pub opcode: u32,
    pub cycles: u32,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: Register::new(),
            opcode: 0,
            cycles: 0,
        }
    }

    pub fn run(&mut self, cycles: u32) {
        inst::ld_rr();
    }
}