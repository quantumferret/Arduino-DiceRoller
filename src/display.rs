use crate::dice_roller::DiceRoller;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::Pin;
use arduino_uno::prelude::*;

const SEGMAP: [u8; 11] = [
    0xC0, 0xF9, 0xA4, 0xB0, 0x99, 0x92, 0x82, 0xF8, 0x80, 0x90, 0xFF,
];
const DIGIT: [u8; 4] = [0xF1, 0xF2, 0xF4, 0xF8];
const LETTER_D: u8 = 0b10100001;
const LOW: u8 = 0x0; // On
const HIGH: u8 = 0x1; // Off

pub enum BitOrder {
    LsbFirst,
    MsbFirst,
}

pub struct Display {
    pub latch_pin: Pin<Output>,
    pub clock_pin: Pin<Output>,
    pub data_pin: Pin<Output>,
    pub bit_order: BitOrder,
}

impl Display {
    pub fn shift_out(&mut self, value: u8) {
        let mut i: u8 = 0;
        let mut v = value;

        while i < 8 {
            match self.bit_order {
                BitOrder::LsbFirst => {
                    if v & 1 == LOW {
                        self.data_pin.set_low().void_unwrap()
                    } else {
                        self.data_pin.set_high().void_unwrap()
                    }
                    v >>= 1;
                }
                BitOrder::MsbFirst => {
                    if v & 128 != 0 {
                        self.data_pin.set_high().void_unwrap()
                    } else {
                        self.data_pin.set_low().void_unwrap()
                    }
                    v <<= 1;
                }
            }

            self.clock_pin.set_high().void_unwrap();
            self.clock_pin.set_low().void_unwrap();
            i += 1;
        }
    }

    pub fn write_number_to_segment(&mut self, segment: u8, value: u8, show_dot: bool) {
        self.latch_pin.set_low().void_unwrap();
        self.shift_out(if show_dot {
            show_decimal(SEGMAP[value as usize])
        } else {
            SEGMAP[value as usize]
        });
        self.shift_out(DIGIT[segment as usize]);
        self.latch_pin.set_high().void_unwrap();
    }

    pub fn write_d(&mut self, segment: u8) {
        self.latch_pin.set_low().void_unwrap();
        self.shift_out(LETTER_D);
        self.shift_out(DIGIT[segment as usize]);
        self.latch_pin.set_high().void_unwrap();
    }

    pub fn display_u16(&mut self, number: u16) {
        if number >= 1000 {
            self.write_number_to_segment(0, (number / 1000 % 10) as u8, false);
        }
        if number >= 100 {
            self.write_number_to_segment(1, (number / 100 % 10) as u8, false);
        }
        if number >= 10 {
            self.write_number_to_segment(2, (number / 10 % 10) as u8, false);
        }
        self.write_number_to_segment(3, (number % 10) as u8, false);
    }

    pub fn display_timer(&mut self, number: u32) {
        if number >= 100_000 {
            self.write_number_to_segment(0, (number / 100_000 % 10) as u8, false);
        }
        if number >= 10_000 {
            self.write_number_to_segment(1, (number / 10_000 % 10) as u8, false);
        }
        if number >= 1000 {
            self.write_number_to_segment(2, (number / 1000 % 10) as u8, true);
        } else {
            self.write_number_to_segment(2, (number / 1000 % 10) as u8, true);
        }
        self.write_number_to_segment(3, (number / 100 % 10) as u8, false);
    }

    pub fn display_config(&mut self, roller: &DiceRoller) {
        self.write_number_to_segment(0, roller.get_dice_amount(), false);
        self.write_d(1);
        let dice = roller.get_dice();
        match dice {
            100 => {
                self.write_number_to_segment(2, 0, false);
                self.write_number_to_segment(3, 0, false);
            }
            d => {
                self.write_number_to_segment(2, d / 10, false);
                self.write_number_to_segment(3, d % 10, false);
            }
        }
    }
}

#[inline]
fn show_decimal(value: u8) -> u8 {
    value & !(1 << 7)
}
