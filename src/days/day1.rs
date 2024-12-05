use crate::utils::read_input;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut l1, mut l2) = (vec![], vec![]);
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        l1.push(nums.next().unwrap().parse::<i32>().unwrap());
        l2.push(nums.next().unwrap().parse::<i32>().unwrap());
    }
    (l1, l2)
}

fn part_1(input: &str) -> i32 {
    let (mut l1, mut l2) = parse(input);

    l1.sort_unstable();
    l2.sort_unstable();

    l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(input: &str) -> i32 {
    let (l1, l2) = parse(input);
    let mut hm = HashMap::new();
    for num in l2 {
        *hm.entry(num).or_insert(0) += 1;
    }

    l1.iter().map(|num| hm.get(num).unwrap_or(&0) * num).sum()
}

pub fn solution() {
    let input = read_input("day1.txt");
    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 11)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 31)
    }
}
