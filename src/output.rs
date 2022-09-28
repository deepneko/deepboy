use std::{cell::RefCell, rc::Rc};
use crate::{defs::{GAMEBOY_WIDTH, GAMEBOY_HEIGHT}, mmc::MMC};

pub struct Output {
    window: minifb::Window,
    mmc: Rc<RefCell<MMC>>,
}

impl Output {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        let mut window = minifb::Window::new(
            "deepboy",
            GAMEBOY_WIDTH,
            GAMEBOY_HEIGHT,
            minifb::WindowOptions::default(),
        ).unwrap();

        let buffer = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT];
        window.update_with_buffer(buffer.as_slice(), GAMEBOY_WIDTH, GAMEBOY_HEIGHT).unwrap();

        Output {
            window: window,
            mmc: mmc,
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
}