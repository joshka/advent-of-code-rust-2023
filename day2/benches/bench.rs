use day2::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1_regex() -> u32 {
    let input = include_str!("../input.txt");
    regex_version::part1(input)
}

#[divan::bench]
fn bench_part1_nom() -> u32 {
    let input = include_str!("../input.txt");
    nom_version::part1(input)
}

#[divan::bench]
fn bench_part2_regex() -> u32 {
    let input = include_str!("../input.txt");
    regex_version::part2(input)
}

#[divan::bench]
fn bench_part2_nom() -> u32 {
    let input = include_str!("../input.txt");
    nom_version::part2(input)
}
