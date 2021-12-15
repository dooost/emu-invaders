use emu_8080::emulator::{IOHandler, State8080};

#[derive(Default)]
pub struct InvadersIOHandler {
    port0: u8,
    port1: u8,
    port2: u8,
    shift_amount: u8,
    shift_data: u16,
}

impl InvadersIOHandler {
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
    fn inp(&mut self, state: State8080, v: u8) -> State8080 {
        let a = match v {
            0 => self.port0,
            1 => self.port1,
            2 => self.port2,
            3 => ((self.shift_data >> u16::from(8 - self.shift_amount)) & 0xff) as u8,
            _ => 0,
        };

        state.setting_a(a)
    }

    fn out(&mut self, state: State8080, v: u8) -> State8080 {
        match v {
            2 => self.shift_amount = state.a & 0x7,
            4 => self.shift_data = (state.a as u16) << 8 | self.shift_data >> 8,
            _ => {}
        }

        state
    }
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
