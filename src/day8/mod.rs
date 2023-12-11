use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::multi::fill;
use nom::sequence::tuple;
use nom::IResult;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

fn instructions() -> &'static Vec<Instruction> {
    static INSTRUCTIONS: OnceLock<Vec<Instruction>> = OnceLock::new();
    INSTRUCTIONS.get_or_init(|| parse_instructions(input::INSTRUCTIONS).unwrap())
}

fn nodes() -> &'static HashMap<NodeId, Node> {
    static NODES: OnceLock<HashMap<NodeId, Node>> = OnceLock::new();
    NODES.get_or_init(|| {
        read_lines(input::NODES)
            .filter_not_empty()
            .parse::<Node>()
            .map(|node| (node.id, node))
            .collect()
    })
}

pub struct Day8;

impl Solution for Day8 {
    fn day(&self) -> u8 {
        8
    }

    fn part_one(&self) -> String {
        format!(
            "Steps to traverse wasteland: {}",
            traverse_wasteland(instructions(), nodes()),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Steps to traverse wasteland as ghost: {}",
            traverse_wasteland_as_ghost(instructions(), nodes()),
        )
    }
}

fn traverse_wasteland(instructions: &[Instruction], nodes: &HashMap<NodeId, Node>) -> usize {
    traverse_wasteland_from(instructions, nodes, [b'A', b'A', b'A'], |id| {
        id == &[b'Z', b'Z', b'Z']
    })
}

fn traverse_wasteland_from<F: Fn(&NodeId) -> bool>(
    instructions: &[Instruction],
    nodes: &HashMap<NodeId, Node>,
    start_node: NodeId,
    is_end: F,
) -> usize {
    let mut steps = 0;

    let mut current_node_id = start_node;
    for instruction in instructions.iter().cycle() {
        if is_end(&current_node_id) {
            break;
        }
        let current_node = nodes[&current_node_id];
        current_node_id = current_node.next_node(instruction);
        steps += 1;
    }

    steps
}

fn traverse_wasteland_as_ghost(
    instructions: &[Instruction],
    nodes: &HashMap<NodeId, Node>,
) -> usize {
    let cycle_lengths = nodes
        .keys()
        .filter(|id| id[2] == b'A')
        .map(|id| traverse_wasteland_from(instructions, nodes, *id, |id| id[2] == b'Z'))
        .collect::<Vec<_>>();
    dbg!(&cycle_lengths);
    find_smallest_number_divisible_by(&cycle_lengths)
}

fn find_smallest_number_divisible_by(numbers: &[usize]) -> usize {
    let min = numbers.iter().min().copied().unwrap_or(0);
    let mut number = min;
    loop {
        if numbers.iter().all(|i| number % i == 0) {
            break number;
        }
        number += min;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    input
        .chars()
        .map(|c| match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(format!("Invalid instruction {c}")),
        })
        .collect()
}

type NodeId = [u8; 3];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn next_node(&self, instruction: &Instruction) -> NodeId {
        match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(("", node)) = parse_node(input) {
            Ok(node)
        } else {
            Err(format!("Invalid node: {input}"))
        }
    }
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    tuple((
        parse_id,
        tag(" = ("),
        parse_id,
        tag(", "),
        parse_id,
        tag(")"),
    ))(input)
    .map(|(input, (id, _, left, _, right, _))| (input, Node { id, left, right }))
}

fn parse_id(input: &str) -> IResult<&str, NodeId> {
    let mut chars = ['0'; 3];
    let (input, _) = fill(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"), &mut chars)(input)?;
    Ok((input, chars.map(|c| c as u8)))
}

#[cfg(test)]
mod test {
    use crate::day8::Instruction::{Left, Right};

    use super::*;

    fn example_nodes() -> &'static HashMap<NodeId, Node> {
        static NODES: OnceLock<HashMap<NodeId, Node>> = OnceLock::new();
        NODES.get_or_init(|| {
            read_lines(
                b"
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
                .as_slice(),
            )
            .filter_not_empty()
            .parse::<Node>()
            .map(|node| (node.id, node))
            .collect()
        })
    }

    fn example2_nodes() -> &'static HashMap<NodeId, Node> {
        static NODES: OnceLock<HashMap<NodeId, Node>> = OnceLock::new();
        NODES.get_or_init(|| {
            read_lines(
                b"
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
                .as_slice(),
            )
            .filter_not_empty()
            .parse::<Node>()
            .map(|node| (node.id, node))
            .collect()
        })
    }

    #[test]
    fn parse_example_instructions() {
        assert_eq!(parse_instructions("RL"), Ok(vec![Right, Left]));
    }

    #[test]
    fn parse_example_node_line_1() {
        assert_eq!(
            parse_node("AAA = (BBB, CCC)").unwrap().1,
            Node {
                id: [b'A', b'A', b'A'],
                left: [b'B', b'B', b'B'],
                right: [b'C', b'C', b'C'],
            },
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(traverse_wasteland(&[Right, Left], example_nodes()), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            traverse_wasteland_as_ghost(&[Left, Right], example2_nodes()),
            6,
        );
    }
}
