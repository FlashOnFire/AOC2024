use crate::common::{Answer, Solution};
use indoc::indoc;

pub struct Day04;

impl Day04 {
    fn parse_str(input: &str) -> Vec<Vec<char>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .filter(|x| !x.is_whitespace())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>()
    }

    fn search_pattern(input_matrix: &[Vec<char>], pattern: Vec<Vec<char>>) -> i32 {
        if pattern.is_empty() || pattern[0].is_empty() {
            return 0;
        }

        let size_x = pattern.len();
        let size_y = pattern[0].len();
        if input_matrix.len() <= size_x || input_matrix[0].len() <= size_y {
            return 0;
        }

        let input_size_x = input_matrix.len();
        let input_size_y = input_matrix[0].len();

        let mut count = 0;
        for i in 0..(input_size_x - size_x + 1) {
            for j in 0..(input_size_y - size_y + 1) {
                let mut found = true;
                'seq: for k in 0..size_x {
                    for l in 0..size_y {
                        if pattern[k][l] == '.' {
                            continue;
                        }
                        if input_matrix[i + k][j + l] != pattern[k][l] {
                            found = false;
                            break 'seq;
                        }
                    }
                }

                if found {
                    count += 1;
                }
            }
        }

        count
    }

    fn flip_v(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        matrix.reverse();
        matrix
    }

    fn flip_h(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        matrix.iter_mut().for_each(|row| row.reverse());
        matrix
    }
}

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Ceres Search"
    }

    fn part_a(&self, input: &str) -> Answer {
        let matrix = Self::parse_str(input);

        let mut count = 0;
        let horizontal_pattern = Self::parse_str("XMAS");
        count += Self::search_pattern(&matrix, horizontal_pattern.clone());

        let horizontal_pattern2 = Self::flip_h(horizontal_pattern);
        count += Self::search_pattern(&matrix, horizontal_pattern2);

        let vertical_pattern = Self::parse_str(indoc! {"
            X
            M
            A
            S
        "});
        count += Self::search_pattern(&matrix, vertical_pattern.clone());

        let vertical_pattern2 = Self::flip_v(vertical_pattern);
        count += Self::search_pattern(&matrix, vertical_pattern2);

        let diagonal_pattern = Self::parse_str(indoc! {"\
        X...
        .M..
        ..A.
        ...S
        "});
        count += Self::search_pattern(&matrix, diagonal_pattern.clone());

        let diagonal_pattern2 = Self::flip_h(diagonal_pattern.clone());
        count += Self::search_pattern(&matrix, diagonal_pattern2.clone());

        let diagonal_pattern3 = Self::flip_v(diagonal_pattern);
        count += Self::search_pattern(&matrix, diagonal_pattern3);

        let diagonal_pattern4 = Self::flip_v(diagonal_pattern2);
        count += Self::search_pattern(&matrix, diagonal_pattern4);

        count.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let matrix = Self::parse_str(input);

        let mut count = 0;
        let pattern_str = indoc!("
            M.S
            .A.
            M.S
        ");
        let pattern = Self::parse_str(pattern_str);
        count += Self::search_pattern(&matrix, pattern.clone());
        count += Self::search_pattern(&matrix, Self::flip_h(pattern.clone()));
        
        let pattern2 = indoc!("
            S.S
            .A.
            M.M
        ");
        let pattern = Self::parse_str(pattern2);
        count += Self::search_pattern(&matrix, pattern.clone());
        count += Self::search_pattern(&matrix, Self::flip_v(pattern.clone()));
        
        count.into()
    }
}

#[cfg(test)]
mod test {
    use super::Day04;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT_A: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    const INPUT_B: &str = indoc! {"
        .M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day04.part_a(INPUT_A), 18.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day04.part_b(INPUT_B), 9.into());
    }
}
