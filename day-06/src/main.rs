const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = cephalopod_math_sum(INPUT);
    assert_eq!(4583860641327, part1);
    println!("{part1}");
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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(4277556, cephalopod_math_sum(EXAMPLE));
    }
}
