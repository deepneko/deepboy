use std::{cell::RefCell, rc::Rc};

use crate::rom::Rom;

pub struct MMC {
    pub rom: Rom, 
}

impl MMC {
    pub fn new() -> Self {
        MMC {
            rom: Rom::new(),
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.rom.load_bootstrap(fname);
    }

    pub fn read(&mut self, addr: usize) {
        if addr < 256 {
            self.rom.boot_rom[addr];
        } else if addr < 0x8000 {
            self.rom.ram[addr];
        }
    }
}