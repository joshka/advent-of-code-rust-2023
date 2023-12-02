use day1::{calibration_sum_part1, calibration_sum_part2};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");
    println!("part 1: {}", calibration_sum_part1(input)?);
    println!("part 1: {}", calibration_sum_part2(input)?);
    Ok(())
}
