advent_of_code::solution!(7);

fn test_result(result: u32, prev_result: u32, inputs: Vec<u32>, index: isize) -> bool {
    if index >= inputs.len() {
        false
    }
    let mut new_result: u32 = prev_result * inputs[index];
    if new_result > result {
        false
    } else if new_result == result {
        true
    } else {
        new_result = prev_result + inputs[index];
        if new_result > result {
            false
        } else if new_result == result {
            true
        } else {
            test_result(result, new_result, inputs, index + 1)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut results: Vec<u32> = Vec::new();
    let mut inputs: Vec<Vec<u32>> = Vec::new();
    splitted_input.iter().for_each(|l| {
        let line: Vec<&str> = l.split(": ").collect();
        results.push(line[0].parse().unwrap());
        inputs.push(line[1].split(' ').map(|x| x.parse().unwrap()).collect());
    });
    for i in 0..inputs.len() {}

    println!("{:?}", inputs);
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
