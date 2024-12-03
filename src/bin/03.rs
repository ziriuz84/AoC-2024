advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let re = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let strings: Vec<&str> = re.find_iter(input).map(|x| x.as_str()).collect();
    strings.iter().for_each(|string| {
        let mut new_string = string.replace("mul(", "");
        new_string = new_string.replace(")", "");
        let numbers: Vec<&str> = new_string.split(",").collect();
        let number1: u32 = numbers[0].parse().unwrap();
        let number2: u32 = numbers[1].parse().unwrap();
        sum += number1 * number2
    });
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}