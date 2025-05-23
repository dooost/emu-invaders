use crate::io::InvadersIOHandler;
use crate::view::InvadersView;
use emu_8080::emulator::State8080;
use std::time::{Duration, Instant};

const CYCLES_PER_SEC: u64 = 2_000_000;
const CYCLES_PER_MICROSEC: u64 = CYCLES_PER_SEC / 1_000_000;
const FPS: u64 = 60;
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SEC / FPS;

#[derive(Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum FrameHalf {
    Top = 1,
    Bottom,
}

pub struct Invaders {
    state: State8080,
    view: InvadersView,
    io_handler: InvadersIOHandler,
}

impl FrameHalf {
    fn toggled(&self) -> Self {
        match self {
            FrameHalf::Top => FrameHalf::Bottom,
            FrameHalf::Bottom => FrameHalf::Top,
        }
    }
}

impl Invaders {
    pub fn new() -> Self {
        let state = State8080::new().loading_file_into_memory_at(
            "res/invaders.rom",
            0x0000,
        );

        Invaders {
            state,
            view: InvadersView::new(),
            io_handler: InvadersIOHandler::new(),
        }
    }

    pub fn run(mut self) {
        let mut last_time = None;
        let mut next_interrupt_time = None;
        let mut next_interrupt_kind = FrameHalf::Top;

        loop {
            self.update_view();

            let now = Instant::now();

            if let None = last_time {
                last_time = Some(now);
                next_interrupt_time = Some(now + Duration::from_micros(CYCLES_PER_FRAME / 2));
            }

            if self.state.interrupt_enabled && now > next_interrupt_time.unwrap() {
                self.state = self.state.generating_interrupt(next_interrupt_kind as u16);
                next_interrupt_kind = next_interrupt_kind.toggled();
                next_interrupt_time = Some(now + Duration::from_micros(CYCLES_PER_FRAME / 2));
            }

            // Run cycles for max 1 second
            let duration = Duration::min(now - last_time.unwrap(), Duration::from_secs(1));
            let cycles_left = CYCLES_PER_MICROSEC as u128 * duration.as_micros();
            let mut cycles_ran = 0;
            while cycles_ran < cycles_left {
                self.state = self.state.evaluating_next(Some(&mut self.io_handler));
                cycles_ran += self.state.last_cycles() as u128;
            }

            last_time = Some(now);
        }
    }

    fn update_view(&mut self) {
        let vmem = &self.state.memory[0x2400..0x4000];
        self.view.draw(vmem.try_into().unwrap());
        self.view.update_keyboard(&mut self.io_handler);
    }
}
