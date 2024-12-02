use crate::common::{Answer, Solution};

pub struct DayXX;

impl Solution for DayXX {
    fn name(&self) -> &'static str {
        ""
    }

    fn part_a(&self, input: &str) -> Answer {
        Answer::Unimplemented
    }

    fn part_b(&self, input: &str) -> Answer {
        Answer::Unimplemented
    }
}

#[cfg(test)]
mod test {
    use super::DayXX;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT_A: &str = indoc! {"

    "};

    const INPUT_B: &str = indoc! {"

    "};

    #[test]
    fn part_a() {
        assert_eq!(DayXX.part_a(INPUT_A), crate::common::Answer::Unimplemented);
    }

    #[test]
    fn part_b() {
        assert_eq!(DayXX.part_b(INPUT_B), crate::common::Answer::Unimplemented);
    }
}