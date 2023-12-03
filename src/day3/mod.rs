use itertools::Itertools;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::input::{read_lines, FilterNotEmpty};
use crate::Solution;

mod input;

fn schematic() -> &'static EngineSchematic {
    static INPUT: OnceLock<EngineSchematic> = OnceLock::new();
    INPUT.get_or_init(|| EngineSchematic::from_lines(read_lines(input::INPUT).filter_not_empty()))
}

pub struct Day3;

impl Solution for Day3 {
    fn day(&self) -> u8 {
        3
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of all part numbers: {}",
            schematic().part_numbers().iter().sum::<u32>(),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Sum of all gear ratios: {}",
            schematic()
                .gears()
                .into_iter()
                .map(|(n1, n2)| n1 * n2)
                .sum::<u32>(),
        )
    }
}

struct EngineSchematic {
    symbols: HashMap<Position, char>,
    numbers: HashMap<usize, Vec<SchematicNumber>>,
}

impl EngineSchematic {
    fn from_lines<L: IntoIterator<Item = String>>(lines: L) -> Self {
        let mut symbols = HashMap::new();
        let mut numbers = HashMap::new();

        for (line, content) in lines.into_iter().enumerate() {
            let mut line_numbers = Vec::new();
            let mut current_number = Vec::new();
            for (column, c) in content.chars().enumerate() {
                if c.is_ascii_digit() {
                    current_number.push(c);
                } else {
                    if !current_number.is_empty() {
                        let value = current_number.iter().collect::<String>().parse().unwrap();
                        line_numbers.push(SchematicNumber {
                            value,
                            line,
                            start: column - current_number.len(),
                            end: column - 1,
                        });
                        current_number.clear()
                    }
                    if c != '.' {
                        symbols.insert(Position(column, line), c);
                    }
                }
            }
            if !current_number.is_empty() {
                let value = current_number.iter().collect::<String>().parse().unwrap();
                line_numbers.push(SchematicNumber {
                    value,
                    line,
                    start: content.len() - current_number.len(),
                    end: content.len() - 1,
                });
            }
            numbers.insert(line, line_numbers);
        }

        Self { symbols, numbers }
    }

    fn part_numbers(&self) -> Vec<u32> {
        self.symbols
            .keys()
            .flat_map(|position| self.adjacent_numbers(position))
            .unique()
            .map(|number| number.value)
            .collect()
    }

    fn adjacent_numbers(&self, position: &Position) -> Vec<SchematicNumber> {
        let start_line = position.1.saturating_sub(1);
        let end_line = position.1 + 1;
        (start_line..=end_line)
            .flat_map(|line| self.numbers.get(&line))
            .flatten()
            .filter(|number| number.is_adjacent(position))
            .copied()
            .collect()
    }

    fn gears(&self) -> Vec<(u32, u32)> {
        self.symbols
            .iter()
            .filter(|(_, c)| **c == '*')
            .map(|(position, _)| self.adjacent_numbers(position))
            .filter_map(|numbers| {
                if numbers.len() == 2 {
                    Some((numbers[0].value, numbers[1].value))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Position(usize, usize);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct SchematicNumber {
    value: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl SchematicNumber {
    fn is_adjacent(&self, position: &Position) -> bool {
        let start = self.start.saturating_sub(1);
        let end = self.end + 1;
        position.0 >= start && position.0 <= end && (position.1.abs_diff(self.line) <= 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example1() -> &'static EngineSchematic {
        static EXAMPLE: OnceLock<EngineSchematic> = OnceLock::new();
        EXAMPLE.get_or_init(|| {
            EngineSchematic::from_lines(
                read_lines(
                    b"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    #[test]
    fn part1_example() {
        assert_eq!(example1().part_numbers().iter().sum::<u32>(), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            example1()
                .gears()
                .into_iter()
                .map(|(n1, n2)| n1 * n2)
                .sum::<u32>(),
            467835,
        );
    }
}
