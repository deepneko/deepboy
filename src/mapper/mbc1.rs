use super::Mapper;

pub struct Mbc1 {
    ram: Vec<u8>,
    ram_enable: bool,
    rom_bank: u8,
}

impl Mbc1 {
    pub fn new(ram: Vec<u8>) -> Self {
        Mbc1 {
            ram: ram,
            ram_enable: false,
            rom_bank: 0x01,
        }
    }
}

impl Mapper for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.ram[addr as usize],
            0x4000..=0x7FFF => {
                let offset = 0x4000 * self.rom_bank as usize;
                self.ram[addr as usize - 0x4000 + offset]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let offset = 0x2000 * self.rom_bank as usize;
                    self.ram[addr as usize - 0xA000 + offset]
                } else {
                    0
                }
            }
            _ => panic!("Mbc1 read: Invalid address."),
        }
    }

    fn write(&mut self, addr: u16, dat: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = true,
            0x2000..=0x3FFF => {
                if dat == 0x00 { self.rom_bank = 0x01 }
                if dat == 0x20 { self.rom_bank = 0x21; return }
                if dat == 0x40 { self.rom_bank = 0x41; return }
                if dat == 0x60 { self.rom_bank = 0x61; return }
                self.rom_bank = dat & 0x1F;
            }
            0x4000..=0x5FFF => {
                println!("Not implemented yet")
            }
            0x6000..=0x7FFF => {
                println!("Not implemented yet")
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let offset = 0x2000 * self.rom_bank as usize;
                    self.ram[addr as usize - 0xA000 + offset] = dat;
                }
            }
            _ => panic!("Mbc1 write: Invalid address."),
        }
        self.ram[addr as usize] = dat;
    }
}
