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
        let mut m = MMC {
            rom: Rom::new(fname),
            ppu: PPU::new(int_flag.clone()),
            joypad: Joypad::new(int_flag.clone()),
            timer: Timer::new(int_flag.clone()),
            wram: [0x00; 0x8000],
            bank: 0x01,
            hram: [0x00; 0x7F],
            int_enable: 0,
            int_flag: int_flag.clone(),
        };
        m.write(0xff05, 0x00);
        m.write(0xff06, 0x00);
        m.write(0xff07, 0x00);
        m.write(0xff10, 0x80);
        m.write(0xff11, 0xbf);
        m.write(0xff12, 0xf3);
        m.write(0xff14, 0xbf);
        m.write(0xff16, 0x3f);
        m.write(0xff16, 0x3f);
        m.write(0xff17, 0x00);
        m.write(0xff19, 0xbf);
        m.write(0xff1a, 0x7f);
        m.write(0xff1b, 0xff);
        m.write(0xff1c, 0x9f);
        m.write(0xff1e, 0xff);
        m.write(0xff20, 0xff);
        m.write(0xff21, 0x00);
        m.write(0xff22, 0x00);
        m.write(0xff23, 0xbf);
        m.write(0xff24, 0x77);
        m.write(0xff25, 0xf3);
        m.write(0xff26, 0xf1);
        m.write(0xff40, 0x91);
        m.write(0xff42, 0x00);
        m.write(0xff43, 0x00);
        m.write(0xff45, 0x00);
        m.write(0xff47, 0xfc);
        m.write(0xff48, 0xff);
        m.write(0xff49, 0xff);
        m.write(0xff4a, 0x00);
        m.write(0xff4b, 0x00);
        m
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
            0xFF04..=0xFF07 => self.timer.write(addr, dat),
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