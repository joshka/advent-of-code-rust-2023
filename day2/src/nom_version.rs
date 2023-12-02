#![allow(unused)]
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use strum_macros::EnumString;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| game(line).unwrap().1)
        .filter(Game::possible)
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| game(line).unwrap().1)
        .map(|game| game.power())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    count: u32,
    color: Color,
}

#[derive(Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }

    fn power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in &self.rounds {
            for cube in &round.cubes {
                match cube.color {
                    Color::Red => red = red.max(cube.count),
                    Color::Green => green = green.max(cube.count),
                    Color::Blue => blue = blue.max(cube.count),
                }
            }
        }
        red * green * blue
    }

    fn possible(&self) -> bool {
        for round in &self.rounds {
            for cube in &round.cubes {
                match cube.color {
                    Color::Red if cube.count > 12 => return false,
                    Color::Green if cube.count > 13 => return false,
                    Color::Blue if cube.count > 14 => return false,
                    _ => {}
                }
            }
        }
        true
    }
}

impl Round {
    fn new(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }
}

impl Cube {
    fn new(count: u32, color: Color) -> Self {
        Self { count, color }
    }
}

fn game(input: &str) -> IResult<&str, Game> {
    // there's probably a neater way to do this using preceded / separated_pair
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;
    Ok((input, Game::new(id, rounds)))
}

fn round(input: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(", "), cube), Round::new)(input)
}

fn cube(input: &str) -> IResult<&str, Cube> {
    map(separated_pair(digit1, tag(" "), color), |(count, color)| {
        Cube::new(count.parse().unwrap(), color)
    })(input)
}

fn color(input: &str) -> IResult<&str, Color> {
    map(alt((tag("red"), tag("green"), tag("blue"))), |color| {
        Color::from_str(color).unwrap()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input.txt")), 2331);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input.txt")), 71585);
    }

    #[test]
    fn test_game() {
        assert_eq!(
            game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            (
                "",
                Game {
                    id: 1,
                    rounds: vec![
                        Round::new(vec![Cube::new(3, Color::Blue), Cube::new(4, Color::Red)]),
                        Round::new(vec![
                            Cube::new(1, Color::Red),
                            Cube::new(2, Color::Green),
                            Cube::new(6, Color::Blue)
                        ]),
                        Round::new(vec![Cube::new(2, Color::Green)])
                    ]
                }
            )
        );
    }

    #[test]
    fn test_round() {
        assert_eq!(
            round("12 red, 13 green, 14 blue"),
            Ok((
                "",
                Round::new(vec![
                    Cube::new(12, Color::Red),
                    Cube::new(13, Color::Green),
                    Cube::new(14, Color::Blue)
                ])
            ))
        );
    }

    #[test]
    fn test_cube() {
        assert_eq!(cube("12 red"), Ok(("", Cube::new(12, Color::Red))))
    }

    #[test]
    fn test_color() {
        assert_eq!(color("red"), Ok(("", Color::Red)));
        assert_eq!(color("green"), Ok(("", Color::Green)));
        assert_eq!(color("blue"), Ok(("", Color::Blue)));
    }
}
