use {{crate_name}}::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> u32 {
    let input = include_str!("../input.txt");
    part1(input).unwrap()
}

#[divan::bench]
fn bench_part2() -> u32 {
    let input = include_str!("../input.txt");
    part2(input).unwrap()
}
