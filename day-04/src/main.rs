use itertools::Itertools;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = accessible_rolls_of_paper(INPUT);
    assert_eq!(1533, part1);
    println!("{part1}");
}

fn accessible_rolls_of_paper(s: &str) -> usize {
    let mut board = BTreeSet::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '@' {
                board.insert((x, y));
            }
        }
    }

    board
        .iter()
        .filter(|(x, y)| {
            let occupied_neighbors = neighbor_offsets()
                .filter(|&(dx, dy)| {
                    let nx = x.checked_add_signed(dx);
                    let ny = y.checked_add_signed(dy);

                    nx.zip(ny).is_some_and(|c| board.contains(&c))
                })
                .count();

            occupied_neighbors < 4
        })
        .count()
}

fn neighbor_offsets() -> impl Iterator<Item = (isize, isize)> {
    const OFFSETS: [isize; 3] = [-1, 0, 1];

    OFFSETS
        .into_iter()
        .cartesian_product(OFFSETS)
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(13, accessible_rolls_of_paper(EXAMPLE));
    }

    #[test]
    fn neighbor_offsets_exercise() {
        assert_eq!(8, neighbor_offsets().count());
    }
}
