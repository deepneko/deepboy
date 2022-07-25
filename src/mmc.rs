use super::gameboy::Gameboy;

pub struct MMC {
    pub gameboy: Gameboy,
}

impl MMC {
    pub fn new(gb: Gameboy) -> Self {
        MMC {
            gameboy: gb,
        }
    }

    pub fn read(&mut self, addr: usize) {
        if addr < 256 {
            self.gameboy.rom.boot_rom[addr];
        } else if addr < 0x8000 {
            self.gameboy.rom.ram[addr];
        }
    }
}