//! Would probably be a lot easier to solve with regex
use crate::utils::read_input;

fn parse(input: &str) -> (i32, i32) {
    let bytes = input.as_bytes();
    let (mut idx, mut p1, mut p2) = (0, 0, 0);
    let mut enabled = true;

    while idx < bytes.len() {
        if bytes[idx..].starts_with(b"mul(") {
            idx += 4;
        } else if bytes[idx..].starts_with(b"don't()") {
            enabled = false;
            idx += 7;
            continue;
        } else if bytes[idx..].starts_with(b"do()") {
            enabled = true;
            idx += 4;
            continue;
        } else {
            idx += 1;
            continue;
        }

        let mut n1 = 0;
        while bytes[idx].is_ascii_digit() {
            n1 = 10 * n1 + (bytes[idx] - b'0') as i32;
            idx += 1
        }

        if bytes[idx] != b',' {
            idx += 1;
            continue;
        } else {
            idx += 1;
        }

        let mut n2 = 0;
        while bytes[idx].is_ascii_digit() {
            n2 = 10 * n2 + (bytes[idx] - b'0') as i32;
            idx += 1
        }

        if bytes[idx] != b')' {
            idx += 1;
            continue;
        } else {
            idx += 1;
        }

        let prod = n1 * n2;
        p1 += prod;
        if enabled {
            p2 += prod;
        }
    }

    (p1, p2)
}

fn part_1(input: &str) -> i32 {
    parse(input).0
}

fn part_2(input: &str) -> i32 {
    parse(input).1
}

pub fn solution() {
    let input = read_input("day3.txt");
    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        )
    }
}
