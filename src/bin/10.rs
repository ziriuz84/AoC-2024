advent_of_code::solution!(10);

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

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    value: u32,
}

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

fn test_points(points: &Vec<Vec<char>>, width: usize, height: usize, i: usize, j: usize) -> bool {
    for (dx, dy) in &DIRECTIONS {
        if i as isize + dx >= 0
            && j as isize + dy >= 0
            && i as isize + dx < width as isize
            && j as isize + dy < height as isize
        {
            let x: usize = (i as isize + dx) as usize;
            let y: usize = (j as isize + dy) as usize;
            if points[x][y] == '0' {
                return true;
            } else if points[x][y].to_digit(10).unwrap() == points[i][j].to_digit(10).unwrap() - 1 {
                return test_points(points, width, height, x, y);
            }
        }
    }
    false
}

fn test_next_point(point: Point, other_point: Point) -> bool {
    other_point.x == point.x - 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    splitted_input.iter().for_each(|line| {
        char_matrix.push(line.chars().collect());
    });
    let width: usize = char_matrix[0].len();
    let height: usize = char_matrix.len();
    let mut points: Vec<Point> = Vec::new();
    for i in 0..char_matrix.len() {
        for j in 0..char_matrix[i].len() {
            if char_matrix[i][j] == '9' {
                if test_points(&char_matrix, width, height, i, j) {
                    sum += 1;
                }
            }
        }
    }

    Some(sum)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
