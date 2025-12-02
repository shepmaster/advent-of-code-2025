const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = password(INPUT);
    assert_eq!(1097, part1);
    println!("{part1}");

    let part2 = password_0x434c49434b(INPUT);
    // Allowed the dial to go to exactly 100
    assert!(part2 < 7112);

    assert!(part2 < 7106);
    assert!(part2 > 6990);

    assert_eq!(7101, part2);
    println!("{part2}");
}

const DIAL_START: u8 = 50;
const DIAL_SIZE: u8 = 100;

fn password(s: &str) -> usize {
    let mut dial = u32::from(DIAL_START);
    spins(s)
        .map(|n| {
            let n = n.rem_euclid(DIAL_SIZE.into());
            dial = dial.strict_add_signed(n);
            dial.rem_euclid(DIAL_SIZE.into())
        })
        .filter(|&d| d == 0)
        .count()
}

fn password_0x434c49434b(s: &str) -> usize {
    password_0x434c49434b_core(DIAL_START.into(), spins(s))
}

fn password_0x434c49434b_core(mut dial: u32, directions: impl IntoIterator<Item = i32>) -> usize {
    let dial_size_u32: u32 = DIAL_SIZE.into();
    let dial_size_i32: i32 = DIAL_SIZE.into();

    directions
        .into_iter()
        .map(|n| {
            assert!(dial < dial_size_u32, "`dial` is out of bounds at {dial}");
            assert_ne!(n, 0);

            let mut new_d = i32::try_from(dial).expect("dial is invalid i32");
            let old_d = new_d;
            let mut crossings = 0;

            // Rotate the dial
            new_d += n;

            // If we rotated it one or more full spins to the left
            while new_d <= -dial_size_i32 {
                new_d += dial_size_i32;
                crossings += 1;
            }

            // If we rotated it one or more full spins to the right
            while new_d >= dial_size_i32 {
                new_d -= dial_size_i32;
                crossings += 1;
            }

            // If we rotated left and crossed over zero
            if let (1, -1) = (old_d.signum(), new_d.signum()) {
                crossings += 1
            }

            // If we rotated to the left and ended on zero
            if let (-1, 0) = (n.signum(), new_d) {
                crossings += 1;
            }

            // Restore our state to 0..DIAL_SIZE
            if new_d < 0 {
                new_d += dial_size_i32;
            }

            // eprintln!("{old_d:3} {n:4} {new_d:3} {crossings:2}");
            dial = new_d.try_into().expect("dial is invalid u32");

            crossings
        })
        .sum()
}

fn spins(s: &str) -> impl Iterator<Item = i32> {
    s.lines().map(|l| {
        let (direction, n) = if let Some(n) = l.strip_prefix("L") {
            (-1, n)
        } else if let Some(n) = l.strip_prefix("R") {
            (1, n)
        } else {
            panic!("Unknown direction");
        };

        let n = n.parse::<i32>().expect("Invalid amount");
        n * direction
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(3, password(EXAMPLE));
    }

    #[test]
    fn part2_example() {
        assert_eq!(6, password_0x434c49434b(EXAMPLE));
    }

    #[test]
    #[should_panic]
    fn part2_bug_1() {
        password_0x434c49434b_core(100, [-1]);
    }

    #[test]
    fn part2_bug_2() {
        assert_eq!(0, password_0x434c49434b_core(0, [-1]));
        assert_eq!(0, password_0x434c49434b_core(0, [1]));
        assert_eq!(1, password_0x434c49434b_core(1, [-1]));
        assert_eq!(1, password_0x434c49434b_core(99, [1]));
        assert_eq!(2, password_0x434c49434b_core(50, [-150]));
        assert_eq!(2, password_0x434c49434b_core(50, [150]));
    }
}
