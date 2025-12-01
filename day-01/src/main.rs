const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = password(INPUT);
    assert_eq!(1097, part1);
    println!("{part1}");
}

fn password(s: &str) -> usize {
    let mut dial = 50_u32;
    s.lines()
        .map(|l| {
            let (direction, n) = if let Some(n) = l.strip_prefix("L") {
                (-1, n)
            } else if let Some(n) = l.strip_prefix("R") {
                (1, n)
            } else {
                panic!("Unknown direction");
            };

            let n = n.parse::<i32>().expect("Invalid amount");
            let n = n * direction;
            let n = n.rem_euclid(100);
            dial = dial.strict_add_signed(n);
            dial %= 100;

            dial
        })
        .filter(|&d| d == 0)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(3, password(EXAMPLE));
    }
}
