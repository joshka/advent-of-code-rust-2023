use day4::{part1, part2};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input)?);
    println!("part 2: {}", part2(input)?);
    Ok(())
}
