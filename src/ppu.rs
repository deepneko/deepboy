use std::{cell::RefCell, rc::Rc};

use crate::register::ByteRegister;

pub enum VideoMode {
    ACCESS_OAM,
    ACCESS_VRAM,
    HBLANK,
    VBLANK,
}

pub const GAMEBOY_WIDTH: u32 = 160;
pub const GAMEBOY_HEIGHT: u32 = 144;
pub const NUM_SPRITES: u32 = 40;
pub const CLOCKS_PER_HBLANK: u32 = 204;
pub const CLOCKS_PER_SCANLINE_OAM: u32 = 80;
pub const CLOCKS_PER_SCANLINE_VRAM: u32 = 172;
pub const CLOCKS_PER_SCANLINE: u32 = 356;
pub const CLOCKS_PER_VBLANK: u32 = 4560;
pub const SCANLINE_PER_FRAME: u32 = 144;
pub const CLOCKS_PER_FRAME: u32 = (CLOCKS_PER_SCANLINE * SCANLINE_PER_FRAME) + CLOCKS_PER_VBLANK;

pub struct PPU {
    vram: [u8; 0x4000],
    int_flag: Rc<RefCell<ByteRegister>>,
    lcd_control: ByteRegister,
    lcd_status: ByteRegister,
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
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        PPU {
            vram: [0; 0x4000],
            int_flag: int_flag,
            lcd_control: ByteRegister::new(),
            lcd_status: ByteRegister::new(),
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
                if self.cycles >= CLOCKS_PER_SCANLINE_VRAM {
                    self.cycles %= CLOCKS_PER_SCANLINE_VRAM;
                    self.mode = VideoMode::HBLANK;

                    if self.lcd_status.check_bit(3) {
                        self.int_flag.borrow_mut().set_bit(1, true);
                    }

                    let ly_coincidence = self.ly_compare == self.line;
                    if self.lcd_status.check_bit(6) && ly_coincidence {
                        self.int_flag.borrow_mut().set_bit(1, true);
                    }

                    self.lcd_status.set_bit(2, ly_coincidence);
                    self.lcd_status.set_bit(1, false);
                    self.lcd_status.set_bit(0, false);
                }
            }

            VideoMode::HBLANK => {
                if self.cycles >= CLOCKS_PER_HBLANK {
                    self.render_scanline();
                    self.line += 1;

                    self.cycles %= CLOCKS_PER_HBLANK;

                    if self.line == 144 {
                        self.mode = VideoMode::VBLANK;
                        self.lcd_status.set_bit(1, false);
                        self.lcd_status.set_bit(0, true);
                        self.int_flag.borrow_mut().set_bit(0, true);
                    } else {
                        self.lcd_status.set_bit(1, true);
                        self.lcd_status.set_bit(0, false);
                        self.mode = VideoMode::ACCESS_OAM;
                    }
                }
            }

            VideoMode::VBLANK => {
                if self.cycles >= CLOCKS_PER_VBLANK {
                    self.line += 1;

                    self.cycles %= CLOCKS_PER_VBLANK;

                    if self.line == 154 {
                        self.render_sprites();
                    }
                }
            }
        }
    }

    pub fn lcd_enabled(&self) -> bool { self.lcd_control.check_bit(7) }
    pub fn window_tile_map(&self) -> bool { self.lcd_control.check_bit(6) }
    pub fn window_enabled(&self) -> bool { self.lcd_control.check_bit(5) }
    pub fn bg_window_tile_data(&self) -> bool { self.lcd_control.check_bit(4) }
    pub fn bg_tile_map(&self) -> bool { self.lcd_control.check_bit(3) }
    pub fn sprite_size(&self) -> bool { self.lcd_control.check_bit(2) }
    pub fn sprite_enabled(&self) -> bool { self.lcd_control.check_bit(1) }
    pub fn bg_enabled(&self) -> bool { self.lcd_control.check_bit(0) }

    pub fn read(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.vram[addr as usize] = dat;
    }

    pub fn render_scanline(&mut self) {
        if !self.lcd_enabled() {
            return;
        }

        if self.bg_enabled() {
            self.draw_bg();
        }

        if self.window_enabled() {
            self.draw_window();
        }
    }

    pub fn render_sprites(&mut self) {
        if !self.sprite_enabled() {
            return;
        }

        for n in 0..40 {
            self.draw_sprite(n);
        }
    }

    pub fn draw_bg(&mut self) {
    }

    pub fn draw_window(&mut self) {

    }

    pub fn draw_sprite(&mut self, n: u8) {

    }
}