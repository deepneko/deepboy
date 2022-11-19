use std::cell::RefCell;
use std::rc::Rc;

use crate::joypad::Joypad;
use crate::register::ByteRegister;
use crate::timer::Timer;

use super::rom::Rom;
use super::ppu::PPU;

pub struct MMC {
    pub rom: Rom, 
    pub ppu: PPU,
    pub joypad: Joypad,
    pub timer: Timer,
    pub wram: [u8; 0x8000],
    pub bank: usize,
    pub hram: [u8; 0x7F],
    pub int_enable: u8,
    pub int_flag: Rc<RefCell<ByteRegister>>,
}

impl MMC {
    pub fn new(fname: &String) -> Self {
        let int_flag = Rc::new(RefCell::new(ByteRegister::new()));
        MMC {
            rom: Rom::new(fname),
            ppu: PPU::new(int_flag.clone()),
            joypad: Joypad::new(),
            timer: Timer::new(),
            wram: [0x00; 0x8000],
            bank: 0x01,
            hram: [0x00; 0x7F],
            int_enable: 0,
            int_flag: int_flag.clone(),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let result = match addr {
            0x0000..=0x7FFF => self.rom.read(addr),
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xA000..=0xBFFF => self.rom.read(addr),
            0xC000..=0xCFFF => self.wram[(addr as usize) - 0xC000],
            0xD000..=0xDFFF => self.wram[(addr as usize) - 0xD000 + (0x1000 * self.bank)],
            0xE000..=0xEFFF => self.wram[addr as usize - 0xE000],
            0xF000..=0xFDFF => self.wram[(addr as usize) - 0xF000 + (0x1000 * self.bank)],
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF00 => self.joypad.read(addr),
            0xFF04..=0xFF07 => self.timer.read(addr),
            0xFF0F => self.int_flag.borrow_mut().data,
            0xFF40..=0xFF45 => self.ppu.read(addr),
            0xFF47..=0xFF4B => self.ppu.read(addr),
            0xFF50 => self.rom.disable_boot_rom,
            0xFF80..=0xFFFE => self.hram[(addr as usize) - 0xFF80],
            0xFFFF => self.int_enable,
            _ => 0,
        };

        if addr != 0xFFFF && addr != 0xFF0F {
            // println!("mmc read addr:0x{:x}, ret:0x{:x}", addr, result);
        }

        result
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        // println!("mmc write addr:0x{:x}, dat:0x{:x}", addr, dat);
        match addr {
            0x0000..=0x7FFF => self.rom.write(addr, dat),
            0x8000..=0x9FFF => self.ppu.write(addr, dat),
            0xa000..=0xBFFF => self.rom.write(addr, dat),
            0xC000..=0xCFFF => self.wram[(addr as usize) - 0xC000] = dat,
            0xD000..=0xDFFF => self.wram[(addr as usize) - 0xD000 + (0x1000 * self.bank)] = dat,
            0xE000..=0xEFFF => self.wram[addr as usize - 0xE000] = dat,
            0xF000..=0xFDFF => self.wram[(addr as usize) - 0xF000 + (0x1000 * self.bank)] = dat,
            0xFE00..=0xFE9F => self.ppu.write(addr, dat),
            0xFF00 => self.joypad.write(addr, dat),
            0xFF0F => self.int_flag.borrow_mut().data = dat,
            0xFF40..=0xFF45 => self.ppu.write(addr, dat),
            0xFF46 => self.oam_dma_transfer(dat),
            0xFF47..=0xFF4B => self.ppu.write(addr, dat),
            0xFF50 => self.rom.disable_boot_rom = dat,
            0xFF80..=0xFFFE => self.hram[(addr as usize) - 0xFF80] = dat,
            0xFFFF => self.int_enable = dat,
            _ => {},
        }
    }

    pub fn oam_dma_transfer(&mut self, dat: u8) {
        let start_addr = 0x100 * dat as u16;

        for i in 0..=0x9F {
            let oam_addr = 0xFE00 + i;
            let oam_dat = self.read(start_addr + i);
            self.ppu.write(oam_addr, oam_dat);
        }
    }
}