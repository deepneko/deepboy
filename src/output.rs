use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Output {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Output {
    const SCREEN_W: u32 = 640;
    const SCREEN_H: u32 = 576;

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let window = video.window("deepboy", Self::SCREEN_W, Self::SCREEN_H)
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        Output {
            sdl_context: sdl_context,
            canvas: canvas,
        }
    }

    pub fn write_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(100, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}