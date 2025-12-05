use std::ops;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = n_fresh_ingredients(INPUT);
    assert_eq!(821, part1);
    println!("{part1}");

    let part2 = n_possible_fresh_ingredients(INPUT);
    // - Was testing if the working range fell inside the candidate
    // range instead of the other way around.
    //
    // - Wasn't using the larger of the two endpoints, if a range fell
    // completely within another.
    assert!(part2 < 360941620277407);
    assert_eq!(344771884978261, part2);
    println!("{part2}");
}

type Id = u64;
type IdRange = ops::RangeInclusive<u64>;

fn n_fresh_ingredients(s: &str) -> usize {
    let mut ls = s.lines();

    let fresh_ranges = extract_ranges(&mut ls);

    ls.map(|l| l.parse::<Id>().expect("invalid id"))
        .filter(|id| fresh_ranges.iter().any(|r| r.contains(id)))
        .count()
}

fn n_possible_fresh_ingredients(s: &str) -> usize {
    let mut ls = s.lines();

    let mut fresh_ranges = extract_ranges(&mut ls);

    fresh_ranges.sort_by_key(|r| *r.start());

    // Shouldn't actually ever loop a second time, but whatever.
    loop {
        // eprintln!("---");

        if !reduce_overlaps(&mut fresh_ranges) {
            break;
        }
    }

    fresh_ranges.into_iter().map(|r| r.count()).sum()
}

fn extract_ranges<'a>(ls: impl IntoIterator<Item = &'a str>) -> Vec<IdRange> {
    ls.into_iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (l, u) = l.split_once("-").expect("malformed fresh range");
            let [l, u] = [l, u].map(|i| i.parse::<Id>().expect("invalid fresh id"));
            l..=u
        })
        .collect()
}

fn reduce_overlaps(fresh_ranges: &mut Vec<IdRange>) -> bool {
    let mut did_reduction = false;
    let mut reduced = Vec::with_capacity(fresh_ranges.len());

    if let [head, candidates @ ..] = &**fresh_ranges {
        let mut w = head.clone();

        for c in candidates {
            let c = c.clone();

            if w.contains(c.start()) {
                // eprintln!("{w:015?} ∩ {c:015?}");
                did_reduction = true;
                let e = Id::max(*w.end(), *c.end());
                w = *w.start()..=e;
            } else {
                // eprintln!("{w:015?} ∩⃠ {c:015?}");
                reduced.push(w);
                w = c;
            }
        }

        reduced.push(w);
    }

    *fresh_ranges = reduced;
    did_reduction
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(3, n_fresh_ingredients(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(14, n_possible_fresh_ingredients(EXAMPLE));
    }

    #[test]
    fn reduce_overlaps_exercise() {
        fn t_reduce_overlaps(v: impl IntoIterator<Item = IdRange>) -> (bool, Vec<IdRange>) {
            let mut v = v.into_iter().collect();
            (reduce_overlaps(&mut v), v)
        }

        assert_eq!((false, vec![0..=10]), t_reduce_overlaps([0..=10]));
        assert_eq!((true, vec![0..=15]), t_reduce_overlaps([0..=10, 5..=15]));
        assert_eq!((true, vec![0..=10]), t_reduce_overlaps([0..=10, 5..=7]));
    }
}
