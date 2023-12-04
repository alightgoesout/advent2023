use std::collections::HashSet;
use std::str::FromStr;
use std::sync::OnceLock;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

fn scratchcards() -> &'static Vec<Scratchcard> {
    static SCRATCHCARDS: OnceLock<Vec<Scratchcard>> = OnceLock::new();
    SCRATCHCARDS.get_or_init(|| {
        read_lines(input::INPUT)
            .filter_not_empty()
            .parse()
            .collect()
    })
}

pub struct Day4;

impl Solution for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of all scratchcards points: {}",
            scratchcards().iter().map(Scratchcard::points).sum::<u32>(),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Total number of scratchcards: {}",
            compute_nb_scratchcards(scratchcards()),
        )
    }
}

fn compute_nb_scratchcards(scratchcards: &[Scratchcard]) -> usize {
    let mut cards_to_process = scratchcards.iter().collect::<Vec<_>>();
    let mut scratchcards_count = scratchcards.len();

    while let Some(scratchcard) = cards_to_process.pop() {
        let mut won_cards = (0..scratchcard.matching_numbers_count())
            .filter_map(|n| scratchcards.get(scratchcard.number + n))
            .collect::<Vec<_>>();
        scratchcards_count += won_cards.len();
        cards_to_process.append(&mut won_cards);
    }

    scratchcards_count
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Scratchcard {
    number: usize,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Scratchcard {
    fn matching_numbers_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .count()
    }

    fn points(&self) -> u32 {
        match self.matching_numbers_count() {
            0 => 0,
            n => 2u32.pow(n as u32 - 1),
        }
    }
}

impl FromStr for Scratchcard {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let result = parse_scratchcard(input);
        if let Ok(("", card)) = result {
            Ok(card)
        } else {
            Err(format!("Invalid card: {input}"))
        }
    }
}

fn parse_scratchcard(input: &str) -> IResult<&str, Scratchcard> {
    tuple((
        tag("Card"),
        multispace1,
        digit1,
        tag(":"),
        multispace1,
        separated_list0(multispace1, digit1),
        tag(" |"),
        multispace1,
        separated_list0(multispace1, digit1),
    ))(input)
    .map(
        |(input, (_, _, number, _, _, winning_numbers, _, _, card_numbers))| {
            (
                input,
                Scratchcard {
                    number: number.parse().unwrap(),
                    winning_numbers: winning_numbers.iter().map(|i| i.parse().unwrap()).collect(),
                    card_numbers: card_numbers.iter().map(|i| i.parse().unwrap()).collect(),
                },
            )
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> &'static Vec<Scratchcard> {
        static SCRATCHCARDS: OnceLock<Vec<Scratchcard>> = OnceLock::new();
        SCRATCHCARDS.get_or_init(|| {
            read_lines(
                b"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
                .as_slice(),
            )
            .filter_not_empty()
            .parse()
            .collect()
        })
    }

    #[test]
    fn parse_card_1_of_example() {
        assert_eq!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<Scratchcard>(),
            Ok(Scratchcard {
                number: 1,
                winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                card_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            })
        );
    }

    #[test]
    fn parse_card_3_of_example() {
        assert_eq!(
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".parse::<Scratchcard>(),
            Ok(Scratchcard {
                number: 3,
                winning_numbers: HashSet::from([1, 21, 53, 59, 44]),
                card_numbers: HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]),
            })
        );
    }

    #[test]
    fn points_should_be_8_for_card_1() {
        let card1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse::<Scratchcard>()
            .unwrap();

        assert_eq!(card1.points(), 8);
    }

    #[test]
    fn points_should_be_2_for_card_2() {
        let card2 = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            .parse::<Scratchcard>()
            .unwrap();

        assert_eq!(card2.points(), 2);
    }

    #[test]
    fn points_should_be_0_for_card_5() {
        let card5 = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
            .parse::<Scratchcard>()
            .unwrap();

        assert_eq!(card5.points(), 0);
    }

    #[test]
    fn part1_example() {
        assert_eq!(example().iter().map(Scratchcard::points).sum::<u32>(), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(compute_nb_scratchcards(example()), 30)
    }
}
