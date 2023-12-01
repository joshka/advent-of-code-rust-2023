use aho_corasick::AhoCorasick;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input.txt");
    println!("sum: {}", calibration_sum_part1(input)?);
    Ok(())
}

fn calibration_sum_part1(input: &str) -> color_eyre::Result<u32> {
    let ac = AhoCorasick::new(&["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])?;
    let sum = input
        .lines()
        .map(|line| {
            let mut matches = ac.find_overlapping_iter(line);
            let first = matches.next().map_or(0, |m| m.pattern().as_u32());
            let second = matches.last().map_or(first, |m| m.pattern().as_u32());
            first * 10 + second
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_sum() {
        let input = include_str!("../../example1.txt");
        assert_eq!(calibration_sum_part1(input).unwrap(), 142);
    }
}
