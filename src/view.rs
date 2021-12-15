use minifb::{Scale, Window, WindowOptions};

use crate::io::{InvadersIOHandler, InvadersKey};

const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;

pub struct InvadersView {
    window: Window,
    buf: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl InvadersView {
    pub fn new() -> Self {
        let mut options = WindowOptions::default();
        options.scale = Scale::X2;

        let window = Window::new("Space Invaders", SCREEN_WIDTH, SCREEN_HEIGHT, options)
            .expect("Failed to create window");

        let mut view = Self {
            window,
            buf: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
        view.update_with_buffer();

        view
    }

    pub fn draw(&mut self, vmem: &[u8; 0x1C00]) {
        // Color regions (Ref: https://github.com/superzazu/invaders/blob/master/src/invaders.c):
        // ,_______________________________.
        // |WHITE            ^             |
        // |                32             |
        // |                 v             |
        // |-------------------------------|
        // |RED              ^             |
        // |                32             |
        // |                 v             |
        // |-------------------------------|
        // |WHITE                          |
        // |         < 224 >               |
        // |                               |
        // |                 ^             |
        // |                120            |
        // |                 v             |
        // |                               |
        // |                               |
        // |                               |
        // |-------------------------------|
        // |GREEN                          |
        // | ^                  ^          |
        // |56        ^        56          |
        // | v       72         v          |
        // |____      v      ______________|
        // |  ^  |          | ^            |
        // |<16> |  < 118 > |16   < 122 >  |
        // |  v  |          | v            |
        // |WHITE|          |         WHITE|
        // `-------------------------------'

        for (i, byte) in vmem.iter().enumerate() {
            let y = i * 8 / SCREEN_HEIGHT as usize;
            let base_x = i * 8 % SCREEN_HEIGHT as usize;

            // Each byte contains 8 pixels
            for shift in 0..8 {
                let x = base_x + shift;

                let canvas_x = y;
                let canvas_y = SCREEN_HEIGHT - x - 1;

                let color: u32 = if (byte >> shift) & 1 == 0 {
                    0x00_00_00_00
                } else if canvas_y >= 32 && canvas_y < 64 {
                    0x00_ff_00_00
                } else if canvas_y >= 184 && (canvas_y < 240 || (canvas_x >= 16 && canvas_x < 134))
                {
                    0x00_00_ff_00
                } else {
                    0x00_ff_ff_ff
                };

                self.buf[canvas_x + canvas_y * SCREEN_WIDTH] = color;
            }
        }

        self.update_with_buffer();
    }

    fn update_with_buffer(&mut self) {
        self.window
            .update_with_buffer(&self.buf, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("Failed to update window with buffer");
    }

    pub fn update_keyboard(&mut self, io_handler: &mut InvadersIOHandler) {
        let keys = vec![
            (minifb::Key::Key1, InvadersKey::P1Start),
            (minifb::Key::W, InvadersKey::P1Shoot),
            (minifb::Key::A, InvadersKey::P1Left),
            (minifb::Key::D, InvadersKey::P1Right),
            (minifb::Key::Key2, InvadersKey::P2Start),
            (minifb::Key::Up, InvadersKey::P2Shoot),
            (minifb::Key::Left, InvadersKey::P2Left),
            (minifb::Key::Right, InvadersKey::P2Right),
            (minifb::Key::C, InvadersKey::Coin),
            (minifb::Key::T, InvadersKey::Tilt),
        ];

        for (win_key, inv_key) in &keys {
            let is_down = self.window.is_key_down(*win_key);
            io_handler.handle_key_change(*inv_key, is_down);
        }
    }
}
