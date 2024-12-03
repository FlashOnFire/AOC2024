use crate::common::Solution;

mod day_01;
mod day_02;
mod template;
mod day_03;

pub const SOLUTIONS: &[&dyn Solution] = &[&day_01::Day01, &day_02::Day02, &day_03::Day03];
