const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_fresh_ingredients(INPUT);
    assert_eq!(821, part1);
    println!("{part1}");
}

type Id = u64;

fn n_fresh_ingredients(s: &str) -> usize {
    let mut ls = s.lines();

    let fresh_ranges = ls
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (l, u) = l.split_once("-").expect("malformed fresh range");
            let [l, u] = [l, u].map(|i| i.parse::<Id>().expect("invalid fresh id"));
            l..=u
        })
        .collect::<Vec<_>>();

    ls.map(|l| l.parse::<Id>().expect("invalid id"))
        .filter(|id| fresh_ranges.iter().any(|r| r.contains(id)))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(3, n_fresh_ingredients(EXAMPLE));
    }
}
