pub const GAMEBOY_WIDTH: usize = 160;
pub const GAMEBOY_HEIGHT: usize = 144;
pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 576;
pub const PIXEL_AREA_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const TILES_PER_LINE: u16 = 32;
pub const TILE_HEIGHT: u16 = 8;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_BYTES: u16 = 16;
pub const BG_MAP_SIZE: u16 = 255; // It is actually 256 but not a problem.
pub const NUM_SPRITES: u32 = 40;

pub const CLOCKS_PER_HBLANK: u32 = 204;
pub const CLOCKS_PER_SCANLINE_OAM: u32 = 80;
pub const CLOCKS_PER_SCANLINE_VRAM: u32 = 172;
pub const CLOCKS_PER_SCANLINE: u32 = CLOCKS_PER_HBLANK + CLOCKS_PER_SCANLINE_OAM + CLOCKS_PER_SCANLINE_VRAM;
pub const CLOCKS_PER_VBLANK: u32 = 4560;
pub const SCANLINE_PER_FRAME: u32 = 144;
pub const CLOCKS_PER_FRAME: u32 = (CLOCKS_PER_SCANLINE * SCANLINE_PER_FRAME) + CLOCKS_PER_VBLANK;

pub const CLOCK_RATE:i32 = 4194304;

#[allow(non_camel_case_types)]
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