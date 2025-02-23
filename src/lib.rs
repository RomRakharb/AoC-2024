#![no_std]

mod day1 {
    pub mod input;
}

// use day1::input::{EXAMPLE_INPUT, INPUT};

pub fn part1(input: &str) -> u32 {
    let mut left: [u32; 1000] = [0; 1000];
    let mut right: [u32; 1000] = [0; 1000];

    input
        .lines()
        .filter_map(|x| x.split_once("   "))
        .enumerate()
        .for_each(|(i, (l, r))| {
            left[i] = l.parse().unwrap();
            right[i] = r.parse().unwrap();
        });

    // TODO: custom sort
    left.sort_unstable();
    right.sort_unstable();

    let mut left = left.iter();
    let mut right = right.iter();
    let mut sum = 0;

    while let (Some(left_value), Some(right_value)) = (left.next(), right.next()) {
        sum += left_value.abs_diff(*right_value);
    }

    sum
}
