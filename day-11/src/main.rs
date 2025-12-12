#![feature(result_option_map_or_default, array_windows)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_paths_to_output(INPUT);
    assert_eq!(649, part1);
    println!("{part1}");

    let part2 = n_paths_svr_to_out_via_dac_and_fft(INPUT);
    assert_eq!(458948453421420, part2);
    println!("{part2}");
}

fn n_paths_to_output(s: &str) -> usize {
    let graph = parse_graph(s);

    const START_NODE: &str = "you";
    const END_NODE: &str = "out";

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

fn n_paths_svr_to_out_via_dac_and_fft(s: &str) -> usize {
    let mut graph = parse_graph(s);
    graph.insert("out", Default::default());

    const START_NODE: &str = "svr";
    const END_NODE: &str = "out";

    const POINT1: &str = "dac";
    const POINT2: &str = "fft";

    if reachable(&graph, POINT1, POINT2) {
        n_paths_for_points(&graph, &[START_NODE, POINT1, POINT2, END_NODE])
    } else {
        n_paths_for_points(&graph, &[START_NODE, POINT2, POINT1, END_NODE])
    }
}

type Graph<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn parse_graph(s: &str) -> Graph<'_> {
    s.lines()
        .map(|l| {
            let (node, connections) = l.split_once(":").expect("Input malformed");

            let node = node.trim();
            let connections = connections
                .split_ascii_whitespace()
                .map(|n| n.trim())
                .collect();

            (node, connections)
        })
        .collect()
}

fn neighbors<'g, 'n>(graph: &'g Graph<'n>, node: &'n str) -> &'g [&'n str] {
    graph.get(node).map_or_default(|n| &n[..])
}

/// Is it possible to get from the start to the end?
fn reachable(graph: &Graph, start_node: &str, end_node: &str) -> bool {
    let mut to_visit = BTreeSet::from_iter([start_node]);

    while let Some(node) = to_visit.pop_first() {
        if node == end_node {
            return true;
        }
        to_visit.extend(neighbors(graph, node));
    }

    false
}

/// Compute the number of paths between each pair of points and
/// compute the total possible paths.
fn n_paths_for_points(graph: &Graph, points: &[&str]) -> usize {
    points
        .array_windows()
        .map(|[s, e]| {
            let within = all_nodes(graph, s, e);
            // eprintln!("{s} -> {e}: {within:?}");
            n_paths(graph, s, e, &within)
        })
        .product()
}

/// Find all possible nodes between two points
fn all_nodes<'a>(graph: &Graph<'a>, start_node: &'a str, end_node: &'a str) -> BTreeSet<&'a str> {
    fn recur<'a>(
        graph: &Graph<'a>,
        node: &'a str,
        end_node: &'a str,
        current_path: &mut Vec<&'a str>,
        all_nodes: &mut BTreeSet<&'a str>,
        dead_nodes: &mut BTreeSet<&'a str>,
    ) -> bool {
        // eprintln!("=> {node}");

        if all_nodes.contains(node) {
            // eprintln!("seen it => true");
            return true;
        }

        if dead_nodes.contains(node) {
            // eprintln!("seen it => false");
            return false;
        }

        current_path.push(node);

        let mut any_found = false;

        if node == end_node {
            //            all_nodes.extend(current_path.iter().copied());
            any_found = true;
        } else {
            match graph.get(node) {
                Some(neighbors) => {
                    for &neighbor in neighbors {
                        let newly_found = recur(
                            graph,
                            neighbor,
                            end_node,
                            current_path,
                            all_nodes,
                            dead_nodes,
                        );

                        // eprintln!("new: {neighbor} was {newly_found}");

                        any_found = any_found || newly_found;
                    }
                }

                None => {
                    dead_nodes.insert(node);
                }
            }
        }

        current_path.pop();

        // eprintln!("exit: {node} was {any_found}");

        if any_found {
            all_nodes.insert(node);
        } else {
            dead_nodes.insert(node);
        }

        any_found
    }

    let mut current_path = Default::default();
    let mut all_nodes = Default::default();
    let mut dead_nodes = Default::default();

    recur(
        graph,
        start_node,
        end_node,
        &mut current_path,
        &mut all_nodes,
        &mut dead_nodes,
    );

    all_nodes
}

/// Find all paths between two points that *only* use a set of nodes.
fn n_paths(graph: &Graph, start_node: &str, end_node: &str, within: &BTreeSet<&str>) -> usize {
    fn recur<'a>(
        graph: &Graph<'a>,
        node: &'a str,
        end_node: &'a str,
        within: &BTreeSet<&str>,
        n_paths: &mut usize,
    ) {
        if !within.contains(node) {
            return;
        }

        if node == end_node {
            *n_paths += 1;
        } else {
            for &neighbor in neighbors(graph, node) {
                recur(graph, neighbor, end_node, within, n_paths)
            }
        }
    }

    let mut n_paths = Default::default();

    recur(graph, start_node, end_node, within, &mut n_paths);

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

    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn part2_example() {
        assert_eq!(2, n_paths_svr_to_out_via_dac_and_fft(EXAMPLE2));
    }
}
