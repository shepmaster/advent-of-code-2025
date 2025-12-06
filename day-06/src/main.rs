use std::ops;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = cephalopod_math_sum(INPUT);
    assert_eq!(4583860641327, part1);
    println!("{part1}");

    let part2 = cephalopod_math_explained_sum(INPUT);
    assert_eq!(11602774058280, part2);
    println!("{part2}");
}

fn cephalopod_math_sum(s: &str) -> u64 {
    let mut numbers = s.lines();
    let operations = numbers.next_back().expect("No operations");
    let operations = operations.split_ascii_whitespace();

    let mut numbers = numbers.map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<u64>().expect("Invalid number"))
    });

    let head = numbers
        .next()
        .expect("Need initial values")
        .collect::<Vec<_>>();

    let results = numbers.fold(head, |mut acc, n| {
        for ((a, n), op) in acc.iter_mut().zip(n).zip(operations.clone()) {
            if op == "+" {
                *a += n;
            } else {
                *a *= n;
            }
        }
        acc
    });

    results.into_iter().sum()
}

fn cephalopod_math_explained_sum(s: &str) -> u64 {
    let mut numbers = s.lines();
    let operations = numbers.next_back().expect("No operations");

    // Convert from strings to Vec / Option / u64
    let numbers = numbers
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(u64::from))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find the bounds of the 2D Vec
    let h = numbers.len();
    let w = numbers.first().map(|n| n.len()).unwrap_or(0);

    // Build up the numbers by walking right-to-left The most
    // significant number occurs before other digits, but that doesn't
    // mean that it is on the first line, so we keep the `None`
    // instead of making everything a zero.
    let mut numbers = (0..w).rev().map(|x| {
        (0..h)
            .map(|y| numbers[y][x])
            .reduce(|l, r| match (l, r) {
                (None, None) => None,
                (None, v) | (v, None) => v,
                (Some(l), Some(r)) => Some(l * 10 + r),
            })
            .flatten()
    });

    operations
        .split_ascii_whitespace()
        .rev()
        .map(|op| {
            let op = if op == "+" {
                ops::Add::add
            } else {
                ops::Mul::mul
            };

            // A whole column of `None` corresponds to the end of a
            // problem, so we group by that.
            numbers
                .by_ref()
                .take_while(|n| n.is_some())
                .flatten()
                .reduce(op)
                .expect("Didn't perform any operations")
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(4277556, cephalopod_math_sum(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(3263827, cephalopod_math_explained_sum(EXAMPLE));
    }
}
