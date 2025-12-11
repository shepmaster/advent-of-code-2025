#![feature(uint_bit_width)]

use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    ops,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = sum_of_minimum_presses(INPUT);
    assert_eq!(491, part1);
    println!("{part1}");

    let part2 = sum_of_minimum_joltage_presses(INPUT);
    // Left my debugging hacks that didn't panic on not-found
    assert!(part2 > 19978);
    assert_eq!(20617, part2);
    println!("{part2}");
}

fn sum_of_minimum_presses(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let machine = Machine::new(l);
            machine
                .minimum_button_sequence()
                .expect("Did not find minimum button sequence")
        })
        .sum()
}

fn sum_of_minimum_joltage_presses(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let machine = Machine::new(l);
            machine
                .minimum_joltage_button_sequence()
                .expect("Did not find minimum button sequence")
        })
        .sum()
}

type Joltage = u16;

struct Machine {
    diagram: u16,
    buttons: Vec<u16>,
    joltage: Vec<Joltage>,
}

impl Machine {
    fn new(l: &str) -> Self {
        let mut parts = l.split_ascii_whitespace();

        let diagram = parts.next().expect("missing diagram");
        let diagram = diagram.trim_matches(['[', ']']);

        assert!((1..=16).contains(&diagram.len()));
        let diagram = diagram
            .chars()
            .map(|c| (c == '#') as u16)
            .rev()
            .fold(0u16, |acc, bit| acc << 1 | bit);

        let joltage = parts.next_back().expect("missing joltage");
        let joltage = joltage.trim_matches(['{', '}']);
        let joltage = joltage
            .split(',')
            .map(|j| j.parse().expect("Invalid joltage"))
            .collect();

        let buttons = parts
            .map(|button| {
                let button = button.trim_matches(['(', ')']);
                button
                    .split(',')
                    .map(|b| b.parse::<Joltage>().expect("Invalid button index"))
                    .fold(0u16, |acc, bit| acc | 1 << bit)
            })
            .collect();

        Self {
            diagram,
            buttons,
            joltage,
        }
    }

