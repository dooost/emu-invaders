use minifb::{Scale, Window, WindowOptions};

use crate::invaders::FrameHalf;

pub const SCREEN_WIDTH: usize = 224;
pub const SCREEN_HEIGHT: usize = 256;

pub struct Display {
    window: Window,
    buf: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        let mut options = WindowOptions::default();
        options.scale = Scale::X2;

        let window = Window::new("Space Invaders", SCREEN_WIDTH, SCREEN_HEIGHT, options)
            .expect("Failed to create window");

        let mut display = Self {
            window,
            buf: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
        display.update_with_buffer();

        display
    }

    pub fn draw(&mut self, vmem: &[u8; 0x1C00], half: FrameHalf) {
        let (start_memory, start_pixel) = if half == FrameHalf::Top {
            (0, 0)
        } else {
            (0xE00, 0x7000)
        };

        for offset in 0..0xE00 {
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
}
