pub enum VideoMode {
    ACCESS_OAM,
    ACCESS_VRAM,
    HBLANK,
    VBLANK,
}

pub const CLOCKS_PER_HBLANK: u32 = 204;
pub const CLOCKS_PER_SCANLINE_OAM: u32 = 80;
pub const CLOCKS_PER_SCANLINE_VRAM: u32 = 172;
pub const CLOCKS_PER_SCANLINE: u32 = 356;
pub const CLOCKS_PER_VBLANK: u32 = 4560;
pub const SCANLINE_PER_FRAME: u32 = 144;
pub const CLOCKS_PER_FRAME: u32 = (CLOCKS_PER_SCANLINE * SCANLINE_PER_FRAME) + CLOCKS_PER_VBLANK;

pub struct LCDControl {
    lcd_control: u8,
}

impl LCDControl {
    pub fn new() -> Self {
        LCDControl {
            lcd_control: 0,
        }
    }

    pub fn get_bit(&self, bit: u8) -> bool {
        (self.lcd_control >> bit) != 0
    }

    pub fn set_bit(&mut self, bit: u8, b: bool) {
        if b {
            self.lcd_control |= 0x1 << bit;
        } else {
            self.lcd_control &= !(0x1 << bit);
        }
    }
}

pub struct LCDStatus {
    lcd_status: u8,
}

impl LCDStatus {
    pub fn new() -> Self {
        LCDStatus {
            lcd_status: 0,
        }
    }

    pub fn get_bit(&self, bit: u8) -> bool {
        (self.lcd_status >> bit) != 0
    }

    pub fn set_bit(&mut self, bit: u8, b: bool) {
        if b {
            self.lcd_status |= 0x1 << bit;
        } else {
            self.lcd_status &= !(0x1 << bit);
        }
    }
}

pub struct PPU {
    vram: [u8; 0x4000],
    lcd_control: LCDControl,
    lcd_status: LCDStatus,
    scroll_x: u8,
    scroll_y: u8,
    line: u8,
    ly_compare: u8,
    window_x: u8,
    window_y: u8,
    bg_palette: u8,
    sprite_palette_0: u8,
    sprite_palette_1: u8,
    dma_transfer: u8,
    mode: VideoMode,
    cycles: u32,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; 0x4000],
            lcd_control: LCDControl::new(),
            lcd_status: LCDStatus::new(),
            scroll_x: 0,
            scroll_y: 0,
            line: 0,
            ly_compare: 0,
            window_x: 0,
            window_y: 0,
            bg_palette: 0,
            sprite_palette_0: 0,
            sprite_palette_1: 0,
            dma_transfer: 0,
            mode: VideoMode::ACCESS_OAM,
            cycles: 0,
        }
    }

    pub fn run(&mut self, cycles: u32) {
        self.cycles += cycles;

        match self.mode {
            VideoMode::ACCESS_OAM => {
                if self.cycles >= CLOCKS_PER_SCANLINE_OAM {
                    self.cycles %= CLOCKS_PER_SCANLINE_OAM;
                    self.lcd_status.set_bit(1, true);
                    self.lcd_status.set_bit(0, true);
                    self.mode = VideoMode::ACCESS_VRAM;
                }
            }

            VideoMode::ACCESS_VRAM => {
            }

            VideoMode::HBLANK => {
            }

            VideoMode::VBLANK => {
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.vram[addr as usize] = dat;
    }

    pub fn write_scanline(&mut self, line: u8) {

    }

    pub fn wirte_sprites(&mut self) {
        
    }
}