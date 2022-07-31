use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use crate::register::*;
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
        let int_enable: u8 = self.read8(IoRegs::IE as usize);
        let int_flag: u8 = self.read8(IoRegs::IF as usize);
        let interrupt: u8 = int_enable & int_flag;

        if self.halt {
            if interrupt > 0 {
                self.cycles += 4;
                return;
            }
        }

        if interrupt > 0 {
            self.halt = false;
        }

        if self.ime && interrupt > 0 {
            // ToDo: Timer cycle

            self.write8(self.regs.sp, (self.regs.pc >> 8) as u8);
            self.regs.sp -= 1;
            self.write8(self.regs.sp, (self.regs.pc & 0xFF) as u8);
            self.regs.sp -= 1;
        }

        if (interrupt & (IntFlag::VBLANK as u8)) > 0 {
            self.regs.pc = 0x40;
            int_flag &= ~(IntFlag::VBLANK);
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        self.mmc.borrow_mut().read(addr)
    }

    pub fn write8(&mut self, addr: usize, dat: u8) {
        self.mmc.borrow_mut().write(addr, dat);
    }
}