#![feature(exact_length_collection, gen_blocks, yield_expr)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = largest_rectangle_area(INPUT);
    assert_eq!(4741848414, part1);
    println!("{part1}");
}

type Dimension = u64;

fn largest_rectangle_area(s: &str) -> Dimension {
    // Misread the problem statement so I parsed a bit
    // differently. Keeping it like this until we see part 2.
    let mut by_x = BTreeMap::new();
    // let mut by_y = BTreeMap::new();

    let coords = s.lines().map(|l| {
        l.split(",")
            .map(|d| d.parse::<Dimension>().expect("Invalid dimension"))
            .collect_array()
            .expect("Wrong number of dimensions for coordinate")
    });

    for [x, y] in coords {
        by_x.entry(x).or_insert_with(BTreeSet::new).insert(y);
        // by_y.entry(y).or_insert_with(BTreeSet::new).insert(x);
    }

    // assert!(
    //     by_x.iter().all(|(_x, ys)| ys.len() >= 2),
    //     "Not all X coordinates form a line",
    // );
    // assert!(
    //     by_y.iter().all(|(_y, xs)| xs.len() >= 2),
    //     "Not all Y coordinates form a line",
    // );

    let coords = by_x
        .iter()
        .flat_map(|(x, ys)| ys.iter().map(move |y| [x, y]));

    let max = iter_pairs(coords)
        .map(|(a, b)| {
            let area = a
                .into_iter()
                .zip(b)
                .map(|(&a, &b)| Dimension::abs_diff(a, b) + 1)
                .product();

            // eprintln!("{a:2?} x {b:2?} == {area:3}");

            area
        })
        .max();

    max.expect("No maximum found")
}

fn iter_pairs<I>(i: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator,
    I::IntoIter: Clone,
    I::Item: Copy,
{
    let mut i = i.into_iter();

    gen move {
        while let Some(head) = i.next() {
            for tail in i.clone() {
                yield (head, tail);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(50, largest_rectangle_area(EXAMPLE));
    }
}
