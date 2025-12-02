#![feature(int_format_into)]

use core::fmt::NumBuffer;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = sum_of_invalid_ids(INPUT);
    assert_eq!(12586854255, part1);
    println!("{part1}");

    let part2 = sum_of_all_invalid_ids(INPUT);
    assert_eq!(17298174201, part2);
    println!("{part2}");
}

fn sum_of_invalid_ids(s: &str) -> u64 {
    id_ranges(s)
        .map(|(s, e)| s..=e)
        // Find the possible bounds of paired upper / lower numbers
        .filter_map(|orig_range| {
            let s = orig_range.clone().filter_map(upper).next();
            let e = orig_range.clone().rev().filter_map(upper).next();

            Some((orig_range, s?..=e?))
        })
        // Find all possible upper / lower pairs
        .flat_map(|(orig_range, search_range)| {
            search_range.flat_map(move |n| {
                let n_digits = n_digits(n);
                let k = 10u64.pow(n_digits);
                let v = n * k + n;

                // Check we are still in-bounds of the original range
                orig_range.contains(&v).then_some(v)
            })
        })
        .sum()
}

fn sum_of_all_invalid_ids(s: &str) -> u64 {
    id_ranges(s)
        .flat_map(|(s, e)| {
            (s..=e).flat_map(|n| {
                let mut buf = NumBuffer::new();
                let buf = n.format_into(&mut buf);
                let buf = buf.as_bytes();

                let mut sequence_lengths = 1..buf.len();
                let any_sequence_repeated = sequence_lengths.any(|l| all_chunks_same(buf, l));

                any_sequence_repeated.then_some(n)
            })
        })
        .sum()
}

fn id_ranges(s: &str) -> impl Iterator<Item = (u64, u64)> {
    s.split(",").map(|p| {
        let (s, e) = p.trim().split_once("-").expect("pair malformed");
        let [s, e] = [s, e].map(|i| i.parse::<u64>().expect("id malformed"));
        (s, e)
    })
}

fn n_digits(i: u64) -> u32 {
    i.ilog10() + 1
}

fn upper(i: u64) -> Option<u64> {
    let n_digits = n_digits(i);
    n_digits.is_multiple_of(2).then(|| {
        let k = 10u64.pow(n_digits / 2);
        i / k
    })
}

fn all_chunks_same<T: Eq>(buf: &[T], n: usize) -> bool {
    let mut chunks = buf.chunks_exact(n);

    if !chunks.remainder().is_empty() {
        return false;
    }

    let Some(head) = chunks.next() else {
        return false;
    };

    chunks.all(|c| head == c)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(1227775554, sum_of_invalid_ids(EXAMPLE));
    }

    #[test]
    fn n_digits_exercise() {
        assert_eq!(1, n_digits(1));
        assert_eq!(2, n_digits(12));
        assert_eq!(3, n_digits(123));
        assert_eq!(4, n_digits(1234));
    }

    #[test]
    fn upper_exercise() {
        assert_eq!(None, upper(1));
        assert_eq!(Some(1), upper(12));
        assert_eq!(None, upper(123));
        assert_eq!(Some(12), upper(1234));
    }

    #[test]
    fn part2_example() {
        assert_eq!(4174379265, sum_of_all_invalid_ids(EXAMPLE));
    }

    #[test]
    fn all_chunks_same_exercise() {
        assert!(all_chunks_same(b"12341234", 4));
        assert!(all_chunks_same(b"123123123", 3));
        assert!(all_chunks_same(b"1212121212", 2));
        assert!(all_chunks_same(b"1111111", 1));
    }
}
