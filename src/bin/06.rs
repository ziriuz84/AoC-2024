advent_of_code::solution!(6);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut width: isize = 0;
    let mut height: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dx: isize = 0;
    let mut dy: isize = -1;
    let mut counter = 0;
    let mut dir = "up";
    let mut coords: Vec<(isize, isize)> = Vec::new();
    splitted_input.iter().for_each(|line| {
        matrix.push(line.chars().collect());
    });
    height = matrix.len() as isize;
    for i in 0..matrix.len() {
        width = matrix[i].len() as isize;

        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                y = i as isize;
                x = j as isize;
                counter += 1;
                coords.push((x, y));
            }
        }
    }
    while x > 0 && y > 0 && x < width - 1 && y < height - 1 {
        let i = y as usize;
        let j = x as usize;
        match dir {
            "up" => {
                if matrix[i - 1][j] == '#' {
                    dir = "right";
                    dx = 1;
                    dy = 0;
                }
            }
            "right" => {
                if matrix[i][j + 1] == '#' {
                    dir = "down";
                    dx = 0;
                    dy = 1;
                }
            }
            "down" => {
                if matrix[i + 1][j] == '#' {
                    dir = "left";
                    dx = -1;
                    dy = 0;
                }
            }
            "left" => {
                if matrix[i][j - 1] == '#' {
                    dir = "up";
                    dx = 0;
                    dy = -1;
                }
            }
            _ => {}
        };
        x += dx;
        y += dy;
        if !coords.contains(&(x, y)) {
            counter += 1;
            coords.push((x, y));
        }
    }

    Some(counter)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
