use std::{cell::RefCell, rc::Rc};
use crate::{defs::{GAMEBOY_WIDTH, GAMEBOY_HEIGHT, Color}, mmc::MMC};

pub struct Output {
    window: minifb::Window,
    mmc: Rc<RefCell<MMC>>,
    joypad_keys: Vec<minifb::Key>,
}

impl Output {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        let mut window_option = minifb::WindowOptions::default();
        window_option.resize = true;
        window_option.scale = minifb::Scale::X2;
        let mut window = minifb::Window::new(
            "deepboy",
            GAMEBOY_WIDTH,
            GAMEBOY_HEIGHT,
            window_option,
        ).unwrap();

        let buffer = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT];
        window.update_with_buffer(buffer.as_slice(), GAMEBOY_WIDTH, GAMEBOY_HEIGHT).unwrap();

        let joypad_keys: Vec<minifb::Key> = vec![
            minifb::Key::Right,
            minifb::Key::Left,
            minifb::Key::Up,
            minifb::Key::Down,
            minifb::Key::A,
            minifb::Key::B,
            minifb::Key::Space,
            minifb::Key::Enter,
        ];

        Output {
            window: window,
            mmc: mmc,
            joypad_keys: joypad_keys,
        }
    }

    pub fn write_screen(&mut self) {
        let mut color_buffer = [[[0x0; 3]; GAMEBOY_WIDTH as usize]; GAMEBOY_HEIGHT as usize];
        for y in 0..GAMEBOY_HEIGHT {
            for x in 0..GAMEBOY_WIDTH {
                let color= self.convert_color(self.mmc.borrow_mut().ppu.frame_buffer[y][x][0]) as u8;
                color_buffer[y][x] = [color, color, color];
            }
        }

        self.mmc.borrow_mut().ppu.reset_buffer();

        let mut screen_buffer = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT];
        let mut i: usize = 0;
        for buffer in color_buffer.iter() {
            for rgb in buffer.iter() {
                let r = (rgb[2] as u32) << 16;
                let g = (rgb[1] as u32) << 8;
                let b = rgb[0] as u32;

                screen_buffer[i] = r | g | b;
                i += 1;
            }
        }

        // screen_buffer = self.debug_screen_out(screen_buffer);
        self.window.update_with_buffer(screen_buffer.as_slice(), GAMEBOY_WIDTH, GAMEBOY_HEIGHT).unwrap();
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

    pub fn handle_keys(&mut self) {
        for key in &self.joypad_keys {
            if self.window.is_key_down(*key) {
                self.mmc.borrow_mut().joypad.key_down(*key);
            } else if self.window.is_key_released(*key) {
                self.mmc.borrow_mut().joypad.key_up(*key);
            }
        }
    }

    pub fn window_is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn debug_screen_out(&self, buf: Vec<u32>) -> Vec<u32>{
        println!("screen_out:");
        for v in buf.iter() {
            println!("{:x}", v);
        }

        return buf;
    }
}