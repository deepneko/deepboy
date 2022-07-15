use std::fs::File;
use std::io::prelude::*;

pub struct Gameboy {
    mbc_type: u8,
    rom_size_type: u8,
    ram_size_type: u8,
}

impl Gameboy {
    pub fn load_rom(&mut self, fname: &String) {
        let mut f = File::open(fname).expect("File not found.");
        let mut buffer: [u8; 0x200000] = [0; 0x200000];
        f.read(&mut buffer);

        self.mbc_type = buffer[0x147];
        self.rom_size_type = buffer[0x148];
        self.ram_size_type = buffer[0x149];

        println!("MBC TYPE:{m}", m=self.mbc_type);
        println!("ROM SIZE TYPE:{m}", m=self.rom_size_type);
        println!("RAM SIZE TYPE:{m}", m=self.ram_size_type);
    }
}