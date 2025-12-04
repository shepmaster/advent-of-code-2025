use itertools::Itertools;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = accessible_rolls_of_paper(INPUT);
    assert_eq!(1533, part1);
    println!("{part1}");

    let part2 = accessible_rolls_of_paper_iterative(INPUT);
    assert_eq!(9206, part2);
    println!("{part2}");
}

fn accessible_rolls_of_paper(s: &str) -> usize {
    let board = parse_board(s);

    find_accessible_rolls_of_paper(&board).count()
}

fn accessible_rolls_of_paper_iterative(s: &str) -> usize {
    let mut board = parse_board(s);
    let mut to_remove = Vec::new();
    let mut total_removed = 0;

    loop {
        to_remove.clear();
        to_remove.extend(find_accessible_rolls_of_paper(&board));

        if to_remove.is_empty() {
            break;
        }

        for removed in &to_remove {
            board.remove(removed);
        }
        total_removed += to_remove.len();
    }

    total_removed
}

type Board = BTreeSet<(usize, usize)>;

fn parse_board(s: &str) -> Board {
    let mut board = BTreeSet::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '@' {
                board.insert((x, y));
            }
        }
    }

    board
}

fn find_accessible_rolls_of_paper(board: &Board) -> impl Iterator<Item = (usize, usize)> {
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
        .copied()
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

    #[test]
    fn part2_example() {
        assert_eq!(43, accessible_rolls_of_paper_iterative(EXAMPLE));
    }
}
