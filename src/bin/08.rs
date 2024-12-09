advent_of_code::solution!(8);

#[derive(Debug)]
struct Antenna {
    x: usize,
    y: usize,
    value: char,
}
#[derive(Debug)]
struct Antinode {
    x: usize,
    y: usize,
    antennas: Vec<Antenna>,
}

impl PartialEq for Antenna {
    fn eq(&self, other: &Antenna) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut antennas: Vec<Antenna> = Vec::new();
    splitted_input.iter().for_each(|line| {
        map.push(line.chars().collect());
    });
    let width = map[0].len();
    let height = map.len();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                let antenna: Antenna = Antenna {
                    x: j,
                    y: i,
                    value: map[i][j],
                };
                antennas.push(antenna);
            }
        }
    }
    for antenna in antennas {
        for antenna2 in antennas {
            if antenna == antenna2 {
                continue;
            } else {
                let dist_x = (antenna.x as i32 - antenna2.x as i32).abs();
                let dist_y = (antenna.y as i32 - antenna2.y as i32).abs();
                let antinode: Antinode = Antinode {
                }
            }
        }
    }
    println!("{:?}", antennas);

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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
