use std::{cell::RefCell, rc::Rc};
use crate::defs::*;
use crate::register::ByteRegister;

pub struct PPU {
    pub frame_buffer: [[[u8; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize],
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
    mode: VideoMode,
    pub v_blank: bool,
    cycles: u32,
    debug: bool,
}

impl PPU {
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        PPU {
            frame_buffer: [[[0xff; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize],
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
            mode: VideoMode::ACCESS_OAM,
            v_blank: false,
            cycles: 0,
            debug: false,
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn run(&mut self, cycles: u32) {
        self.cycles += cycles;
        if self.debug {
            println!("ppu.mode: {}", self.mode as usize);
            println!("ppu.line: {}", self.line);
            println!("ppu.cycles: {}", self.cycles);
        }

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
                        self.v_blank = true;
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
                if self.cycles >= CLOCKS_PER_SCANLINE {
                    self.line += 1;

                    self.cycles %= CLOCKS_PER_SCANLINE;

                    if self.line == 154 {
                        self.render_sprites();
                        self.line = 0;
                        self.mode = VideoMode::ACCESS_OAM;
                        self.lcd_status.set_bit(1, true);
                        self.lcd_status.set_bit(0, false);
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
        let mut result: u8 = 0;
        match addr {
            0x8000..=0x9FFF => result = self.vram[addr as usize - 0x8000],
            0xFE00..=0xFE9F => result = self.oamram[addr as usize - 0xFE00],
            0xFF40 => result = self.lcd_control.get(),
            0xFF41 => result = self.lcd_status.get(),
            0xFF42 => result = self.scroll_y,
            0xFF43 => result = self.scroll_x,
            0xFF44 => result = self.line,
            0xFF45 => result = self.ly_compare,
            0xFF47 => result = self.bg_palette.get(),
            0xFF48 => result = self.sprite_palette0.get(),
            0xFF49 => result = self.sprite_palette1.get(),
            0xFF4A => result = self.window_y,
            0xFF4B => result = self.window_x,
            _ => panic!("PPU: Unknown address."),
        }

        if self.debug {
            println!("ppu.read {:x} {:x}", addr, result);
        }

        result
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        if self.debug {
            println!("ppu.write {:x} {:x}", addr, dat);
        }

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

        let screen_y: u16 = self.line as u16;
        (0..GAMEBOY_WIDTH as u16).for_each(|screen_x| {
            let scrolled_x = screen_x + self.scroll_x as u16;
            let scrolled_y = screen_y + self.scroll_y as u16;

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
            let pixel_color = (((pixel2 >> (7 - tile_pixel_x)) & 1) << 1) | ((pixel1 >> (7 - tile_pixel_x) & 1));
            let real_color = self.convert_color(palette[pixel_color as usize]) as u8;
            self.frame_buffer[screen_y as usize][screen_x as usize] = [real_color, real_color, real_color];

            if self.debug {
                println!("scrolled_x:{:x} scrolled_y:{:x}", scrolled_x, scrolled_y);
                println!("bg_map_x:{:x} bg_map_y:{:x}", bg_map_x, bg_map_y);
                println!("tile_index:{:x} tile_id_addr:{:x}", tile_index, tile_id_addr);
                println!("lcd_control:{:x} lcd_status:{:x}", self.lcd_control.get(), self.lcd_status.get());
                println!("ppu.read {:x} {:x}", tile_id_addr, tile_id);
                println!("ppu.read {:x} {:x}", tile_line_addr, pixel1);
                println!("ppu.read {:x} {:x}", tile_line_addr+1, pixel2);
                println!("pixel_color:{:x}", pixel_color);
            }
        });
    }

    pub fn draw_window(&mut self) {

    }

    pub fn draw_sprite(&mut self, n: u8) {

    }

    pub fn load_palette(&self, palette_reg: ByteRegister) -> [u8; 4] {
        let color0 = palette_reg.get_bit(1) << 1 | palette_reg.get_bit(0); 
        let color1 = palette_reg.get_bit(3) << 1 | palette_reg.get_bit(2); 
        let color2 = palette_reg.get_bit(5) << 1 | palette_reg.get_bit(4); 
        let color3 = palette_reg.get_bit(7) << 1 | palette_reg.get_bit(6); 

        return [color0, color1, color2, color3];
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