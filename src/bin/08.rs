advent_of_code::solution!(8);

#[derive(Debug)]
struct Antenna {
    x: i32,
    y: i32,
    value: char,
}
#[derive(Debug)]
struct Antinode {
    x: i32,
    y: i32,
}

impl PartialEq for Antenna {
    fn eq(&self, other: &Antenna) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value
    }
}

impl PartialEq for Antinode {
    fn eq(&self, other: &Antinode) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Antenna {
    fn clone(&self) -> Self {
        Antenna {
            x: self.x,
            y: self.y,
            value: self.value,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut antennas: Vec<Antenna> = Vec::new();
    let mut antinodes: Vec<Antinode> = Vec::new();
    let mut couple_antennas: Vec<(Antenna, Antenna)> = Vec::new();
    let mut antenna_positions: Vec<(i32, i32)> = Vec::new();
    let mut sum = 0;
    splitted_input.iter().for_each(|line| {
        map.push(line.chars().collect());
    });
    let width = map[0].len();
    let height = map.len();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                let antenna: Antenna = Antenna {
                    x: j as i32,
                    y: i as i32,
                    value: map[i][j],
                };
                antennas.push(antenna);
                antenna_positions.push((j as i32, i as i32));
            }
        }
    }
    antennas.clone().iter().for_each(|antenna| {
        antennas.clone().iter().for_each(|antenna2| {
            if !couple_antennas.contains(&(antenna.clone(), antenna2.clone()))
                && !couple_antennas.contains(&(antenna2.clone(), antenna.clone()))
                && antenna != antenna2
                && antenna.value == antenna2.value
            {
                let dist_x = antenna.x - antenna2.x;
                let dist_y = antenna.y - antenna2.y;
                let antinode_a: Antinode = Antinode {
                    x: antenna.x - dist_x,
                    y: antenna.y - dist_y,
                };
                let antinode_b: Antinode = Antinode {
                    x: antenna2.x + dist_x,
                    y: antenna2.y + dist_y,
                };
                if !antinodes.contains(&antinode_a) {
                    antinodes.push(antinode_a);
                }
                if !antinodes.contains(&antinode_b) {
                    antinodes.push(antinode_b);
                }
                couple_antennas.push((antenna.clone(), antenna2.clone()));
            }
        });
    });

    antinodes.iter().for_each(|antinode| {
        if antinode.x >= 0
            && antinode.x < width as i32
            && antinode.y >= 0
            && antinode.y < height as i32
        {
            println!("{} {}", antinode.x, antinode.y);
            sum += 1;
        }
    });
    Some(sum as u32)
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
