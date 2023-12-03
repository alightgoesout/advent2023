use std::sync::OnceLock;

use crate::input::read_lines;
use crate::Solution;

mod input;

fn calibration_document() -> &'static Vec<String> {
    static CALIBRATION_DOCUMENT: OnceLock<Vec<String>> = OnceLock::new();
    CALIBRATION_DOCUMENT.get_or_init(|| read_lines(input::INPUT).collect())
}

pub struct Day1;

impl Solution for Day1 {
    fn day(&self) -> u8 {
        1
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of all of the calibration values: {}",
            sum_of_calibration_values(calibration_document())
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Sum of all of the fixed calibration values: {}",
            sum_of_fixed_calibration_values(calibration_document())
        )
    }
}

fn sum_of_calibration_values<I: IntoIterator<Item = &'static String>>(lines: I) -> u32 {
    lines
        .into_iter()
        .map(|line| parse_calibration_value(line))
        .sum()
}

fn sum_of_fixed_calibration_values<I: IntoIterator<Item = &'static String>>(lines: I) -> u32 {
    lines
        .into_iter()
        .map(|line| parse_calibration_value_with_letter_digits(line))
        .sum()
}

fn parse_calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| ('1'..='9').contains(c)).unwrap();
    let second_digit = line
        .chars()
        .rev()
        .find(|c| ('1'..='9').contains(c))
        .unwrap();
    to_u32(first_digit) * 10 + to_u32(second_digit)
}

fn to_u32(digit: char) -> u32 {
    (digit as u32) - 0x30
}

fn parse_calibration_value_with_letter_digits(line: &str) -> u32 {
    let line = line.as_bytes();
    let first_digit = find_first_digit(line);
    let second_digit = find_last_digit(line);
    first_digit * 10 + second_digit
}

const DIGIT_NAMES: [&[u8]; 10] = [
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_first_digit(line: &[u8]) -> u32 {
    (0..line.len())
        .find_map(|index| {
            find_digit(&line[index])
                .or_else(|| find_letter_digit_at_index(line, index, DIGIT_NAMES))
        })
        .unwrap()
}

fn find_last_digit(line: &[u8]) -> u32 {
    (0..line.len())
        .rev()
        .find_map(|index| {
            find_digit(&line[index])
                .or_else(|| find_letter_digit_at_index(line, index, DIGIT_NAMES))
        })
        .unwrap()
}

fn find_digit(c: &u8) -> Option<u32> {
    if (b'1'..=b'9').contains(c) {
        Some((*c - 0x30) as u32)
    } else {
        None
    }
}

fn find_letter_digit_at_index(line: &[u8], index: usize, digit_names: [&[u8]; 10]) -> Option<u32> {
    (1..=9).find(|digit| has_digit(line, index, digit_names[*digit as usize]))
}

fn has_digit(line: &[u8], index: usize, digit_letters: &[u8]) -> bool {
    if line.len() < index + digit_letters.len() {
        return false;
    }
    for i in 0..digit_letters.len() {
        if line[index + i] != digit_letters[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    fn example1() -> &'static Vec<String> {
        static EXAMPLE: OnceLock<Vec<String>> = OnceLock::new();
        EXAMPLE.get_or_init(|| {
            read_lines(
                b"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
                .as_slice(),
            )
            .collect()
        })
    }

    fn example2() -> &'static Vec<String> {
        static EXAMPLE: OnceLock<Vec<String>> = OnceLock::new();
        EXAMPLE.get_or_init(|| {
            read_lines(
                b"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
                .as_slice(),
            )
            .collect()
        })
    }

    #[test]
    fn parse_calibration_value_should_return_12_for_the_first_line_of_example1() {
        assert_eq!(parse_calibration_value(&example1()[0]), 12);
    }

    #[test]
    fn parse_calibration_value_should_return_38_for_the_second_line_of_example1() {
        assert_eq!(parse_calibration_value(&example1()[1]), 38);
    }

    #[test]
    fn parse_calibration_value_should_return_15_for_the_third_line_of_example1() {
        assert_eq!(parse_calibration_value(&example1()[2]), 15);
    }

    #[test]
    fn parse_calibration_value_should_return_77_for_the_fourth_line_of_example1() {
        assert_eq!(parse_calibration_value(&example1()[3]), 77);
    }

    #[test]
    fn part1_example() {
        assert_eq!(sum_of_calibration_values(example1()), 142);
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_29_for_the_first_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[0]),
            29,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_83_for_the_second_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[1]),
            83,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_13_for_the_third_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[2]),
            13,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_24_for_the_fourth_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[3]),
            24,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_42_for_the_fifth_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[4]),
            42,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_14_for_the_sixth_line_of_example2()
    {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[5]),
            14,
        );
    }

    #[test]
    fn parse_calibration_value_with_letter_digits_should_return_76_for_the_seventh_line_of_example2(
    ) {
        assert_eq!(
            parse_calibration_value_with_letter_digits(&example2()[6]),
            76,
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(sum_of_fixed_calibration_values(example2()), 281);
    }
}
