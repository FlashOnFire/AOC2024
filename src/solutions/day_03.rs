use crate::common::{Answer, Solution};
use regex::Regex;

pub struct Day03;

impl Day03 {
    fn parse(&self, input: &str) -> i32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(input.trim())
            .map(|c| {
                c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * c.get(2).unwrap().as_str().parse::<i32>().unwrap()
            })
            .sum::<i32>()
    }
}

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Mull It Over"
    }

    fn part_a(&self, input: &str) -> Answer {
        self.parse(input).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        input
            .split_inclusive("do()")
            .map(|str| str.split_once("don't()").map(|s| s.0).unwrap_or(str))
            .map(|str| self.parse(str))
            .sum::<i32>()
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::Day03;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT_A: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    const INPUT_B: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day03.part_a(INPUT_A), crate::common::Answer::Number(161));
    }

    #[test]
    fn part_b() {
        assert_eq!(Day03.part_b(INPUT_B), crate::common::Answer::Number(48));
    }
}
