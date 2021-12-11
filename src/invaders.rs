use crate::display::Display;
use emu_8080::emulator::State8080;
use std::time::{Duration, Instant};

const CYCLES_PER_FRAME: u64 = 4_000_000 / 60;

#[derive(Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum FrameHalf {
    Top,
    Bottom,
}

pub struct Invaders {
    state: State8080,
    display: Display,
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
        let state = State8080::new()
            .loading_file_into_memory_at(
                "/Users/prezi/Developer/emu-8080/resources/invaders.h",
                0x0000,
            )
            .loading_file_into_memory_at(
                "/Users/prezi/Developer/emu-8080/resources/invaders.g",
                0x0800,
            )
            .loading_file_into_memory_at(
                "/Users/prezi/Developer/emu-8080/resources/invaders.f",
                0x1000,
            )
            .loading_file_into_memory_at(
                "/Users/prezi/Developer/emu-8080/resources/invaders.e",
                0x1800,
            );
        let display = Display::new();

        Invaders { state, display }
    }

    pub fn run(mut self) {
        // let mut self = self;

        // let mut last_time = None;
        // let mut next_interrupt_time = None;
        // let mut next_interrupt_kind = FrameHalf::Top;

        loop {
            self = self.frame();
        }
    }

    fn frame(self) -> Self {
        let new = self
            .half_frame(FrameHalf::Top)
            .half_frame(FrameHalf::Bottom);

        new
    }

    fn half_frame(mut self, half: FrameHalf) -> Self {
        let mut state = self.state;
        let mut cycles_spent = 0;
        while cycles_spent < CYCLES_PER_FRAME / 2 {
            state = state.evaluating_next();
            let cycles = state.last_cycles();

            cycles_spent += cycles as u64;
        }

        let vmem = &state.memory[0x2400..0x4000];
        self.display.draw(vmem.try_into().unwrap(), half);

        std::thread::sleep(Duration::from_micros(8000));

        if state.interrupt_enabled {
            state = state.generating_interrupt(half as u16);
        }

        Self { state, ..self }
    }
}
