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

    pub fn read(&mut self, addr: u16) -> u8 {
        let uaddr = addr as usize;
        match addr {
            0x0000..=0x00FF => self.rom.boot_rom[uaddr],
            0x00FF..=0x8000 => self.rom.ram[uaddr],
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, dat: u8) {

    }
}