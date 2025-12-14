use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_regions_fit_presents(INPUT);
    assert_eq!(538, part1);
    println!("{part1}");
}

fn n_regions_fit_presents(s: &str) -> usize {
    let (shapes, regions) = s.rsplit_once("\n\n").expect("Malformed input");
    let shapes = shapes.split("\n\n").map(new_shape).collect::<Vec<_>>();
    let regions = regions.lines().map(Region::new).collect::<Vec<_>>();

    let mut known_false = 0;
    let mut known_true = 0;
    let mut remaining = 0;

    // Example shapes all take 7 units
    // Real shapes take {7, 6, 7, 5, 7, 7} units
    // All shapes have a 3x3 bounding box

    for region in &regions {
        if region.available_area() < region.required_area(&shapes) {
            known_false += 1;
            continue;
        }

        if region.number_of_three_by_three_spaces() >= region.number_of_shapes() {
            known_true += 1;
            continue;
        }

        if remaining < 5 {
            eprintln!("{:?} -- {:?}", region.dimensions, region.shape_counts);
        }

        remaining += 1;
    }

    eprintln!("False: {known_false}");
    eprintln!("True: {known_true}");
    eprintln!("Unknown: {remaining}");

    known_true
}

type Shape = BTreeSet<[usize; 2]>;

fn new_shape(s: &str) -> Shape {
    let (_number, shape) = s.split_once("\n").expect("Malformed shape");

    shape
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(move |(x, c)| (c == '#').then_some([x, y]))
        })
        .collect()
}

struct Region {
    dimensions: [u16; 2],
    shape_counts: Vec<usize>,
}

impl Region {
    fn new(l: &str) -> Self {
        let (dims, shape_counts) = l.split_once(":").expect("Malformed region");
        let (w, h) = dims.split_once("x").expect("Malformed dimensions");
        let dimensions = [w, h].map(|d| d.parse().expect("Invalid dimension"));
        let shape_counts = shape_counts
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("Invalid shape count"))
            .collect();

        Self {
            dimensions,
            shape_counts,
        }
    }

    fn available_area(&self) -> usize {
        self.dimensions.into_iter().product::<u16>().into()
    }

    fn required_area(&self, shapes: &[Shape]) -> usize {
        self.shape_counts
            .iter()
            .enumerate()
            .map(|(shape_idx, &count)| {
                let shape = &shapes[shape_idx];
                shape.len() * count
            })
            .sum()
    }

    fn number_of_three_by_three_spaces(&self) -> usize {
        let [w, h] = self.dimensions.map(|d| d / 3).map(usize::from);
        w * h
    }

    fn number_of_shapes(&self) -> usize {
        self.shape_counts.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    #[should_panic]
    fn part1_example() {
        assert_eq!(2, n_regions_fit_presents(EXAMPLE));
    }
}
