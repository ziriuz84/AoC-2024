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

fn search_word(matrix: Vec<Vec<char>>, word: &str) -> HashSet<(isize, isize)>{
    let results = HashSet::new();
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            for &(dx,dy) in &DIRECTIONS{

            }
        }
    }
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    splitted_input.iter().for_each(|x| {
        let temp: Vec<char> = x.chars().collect();
        matrix.push(temp);
    });
    let results = HashSet::new()
    println!("{:?}", matrix);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
