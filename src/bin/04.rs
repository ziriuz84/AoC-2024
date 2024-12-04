advent_of_code::solution!(4);

use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (1, -1),
    (-1, 1),
];

fn search_word(matrix: Vec<Vec<char>>, word: &str) -> i32 {
    let mut counter: i32 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == word.chars().nth(0).unwrap() {
                for &(dx, dy) in &DIRECTIONS {
                    let mut n_match = 0;
                    let (mut x, mut y) = (i as isize, j as isize);
                    while n_match < word.len()
                        && x >= 0
                        && y >= 0
                        && x < matrix.len() as isize
                        && y < matrix[i].len() as isize
                        && matrix[x as usize][y as usize] == word.chars().nth(n_match).unwrap()
                    {
                        x += dx;
                        y += dy;
                        n_match += 1;
                    }
                    if n_match == word.len() {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn search_x_word(matrix: Vec<Vec<char>>) -> i32 {
    let mut counter: i32 = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] == 'A' {
                if matrix[i + 1][j + 1] == 'M'
                    && matrix[i - 1][j + 1] == 'M'
                    && matrix[i - 1][j - 1] == 'S'
                    && matrix[i + 1][j - 1] == 'S'
                {
                    counter += 1;
                }
                if matrix[i + 1][j + 1] == 'S'
                    && matrix[i - 1][j + 1] == 'M'
                    && matrix[i - 1][j - 1] == 'M'
                    && matrix[i + 1][j - 1] == 'S'
                {
                    counter += 1;
                }
                if matrix[i + 1][j + 1] == 'S'
                    && matrix[i - 1][j + 1] == 'S'
                    && matrix[i - 1][j - 1] == 'M'
                    && matrix[i + 1][j - 1] == 'M'
                {
                    counter += 1;
                }
                if matrix[i + 1][j + 1] == 'M'
                    && matrix[i - 1][j + 1] == 'S'
                    && matrix[i - 1][j - 1] == 'S'
                    && matrix[i + 1][j - 1] == 'M'
                {
                    counter += 1;
                }
            }
        }
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    splitted_input.iter().for_each(|x| {
        let temp: Vec<char> = x.chars().collect();
        matrix.push(temp);
    });
    let results = search_word(matrix, "XMAS");
    Some(results as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    splitted_input.iter().for_each(|x| {
        let temp: Vec<char> = x.chars().collect();
        matrix.push(temp);
    });
    let results = search_x_word(matrix);
    Some(results as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
