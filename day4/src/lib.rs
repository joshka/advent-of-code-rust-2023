use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u8},
    multi,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use nom_supreme::final_parser::final_parser;
use yansi::Paint;

pub fn part1(input: &str) -> color_eyre::Result<u64> {
    Ok(input
        .lines()
        .map(|line| Card::from(line).unwrap())
        .inspect(|card| println!("{card}"))
        .map(|card| card.score())
        .sum())
}

pub fn part2(input: &str) -> color_eyre::Result<u32> {
    let win_counts = input
        .lines()
        .map(|line| Card::from(line).unwrap())
        .inspect(|card| println!("{card}"))
        .map(|card| card.win_count())
        .collect_vec();
    let mut card_count = vec![1; win_counts.len()];
    for (i, &win_count) in win_counts.iter().enumerate() {
        println!("Card {:3}: {} / {}", i + 1, win_count, card_count[i]);
        for j in 0..win_count {
            let next = i + j as usize + 1;
            card_count[next] += card_count[i];
        }
    }
    Ok(card_count.iter().sum())
}

struct Card {
    id: u8,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn from(input: &str) -> Result<Card, nom::error::Error<&str>> {
        let card = final_parser(parse)(input)?;
        Ok(card)
    }

    fn score(&self) -> u64 {
        let count = self.win_count();
        if count == 0 {
            return 0;
        } else {
            return 2_u64.pow(count - 1);
        }
    }

    fn win_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

fn parse(input: &str) -> IResult<&str, Card> {
    separated_pair(
        preceded(tag("Card").and(multispace1), u8),
        tag(": ").and(multispace0),
        separated_pair(
            multi::separated_list1(multispace1, u8),
            tag(" | ").and(multispace0),
            multi::separated_list1(multispace1, u8),
        ),
    )
    .map(|(id, (winning, have))| Card {
        id,
        winning_numbers: winning,
        numbers: have,
    })
    .parse(input)
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card {:3}: {} | {} = {} / {}",
            self.id,
            self.winning_numbers
                .iter()
                .map(|n| format!("{:2}", Paint::green(n)))
                .join(" "),
            self.numbers
                .iter()
                .map(|n| if self.winning_numbers.contains(n) {
                    format!("{:2}", Paint::red(n))
                } else {
                    format!("{:2}", n)
                })
                .join(" "),
            self.score(),
            self.win_count()
        )
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_card(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(Card::from(input).unwrap().score(), expected);
    }

    #[test]
    fn test_example1() {
        assert_eq!(part1(include_str!("../example1.txt")).unwrap(), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 21158);
    }

    #[test]
    fn test_example2() {
        assert_eq!(part2(include_str!("../example2.txt")).unwrap(), 30);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 6050769);
    }
}
