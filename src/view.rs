use minifb::{Scale, Window, WindowOptions};

use crate::invaders::FrameHalf;
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
        // for i in

        // let (start_memory, start_pixel) = if half == FrameHalf::Top {
        //     (0, 0)
        // } else {
        //     (0xE00, 0x7000)
        // };

        let (start_memory, start_pixel) = (0, 0);

        for offset in 0..0x1C00 {
            let byte = vmem[start_memory + offset];

            for bit in 0..8 {
                let color: u32 = if byte & (1 << bit) == 0 {
                    0x00_00_00_00
                } else {
                    0xff_ff_ff_ff
                };

                let x = (start_pixel + 8 * offset + bit) / SCREEN_HEIGHT;
                let y = SCREEN_HEIGHT - 1 - (start_pixel + 8 * offset + bit) % SCREEN_HEIGHT;

                self.buf[x + y * SCREEN_WIDTH] = color;
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
