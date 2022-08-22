pub struct PPU {
    ram: [u8; 0x4000],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            ram: [0; 0x4000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.ram[addr as usize] = dat;
    }

    pub fn write_scanline(&mut self, line: u8) {

    }

    pub fn wirte_sprites(&mut self) {
        
    }
}