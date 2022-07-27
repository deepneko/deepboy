use std::{cell::RefCell, rc::Rc};

use crate::rom::Rom;

pub struct MMC {
    pub rom: Rc<RefCell<Rom>>,
}

impl MMC {
    pub fn new(rom: Rc<RefCell<Rom>>) -> Self {
        MMC {
            rom: rom,
        }
    }

    pub fn read(&mut self, addr: usize) {
        if addr < 256 {
            self.rom.borrow().boot_rom[addr];
        } else if addr < 0x8000 {
            self.rom.borrow().ram[addr];
        }
    }
}