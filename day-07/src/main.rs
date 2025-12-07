use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_beam_splits(INPUT);
    assert_eq!(1594, part1);
    println!("{part1}");

    let part2 = n_universes(INPUT);
    assert_eq!(15650261281478, part2);
    println!("{part2}");
}

#[derive(Default)]
struct Manifold {
    manifold: BTreeSet<(usize, usize)>,
    h: usize,
}

type LaserPositions = BTreeMap<usize, usize>;

fn n_beam_splits(s: &str) -> usize {
    let (manifold, mut laser_positions) = parse_manifold(s);

    run_experiment(&manifold, &mut laser_positions)
}

fn n_universes(s: &str) -> usize {
    let (manifold, mut laser_positions) = parse_manifold(s);

    run_experiment(&manifold, &mut laser_positions);

    laser_positions.values().sum()
}

fn parse_manifold(s: &str) -> (Manifold, LaserPositions) {
    let mut manifold = BTreeSet::new();
    let mut laser_positions = BTreeMap::new();
    let mut h = 0;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => { /* no-op */ }

                '^' => {
                    manifold.insert((x, y));
                }

                'S' => {
                    laser_positions.insert(x, 1);
                }

                o => panic!("Unknown character `{o}`"),
            }
        }

        h = y;
    }

    (Manifold { manifold, h }, laser_positions)
}

fn run_experiment(manifold: &Manifold, laser_positions: &mut LaserPositions) -> usize {
    let mut next_laser_positions = LaserPositions::new();
    let mut splits = 0;

    for y in 0..manifold.h {
        next_laser_positions.clear();

        for (&x, &n) in &*laser_positions {
            if manifold.manifold.contains(&(x, y + 1)) {
                splits += 1;
                *next_laser_positions.entry(x - 1).or_insert(0) += n;
                *next_laser_positions.entry(x + 1).or_insert(0) += n;
            } else {
                *next_laser_positions.entry(x).or_insert(0) += n;
            }
        }

        mem::swap(laser_positions, &mut next_laser_positions);
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

    #[test]
    fn part2_example() {
        assert_eq!(40, n_universes(EXAMPLE));
    }
}
