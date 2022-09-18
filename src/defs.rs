pub const GAMEBOY_WIDTH: u8 = 160;
pub const GAMEBOY_HEIGHT: u8 = 144;
pub const TILES_PER_LINE: u8 = 32;
pub const TILE_HEIGHT: u8 = 8;
pub const TILE_WIDTH: u8 = 8;
pub const TILE_BYTES: u8 = 16;
pub const BG_MAP_SIZE: u8 = 255; // It is actually 256 but not a problem.
pub const NUM_SPRITES: u32 = 40;

pub const CLOCKS_PER_HBLANK: u32 = 204;
pub const CLOCKS_PER_SCANLINE_OAM: u32 = 80;
pub const CLOCKS_PER_SCANLINE_VRAM: u32 = 172;
pub const CLOCKS_PER_SCANLINE: u32 = 356;
pub const CLOCKS_PER_VBLANK: u32 = 4560;
pub const SCANLINE_PER_FRAME: u32 = 144;
pub const CLOCKS_PER_FRAME: u32 = (CLOCKS_PER_SCANLINE * SCANLINE_PER_FRAME) + CLOCKS_PER_VBLANK;

pub const CLOCK_RATE:i32 = 4194304;

pub enum VideoMode {
    ACCESS_OAM,
    ACCESS_VRAM,
    HBLANK,
    VBLANK,
}

#[derive(Clone)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Gray,
}

/*
pub struct Palette {
    pub color0: Color,
    pub color1: Color,
    pub color2: Color,
    pub color3: Color,
}

impl Palette {
    pub fn new(c0: Color, c1: Color, c2: Color, c3: Color) -> Self {
        Palette {
            color0: c0,
            color1: c1,
            color2: c2,
            color3: c3,
        }
    }
}
*/