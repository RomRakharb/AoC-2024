#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use aoc_2024::part1;

mod day1 {
    pub mod input;
}
mod microbitv2;
mod time;

use day1::input::INPUT;
use microbitv2::Board;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Start");
    rprintln!("{}", part1(INPUT));

    let board = Board::new();
    let mut led_matrix = board.led_matrix;
    let mut is_on = true;

    loop {
        rprintln!("Looping...");
        if is_on {
            led_matrix.display([
                [1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ]);
        } else {
            led_matrix.display([
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ]);
        }
        is_on = !is_on;
    }
}