    fn minimum_button_sequence(&self) -> Option<usize> {
        let mut visited = BTreeSet::new();
        visited.insert(0u16);
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0usize, 0u16));

        while let Some((depth, lights)) = to_visit.pop_front() {
            if lights == self.diagram {
                return Some(depth);
            }

            for b in &self.buttons {
                let next_lights = lights ^ b;

                if visited.insert(next_lights) {
                    to_visit.push_back((depth + 1, next_lights));
                }
            }
        }

        None
    }

    /// The general idea is that each button impacts a set of
    /// counters. We can set up a system of equations mapping each
    /// button to those counters and set that equal to the counter
    /// joltage. We also get an equation for the total number of
    /// presses.
    ///
    /// For example, if we have button `b0 (0, 2)` and `b1 (1, 2)`
    /// with a target joltage of `{1,2,3}`, we can set this up as the
    /// matrix:
    ///
    /// ```
    /// b0 b1  j
    ///  1  0  1 // buttons contributing to j0
    ///  0  1  2 // buttons contributing to j1
    ///  1  1  3 // buttons contributing to j2
    ///  1  1  N // total button pushes
    /// ```
    ///
    /// `N` is what we are solving for, but we can get a range of
    /// values to look in to prune the space down.
    ///
    /// We also sometimes get a system of equations that leaves free
    /// variables. In that case, we search all possibilities of the
    /// free variables up to the total number of button pushes.
    fn minimum_joltage_button_sequence(&self) -> Option<usize> {
        let do_dbug = false;

        if do_dbug {
            eprintln!("Target jolts: {:?}", self.joltage);
        }

        let expanded_buttons = self.expanded_buttons();

        let mut jolts_affected_by = vec![vec![]; self.joltage.len()];

        for (btn_idx, button) in expanded_buttons.iter().enumerate() {
            for (jolt_idx, &b) in button.iter().enumerate() {
                if b != 0 {
                    jolts_affected_by[jolt_idx].push(btn_idx);
                }
            }
        }

        if do_dbug {
            for (idx, b) in expanded_buttons.iter().enumerate() {
                eprintln!("[btn {idx}]: {b:?}");
            }

            for (idx, bs) in jolts_affected_by.iter().enumerate() {
                eprintln!(
                    "[jolt {idx}]: affected by buttons {bs:?}, must sum to {}",
                    self.joltage[idx]
                );
            }
        }

        // Constructing this matrix could probably be simplified, but
        // the intermediate variables are useful for debug printing.
        let mut matrix = jolts_affected_by
            .iter()
            .zip(&self.joltage)
            .map(|(bs, j)| {
                (0..expanded_buttons.len())
                    .map(|k| if bs.contains(&k) { 1 } else { 0 })
                    .chain([*j])
                    .map(MatrixVal::from)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if do_dbug {
            eprintln!("Initial matrix:");
            dump(&matrix);
        }

        let total_presses = vec![1; matrix[0].len()];
        matrix.push(total_presses);

        self.search_range()
            .find(|&n_button_presses| {
                let n_button_presses = MatrixVal::from(n_button_presses);

                if do_dbug {
                    eprintln!("Trying {n_button_presses} presses");
                }

                let mut matrix = matrix.clone();

                // Set the last equation to equal our total button presses;
                *matrix.last_mut().unwrap().last_mut().unwrap() = n_button_presses;

                gaussian_elimination(&mut matrix);
                let Some(solutions) = solve_matrix(&matrix, n_button_presses) else {
                    return false;
                };

                // Just double-checking
                let sum = solutions.iter().copied().sum::<MatrixVal>();
                assert_eq!(sum, n_button_presses);

                if do_dbug {
                    eprintln!("Found solution! {solutions:?}");
                }

                true
            })
            .map(Into::into)
    }

    #[expect(dead_code)]
    fn minimum_joltage_button_sequence_meh(&self) -> Option<usize> {
        eprintln!("Target jolts: {:?}", self.joltage);

        // Check the buttons that toggle the most first
        // expanded_buttons.sort_by_key(|b| b.iter().filter(|&&b| b != 0).count());
        // expanded_buttons.reverse();

        let mut button_maxes = self
            .expanded_buttons()
            .into_iter()
            .map(|button| {
                let max_presses = button
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, b)| {
                        let v = self.joltage[idx] * b;
                        if v == 0 { None } else { Some(v as usize) }
                    })
                    .min()
                    .expect("Button will never");

                (max_presses, button)
            })
            .collect::<Vec<_>>();

        button_maxes.sort_by_key(|&(m, _)| m);
        //        button_maxes.reverse();
        let (button_maxes, expanded_buttons): (Vec<_>, Vec<_>) = button_maxes.into_iter().unzip();

        for (idx, b) in expanded_buttons.iter().enumerate() {
            eprintln!("[btn {idx}]: {b:?}");
        }

        let mut scratch = vec![0; self.joltage.len()];

        eprintln!("Maximum times each button may be pressed: {button_maxes:?}");

        let mut search_range = self.search_range();
        eprintln!("Checking total presses in the range {search_range:?}");

        search_range
            .find(|&total_presses| {
                let total_presses = usize::from(total_presses);

                eprintln!("=={total_presses}==");

                distribute_balls_indistinguishable(total_presses, &button_maxes, |button_presses| {
                    // eprintln!("Testing button presses: {button_presses:?}");

                    scratch.fill(0);

                    for (btn, &presses) in expanded_buttons.iter().zip(button_presses) {
                        for (s, b) in scratch.iter_mut().zip(btn) {
                            let presses = Joltage::try_from(presses).unwrap_or(0);
                            *s += b * presses;
                        }
                    }

                    scratch == self.joltage
                })
            })
            .map(Into::into)
    }

    #[expect(dead_code)]
    fn minimum_joltage_button_sequence_naive(&self) -> Option<usize> {
        use std::sync::Arc;
        let joltage = Arc::<[Joltage]>::from(self.joltage.clone());

        let mut visited = BTreeSet::new();
        visited.insert(joltage.clone());
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0usize, joltage));

        while let Some((depth, joltage)) = to_visit.pop_front() {
            if joltage.iter().all(|&j| j == 0) {
                return Some(depth);
            }

            'button: for b in self.buttons.iter().rev() {
                let mut next_joltage = joltage.to_vec();

                for bit in 0..=b.bit_width() {
                    if b & (1 << bit) != 0 {
                        let idx = usize::try_from(bit).expect("Bit out of range");
                        match joltage[idx].checked_sub(1) {
                            Some(j) => next_joltage[idx] = j,
                            None => continue 'button,
                        }
                    }
                }

                let next_joltage = Arc::<[Joltage]>::from(next_joltage);
                if visited.insert(next_joltage.clone()) {
                    to_visit.push_back((depth + 1, next_joltage));
                }
            }
        }

        None
    }

    fn expanded_buttons(&self) -> Vec<Vec<Joltage>> {
        self.buttons
            .iter()
            .map(|b| {
                let mut j = vec![0; self.joltage.len()];
                for (idx, j) in j.iter_mut().enumerate() {
                    if b & (1 << idx) != 0 {
                        *j = 1;
                    }
                }
                j
            })
            .collect()
    }

    fn search_range(&self) -> ops::RangeInclusive<Joltage> {
        // If there's one button for all joltages
        let min_presses = self.joltage.iter().copied().max().unwrap_or(0);
        // If there's a unique button for each joltage
        let max_presses = self.joltage.iter().copied().sum();

        min_presses..=max_presses
    }
}

