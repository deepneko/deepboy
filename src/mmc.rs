use super::rom::Rom;
use super::ppu::PPU;

pub struct MMC {
    pub rom: Rom, 
    pub ppu: PPU,
}

impl MMC {
    pub fn new() -> Self {
        MMC {
            rom: Rom::new(),
            ppu: PPU::new(),
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
            0xa000..=0xBFFF => self.rom.read(addr),
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        match addr {
            0x0000..=0x7FFF => self.rom.write(addr, dat),
            0x8000..=0x9FFF => self.ppu.write(addr, dat),
            0xa000..=0xBFFF => self.rom.write(addr, dat),
            _ => {},
        }
    }
}