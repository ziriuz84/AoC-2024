advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in splitted_input.clone() {
        let data: Vec<u32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        if data.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3) {
            sum += 1;
        }
        if data.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3) {
            sum += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
