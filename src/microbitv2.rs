use cortex_m::asm::nop;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf52833_hal::{
    self as hal,
    gpio::{Floating, Input, Output, Pin, PullUp, PushPull},
};

pub struct Board {
    pub led_matrix: LEDmatrix,
    pub buttons: Buttons,
}

impl Board {
    pub fn new() -> Self {
        let p = pac::Peripherals::take().unwrap();
        let port0 = hal::gpio::p0::Parts::new(p.P0);
        let port1 = hal::gpio::p1::Parts::new(p.P1);

        Self {
            led_matrix: LEDmatrix {
                col: [
                    port0.p0_28.into_push_pull_output(Level::High).degrade(),
                    port0.p0_11.into_push_pull_output(Level::High).degrade(),
                    port0.p0_31.into_push_pull_output(Level::High).degrade(),
                    port1.p1_05.into_push_pull_output(Level::High).degrade(),
                    port0.p0_30.into_push_pull_output(Level::High).degrade(),
                ],
                row: [
                    port0.p0_21.into_push_pull_output(Level::Low).degrade(),
                    port0.p0_22.into_push_pull_output(Level::Low).degrade(),
                    port0.p0_15.into_push_pull_output(Level::Low).degrade(),
                    port0.p0_24.into_push_pull_output(Level::Low).degrade(),
                    port0.p0_19.into_push_pull_output(Level::Low).degrade(),
                ],
            },

            buttons: Buttons {
                button_a: port0.p0_14.into_floating_input().degrade(),
                button_b: port0.p0_23.into_floating_input().degrade(),
            },
        }
    }
}

pub struct LEDmatrix {
    pub col: [Pin<Output<PushPull>>; 5],
    pub row: [Pin<Output<PushPull>>; 5],
}

impl LEDmatrix {
    pub fn display(&mut self, display: [[u8; 5]; 5]) {
        for _ in 0..50 {
            for (i, row) in display.iter().enumerate() {
                let _ = self.row[i].set_state(PinState::from(true));
                for (j, col) in row.iter().enumerate() {
                    let _ = self.col[j].set_state(PinState::from(*col != 1));
                }
                for _ in 0..1_000 {
                    nop();
                }
                let _ = self.row[i].set_state(PinState::from(false));
            }
        }
    }
}

pub struct Buttons {
    pub button_a: Pin<Input<Floating>>,
    pub button_b: Pin<Input<Floating>>,
}
