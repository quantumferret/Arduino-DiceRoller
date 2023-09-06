use crate::button::State::*;
use crate::millis;
use arduino_hal::hal::port::mode::{Floating, Input};
use arduino_hal::hal::port::Pin;
use arduino_hal::prelude::*;

const DEBOUNCE_TIME: u32 = 30;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum State {
    Up = 0,
    Down = 1,
    Debouncing = 2,
}

pub struct Button {
    pin: Pin<Input<Floating>>,
    state: State,
    deadline: u32,
}

impl Button {
    pub fn new(pin: Pin<Input<Floating>>) -> Button {
        Button {
            pin,
            state: Up,
            deadline: 0,
        }
    }

    pub fn get_pulse(&mut self) -> State {
        if self.pin.is_high() {
            self.state = Up;
            return self.state;
        }

        let now = millis();

        return if now - self.deadline > DEBOUNCE_TIME {
            self.state = Down;
            self.deadline = now;
            self.state
        } else {
            self.state = Debouncing;
            self.deadline = now;
            self.state
        };
    }
}
