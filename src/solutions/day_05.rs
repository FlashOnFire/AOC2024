use crate::common::{Answer, Solution};
use itertools::Itertools;
use rayon::iter::ParallelIterator;

use rayon::prelude::{IntoParallelRefIterator, ParallelBridge};

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Print Queue"
    }

    fn part_a(&self, input: &str) -> Answer {
        let rules: Vec<_> = parse_rules(input);
        let updates: Vec<Vec<_>> = parse_updates(input);

        updates
            .iter()
            .filter(|update| is_correctly_ordered(&rules, update))
            .map(|update| *update.get(update.len() / 2).unwrap())
            .sum::<i32>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let rules: Vec<_> = parse_rules(input);
        let updates: Vec<Vec<i32>> = parse_updates(input);

        let wrong_updates: Vec<Vec<i32>> = updates
            .par_iter()
            .filter(|update| !is_correctly_ordered(&rules, update))
            .map(|update| update.to_vec())
            .collect();
        println!("Found {} wrong updates", wrong_updates.len());

        wrong_updates
            .par_iter()
            .map(|update| correct_update(&rules, update.clone()))
            .map(|update| *update.get(update.len() / 2).unwrap())
            .sum::<i32>()
            .into()
    }
}

fn parse_rules(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map_while(|str| str.split_once('|'))
        .map(|(before, after)| {
            (
                before.parse::<i32>().unwrap(),
                after.parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty() && !line.contains('|'))
        .map(|update| {
            update
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_correctly_ordered(rules: &Vec<(i32, i32)>, update: &[i32]) -> bool {
    rules.par_iter().all(|(before, after)| {
        if update.contains(before)
            && update.contains(after)
            && update.iter().position(|x| x == before) > update.iter().position(|x| x == after)
        {
            return false;
        }
        true
    })
}

fn correct_update(rules: &Vec<(i32, i32)>, mut update: Vec<i32>) -> Vec<i32> {
    let relevant_rules: Vec<(i32, i32)> = rules
        .iter()
        .filter(|(before, after)| update.contains(before) && update.contains(after))
        .copied()
        .collect();

    while !is_correctly_ordered(rules, &update) {
        for (before, after) in &relevant_rules {
            let i = update.iter().position(|x| x == before).unwrap();
            let j = update.iter().position(|x| x == after).unwrap();
            if i == j + 1 {
                update.swap(i, j);
            }
        }
    }

    update
}

#[allow(dead_code)]
fn is_correctly_ordered2(rules: &Vec<(i32, i32)>, update: &Vec<&i32>) -> bool {
    for (before, after) in rules {
        if update.contains(&before)
            && update.contains(&after)
            && update.iter().position(|&x| x == before) > update.iter().position(|&x| x == after)
        {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn correct_update_bruteforce(rules: &Vec<(i32, i32)>, update: Vec<i32>) -> Vec<i32> {
    let good_update = update
        .iter()
        .permutations(update.len())
        .par_bridge()
        .find_first(|combination| is_correctly_ordered2(rules, combination))
        .unwrap();

    println!("yippee");
    good_update.iter().copied().copied().collect()
}

#[cfg(test)]
mod test {
    use super::Day05;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
        
        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day05.part_a(INPUT), 143.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day05.part_b(INPUT), 123.into());
    }
}
