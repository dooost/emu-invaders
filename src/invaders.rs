use crate::display::Display;
use emu_8080::emulator::State8080;
use std::time::{Duration, Instant};

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
        let mut state = self.state;

        let mut last_time = None;
        let mut next_interrupt_time = None;
        let mut next_interrupt_kind = FrameHalf::Top;

        loop {
            let now = Instant::now();

            if let None = last_time {
                last_time = Some(now);
                next_interrupt_time = Some(now + Duration::from_micros(16667));
            }

            if state.interrupt_enabled && now > next_interrupt_time.unwrap() {
                state = state.generating_interrupt(next_interrupt_kind as u16);
                next_interrupt_kind = next_interrupt_kind.toggled();
                next_interrupt_time = Some(now + Duration::from_micros(8334));
            }

            let since_last = now - last_time.unwrap();

            let cycles_left = 2 * since_last.as_micros();
            let mut cycles_ran = 0;

            while cycles_left > cycles_ran {
                state = state.evaluating_next();
                cycles_ran += state.last_cycles() as u128;
            }

            let vmem = &state.memory[0x2400..0x4000];
            self.display
                .draw(vmem.try_into().unwrap(), next_interrupt_kind.toggled());

            last_time = Some(now);
        }
    }
}
