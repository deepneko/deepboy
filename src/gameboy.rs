use std::fs::File;
use std::io::prelude::*;

pub struct Gameboy {
}

impl Gameboy {
    pub fn load_rom(fname: &String) {
        let mut f = File::open(fname).expect("File not found.");
        let mut buffer: [u8; 0x200000] = [0; 0x200000];
        f.read(&mut buffer);

        let mbc_type: u8 = buffer[0x147];
        let rom_size_type: u8 = buffer[0x148];
        let ram_size_type: u8 = buffer[0x149];

        println!("MBC TYPE:{m}", m=mbc_type);
        println!("ROM SIZE TYPE:{m}", m=rom_size_type);
        println!("RAM SIZE TYPE:{m}", m=ram_size_type);
    }
}