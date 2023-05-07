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
            frame_buffer: [[[0x0; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize],
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
            println!("ppu.line: {:x}", self.line);
            println!("ppu.cycles: {:x}", self.cycles);
        }

        match self.mode {
            VideoMode::ACCESS_OAM => {
                if self.debug {
                    println!("ppu.mode: OAM");
                }
                if self.cycles >= CLOCKS_PER_SCANLINE_OAM {
                    self.cycles %= CLOCKS_PER_SCANLINE_OAM;
                    self.lcd_status.set_bit(1, true);
                    self.lcd_status.set_bit(0, true);
                    self.mode = VideoMode::ACCESS_VRAM;
                }
            }

            VideoMode::ACCESS_VRAM => {
                if self.debug {
                    println!("ppu.mode: VRAM");
                }
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
                if self.debug {
                    println!("ppu.mode: HBLANK");
                }
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
                if self.debug {
                    println!("ppu.mode: VBLANK");
                }
                if self.cycles >= CLOCKS_PER_SCANLINE {
                    self.line += 1;

                    self.cycles %= CLOCKS_PER_SCANLINE;

                    if self.line == 154 {
                        self.render_sprites();
                        self.v_blank = true;
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
            0xFF40 => {
                self.lcd_control.set(dat);
                if !self.lcd_enabled() {
                    self.reset_buffer();
                    self.v_blank = true;
                }
            }
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

    pub fn reset_buffer(&mut self) {
        self.frame_buffer = [[[0x0; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize];
    }

    pub fn get_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize - 0x8000]
    }

    pub fn render_scanline(&mut self) {
        if self.debug {
            println!("render_scanline bg_enabled:{} window_enabled:{}", self.bg_enabled(), self.window_enabled());
        }

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

        for n in 0..NUM_SPRITES {
            self.draw_sprite(n);
        }
    }

    pub fn draw_bg(&mut self) {
        let palette = self.load_palette(self.bg_palette);

        let tile_set_addr: u16 = if self.bg_window_tile_data() {
            0x8000
        } else {
            0x8800
        };

        let tile_map_addr: u16 = if !self.bg_tile_map() {
            0x9800
        } else {
            0x9C00
        };

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
            let real_color = palette[pixel_color as usize];

            self.frame_buffer[screen_y as usize][screen_x as usize] = [real_color, real_color, real_color];

            if self.debug {
                println!("screen_x:{:x} screen_y:{:x}", screen_x, screen_y);
                println!("scrolled_x:{:x} scrolled_y:{:x}", scrolled_x, scrolled_y);
                println!("bg_map_x:{:x} bg_map_y:{:x}", bg_map_x, bg_map_y);
                println!("lcd_control:{:x} lcd_status:{:x}", self.lcd_control.get(), self.lcd_status.get());
                println!("tile_index:{:x} tile_id_addr:{:x}", tile_index, tile_id_addr);
                println!("tile_id:{:x}", tile_id);
                println!("tile_pixel_y:{:x}", tile_pixel_y);
                println!("tile_set_addr:{:x}", tile_set_addr);
                println!("tile_offset:{:x}", tile_offset);
                println!("tile_line_offset:{:x}", tile_line_offset);
                println!("tile_line_addr:{:x}", tile_line_addr);
                println!("real_color:{:x}", real_color);
            }
        });

        if self.debug {
            self.debug_frame_out("draw_bg");
        }
    }

    pub fn draw_window(&mut self) {
        let palette = self.load_palette(self.bg_palette);

        let tile_set_addr: u16 = if self.bg_window_tile_data() {
            0x8000
        } else {
            0x8800
        };

        let tile_map_addr: u16 = if !self.window_tile_map() {
            0x9800
        } else {
            0x9C00
        };

        let screen_y: u16 = self.line as u16;
        let scrolled_y: u16 = screen_y.wrapping_sub(self.window_y as u16);
        if scrolled_y > GAMEBOY_HEIGHT as u16 { return; }

        (0..GAMEBOY_WIDTH as u16).for_each(|screen_x| {
            let scrolled_x = screen_x + self.window_x as u16 - 7;

            let tile_x = scrolled_x / TILE_WIDTH;
            let tile_y = scrolled_y / TILE_HEIGHT;

            let tile_pixel_x = scrolled_x % TILE_WIDTH;
            let tile_pixel_y = scrolled_y % TILE_HEIGHT;

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
            let real_color = palette[pixel_color as usize];

            self.frame_buffer[screen_y as usize][screen_x as usize] = [real_color, real_color, real_color];
        });

        if self.debug {
            self.debug_frame_out("draw_window");
        }
    }

    pub fn draw_sprite(&mut self, n: u16) {
        let oam_offset = n * SPRITE_BYTRES;
        let sprite_y = self.oamram[oam_offset as usize] as u16;
        let sprite_x = self.oamram[oam_offset as usize + 1] as u16;

        if sprite_y == 0 || sprite_y >= 160 { return; }
        if sprite_x == 0 || sprite_x >= 168 { return; }

        let sprite_size = if self.sprite_size() {
            2
        } else {
            1
        };

        let tile_location = 0x8000;

        let pattern = self.oamram[oam_offset as usize + 2];
        let sprite_attr = self.oamram[oam_offset as usize + 3];

        let palette = if sprite_attr >> 4 & 0x1 != 0 {
            self.load_palette(self.sprite_palette1)
        } else {
            self.load_palette(self.sprite_palette0)
        };

        let flip_x: bool = sprite_attr >> 5 & 0x1 != 0;
        let flip_y: bool = sprite_attr >> 6 & 0x1 != 0;
        let behind_bg: bool = sprite_attr >> 7 & 0x1 != 0;

        let tile_offset = pattern as u16 * TILE_BYTES;

        let pattern_addr = tile_location + tile_offset;

        if self.debug {
            println!("oam_offset:{:x}", oam_offset);
            println!("sprite_y:{:x}, sprite_x:{:x}", sprite_y, sprite_x);
            self.debug_oam_out();
            println!("sprite_y:{:x}, sprite_x:{:x}", sprite_y, sprite_x);
            println!("sprite_size:{:x}", sprite_size);
            println!("tile_location:{:x}", tile_location);
            println!("pattern:{:x}", pattern);
            println!("sprite_attr:{:x}", sprite_attr);
            println!("flip_y:{}, flip_x:{}", flip_y, flip_x);
            println!("behind_bg:{}", behind_bg);
            println!("tile_offset:{:x}", tile_offset);
            println!("pattern_addr:{:x}", pattern_addr);
        }

        /* Create Tile */
        let mut tile_buffer: [u8; (TILE_HEIGHT * 2 * TILE_WIDTH) as usize] = [0; (TILE_HEIGHT * 2 * TILE_WIDTH) as usize];
        (0..TILE_HEIGHT).for_each(|x: u16|{
            (0..(sprite_size * TILE_HEIGHT)).for_each(|y: u16|{
                tile_buffer[(y * TILE_HEIGHT + x) as usize] = 0;
            });
        });

        (0..(TILE_HEIGHT * sprite_size)).for_each(|tile_line: u16|{
            let index = tile_line * 2;
            let start = pattern_addr + index;

            let pixel1 = self.get_vram(start);
            let pixel2 = self.get_vram(start + 1);

            let mut pixel_line: Vec<u8> = Vec::new();
            (0..8).for_each(|i: u8|{
                let pixel_color = (((pixel2 >> (7 - i)) & 1) << 1) | ((pixel1 >> (7 - i) & 1));
                pixel_line.push(pixel_color);
            });

            (0..TILE_WIDTH).for_each(|x: u16|{
                tile_buffer[(tile_line * TILE_HEIGHT + x) as usize] = pixel_line[x as usize];
            });
        });

        if self.debug {
            self.debug_tile_out(&tile_buffer);
        }
        /* Create Tile done */

        let start_y = sprite_y.wrapping_sub(16);
        let start_x = sprite_x.wrapping_sub(8);
        let white = [0, 0, 0];

        for y in 0..(sprite_size * TILE_HEIGHT) {
            for x in 0..TILE_WIDTH {
                let flipped_y = if !flip_y { y } else { sprite_size * TILE_HEIGHT - y - 1 };
                let flipped_x = if !flip_x { x } else { TILE_WIDTH - x - 1 };

                let pixel_color = tile_buffer[(flipped_y * TILE_HEIGHT + flipped_x) as usize];
                if self.debug {
                    println!("flipped_y:{:x}, flipped_x:{:x}", flipped_y, flipped_x);
                }

                if pixel_color == 0 {
                    continue;
                }

                let screen_x = start_x.wrapping_add(x);
                let screen_y = start_y.wrapping_add(y);
                if self.debug {
                    println!("screen_x:{:x}, screen_y:{:x}", screen_x, screen_y);
                }
                if(screen_x >= GAMEBOY_WIDTH as u16 || screen_y >= GAMEBOY_HEIGHT as u16) {
                    continue;
                }

                if behind_bg {
                    if self.frame_buffer[screen_y as usize][screen_x as usize] != white {
                        continue;
                    }
                }

                let real_color = palette[pixel_color as usize];
                if self.debug {
                    println!("real_color:{:x}", real_color);
                }
                self.frame_buffer[screen_y as usize][screen_x as usize] = [real_color, real_color, real_color];
            }
        }

        if self.debug {
            self.debug_frame_out("draw_sprite");
        }
    }

    pub fn load_palette(&self, palette_reg: ByteRegister) -> [u8; 4] {
        let color0 = palette_reg.get_bit(1) << 1 | palette_reg.get_bit(0); 
        let color1 = palette_reg.get_bit(3) << 1 | palette_reg.get_bit(2); 
        let color2 = palette_reg.get_bit(5) << 1 | palette_reg.get_bit(4); 
        let color3 = palette_reg.get_bit(7) << 1 | palette_reg.get_bit(6); 

        return [color0, color1, color2, color3];
    }

    pub fn debug_oam_out(&self) {
        print!("oam_out:");
        for v in self.oamram.iter() {
            print!("{:x}", v);
        }
        println!("");
    }

    pub fn debug_tile_out(&self, tile: &[u8]) {
        print!("tile_out:");
        for v in tile.iter() {
            print!("{:x}", v);
        }
        println!("");
    }
    
    pub fn debug_frame_out(&self, s: &str) {
        println!("{}:", s);
        for y in 0..GAMEBOY_HEIGHT {
            for x in 0..GAMEBOY_WIDTH {
                println!("y:{:x}, x:{:x}, color:{:x}", y, x, self.frame_buffer[y][x][0]);
            }
        }
    }
}