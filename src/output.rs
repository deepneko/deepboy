use std::{ptr::null_mut, ffi::c_void};
use sdl2::{sys::*, render::SdlError, get_error};

use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Output {
    _window: *mut SDL_Window,
    _renderer: *mut SDL_Renderer,
    _screen_texture: *mut SDL_Texture,
}

impl Output {
    pub fn new() -> Self {
        let mut window: *mut SDL_Window = null_mut();
        let mut renderer: *mut SDL_Renderer = null_mut();
        let mut screen_texture: *mut SDL_Texture = null_mut();

        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                panic!("Failed to initialize sdl2.");
            }

            window = SDL_CreateWindow(
                "deepboy" as *const _ as *const i8,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                SDL_WindowFlags::SDL_WINDOW_OPENGL as u32
            );
            if window.is_null() {
                panic!("Failed to create window.");
            }

            renderer = SDL_CreateRenderer(
                window,
                -1,
                SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32 | SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32,
            );
            if renderer.is_null() {
                panic!("Failed to create renderer.");
            }

            screen_texture = SDL_CreateTexture(
                renderer,
                SDL_PixelFormatEnum::SDL_PIXELFORMAT_ABGR8888 as u32,
                SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32
            );
            if screen_texture.is_null() {
                panic!("Failed to create texture.");
            }
        }

        Output {
            _window: window,
            _renderer: renderer,
            _screen_texture: screen_texture,
        }
    }

    pub fn write_screen(&mut self) {
        self.event_handling();

        unsafe {
            SDL_RenderClear(self._renderer);

            let mut pixels = null_mut();
            let mut pitch = 0;
            SDL_LockTexture(self._screen_texture, null_mut(), &mut pixels, &mut pitch);

            SDL_UnlockTexture(self._screen_texture);

            SDL_RenderCopy(self._renderer, self._screen_texture, null_mut(), null_mut());
            SDL_RenderPresent(self._renderer);
        }
    }

    pub fn event_handling(&self) {
        unsafe {
            let mut event: *mut SDL_Event = null_mut();
            while SDL_PollEvent(event) > 0 {
                match (*event).type_ {
                    SDL_QuitEvent => { SDL_Quit() },
                    _ => {},
                }
            }
        }
    }
}