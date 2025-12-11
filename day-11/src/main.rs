use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

const START_NODE: &str = "you";
const END_NODE: &str = "out";

fn main() {
    let part1 = n_paths_to_output(INPUT);
    assert_eq!(649, part1);
    println!("{part1}");
}

fn n_paths_to_output(s: &str) -> usize {
    let graph = s
        .lines()
        .map(|l| {
            let (node, connections) = l.split_once(":").expect("Input malformed");

            let node = node.trim();
            let connections = connections
                .split_ascii_whitespace()
                .map(|n| n.trim())
                .collect();

            (node, connections)
        })
        .collect::<BTreeMap<_, Vec<_>>>();

    let mut to_visit = vec![START_NODE];
    let mut n_paths = 0;

    while let Some(node) = to_visit.pop() {
        if node == END_NODE {
            n_paths += 1;
        } else {
            for &connection in &graph[node] {
                to_visit.push(connection);
            }
        }
    }

    n_paths
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(5, n_paths_to_output(EXAMPLE));
    }
}
