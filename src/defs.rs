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
pub const SPRITE_BYTRES: u16 = 4;
pub const NUM_SPRITES: u16 = 40;

pub const CLOCKS_PER_HBLANK: u32 = 204;
pub const CLOCKS_PER_SCANLINE_OAM: u32 = 80;
pub const CLOCKS_PER_SCANLINE_VRAM: u32 = 172;
pub const CLOCKS_PER_SCANLINE: u32 = CLOCKS_PER_HBLANK + CLOCKS_PER_SCANLINE_OAM + CLOCKS_PER_SCANLINE_VRAM;
pub const CLOCKS_PER_VBLANK: u32 = 4560;
pub const SCANLINE_PER_FRAME: u32 = 144;
pub const CLOCKS_PER_FRAME: u32 = (CLOCKS_PER_SCANLINE * SCANLINE_PER_FRAME) + CLOCKS_PER_VBLANK;

pub const CLOCK_RATE:i32 = 4194304;
pub const STEP_TIME: u32 = 16;
pub const STEP_CYCLES: u32 = (STEP_TIME as f64 / (1000_f64 / CLOCK_RATE as f64)) as u32;

#[allow(non_camel_case_types)]
pub enum VideoMode {
    HBLANK = 0,
    VBLANK = 1,
    ACCESS_OAM = 2,
    ACCESS_VRAM = 3,
}

#[derive(Clone)]
pub enum Color {
    White = 0xff,
    LightGray = 0xc0,
    DarkGray = 0x60,
    Gray = 0x00,
}

pub enum BankMode {
    Rom,
    Ram,
}