use day2::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");
    println!("part 1 (regex): {}", regex_version::part1(input));
    println!("part 1 (nom): {}", nom_version::part1(input));
    println!("part 1 (regex): {}", regex_version::part2(input));
    println!("part 1 (nom): {}", nom_version::part2(input));
    Ok(())
}
