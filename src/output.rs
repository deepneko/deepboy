use std::{cell::RefCell, rc::Rc};
use crate::{defs::{GAMEBOY_WIDTH, GAMEBOY_HEIGHT}, mmc::MMC};

pub struct Output {
    window: minifb::Window,
    mmc: Rc<RefCell<MMC>>,
    joypad_keys: Vec<(minifb::Key, u8)>,
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

        let joypad_keys: Vec<(minifb::Key, u8)> = vec![
            (minifb::Key::Right, 0b0001),
            (minifb::Key::Left, 0b0010),
            (minifb::Key::Up, 0b0100),
            (minifb::Key::Down, 0b1000),
            (minifb::Key::A, 0b0001_0000),
            (minifb::Key::B, 0b0010_0000),
            (minifb::Key::Space, 0b0100_0000),
            (minifb::Key::Enter, 0b1000_0000),
        ];

        Output {
            window: window,
            mmc: mmc,
            joypad_keys: joypad_keys,
        }
    }

    pub fn write_screen(&mut self) {
        let mut screen_buffer = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT];
        let mut i: usize = 0;

        for buffer in self.mmc.borrow_mut().ppu.frame_buffer.iter() {
            for rgb in buffer.iter() {
                let a = 0xff << 24;
                let b = rgb[0] as u32;
                let g = (rgb[1] as u32) << 8;
                let r = (rgb[2] as u32) << 16;

                screen_buffer[i] = a | b | g | r;
                i += 1;
            }
        }

        self.window.update_with_buffer(screen_buffer.as_slice(), GAMEBOY_WIDTH, GAMEBOY_HEIGHT).unwrap();
    }

    pub fn handle_keys(&mut self) {
        for (joypad_key, key) in &self.joypad_keys {
            if self.window.is_key_down(*joypad_key) {
                self.mmc.borrow_mut().joypad.key_down(*key);
            } else {
                self.mmc.borrow_mut().joypad.key_up(*key);
            }
        }
    }
}