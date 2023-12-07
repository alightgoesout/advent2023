use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

use nom::character::complete::{digit1, one_of, space1};
use nom::multi::fill;
use nom::sequence::tuple;
use nom::IResult;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

fn hands() -> &'static Vec<Hand> {
    static HANDS: OnceLock<Vec<Hand>> = OnceLock::new();
    HANDS.get_or_init(|| {
        read_lines(input::INPUT)
            .filter_not_empty()
            .parse()
            .collect()
    })
}

fn hands_with_jokers() -> &'static Vec<Hand> {
    static HANDS_WITH_JOKERS: OnceLock<Vec<Hand>> = OnceLock::new();
    HANDS_WITH_JOKERS.get_or_init(|| hands().iter().map(|hand| hand.to_jokers()).collect())
}

pub struct Day7;

impl Solution for Day7 {
    fn day(&self) -> u8 {
        7
    }

    fn part_one(&self) -> String {
        format!("Total winnings: {}", total_winnings(hands()))
    }

    fn part_two(&self) -> String {
        format!(
            "Total winnings with jokers: {}",
            total_winnings(hands_with_jokers()),
        )
    }
}

fn total_winnings(hands: &[Hand]) -> usize {
    let mut hands: Vec<_> = hands.iter().collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(format!("Invalid card: {value}")),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let hand_type = get_hand_type(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }

    fn to_jokers(self) -> Self {
        let cards = self.cards.map(|card| {
            if card == Card::Jack {
                Card::Joker
            } else {
                card
            }
        });
        Self::new(cards, self.bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(("", hand)) = parse_hand(input) {
            Ok(hand)
        } else {
            Err(format!("Invalid hand: {input}"))
        }
    }
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut combinations = HashMap::new();
    for card in cards {
        combinations
            .entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1u8);
    }
    remove_jokers(&mut combinations);
    match combinations.values().max() {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => {
            if combinations.len() == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        Some(2) => {
            if combinations.len() == 3 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn remove_jokers(combinations: &mut HashMap<Card, u8>) {
    if let Some(jokers) = combinations.remove(&Card::Joker) {
        let (card, count) = combinations
            .iter()
            .max_by(compare_combinations)
            .unwrap_or((&Card::Ace, &0));
        combinations.insert(*card, *count + jokers);
    }
}

fn compare_combinations<'a>(
    (_, count1): &(&'a Card, &'a u8),
    (_, count2): &(&'a Card, &'a u8),
) -> Ordering {
    count1.cmp(count2)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let mut cards = [Card::Two; 5];
    let (input, (_, _, bid)) = tuple((fill(parse_card, &mut cards), space1, digit1))(input)?;
    Ok((input, Hand::new(cards, bid.parse().unwrap())))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    one_of("123456789TJQKA")(input).map(|(input, c)| (input, Card::try_from(c).unwrap()))
}

#[cfg(test)]
mod test {
    use Card::*;
    use HandType::*;

    use super::*;

    const EXAMPLE: &[u8] = b"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    fn example() -> &'static Vec<Hand> {
        static HANDS: OnceLock<Vec<Hand>> = OnceLock::new();
        HANDS.get_or_init(|| read_lines(EXAMPLE).filter_not_empty().parse().collect())
    }

    fn example_with_jokers() -> &'static Vec<Hand> {
        static HANDS_WITH_JOKERS: OnceLock<Vec<Hand>> = OnceLock::new();
        HANDS_WITH_JOKERS.get_or_init(|| example().iter().map(|hand| hand.to_jokers()).collect())
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            example(),
            &[
                Hand {
                    cards: [Three, Two, Ten, Three, King],
                    bid: 765,
                    hand_type: OnePair,
                },
                Hand {
                    cards: [Ten, Five, Five, Jack, Five],
                    bid: 684,
                    hand_type: ThreeOfAKind,
                },
                Hand {
                    cards: [King, King, Six, Seven, Seven],
                    bid: 28,
                    hand_type: TwoPairs,
                },
                Hand {
                    cards: [King, Ten, Jack, Jack, Ten],
                    bid: 220,
                    hand_type: TwoPairs,
                },
                Hand {
                    cards: [Queen, Queen, Queen, Jack, Ace],
                    bid: 483,
                    hand_type: ThreeOfAKind,
                },
            ],
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(total_winnings(example()), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(total_winnings(example_with_jokers()), 5905);
    }

    #[test]
    fn test_order_with_jokers() {
        let hand1 = "JKKK2 100".parse::<Hand>().unwrap().to_jokers();
        let hand2 = "QQQQ2 100".parse::<Hand>().unwrap().to_jokers();

        assert_eq!(hand1.hand_type, FourOfAKind);
        assert_eq!(hand2.hand_type, FourOfAKind);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_jokers1() {
        assert_eq!(
            "Q97J7 740".parse::<Hand>().unwrap().to_jokers(),
            Hand {
                cards: [Queen, Nine, Seven, Joker, Seven],
                bid: 740,
                hand_type: ThreeOfAKind,
            },
        )
    }

    #[test]
    fn test_jokers2() {
        assert_eq!(
            "3JKKJ 832".parse::<Hand>().unwrap().to_jokers(),
            Hand {
                cards: [Three, Joker, King, King, Joker],
                bid: 832,
                hand_type: FourOfAKind,
            },
        )
    }

    #[test]
    fn test_jokers3() {
        assert_eq!(
            "J6AAJ 756".parse::<Hand>().unwrap().to_jokers(),
            Hand {
                cards: [Joker, Six, Ace, Ace, Joker],
                bid: 756,
                hand_type: FourOfAKind,
            },
        )
    }

    #[test]
    fn test_jokers4() {
        assert_eq!(
            "JJ22J 252".parse::<Hand>().unwrap().to_jokers(),
            Hand {
                cards: [Joker, Joker, Two, Two, Joker],
                bid: 252,
                hand_type: FiveOfAKind,
            },
        )
    }
}
