#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod button;
mod dice_roller;
mod display;
mod mini_rng;
mod stopwatch;

use crate::button::{
    Button,
    State::{Debouncing, Down, Up},
};
use crate::dice_roller::DiceRoller;
use crate::dice_roller::Mode::{ConfigurationMode, NormalMode};
use crate::mini_rng::Rng;
use core::{cell, num::Wrapping};
use panic_halt as _;

const PRESCALER: u32 = 64;
const TIMER_COUNTS: u32 = 250;
const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000; //currently set to overflow in 1ms intervals

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

fn millis_init(tc0: arduino_uno::pac::TC0) {
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let millis_cell = MILLIS_COUNTER.borrow(cs);
        let millis_counter = millis_cell.get();
        millis_cell.set(millis_counter + MILLIS_INCREMENT);
    })
}

#[inline]
pub fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    millis_init(peripherals.TC0);

    // enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut display = display::Display {
        latch_pin: pins.d4.into_output(&mut pins.ddr).downgrade(),
        clock_pin: pins.d7.into_output(&mut pins.ddr).downgrade(),
        data_pin: pins.d8.into_output(&mut pins.ddr).downgrade(),
        bit_order: display::BitOrder::MsbFirst,
    };

    let mut button1 = Button::new(pins.a1.into_floating_input(&mut pins.ddr).downgrade());
    let mut button2 = Button::new(pins.a2.into_floating_input(&mut pins.ddr).downgrade());
    let mut button3 = Button::new(pins.a3.into_floating_input(&mut pins.ddr).downgrade());

    let mut roller = DiceRoller::new();
    let mut rng = Rng::new(millis());
    let mut random: u32;
    let mut stopwatch = stopwatch::Stopwatch::new();

    loop {
        if button1.get_pulse() == Down {
            match roller.mode {
                ConfigurationMode => roller.mode = NormalMode,
                NormalMode => {
                    random = rng.rand_u32();
                    rng.reseed((Wrapping(random) + Wrapping(millis())).0);
                    stopwatch.start();
                    roller.rolling = true;
                }
            }
        } else if button1.get_pulse() == Up && roller.rolling {
            stopwatch.stop();
            random = rng.rand_u32();
            rng.reseed((Wrapping(random) + Wrapping(millis()) + Wrapping(stopwatch.get_time())).0);
            roller.rolling = false;
            roller.roll(&mut rng);
        } else if button2.get_pulse() == Down {
            match roller.mode {
                ConfigurationMode => roller.change_dice_amount(),
                NormalMode => roller.mode = ConfigurationMode,
            }
        } else if button3.get_pulse() == Down {
            match roller.mode {
                ConfigurationMode => roller.change_dice(),
                NormalMode => roller.mode = ConfigurationMode,
            }
        }

        match roller.mode {
            ConfigurationMode => {
                display.display_config(&roller);
            }
            NormalMode => {
                if roller.rolling {
                    display.display_timer(stopwatch.get_time());
                } else {
                    display.display_u16(roller.get_roll());
                }
            }
        }
    }
}
