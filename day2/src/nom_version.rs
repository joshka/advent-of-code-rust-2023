use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1,
    sequence::separated_pair, IResult, Parser,
};
use nom_supreme::{final_parser, ParserExt};

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::parse(line).unwrap())
        .filter(Game::possible)
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::parse(line).unwrap())
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        let (r, g, b) =
            self.rounds
                .iter()
                .flat_map(|r| r.cubes.iter())
                .fold((0, 0, 0), |(r, g, b), c| match c.color {
                    Color::Red => (r.max(c.count), g, b),
                    Color::Green => (r, g.max(c.count), b),
                    Color::Blue => (r, g, b.max(c.count)),
                });
        r * g * b
    }

    fn possible(&self) -> bool {
        self.rounds
            .iter()
            .flat_map(|r| r.cubes.iter())
            .all(|c| match c.color {
                Color::Red if c.count <= 12 => true,
                Color::Green if c.count <= 13 => true,
                Color::Blue if c.count <= 14 => true,
                _ => false,
            })
    }

    pub fn parse(input: &str) -> Result<Self, nom::error::Error<&str>> {
        let parser = separated_pair(
            tag("Game ").precedes(u32),
            tag(": "),
            separated_list1(tag("; "), Round::parse),
        )
        .map(|(id, rounds)| Self::new(id, rounds));
        final_parser::final_parser(parser)(input)
    }
}

impl Round {
    fn new(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(tag(", "), Cube::parse)
            .map(Self::new)
            .parse(input)
    }
}

impl Cube {
    const fn new(count: u32, color: Color) -> Self {
        Self { count, color }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(u32, tag(" "), Color::parse)
            .map(|(count, color)| Self::new(count, color))
            .parse(input)
    }
}

impl Color {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("red").value(Self::Red),
            tag("green").value(Self::Green),
            tag("blue").value(Self::Blue),
        ))
        .parse(input)
    }
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
            Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(Game {
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
            })
        );
    }

    #[test]
    fn test_round() {
        assert_eq!(
            Round::parse("12 red, 13 green, 14 blue"),
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
        assert_eq!(Cube::parse("12 red"), Ok(("", Cube::new(12, Color::Red))))
    }

    #[test]
    fn test_color() {
        assert_eq!(Color::parse("red"), Ok(("", Color::Red)));
        assert_eq!(Color::parse("green"), Ok(("", Color::Green)));
        assert_eq!(Color::parse("blue"), Ok(("", Color::Blue)));
    }
}