type Matrix = Vec<Vec<MatrixVal>>;
type MatrixVal = i32;

fn dump(matrix: &Matrix) {
    for r in matrix {
        for c in r {
            eprint!("{c:2} ");
        }
        eprintln!();
    }
}

// https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
// https://en.wikipedia.org/wiki/Bareiss_algorithm
//
// Rewritten from scratch to use zero-based indices (with
// upper-exclusive ranges) and for better understanding.  Also
// attempts to convert to row-reduced echelon form (RREF).
#[expect(clippy::needless_range_loop)]
fn gaussian_elimination(matrix: &mut Matrix) {
    let do_dbug = false;

    if do_dbug {
        dump(matrix);
    }

    let height = matrix.len();
    let width = matrix[0].len();

    let mut row = 0;
    let mut col = 0;

    while row < height && col < width {
        if do_dbug {
            eprintln!("--- {row},{col}");
            dump(matrix);
        }

        // Find a row closest to 1 so we are more likely to divide evenly
        let max_row = (row..height)
            .min_by(|&i0, &i1| {
                let [v0, v1] = [i0, i1].map(|i| matrix[i][col]);

                // but not 0!
                let [d0, d1] = [v0, v1].map(|e| {
                    let e = e.abs();
                    if e == 0 { MatrixVal::MAX } else { e }
                });

                // Prefer earlier rows in case of a tie
                d0.cmp(&d1).then(i0.cmp(&i1).reverse())
            })
            .unwrap();

        if matrix[max_row][col] == 0 {
            if do_dbug {
                eprintln!("shifting right");
            }
            col += 1;
            continue;
        }

        if max_row != row {
            matrix.swap(max_row, row);

            if do_dbug {
                eprintln!("swapped");
                dump(matrix);
            }
        }

        let pivot = matrix[row][col];

        if pivot < 0 {
            for c in col..width {
                matrix[row][c] *= -1;
            }

            if do_dbug {
                eprintln!("negated");
                dump(matrix);
            }
        }

        let pivot = matrix[row][col];

        if pivot != 1 {
            if (col..width).all(|c| matrix[row][c] % pivot == 0) {
                for c in col..width {
                    matrix[row][c] /= pivot;
                }
            }

            if do_dbug {
                eprintln!("normalized");
                dump(matrix);
            }
        }

        let pivot = matrix[row][col];

        // Clean up rows below this one
        for r in row + 1..height {
            let column_to_zero = matrix[r][col];

            if column_to_zero != 0 {
                if do_dbug {
                    eprintln!("row needs to be fixed {:?}", &matrix[r]);
                }

                for c in col..width {
                    matrix[r][c] *= pivot;
                    matrix[r][c] -= column_to_zero * matrix[row][c];
                }

                if do_dbug {
                    eprintln!("row now {:?}", &matrix[r]);
                }
            }
        }

        // Clean up rows above this one
        if pivot == 1 {
            for r in 0..row {
                let column_to_zero = matrix[r][col];

                if column_to_zero != 0 {
                    if do_dbug {
                        eprintln!("row needs to be fixed {:?}", &matrix[r]);
                    }

                    for c in col..width {
                        matrix[r][c] -= column_to_zero * matrix[row][c];
                    }

                    if do_dbug {
                        eprintln!("row now {:?}", &matrix[r]);
                    }
                }
            }
        }

        row += 1;
        col += 1;
    }

    if do_dbug {
        eprintln!("Gaussian elimination:");
        dump(matrix);
    }
}

