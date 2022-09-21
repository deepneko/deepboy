use std::{cell::RefCell, rc::Rc};
use crate::defs::{*, self};
use crate::register::ByteRegister;

pub struct PPU {
    buffer: [[[u8; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize],
    vram: [u8; 0x4000],
    oamram: [u8; 0xa0],
    int_flag: Rc<RefCell<ByteRegister>>,
    lcd_control: ByteRegister,
    lcd_status: ByteRegister,
    scroll_x: u8,
    scroll_y: u8,
    line: u8,
    ly_compare: u8,
    window_x: u8,
    window_y: u8,
    bg_palette: ByteRegister,
    sprite_palette0: ByteRegister,
    sprite_palette1: ByteRegister,
    dma_transfer: u8,
    mode: VideoMode,
    cycles: u32,
}

impl PPU {
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        PPU {
            buffer: [[[0xff; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize],
            vram: [0; 0x4000],
            oamram: [0; 0xa0],
            int_flag: int_flag,
            lcd_control: ByteRegister::new(),
            lcd_status: ByteRegister::new(),
            scroll_x: 0,
            scroll_y: 0,
            line: 0,
            ly_compare: 0,
            window_x: 0,
            window_y: 0,
            bg_palette: ByteRegister::new(),
            sprite_palette0: ByteRegister::new(),
            sprite_palette1: ByteRegister::new(),
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
        // print!("ppu.read {:x}", addr);
        match addr {
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000],
            0xFE00..=0xFE9F => self.oamram[addr as usize - 0xFE00],
            0xFF40 => self.lcd_control.get(),
            0xFF41 => self.lcd_status.get(),
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.line,
            0xFF45 => self.ly_compare,
            0xFF47 => self.bg_palette.get(),
            0xFF48 => self.sprite_palette0.get(),
            0xFF49 => self.sprite_palette1.get(),
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _ => panic!("PPU: Unknown address."),
        }
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        // print!("ppu.write {:x}", addr);
        match addr {
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000] = dat,
            0xFE00..=0xFE9F => self.oamram[addr as usize - 0xFE00] = dat,
            0xFF40 => self.lcd_control.set(dat),
            0xFF41 => self.lcd_status.set(dat),
            0xFF42 => self.scroll_y = dat,
            0xFF43 => self.scroll_x = dat,
            0xFF44 => self.line = dat,
            0xFF45 => self.ly_compare = dat,
            0xFF47 => self.bg_palette.set(dat),
            0xFF48 => self.sprite_palette0.set(dat),
            0xFF49 => self.sprite_palette1.set(dat),
            0xFF4A => self.window_y = dat,
            0xFF4B => self.window_x = dat,
            _ => panic!("PPU: Unknown address."),
        }
    }

    pub fn get_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize - 0x8000]
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
        let palette = self.load_palette(self.bg_palette);
        let mut tile_set_addr: u16 = 0;
        let mut tile_map_addr: u16 = 0;

        if self.bg_window_tile_data() {
            tile_set_addr = 0x8000;
        } else {
            tile_map_addr = 0x8800;
        }

        if !self.bg_tile_map() {
            tile_map_addr = 0x9800;
        } else {
            tile_map_addr = 0x9C00;
        }

        let screen_x: u8 = 0;
        let screen_y: u8 = self.line;

        for screen_x in 0..GAMEBOY_WIDTH {
            let scrolled_x = screen_x + self.scroll_x;
            let scrolled_y = screen_y + self.scroll_y;

            let bg_map_x = scrolled_x % BG_MAP_SIZE;
            let bg_map_y = scrolled_y % BG_MAP_SIZE;

            let tile_x = bg_map_x / TILE_WIDTH;
            let tile_y = bg_map_y / TILE_HEIGHT;

            let tile_pixel_x = bg_map_x % TILE_WIDTH;
            let tile_pixel_y = bg_map_y % TILE_HEIGHT;

            let tile_index = tile_y  * TILES_PER_LINE + tile_x;
            let tile_id_addr: u16 = tile_map_addr + tile_index as u16;

            let tile_id = self.get_vram(tile_id_addr);

            let tile_offset = if self.bg_window_tile_data() {
                i16::from(tile_id)
            } else {
                i16::from(tile_id as i8) + 128
            } as u16
              * 16;

            let tile_line_offset = u16::from(tile_pixel_y) * 2;

            let tile_line_addr = tile_set_addr + tile_offset + tile_line_offset;

            let pixel1 = self.get_vram(tile_line_addr);
            let pixel2 = self.get_vram(tile_line_addr + 1);

            let pixel_color = (pixel2 << (7 - tile_pixel_x)) << 1 | (pixel1 << (7 - tile_pixel_x));

            self.buffer[screen_y as usize][screen_x as usize] = [pixel_color, pixel_color, pixel_color];
        }
    }

    pub fn draw_window(&mut self) {

    }

    pub fn draw_sprite(&mut self, n: u8) {

    }

    pub fn load_palette(&self, palette_reg: ByteRegister) -> [Color; 4] {
        let color0 = palette_reg.get_bit(1) << 1 | palette_reg.get_bit(0); 
        let color1 = palette_reg.get_bit(3) << 1 | palette_reg.get_bit(2); 
        let color2 = palette_reg.get_bit(5) << 1 | palette_reg.get_bit(4); 
        let color3 = palette_reg.get_bit(7) << 1 | palette_reg.get_bit(6); 

        return [ 
            self.convert_color(color0),
            self.convert_color(color1),
            self.convert_color(color2),
            self.convert_color(color3)
        ];
    }

    pub fn convert_color(&self, color: u8) -> Color {
        match color {
            0 => return Color::White,
            1 => return Color::LightGray,
            2 => return Color::DarkGray,
            3 => return Color::Gray,
            _ => panic!("Undefined color."),
        };
    }
}