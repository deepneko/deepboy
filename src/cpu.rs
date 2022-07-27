use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use super::register::Register;
use super::instruction as inst;

pub struct CPU {
    pub mmc: Rc<RefCell<MMC>>,
    pub regs: Register,
    pub opcode: u32,
    pub cycles: u32,
}

impl CPU {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        CPU {
            mmc: mmc,
            regs: Register::new(),
            opcode: 0,
            cycles: 0,
        }
    }

    pub fn run(&mut self, cycles: u32) {
    }

    pub fn read(&mut self, addr: usize) {
        self.mmc.borrow_mut().read(addr);
    }
}