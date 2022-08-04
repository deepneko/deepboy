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

    pub fn read8(&mut self, addr: usize) -> u8 {
        if addr < 256 {
            self.rom.boot_rom[addr]
        } else if addr < 0x8000 {
            self.rom.ram[addr]
        } else {
            0
        }
    }

    pub fn read16(&mut self, addr: usize) -> u16 {
        u16::from(self.read8(addr)) | u16::from(self.read8(addr + 1)) << 8
    }

    pub fn write(&mut self, addr: usize, dat: u8) {

    }
}