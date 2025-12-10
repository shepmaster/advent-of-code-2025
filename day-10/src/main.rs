use std::collections::{BTreeSet, VecDeque};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = sum_of_minimum_presses(INPUT);
    assert_eq!(491, part1);
    println!("{part1}");
}

fn sum_of_minimum_presses(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();

            let diagram = parts.next().expect("missing diagram");
            let diagram = diagram.trim_matches(['[', ']']);

            assert!((1..=16).contains(&diagram.len()));
            let diagram = diagram
                .chars()
                .map(|c| (c == '#') as u16)
                .rev()
                .fold(0u16, |acc, bit| acc << 1 | bit);

            let _joltage = parts.next_back().expect("missing joltage");

            let buttons = parts
                .map(|button| {
                    let button = button.trim_matches(['(', ')']);
                    button
                        .split(',')
                        .map(|b| b.parse::<u8>().expect("Invalid button index"))
                        .fold(0u16, |acc, bit| acc | 1 << bit)
                })
                .collect::<Vec<_>>();

            minimum_button_sequence(diagram, &buttons)
                .expect("Did not find minimum button sequence")
        })
        .sum()
}

fn minimum_button_sequence(diagram: u16, buttons: &[u16]) -> Option<usize> {
    let mut visited = BTreeSet::new();
    visited.insert(0u16);
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0usize, 0u16));

    while let Some((depth, lights)) = to_visit.pop_front() {
        if lights == diagram {
            return Some(depth);
        }

        for b in buttons {
            let next_lights = lights ^ b;

            if visited.insert(next_lights) {
                to_visit.push_back((depth + 1, next_lights));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(7, sum_of_minimum_presses(EXAMPLE));
    }
}
