use crate::common::{Answer, Solution};
use fxhash::{hash, FxHashSet};
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;

pub struct Day06;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    #[inline(always)]
    fn turn_90_degrees(&mut self) {
        *self = match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
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
        Facing::Up => (guard.0, guard.1 - 1),
        Facing::Down => (guard.0, guard.1 + 1),
        Facing::Left => (guard.0 - 1, guard.1),
        Facing::Right => (guard.0 + 1, guard.1),
    };

    if obstacles.contains(&new_position) || (bonus_obstacle == Some(new_position)) {
        facing.turn_90_degrees();
        advance(obstacles, guard, facing, bonus_obstacle);
    } else {
        *guard = new_position;
    }
}

#[inline(always)]
fn advance2(
    obstacles: &FxHashSet<(i32, i32)>,
    guard: &mut (i32, i32),
    facing: &mut Facing,
    bonus_obstacle: Option<(i32, i32)>,
) {
    let new_position = match &facing {
        Facing::Up => (guard.0, guard.1 - 1),
        Facing::Down => (guard.0, guard.1 + 1),
        Facing::Left => (guard.0 - 1, guard.1),
        Facing::Right => (guard.0 + 1, guard.1),
    };

    if obstacles.contains(&new_position) || (bonus_obstacle == Some(new_position)) {
        facing.turn_90_degrees();
        advance2(obstacles, guard, facing, bonus_obstacle);
    } else {
        *guard = new_position;
    }
}

#[inline(always)]
fn compute_path(input: &str) -> Vec<(i32, i32)> {
    let mut obstacles = vec![];
    let mut guard: (i32, i32) = (-1, -1);
    let mut facing = Facing::Up;

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
    let size_x = input.lines().next().unwrap().len();
    let mut path = vec![];

    while guard.0 >= 0 && guard.1 >= 0 && guard.0 < size_x as i32 && guard.1 < size_y as i32 {
        path.push(guard);
        advance(&obstacles, &mut guard, &mut facing, None);
    }

    path
}

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Guard Gallivant"
    }

    fn part_a(&self, input: &str) -> Answer {
        compute_path(input).iter().unique().count().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut obstacles: FxHashSet<(i32, i32)> = FxHashSet::default();
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
        debug_assert_ne!(default_guard, (-1, -1));

        let size_y = input.lines().count() as i32;
        let size_x = input.lines().nth(0).unwrap().len() as i32;

        let mut base_path = vec![];

        let mut guard = default_guard;
        let mut facing = Facing::Up;

        //advance first to make sure starting point is not in base path
        advance2(&obstacles, &mut guard, &mut facing, Some((0, 0)));
        while guard.0 >= 0 && guard.1 >= 0 && guard.0 < size_x && guard.1 < size_y {
            base_path.push(guard);
            advance2(&obstacles, &mut guard, &mut facing, None);
        }

        base_path
            .iter()
            .unique_by(hash)
            .par_bridge()
            .map(|pos| {
                let mut guard = default_guard;
                let mut facing = Facing::Up;

                let mut path = (
                    FxHashSet::default(),
                    FxHashSet::default(),
                    FxHashSet::default(),
                    FxHashSet::default(),
                );

                loop {
                    if !match &facing {
                        Facing::Up => {
                            if guard.1 < 0 {
                                return 0;
                            }
                            path.0.insert(guard)
                        }
                        Facing::Down => {
                            if guard.1 >= size_y {
                                return 0;
                            }
                            path.1.insert(guard)
                        }
                        Facing::Left => {
                            if guard.0 < 0 {
                                return 0;
                            }
                            path.2.insert(guard)
                        }
                        Facing::Right => {
                            if guard.0 >= size_x {
                                return 0;
                            }
                            path.3.insert(guard)
                        }
                    } {
                        return 1;
                    }

                    advance2(&obstacles, &mut guard, &mut facing, Some(*pos));
                }
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
