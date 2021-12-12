use crate::display::Display;
use emu_8080::emulator::{IOHandler, State8080};
use std::time::Duration;

const CYCLES_PER_FRAME: u64 = 4_000_000 / 60;

#[derive(Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum FrameHalf {
    Top = 1,
    Bottom,
}

#[derive(Default)]
pub struct InvadersIOHandler {
    shift_amount: u8,
    shift_data: u16,
}

pub struct Invaders {
    state: State8080,
    display: Display,
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

impl IOHandler for InvadersIOHandler {
    fn inp(&mut self, state: State8080, v: u8) -> State8080 {
        let a = match v {
            0 => 1,
            1 => 0,
            2 => 0,
            3 => ((self.shift_data >> u16::from(8 - self.shift_amount)) & 0xff) as u8,
            _ => 0,
        };

        println!("Inp!");

        state.setting_a(a)
    }

    fn out(&mut self, state: State8080, v: u8) -> State8080 {
        match v {
            2 => self.shift_amount = state.a & 0x7,
            4 => self.shift_data = (state.a as u16) << 8 | self.shift_data >> 8,
            _ => {}
        }

        println!("Out!");

        state
    }
}

impl Invaders {
    pub fn new() -> Self {
        let state = State8080::new().loading_file_into_memory_at(
            "/Users/prezi/Developer/emu-invaders/res/invaders.rom",
            0x0000,
        );
        let display = Display::new();

        Invaders {
            state,
            display,
            io_handler: InvadersIOHandler::default(),
        }
    }

    pub fn run(mut self) {
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
            state = state.evaluating_next(Some(&mut self.io_handler));
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
