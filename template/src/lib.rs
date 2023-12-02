pub fn part1(_input: &str) -> color_eyre::Result<u32> {
    Ok(0)
}

pub fn part2(_input: &str) -> color_eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(part1(include_str!("../example2.txt")).unwrap(), 0);
    }

    #[test]
    fn test_example2() {
        assert_eq!(part2(include_str!("../example2.txt")).unwrap(), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 0);
    }
}
