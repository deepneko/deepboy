use crate::defs::BankMode;
use super::Mapper;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enable: bool,
    rom_bank: u8,
    bank_mode: BankMode,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self {
        Mbc1 {
            rom: rom,
            ram: ram,
            ram_enable: false,
            rom_bank: 0x01,
            bank_mode: BankMode::Rom,
        }
    }

    pub fn rom_bank(&self) -> usize {
        let bank = match self.bank_mode {
            BankMode::Rom => self.rom_bank & 0x7f,
            BankMode::Ram => self.rom_bank & 0x1f,
        };
        bank as usize
    }

    pub fn ram_bank(&self) -> usize {
        let bank = match self.bank_mode {
            BankMode::Rom => 0,
            BankMode::Ram => (self.rom_bank & 0x60) >> 5,
        };
        bank as usize
    }
}

impl Mapper for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let offset = 0x4000 * self.rom_bank() as usize;
                self.rom[addr as usize - 0x4000 + offset]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let offset = 0x2000 * self.ram_bank() as usize;
                    self.ram[addr as usize - 0xA000 + offset]
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, dat: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = (dat & 0x0f) == 0x0a,
            0x2000..=0x3FFF => {
                if dat == 0x00 { self.rom_bank = 0x01 }
                if dat == 0x20 { self.rom_bank = 0x21; return }
                if dat == 0x40 { self.rom_bank = 0x41; return }
                if dat == 0x60 { self.rom_bank = 0x61; return }
                self.rom_bank = dat & 0x1F;
            }
            0x4000..=0x5FFF => {
                self.rom_bank = self.rom_bank & 0x9f | ((dat & 0x03) << 5);
            }
            0x6000..=0x7FFF => {
                match dat {
                    0x00 => self.bank_mode = BankMode::Rom,
                    0x01 => self.bank_mode = BankMode::Ram,
                    n => panic!("Invalid bank_mode. {}", n),
                };
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let offset = 0x2000 * self.ram_bank() as usize;
                    self.ram[addr as usize - 0xA000 + offset] = dat;
                }
            }
            _ => {},
        }
    }
}
