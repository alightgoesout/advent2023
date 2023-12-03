use std::str::FromStr;
use std::sync::OnceLock;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use crate::Solution;

mod input;

fn games() -> &'static Vec<Game> {
    static GAMES: OnceLock<Vec<Game>> = OnceLock::new();
    GAMES.get_or_init(|| {
        read_lines(input::INPUT)
            .filter_not_empty()
            .parse()
            .collect()
    })
}

pub struct Day2;

impl Solution for Day2 {
    fn day(&self) -> u8 {
        2
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of IDs of possible games for 12 reds, 13 greens, and 14 blues: {}",
            sum_of_possible_game_ids(games(), 12, 13, 14),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Sum of minimum powers of all games: {}",
            sum_of_minimum_powers(games()),
        )
    }
}

fn sum_of_possible_game_ids(games: &[Game], red: u32, green: u32, blue: u32) -> u32 {
    games
        .iter()
        .filter(|game| game.is_possible(red, green, blue))
        .map(|game| game.number)
        .sum()
}

fn sum_of_minimum_powers(games: &[Game]) -> u32 {
    games.iter().map(Game::minimum_power).sum()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    number: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        red >= self.draws.iter().map(|draw| draw.red).max().unwrap_or(0)
            && green >= self.draws.iter().map(|draw| draw.green).max().unwrap_or(0)
            && blue >= self.draws.iter().map(|draw| draw.blue).max().unwrap_or(0)
    }

    fn minimum_power(&self) -> u32 {
        self.draws.iter().map(|draw| draw.red).max().unwrap_or(0)
            * self.draws.iter().map(|draw| draw.green).max().unwrap_or(0)
            * self.draws.iter().map(|draw| draw.blue).max().unwrap_or(0)
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Ok(("", game)) = parse_game(line) {
            Ok(game)
        } else {
            Err(format!("Invalid game: '{line}'"))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

enum CubeColors {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColors {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("Unknown color: {s}")),
        }
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    tuple((
        tag("Game "),
        digit1,
        tag(": "),
        separated_list0(tag("; "), parse_draw),
    ))(input)
    .map(|(input, (_, number, _, draws))| {
        (
            input,
            Game {
                number: number.parse().unwrap(),
                draws,
            },
        )
    })
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    separated_list0(tag(", "), parse_cube_draw)(input).map(|(input, colors)| {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (n, color) in colors {
            match color {
                CubeColors::Red => red += n,
                CubeColors::Green => green += n,
                CubeColors::Blue => blue += n,
            }
        }
        (input, Draw { red, green, blue })
    })
}

fn parse_cube_draw(input: &str) -> IResult<&str, (u32, CubeColors)> {
    tuple((digit1, tag(" "), alpha1))(input).map(|(input, (number, _, color))| {
        (input, (number.parse().unwrap(), color.parse().unwrap()))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> Vec<Game> {
        read_lines(
            b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .as_slice(),
        )
        .parse()
        .collect()
    }

    #[test]
    fn parse_game_1_of_example() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse(),
            Ok(Game {
                number: 1,
                draws: vec![
                    Draw {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Draw {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Draw {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            })
        );
    }

    #[test]
    fn game_1_of_example_should_be_possible_for_12_13_14() {
        let game1: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        assert!(game1.is_possible(12, 13, 14));
    }

    #[test]
    fn game_3_of_example_should_not_be_possible_for_12_13_14() {
        let game3: Game =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse()
                .unwrap();
        assert!(!game3.is_possible(12, 13, 14));
    }

    #[test]
    fn part1_example() {
        assert_eq!(sum_of_possible_game_ids(&example(), 12, 13, 14), 8);
    }

    #[test]
    fn minimum_power_of_game_1_should_be_48() {
        let game1: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        assert_eq!(game1.minimum_power(), 48);
    }

    #[test]
    fn part2_example() {
        assert_eq!(sum_of_minimum_powers(&example()), 2286);
    }
}
