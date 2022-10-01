use std::cell::RefCell;
use std::rc::Rc;

use crate::register::ByteRegister;

use super::rom::Rom;
use super::ppu::PPU;

pub struct MMC {
    pub rom: Rom, 
    pub ppu: PPU,
    pub wram: [u8; 0x8000],
    pub bank: usize,
    pub hram: [u8; 0x7F],
    pub int_enable: u8,
    pub int_flag: Rc<RefCell<ByteRegister>>,
    pub disable_boot_rom: u8,
}

impl MMC {
    pub fn new() -> Self {
        let int_flag = Rc::new(RefCell::new(ByteRegister::new()));
        MMC {
            rom: Rom::new(),
            ppu: PPU::new(int_flag.clone()),
            wram: [0x00; 0x8000],
            bank: 0x01,
            hram: [0x00; 0x7F],
            int_enable: 0,
            int_flag: int_flag.clone(),
            disable_boot_rom: 0,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.rom.load_bootstrap(fname);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom.read(addr),
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xA000..=0xBFFF => self.rom.read(addr),
            0xC000..=0xCFFF => self.wram[(addr as usize) - 0xC000],
            0xD000..=0xDFFF => self.wram[(addr as usize) - 0xD000 + (0x1000 * self.bank)],
            0xE000..=0xEFFF => self.wram[addr as usize - 0xE000],
            0xF000..=0xFDFF => self.wram[(addr as usize) - 0xF000 + (0x1000 * self.bank)],
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF0F => self.int_flag.borrow_mut().data,
            0xFF40..=0xFF45 => self.ppu.read(addr),
            0xFF47..=0xFF4B => self.ppu.read(addr),
            0xFF80..=0xFFFE => self.hram[(addr as usize) - 0xFF80],
            0xFFFF => self.int_enable,
            0xFF50.. => self.disable_boot_rom,
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        // println!("addr:0x{:x}, dat:0x{:x}", addr, dat);
        match addr {
            0x0000..=0x7FFF => self.rom.write(addr, dat),
            0x8000..=0x9FFF => self.ppu.write(addr, dat),
            0xa000..=0xBFFF => self.rom.write(addr, dat),
            0xC000..=0xCFFF => self.wram[(addr as usize) - 0xC000] = dat,
            0xD000..=0xDFFF => self.wram[(addr as usize) - 0xD000 + (0x1000 * self.bank)] = dat,
            0xE000..=0xEFFF => self.wram[addr as usize - 0xE000] = dat,
            0xF000..=0xFDFF => self.wram[(addr as usize) - 0xF000 + (0x1000 * self.bank)] = dat,
            0xFE00..=0xFE9F => self.ppu.write(addr, dat),
            0xFF0F => self.int_flag.borrow_mut().data = dat,
            0xFF40..=0xFF45 => self.ppu.write(addr, dat),
            0xFF47..=0xFF4B => self.ppu.write(addr, dat),
            0xFF80..=0xFFFE => self.hram[(addr as usize) - 0xFF80] = dat,
            0xFFFF => self.int_enable = dat,
            0xFF50 => self.disable_boot_rom = dat,
            _ => {},
        }
    }
}