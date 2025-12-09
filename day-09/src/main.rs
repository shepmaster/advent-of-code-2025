#![feature(exact_length_collection, gen_blocks, iter_array_chunks, yield_expr)]

use std::{
    collections::{BTreeMap, BTreeSet},
    ops,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = largest_rectangle_area(INPUT);
    assert_eq!(4741848414, part1);
    println!("{part1}");

    let part2 = largest_filled_rectangle_area(INPUT);
    assert_eq!(1508918480, part2);
    println!("{part2}");
}

type Dimension = u64;

fn largest_rectangle_area(s: &str) -> Dimension {
    let tiles = Tiles::new(s);
    let coords = tiles.coordinates();

    let max = iter_pairs(coords)
        .map(|(a, b)| Coordinate::area(a, b))
        .max();

    max.expect("No maximum found")
}

fn largest_filled_rectangle_area(s: &str) -> Dimension {
    let tiles = Tiles::new(s);
    let coords = tiles.coordinates();

    let max = iter_pairs(coords)
        .filter_map(|(a, b)| {
            let Coordinate([x0, y0]) = a;
            let Coordinate([x1, y1]) = b;

            fn sort_inline<T: Ord, const N: usize>(mut v: [T; N]) -> [T; N] {
                v.sort();
                v
            }

            let [x0, x1] = sort_inline([x0, x1]);
            let [y0, y1] = sort_inline([y0, y1]);

            // Moving left-to-right...
            let n_x_intersections = tiles
                .x_lines_in_range(x0..=x1)
                .filter(|y_line| {
                    // ... on the top or bottom
                    y_line.contains(&y0) || y_line.contains(&y1)
                })
                .count();

            // Moving top-to-bottom...
            let n_y_intersections = tiles
                .y_lines_in_range(y0..=y1)
                .filter(|x_line| {
                    // ... on the left or right
                    x_line.contains(&x0) || x_line.contains(&x1)
                })
                .count();

            let only_intersects_self = n_x_intersections <= 2 && n_y_intersections <= 2;

            only_intersects_self.then(|| Coordinate::area(a, b))
        })
        .max();

    max.expect("No maximum found")
}

struct Tiles {
    by_x: IndexedPoints,
    by_y: IndexedPoints,
}

type RangeThing = ops::RangeInclusive<Dimension>;

impl Tiles {
    fn new(s: &str) -> Self {
        let mut by_x = BTreeMap::new();
        let mut by_y = BTreeMap::new();

        let coords = s.lines().map(|l| {
            l.split(",")
                .map(|d| d.parse::<Dimension>().expect("Invalid dimension"))
                .collect_array()
                .expect("Wrong number of dimensions for coordinate")
        });

        for [x, y] in coords {
            by_x.entry(x).or_insert_with(BTreeSet::new).insert(y);
            by_y.entry(y).or_insert_with(BTreeSet::new).insert(x);
        }

        let by_x = IndexedPoints(by_x);
        let by_y = IndexedPoints(by_y);

        assert!(by_x.forms_line(), "Not all X coordinates form a line");
        assert!(by_y.forms_line(), "Not all Y coordinates form a line");

        Tiles { by_x, by_y }
    }

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> + Clone {
        self.by_x.coordinates()
    }

    fn x_lines_in_range(&self, range: RangeThing) -> impl Iterator<Item = RangeThing> {
        self.by_x.lines_in_range(range)
    }

    fn y_lines_in_range(&self, range: RangeThing) -> impl Iterator<Item = RangeThing> {
        self.by_y.lines_in_range(range)
    }
}

struct IndexedPoints(BTreeMap<Dimension, BTreeSet<Dimension>>);

impl IndexedPoints {
    fn forms_line(&self) -> bool {
        self.0
            .iter()
            .all(|(_maj, minors)| !minors.is_empty() && minors.len().is_multiple_of(2))
    }

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> + Clone {
        self.0
            .iter()
            .flat_map(|(x, ys)| ys.iter().map(move |y| Coordinate([*x, *y])))
    }

    fn lines_in_range(&self, range: RangeThing) -> impl Iterator<Item = RangeThing> {
        self.0
            .range(range)
            .flat_map(|(_major, minors)| minors.iter().array_chunks().map(|[&yy1, &yy2]| yy1..=yy2))
    }
}

#[derive(Debug, Copy, Clone)]
struct Coordinate([u64; 2]);

impl Coordinate {
    #[expect(clippy::let_and_return)]
    fn area(self, other: Self) -> Dimension {
        let area = self
            .0
            .into_iter()
            .zip(other.0)
            .map(|(a, b)| Dimension::abs_diff(a, b) + 1)
            .product();

        // eprintln!("{a:2?} x {b:2?} == {area:3}");

        area
    }
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

    #[test]
    fn part2_example() {
        assert_eq!(24, largest_filled_rectangle_area(EXAMPLE));
    }
}
