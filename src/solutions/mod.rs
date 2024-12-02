use crate::common::Solution;

mod day_01;
mod day_02;
mod template;

pub const SOLUTIONS: &[&dyn Solution] = &[&day_01::Day01, &day_02::Day02];
