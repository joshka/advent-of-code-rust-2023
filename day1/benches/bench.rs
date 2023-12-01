use day1::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> u32 {
    let input = include_str!("../input.txt");
    calibration_sum_part1(divan::black_box(input)).unwrap()
}

#[divan::bench]
fn part2() -> u32 {
    let input = include_str!("../input.txt");
    calibration_sum_part2(divan::black_box(input)).unwrap()
}
