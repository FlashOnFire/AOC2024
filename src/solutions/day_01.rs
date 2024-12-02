use itertools::Itertools;

use crate::common::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Historian Hysteria"
    }

    fn part_a(&self, input: &str) -> Answer {
        let (mut left, mut right) = input
            .lines()
            .map(|l| l.split_once("   ").unwrap())
            .map(|(n1, n2)| (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()))
            .collect::<(Vec<_>, Vec<_>)>();

        left.sort();
        right.sort();

        let sum: i32 = left
            .into_iter()
            .zip(right)
            .map(|(a, b)| (a - b).abs())
            .sum();

        Answer::Number(sum as u64)
    }

    fn part_b(&self, input: &str) -> Answer {
        let (left, right) = input
            .lines()
            .map(|l| l.split_once("   ").unwrap())
            .map(|(n1, n2)| (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()))
            .collect::<(Vec<_>, Vec<_>)>();

        let counts = right.into_iter().counts();

        let sum = left
            .into_iter()
            .map(|n| counts.get(&n).unwrap_or(&0) * (n as usize))
            .sum::<usize>();

        Answer::Number(sum as u64)
    }
}

#[cfg(test)]
mod test {
    use super::Day01;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day01.part_a(INPUT), 11.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day01.part_b(INPUT), 31.into());
    }
}