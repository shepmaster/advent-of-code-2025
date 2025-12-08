#![feature(exact_length_collection)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = product_of_top_three_largest_circuits::<1000>(INPUT);
    assert_eq!(103488, part1);
    println!("{part1}");
}

type Dimension = u32;
type Point = [Dimension; 3];
type Magnitude = u64;

fn product_of_top_three_largest_circuits<const N_PAIRS: usize>(s: &str) -> usize {
    let junction_boxes = s
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse().expect("Invalid number"))
                .collect_array()
                .expect("Needed exactly 3 numbers")
        })
        .collect::<Vec<Point>>();

    // Mapping of "distance" to a pair of junction boxes
    let mut distances = Vec::new();

    let mut remaining = &junction_boxes[..];
    while let &[head, ref next_remaining @ ..] = remaining {
        for &next in next_remaining {
            let mut pair = [head, next];
            pair.sort(); // Just for readability in debug output
            distances.push((distance_magnitude(head, next), pair));
        }
        remaining = next_remaining;
    }

    // By shortest distance
    distances.sort_by_key(|&(d, _)| d);

    let mut isolated_boxes = BTreeSet::from_iter(junction_boxes);

    // Using a map here so that we can have stable IDs, which is
    // really just for debug output. A vector where the IDs were the
    // indices and changed over time worked fine.
    let mut circuits = BTreeMap::<usize, BTreeSet<_>>::new();
    let mut circuit_id = 0;
    let mut get_new_id = || {
        let id = circuit_id;
        circuit_id += 1;
        id
    };

    for (_distance, [a, b]) in distances.into_iter().take(N_PAIRS) {
        let find_circuit = |pt| {
            circuits
                .iter()
                .find(|(_, c)| c.contains(&pt))
                .map(|(id, _)| *id)
        };

        let circuit_a = find_circuit(a);
        let circuit_b = find_circuit(b);

        match (circuit_a, circuit_b) {
            // Neither junction box is in a circuit. Create a new
            // circuit with the two of them
            (None, None) => {
                let id = get_new_id();
                // eprintln!("Adding {a:?} and {b:?} to new circuit {id}");

                assert!(isolated_boxes.remove(&a));
                assert!(isolated_boxes.remove(&b));

                let circuit = BTreeSet::from_iter([a, b]);
                circuits.insert(id, circuit);
            }

            // A is not in a circuit, but B is. Add A to B's circuit.
            (None, Some(circuit_b)) => {
                // eprintln!("Adding {a:?} to circuit {circuit_b} ({b:?})");

                assert!(isolated_boxes.remove(&a));

                circuits
                    .get_mut(&circuit_b)
                    .expect("circuit missing")
                    .insert(a);
            }

            // B is not in a circuit, but A is. Add B to A's circuit.
            (Some(circuit_a), None) => {
                // eprintln!("Adding {b:?} to circuit {circuit_a} ({a:?})");

                assert!(isolated_boxes.remove(&b));

                circuits
                    .get_mut(&circuit_a)
                    .expect("circuit missing")
                    .insert(b);
            }

            // Both junction boxes are in circuits.
            (Some(circuit_a), Some(circuit_b)) => {
                if circuit_a == circuit_b {
                    // Already in the same circuit, nothing to do.
                    // eprintln!("{a:?} and {b:?} are in the same circuit");
                } else {
                    // Merge the circuits
                    let id = circuit_a;
                    // eprintln!("Merging circuit {circuit_b} ({b:?}) into {circuit_a} ({a:?})");

                    let mut circuit_a = circuits.remove(&circuit_a).expect("Circuit missing");
                    let circuit_b = circuits.remove(&circuit_b).expect("Circuit missing");
                    circuit_a.extend(circuit_b);
                    circuits.insert(id, circuit_a);
                }
            }
        }
    }

    let mut circuit_sizes = circuits.values().map(|c| c.len()).collect::<Vec<_>>();
    circuit_sizes.sort();

    circuit_sizes.iter().rev().take(3).product()
}

// Used for comparison, not exact distance
fn distance_magnitude(a: Point, b: Point) -> Magnitude {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| Magnitude::from(Dimension::abs_diff(a, b)).pow(2))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(40, product_of_top_three_largest_circuits::<10>(EXAMPLE));
    }
}
