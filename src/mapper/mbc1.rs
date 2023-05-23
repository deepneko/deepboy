use crate::defs::BankMode;
use super::Mapper;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enable: bool,
    rom_bank: u8,
    ram_bank: u8,
    bank_mode: BankMode,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self {
        Mbc1 {
            rom: rom,
            ram: ram,
            ram_enable: false,
            rom_bank: 0x01,
            ram_bank: 0,
            bank_mode: BankMode::Rom,
        }
    }
}

impl Mapper for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        // println!("MBC1 read addr:{:x}, ram_enable:{}", addr, self.ram_enable);
        match addr {
            0x0000..=0x3FFF => {
                match self.bank_mode {
                    BankMode::Rom => self.rom[addr as usize],
                    BankMode::Ram => {
                        let offset = 0x4000 * ((self.ram_bank << 5) + self.rom_bank) as usize;
                        self.rom[addr as usize + offset]
                    }
                }
            }
            0x4000..=0x7FFF => {
                let offset = 0x4000 * self.rom_bank as usize;
                self.rom[(addr as usize - 0x4000 + offset)]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let offset = 0x2000 * self.ram_bank as usize;
                    self.ram[addr as usize - 0xA000 + offset]
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, dat: u8) {
        // println!("MBC1 write addr:{:x}, dat:{:x}", addr, dat);
        match addr {
            0x0000..=0x1FFF => {
                if dat & 0xf == 0x00 { self.ram_enable = false }
                if dat & 0xf == 0x0a { self.ram_enable = true }
            }
            0x2000..=0x3FFF => {
                if dat == 0x00 { self.rom_bank = 0x01 }
                if dat == 0x20 { self.rom_bank = 0x21; return }
                if dat == 0x40 { self.rom_bank = 0x41; return }
                if dat == 0x60 { self.rom_bank = 0x61; return }
                self.rom_bank = dat & 0x1F;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = dat & 0x03;
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
                    let offset = 0x2000 * self.ram_bank as usize;
                    self.ram[addr as usize - 0xA000 + offset] = dat;
                }
            }
            _ => {},
        }
    }
}
