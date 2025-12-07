use std::{collections::BTreeSet, mem};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_beam_splits(INPUT);
    assert_eq!(1594, part1);
    println!("{part1}");
}

fn n_beam_splits(s: &str) -> usize {
    let mut manifold = BTreeSet::new();
    let mut laser_positions = BTreeSet::new();
    let mut h = 0;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => { /* no-op */ }

                '^' => {
                    manifold.insert((x, y));
                }

                'S' => {
                    laser_positions.insert(x);
                }

                o => panic!("Unknown character `{o}`"),
            }
        }

        h = y;
    }

    let mut next_laser_positions = BTreeSet::new();
    let mut splits = 0;

    for y in 0..h {
        next_laser_positions.clear();

        for &x in &laser_positions {
            if manifold.contains(&(x, y + 1)) {
                splits += 1;
                next_laser_positions.insert(x - 1);
                next_laser_positions.insert(x + 1);
            } else {
                next_laser_positions.insert(x);
            }
        }

        mem::swap(&mut laser_positions, &mut next_laser_positions);
    }

    splits
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(21, n_beam_splits(EXAMPLE));
    }
}
