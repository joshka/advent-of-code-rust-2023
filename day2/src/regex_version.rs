use itertools::Itertools;
use regex::Regex;

pub fn part1(input: &str) -> u32 {
    let mut count = 0;
    let game_re = Regex::new(r"Game (?<game>\d+): (?<rest>.*)").unwrap();
    for line in input.lines() {
        let (_, [game, rest]) = game_re.captures(line).map(|cap| cap.extract()).unwrap();
        if rest.split("; ").all(possible) {
            count += game.parse::<u32>().unwrap();
        }
    }
    count
}

fn possible(line: &str) -> bool {
    let pull_re = Regex::new(r"(?<amount>\d+) (?<color>\w+)").unwrap();
    let pulls = line.split("; ").collect_vec();
    for pull in pulls {
        for (_, [amount, color]) in pull_re.captures_iter(pull).map(|cap| cap.extract()) {
            let amount = amount.parse::<u32>().unwrap();
            match color {
                "red" if amount <= 12 => {}
                "green" if amount <= 13 => {}
                "blue" if amount <= 14 => {}
                _ => return false,
            }
        }
    }
    true
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    let game_re = Regex::new(r"Game (?<game>\d+): (?<rest>.*)").unwrap();
    for line in input.lines() {
        let (_, [_game, rest]) = game_re.captures(line).map(|cap| cap.extract()).unwrap();
        count += power(rest);
    }
    count
}

pub fn power(game: &str) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let pull_re = Regex::new(r"(?<amount>\d+) (?<color>\w+)").unwrap();
    for (_, [amount, color]) in pull_re.captures_iter(game).map(|cap| cap.extract()) {
        let amount = amount.parse::<u32>().unwrap();
        match color {
            "red" => red = red.max(amount),
            "green" => green = green.max(amount),
            "blue" => blue = blue.max(amount),
            _ => {}
        }
    }
    red * green * blue
}
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input.txt")), 2331);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input.txt")), 71585);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_possible(#[case] line: &str, #[case] expected: bool) {
        assert_eq!(possible(line), expected);
    }

    #[test]
    fn test_part1_example() -> color_eyre::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input), 8);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> color_eyre::Result<()> {
        let input = include_str!("../example2.txt");
        assert_eq!(part2(input), 2286);
        Ok(())
    }
}