fn solve_matrix(matrix: &Matrix, max: MatrixVal) -> Option<Vec<MatrixVal>> {
    let do_dbug = false;

    let n_variables = matrix[0].len() - 1;

    let pivot_columns = matrix
        .iter()
        .flat_map(|r| r.iter().enumerate().find(|&(_idx, &v)| v != 0))
        .collect::<BTreeMap<_, _>>();

    let free_variables = (0..n_variables)
        .filter(|var_idx| !pivot_columns.contains_key(var_idx))
        .collect::<BTreeSet<_>>();

    let mut solutions = vec![0; n_variables];

    if do_dbug {
        eprintln!(
            "=> {} free variables: {free_variables:?}",
            free_variables.len(),
        );
    }

    // `multi_cartesian_product` produces a single empty `Vec` when
    // the iterator is empty, which works quite nicely here!
    let mut free_variable_candidates = free_variables
        .iter()
        .map(|&fvi| (0..=max).map(move |fv| (fvi, fv)))
        .multi_cartesian_product();

    let solved = free_variable_candidates.any(|fv| {
        solutions.fill(0);

        for (fvi, fv) in fv {
            if do_dbug {
                eprintln!("Setting x{fvi} = {fv}");
            }
            solutions[fvi] = fv;
        }

        solve_matrix_one(matrix, &mut solutions)
    });

    solved.then_some(solutions)
}

fn solve_matrix_one(matrix: &Matrix, solutions: &mut [MatrixVal]) -> bool {
    let do_dbug = false;

    for row in matrix.iter().rev() {
        if do_dbug {
            eprintln!("--");
        }

        let [coeffs @ .., rhs] = &row[..] else {
            panic!("malformed")
        };

        if do_dbug {
            eprintln!("{coeffs:?} => {rhs}");
        }

        let mut nonzero_coeffs = coeffs
            .iter()
            .copied()
            .enumerate()
            .skip_while(|&(_, c)| c == 0);
        let Some((c_idx, c)) = nonzero_coeffs.next() else {
            if *rhs == 0 {
                if do_dbug {
                    eprintln!("zero row; useless");
                }

                continue;
            } else {
                if do_dbug {
                    eprintln!("inconsistent");
                }
                return false;
            }
        };

        if do_dbug {
            eprint!("{c} * x{c_idx}");
        }

        let sum = nonzero_coeffs
            .map(|(c2_idx, c2)| {
                if do_dbug {
                    #[allow(clippy::collapsible_if)]
                    if c2 != 0 {
                        eprint!(" + {c2} * x{c2_idx}");
                    }
                }

                c2 * solutions[c2_idx]
            })
            .sum::<MatrixVal>();

        if do_dbug {
            eprintln!(" = {rhs}");
        }

        let res = rhs - sum;

        if res % c != 0 {
            if do_dbug {
                eprintln!("No integer solution");
                eprintln!("solutions[{c_idx}] = {res} / {c}");
                eprintln!("{solutions:?}");
            }
            return false;
        }

        let res = res / c;

        if res < 0 {
            if do_dbug {
                eprintln!("No positive solution");
                eprintln!("solutions[{c_idx}] = {res}");
                eprintln!("{solutions:?}");
            }
            return false;
        }

        solutions[c_idx] = res;
    }

    true
}

