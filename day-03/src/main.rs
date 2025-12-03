const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = sum_of_max_joltage(INPUT);
    // Was preferring the last maximum value instead of first when equal
    assert!(part1 > 16923);
    assert_eq!(17100, part1);
    println!("{part1}");
}

fn sum_of_max_joltage(s: &str) -> u32 {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("invalid digit")))
        .map(max_joltage)
        .sum()
}

fn max_joltage(batteries: impl IntoIterator<Item = u32>) -> u32 {
    const N_BATTERIES: usize = 2;

    let batteries = batteries.into_iter().collect::<Vec<_>>();

    let mut start_idx = 0;
    (0..N_BATTERIES)
        .map(|n| {
            let end_idx = batteries.len() - (N_BATTERIES - n - 1);
            let viable_batteries = &batteries[start_idx..end_idx];

            let (max_idx, max) = viable_batteries
                .iter()
                .copied()
                .enumerate()
                .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
                .expect("Could not find a maximum");

            start_idx = max_idx + 1;

            max
        })
        .fold(0, |sum, n| sum * 10 + n)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(357, sum_of_max_joltage(EXAMPLE));
    }

    #[test]
    fn max_joltage_prefers_first_of_equal() {
        assert_eq!(66, max_joltage([6, 6]));
        assert_eq!(66, max_joltage([6, 1, 6]));
        assert_eq!(66, max_joltage([6, 1, 6, 1]));
    }
}
