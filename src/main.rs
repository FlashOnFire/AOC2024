use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let mut input1_base = vec![];
    let mut input2_base = vec![];

    for line in input.lines() {
        let numbers = line
            .split("   ")
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        input1_base.push(*numbers.first().unwrap());
        input2_base.push(*numbers.get(1).unwrap());
    }

    // part1
    let mut input1 = input1_base.clone();
    let mut input2 = input2_base.clone();

    input1.sort();
    input2.sort();

    let dist: i32 = input1
        .iter()
        .zip(input2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Day1 part A: {}", dist);

    //part b
    let input1 = input1_base.clone();
    let input2 = input2_base.clone();

    let mut map = HashMap::new();
    for x in input2 {
        map.insert(x, *map.get(&x).unwrap_or(&0) + 1);
    }

    let answer: i32 = input1.iter().map(|x| map.get(x).unwrap_or(&0) * x).sum();
    
    println!("Day1 part B: {}", answer);
}
