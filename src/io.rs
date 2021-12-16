use crate::sound::SoundController;
use emu_8080::emulator::{IOHandler, State8080};

pub struct InvadersIOHandler {
    port0: u8,
    port1: u8,
    port2: u8,
    shift_amount: u8,
    shift_data: u16,
    sound3: u8,
    sound5: u8,

    sound_controller: SoundController,
}

impl InvadersIOHandler {
    pub fn new() -> Self {
        Self {
            port0: 0,
            port1: 0,
            port2: 0,
            shift_amount: 0,
            shift_data: 0,
            sound3: 0,
            sound5: 0,

            sound_controller: SoundController::new(),
        }
    }

    pub fn handle_key_change(&mut self, key: InvadersKey, is_down: bool) {
        let bit = key.bit();

        let port_ref = match key.port() {
            InputPort::Port0 => &mut self.port0,
            InputPort::Port1 => &mut self.port1,
            InputPort::Port2 => &mut self.port2,
        };

        if is_down {
            *port_ref |= bit;
        } else {
            *port_ref &= !bit;
        }
    }
}

impl IOHandler for InvadersIOHandler {
    fn inp(&mut self, state: State8080, port: u8) -> State8080 {
        let a = match port {
            0 => self.port0,
            1 => self.port1,
            2 => self.port2,
            3 => ((self.shift_data >> u16::from(8 - self.shift_amount)) & 0xff) as u8,
            _ => 0,
        };

        state.setting_a(a)
    }

    fn out(&mut self, state: State8080, port: u8) -> State8080 {
        let val = state.a;
        match port {
            2 => self.shift_amount = val & 0x7,
            3 => {
                let new_sounds = (0..4).map(|i| bit(val, i));
                let old_sounds = (0..4).map(|i| bit(self.sound3, i));

                for (i, (new, old)) in new_sounds.zip(old_sounds).enumerate() {
                    if new && !old {
                        // Play sound if the new value is set and was not previously
                        self.sound_controller.play_once(i);
                    } else if i == 0 && !new && old {
                        // Stop playing when changed to false for UFO sound
                        // self.sound_controller.stop_sound(i);
                    }
                }
                self.sound3 = val;
            }
            4 => self.shift_data = (val as u16) << 8 | self.shift_data >> 8,
            5 => {
                let new_sounds = (0..4).map(|i| bit(val, i));
                let old_sounds = (0..4).map(|i| bit(self.sound5, i));

                for (i, (new, old)) in new_sounds.zip(old_sounds).enumerate() {
                    if new && !old {
                        // Play sound if the new value is set and was not previously
                        self.sound_controller.play_once(i + 4);
                    }
                }
                self.sound5 = val;
            }
            _ => {}
        }

        state
    }
}

fn bit(val: u8, bit_i: u8) -> bool {
    val & (1 << bit_i) > 0
}

#[derive(Clone, Copy)]
enum InputPort {
    Port0,
    Port1,
    Port2,
}

#[derive(Clone, Copy)]
pub enum InvadersKey {
    P1Start,
    P1Shoot,
    P1Left,
    P1Right,

    P2Start,
    P2Shoot,
    P2Left,
    P2Right,

    Coin,

    Tilt,
}

impl InvadersKey {
    fn bit(&self) -> u8 {
        match self {
            InvadersKey::P1Start => 0b100,
            InvadersKey::P1Shoot => 0b10000,
            InvadersKey::P1Left => 0b100000,
            InvadersKey::P1Right => 0b1000000,

            InvadersKey::P2Start => 0b10,
            InvadersKey::P2Shoot => 0b10000,
            InvadersKey::P2Left => 0b100000,
            InvadersKey::P2Right => 0b1000000,

            InvadersKey::Coin => 0b1,
            InvadersKey::Tilt => 0b100,
        }
    }

    fn port(&self) -> InputPort {
        match self {
            InvadersKey::P1Start => InputPort::Port1,
            InvadersKey::P1Shoot => InputPort::Port1,
            InvadersKey::P1Left => InputPort::Port1,
            InvadersKey::P1Right => InputPort::Port1,

            InvadersKey::P2Start => InputPort::Port1,
            InvadersKey::P2Shoot => InputPort::Port2,
            InvadersKey::P2Left => InputPort::Port2,
            InvadersKey::P2Right => InputPort::Port2,

            InvadersKey::Coin => InputPort::Port1,
            InvadersKey::Tilt => InputPort::Port2,
        }
    }
}
