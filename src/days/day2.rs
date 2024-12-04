use crate::utils::read_input;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut l = vec![];
    for line in input.trim().lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        l.push(nums);
    }
    l
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff < 0 {
            increasing = false
        } else {
            decreasing = false
        }
    }

    increasing || decreasing
}

fn part_1(input: &str) -> i32 {
    let reports = parse_input(input);

    reports.iter().filter(|report| is_safe(&report)).count() as i32
}

fn part_2(input: &str) -> i32 {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|&report| {
            if is_safe(&report) {
                return true;
            }

            for i in 0..report.len() {
                let mut temp = report.clone();
                temp.remove(i);
                if is_safe(&temp) {
                    return true;
                }
            }

            false
        })
        .count() as i32
}

pub fn solution() {
    let input = read_input("day2.txt");
    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 2)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4)
    }
}
