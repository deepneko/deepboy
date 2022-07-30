use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use crate::register::IoRegs;
use super::register::Register;
use super::instruction as inst;

pub struct CPU {
    pub mmc: Rc<RefCell<MMC>>,
    pub regs: Register,
    pub opcode: u32,
    pub cycles: u32,
    pub halt: bool,
    pub ime: bool,
    pub ei_delay: bool,
}

impl CPU {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        CPU {
            mmc: mmc,
            regs: Register::new(),
            opcode: 0,
            cycles: 0,
            halt: false,
            ime: false,
            ei_delay: false,
        }
    }

    pub fn run(&mut self, cycle: u32) {
        self.cycles += cycle;
        let opcode: u8 = self.read8(self.regs.pc);
        let int_enable: u8 = self.read8(IoRegs::RIE as usize);
        let int_flag: u8 = self.read8(IoRegs::RIF as usize);
        let interrupt: bool = (int_enable & int_flag) > 0;

        if self.halt {
            if !interrupt {
                self.cycles += 4;
                return;
            }
        }

        if interrupt {
            self.halt = false;
        }

        if self.ime && interrupt {
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        self.mmc.borrow_mut().read(addr)
    }
}