fn distribute_balls_indistinguishable(
    n_balls: usize,
    // 0    m_buckets: usize,
    // buttons: &[Vec<Joltage>],
    max_per_bucket: &[usize],
    mut check: impl FnMut(&[usize]) -> bool,
) -> bool {
    let m_buckets = max_per_bucket.len();
    let mut distribution = vec![0; m_buckets];
    distribute_balls_indistinguishable_inner(
        n_balls,
        m_buckets,
        max_per_bucket,
        &mut distribution,
        &mut check,
    )
}

fn distribute_balls_indistinguishable_inner(
    n_balls: usize,
    m_buckets: usize,
    // buttons: &[Vec<Joltage>],
    max_per_bucket: &[usize],
    distribution: &mut [usize],
    check: &mut impl FnMut(&[usize]) -> bool,
) -> bool {
    // eprintln!("> {n_balls} x {m_buckets}");

    let start_idx = distribution.len() - m_buckets;
    let focus = &mut distribution[start_idx..];

    if n_balls == 0 {
        focus.fill(0);
        return check(distribution);
    }

    match focus {
        [] => unreachable!(),

        [head] => {
            *head = n_balls;
            check(distribution)
        }

        [_head, ..] => {
            let limit = max_per_bucket[start_idx];
            let local_max = usize::min(n_balls, limit);
            // eprintln!("{n_balls} vs {limit}");

            (0..=local_max).rev().any(|balls_in_first_bucket| {
                distribution[start_idx] = balls_in_first_bucket;
                let remaining_balls = n_balls - balls_in_first_bucket;

                // eprintln!(">> {balls_in_first_bucket} + {remaining_balls}");

                distribute_balls_indistinguishable_inner(
                    remaining_balls,
                    m_buckets - 1,
                    max_per_bucket,
                    distribution,
                    check,
                )
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(7, sum_of_minimum_presses(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(33, sum_of_minimum_joltage_presses(EXAMPLE));
    }

    #[test]
    fn gaussian_elimination_exercise() {
        let mut m = qmatrix(
            " 0  0  1  1  0  1  0  1  0  1  1  0 57
              0  0  1  0  1  0  0  0  1  0  0  0 31
              0  0  0  1  1  0  0  1  0  1  1  0 44
              0  0  1  0  0  0  0  1  0  1  1  1 54
              0  0  1  1  0  1  1  0  0  0  1  1 68
              0  1  0  1  0  1  0  0  1  0  0  0 54
              1  0  0  1  0  0  1  0  1  0  1  0 52
              0  1  0  0  0  1  0  1  0  0  1  0 48
              0  1  0  0  1  0  0  1  0  0  1  1 62
              0  0  1  1  0  0  0  1  1  0  1  0 47
              1  1  1  1  1  1  1  1  1  1  1  1 73",
        );

        gaussian_elimination(&mut m);
    }

    #[test]
    fn solve_matrix_exercise() {
        let m = qmatrix(
            " 1  0  0  0  0  0  0  2  0  0  4  0  3 120
              0  1  0  0  0  0  0  1  1  0  2  0  1 55
              0  0  1  0  0  0  0  2  0  0  3 -1  2 65
              0  0  0  1  0  0  0 -2  0  0 -5  0 -3 -116
              0  0  0  0  1  0  0  1  0  0  3  1  2 88
              0  0  0  0  0  1  0 -2  0  0 -4  1 -3 -86
              0  0  0  0  0  0  1 -1  0  0 -2  1 -1 -10
              0  0  0  0  0  0  0  2  1  0  4  1  2 101
              0  0  0  0  0  0  0  0  3  0  0  3 -4 -23
              0  0  0  0  0  0  0  0  0  1  0 -1  0 -5
              0  0  0  0  0  0  0  0  0  0  6  6 10 338",
        );

        let _solutions = solve_matrix(&m, 338).unwrap();
    }

    fn qmatrix(s: &str) -> Matrix {
        s.lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}
