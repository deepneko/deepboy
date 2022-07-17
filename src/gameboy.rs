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

    pub fn load_rom(&mut self, fname: &String) {
        self.rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.rom.load_bootstrap(fname);
    }
}