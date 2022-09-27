use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT, PIXEL_AREA_SIZE};

pub struct Output {
}

impl Output {
    pub fn new() -> Self {
        let mut window = minifb::Window::new(
            "deepboy",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            minifb::WindowOptions::default(),
        );

        Output {
        }
    }

    pub fn write_screen(&mut self) {
    }

    pub fn event_handling(&self) {
    }
}