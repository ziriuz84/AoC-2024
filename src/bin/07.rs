advent_of_code::solution!(7);
use rand::Rng;

fn test_result(result: u64, prev_result: u64, inputs: Vec<u64>, index: usize) -> bool {
    if index >= inputs.len() {
        return false;
    }
    if result < prev_result + inputs[index] && result < prev_result * inputs[index] {
        return false;
    }
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut new_result: u64 = 0;
    if random_number > 0.5 {
        new_result = prev_result * inputs[index];
    } else {
        new_result = prev_result + inputs[index];
    }
    if new_result == result {
        true
    } else {
        test_result(result, new_result, inputs, index + 1)
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut results: Vec<u64> = Vec::new();
    let mut inputs: Vec<Vec<u64>> = Vec::new();
    let mut sum = 0;
    splitted_input.iter().for_each(|l| {
        let line: Vec<&str> = l.split(": ").collect();
        results.push(line[0].parse().unwrap());
        inputs.push(line[1].split(' ').map(|x| x.parse().unwrap()).collect());
    });
    for i in 0..inputs.len() {
        for _j in 0..50000 {
            if test_result(results[i], 0, inputs[i].clone(), 0) {
                println!("{} {:?}", results[i], inputs[i].clone());
                sum += results[i];
                break;
            }
        }
    }

    Some(sum)
    //None
}

pub fn part_two(input: &str) -> Option<u64> {
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
