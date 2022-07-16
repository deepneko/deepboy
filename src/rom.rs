use std::fs::File;
use std::io::prelude::*;

pub struct Rom {
    pub mbc_type: u8,
    pub rom_size_type: u8,
    pub ram_size_type: u8,
    pub ram: Vec<u8>,
    pub ram_size: u32,
}

impl Rom {
    pub fn new() -> Self {
        Rom {
            mbc_type: 0,
            rom_size_type: 0,
            ram_size_type: 0,
            ram: Vec::new(),
            ram_size: 0,
        }
    }

    pub fn load(&mut self, fname: &String) {
        let mut f = File::open(fname).expect("File not found.");
        let mut buffer: [u8; 0x200000] = [0; 0x200000];
        f.read(&mut buffer);

        self.mbc_type = buffer[0x147];
        self.rom_size_type = buffer[0x148];
        self.ram_size_type = buffer[0x149];

        println!("MBC TYPE:{m}", m=self.mbc_type);
        println!("ROM SIZE TYPE:{m}", m=self.rom_size_type);
        println!("RAM SIZE TYPE:{m}", m=self.ram_size_type);

        match self.mbc_type {
            0 => {println!("NO MBC")},
            1 => {println!("MBC1")},
            3 => {println!("MBC3")},
            _ => {println!("Invalid MBC TYPE")}
        }

        match self.ram_size_type {
            0 => self.ram_size = 0x200,
            1 => self.ram_size = 0,
            3 => self.ram_size = 0x8000,
            _ => println!("Invalid RAM SIZE TYPE")
        }
    }
}