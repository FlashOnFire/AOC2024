use crate::common::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Red-Nosed Reports"
    }

    fn part_a(&self, input: &str) -> Answer {
        Answer::from(
            input
                .lines()
                .map(|line| line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
                .map(|numbers: Vec<i32>| {
                    let direction = numbers[0] < numbers[1];
                    numbers.windows(2).fold(true, |acc, tab| {
                        let (a, b) = (tab[0], tab[1]);

                        acc && (1..=3).contains(&(a - b).abs())
                            && (if direction { a < b } else { a > b })
                    })
                })
                .fold(0, |acc, bool| acc + if bool { 1 } else { 0 }),
        )
    }

    fn part_b(&self, input: &str) -> Answer {
        Answer::from(
            input
                .lines()
                .map(|line| line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
                .map(|numbers: Vec<i32>| {
                    let mut test_numbers = vec![numbers.clone()];
                    for i in 0..numbers.len() {
                        let mut test = numbers.clone();
                        test.remove(i);
                        test_numbers.push(test);
                    }
                    
                    test_numbers.iter().any(|numbers| {
                        let direction = numbers[0] < numbers[1];
                        numbers.windows(2).fold(true, |acc, tab| {
                            let (a, b) = (tab[0], tab[1]);

                            acc && (1..=3).contains(&(a - b).abs())
                                && (if direction { a < b } else { a > b })
                        })
                    })
                })
                .fold(0, |acc, bool| acc + if bool { 1 } else { 0 }),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Day02;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT_A: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    const INPUT_B: &str = indoc! {"

    "};

    #[test]
    fn part_a() {
        assert_eq!(Day02.part_a(INPUT_A), crate::common::Answer::Unimplemented);
    }

    #[test]
    fn part_b() {
        assert_eq!(Day02.part_b(INPUT_B), crate::common::Answer::Unimplemented);
    }
}
