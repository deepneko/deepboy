use super::rom::Rom;

pub struct Gameboy {
    rom: Rom,
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            rom: Rom::new(),
        }
    }

    pub fn load(&mut self, rom_name: &String) {
        self.rom.load(rom_name);
    }
}