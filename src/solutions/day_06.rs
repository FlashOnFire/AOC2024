use crate::common::{Answer, Solution};
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelBridge};
use std::collections::HashSet;

pub struct Day06;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Facing {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Facing {
    fn turn_90_degrees(&mut self) {
        *self = match self {
            Facing::UP => Facing::RIGHT,
            Facing::RIGHT => Facing::DOWN,
            Facing::DOWN => Facing::LEFT,
            Facing::LEFT => Facing::UP,
        }
    }
}

fn advance(
    obstacles: &[(i32, i32)],
    guard: &mut (i32, i32),
    facing: &mut Facing,
    bonus_obstacle: Option<(i32, i32)>,
) {
    let new_position = match &facing {
        Facing::UP => (guard.0, guard.1 - 1),
        Facing::DOWN => (guard.0, guard.1 + 1),
        Facing::LEFT => (guard.0 - 1, guard.1),
        Facing::RIGHT => (guard.0 + 1, guard.1),
    };

    if obstacles.contains(&new_position) || (bonus_obstacle == Some(new_position)) {
        facing.turn_90_degrees();
        advance(obstacles, guard, facing, bonus_obstacle);
    } else {
        *guard = new_position;
    }
}

fn advance2(
    obstacles: &HashSet<(i32, i32)>,
    guard: &mut (i32, i32),
    facing: &mut Facing,
    bonus_obstacle: Option<(i32, i32)>,
) {
    let new_position = match &facing {
        Facing::UP => (guard.0, guard.1 - 1),
        Facing::DOWN => (guard.0, guard.1 + 1),
        Facing::LEFT => (guard.0 - 1, guard.1),
        Facing::RIGHT => (guard.0 + 1, guard.1),
    };

    if obstacles.contains(&new_position) || (bonus_obstacle == Some(new_position)) {
        facing.turn_90_degrees();
        advance2(obstacles, guard, facing, bonus_obstacle);
    } else {
        *guard = new_position;
    }
}

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Guard Gallivant"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut obstacles = vec![];
        let mut guard: (i32, i32) = (-1, -1);
        let mut facing = Facing::UP;

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    obstacles.push((x as i32, y as i32));
                } else if char == '^' {
                    guard = (x as i32, y as i32);
                }
            }
        }
        assert_ne!(guard, (-1, -1));

        let size_y = input.lines().count();
        let size_x = input.lines().nth(0).unwrap().len();
        let mut path = vec![];

        while guard.0 >= 0 && guard.1 >= 0 && guard.0 < size_x as i32 && guard.1 < size_y as i32 {
            path.push(guard);
            advance(&obstacles, &mut guard, &mut facing, None);
        }

        for y in 0..size_y {
            for x in 0..size_x {
                let pos = (x as i32, y as i32);
                if obstacles.contains(&pos) {
                    print!("#");
                } else if path.contains(&pos) {
                    print!("X");
                } else if guard == pos {
                    print!("O");
                } else {
                    print!(".")
                }
            }
            println!();
        }

        path.iter().unique().count().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut default_guard: (i32, i32) = (-1, -1);

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    obstacles.insert((x as i32, y as i32));
                } else if char == '^' {
                    default_guard = (x as i32, y as i32);
                }
            }
        }
        assert_ne!(default_guard, (-1, -1));

        let size_y = input.lines().count() as i32;
        let size_x = input.lines().nth(0).unwrap().len() as i32;

        (0..size_x)
            .cartesian_product(0..size_y)
            .par_bridge()
            .filter(|pos| !obstacles.contains(pos) && *pos != default_guard)
            .map(|pos| {
                let mut guard = default_guard;
                let mut facing = Facing::UP;

                let mut path = (
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                );

                while guard.0 >= 0 && guard.1 >= 0 && guard.0 < size_x && guard.1 < size_y {
                    if (!match (&facing) {
                        Facing::UP => path.0.insert(guard),
                        Facing::DOWN => path.1.insert(guard),
                        Facing::LEFT => path.2.insert(guard),
                        Facing::RIGHT => path.3.insert(guard),
                    }) {
                        return 1;
                    }

                    advance2(&obstacles, &mut guard, &mut facing, Some(pos));
                }

                0
            })
            .sum::<i32>()
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::Day06;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day06.part_a(INPUT), 41.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day06.part_b(INPUT), 6.into());
    }
}
