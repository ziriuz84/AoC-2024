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
    println!("{}", input);
    let mut enabled = true;
    let re = regex::Regex::new(r"(don't\(\))|(do\(\))|mul\(\d+,\d+\)").unwrap();
    let re = regex::Regex::new(r"(don't\(\))|(do\(\))|mul\(([1-9]|[1-9][0-9]|[1-9][0-9][0-9]),([1-9]|[1-9][0-9]|[1-9][0-9][0-9])\)").unwrap();
    let result = re.captures_iter(input).fold(0, |acc, cap| {
        let mut sum: u32 = 0;
        println!("{:?}", cap);
        if cap.get(1).is_some_and(|x| x.as_str() == "don't()") {
            enabled = false;
        };
        if cap.get(2).is_some_and(|x| x.as_str() == "do()") {
            enabled = true;
        };
        if enabled && cap.get(3).is_some() && cap.get(4).is_some() {
            sum = cap.get(3).unwrap().as_str().parse::<u32>().unwrap()
                * cap.get(4).unwrap().as_str().parse::<u32>().unwrap();
            println!("{}", sum);
        }
        println!("{}", enabled);
        acc + sum
    });
    Some(result)
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(input);
        assert_eq!(result, Some(48));
    }
}
