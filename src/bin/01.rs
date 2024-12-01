advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    let mut numbers_a: Vec<u32> = Vec::new();
    let mut numbers_b: Vec<u32> = Vec::new();
    for line in splitted_input {
        let data: Vec<u32> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        let mut first = true;
        for d in data {
            if first {
                numbers_a.push(d);
                first = false;
            } else {
                numbers_b.push(d);
            }
        }
    }

    numbers_a.sort();
    numbers_b.sort();

    for i in 0..numbers_a.len() {
        if numbers_a[i] > numbers_b[i] {
            sum += numbers_a[i] - numbers_b[i];
        } else {
            sum += numbers_b[i] - numbers_a[i];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    let mut numbers_a: Vec<u32> = Vec::new();
    let mut numbers_b: Vec<u32> = Vec::new();
    for line in splitted_input {
        let data: Vec<u32> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        let mut first = true;
        for d in data {
            if first {
                numbers_a.push(d);
                first = false;
            } else {
                numbers_b.push(d);
            }
        }
    }

    (0..numbers_a.len()).for_each(|i| {
        if let (true, count) = count_occurrences(&numbers_b, numbers_a[i]) {
            sum += numbers_a[i] * count as u32;
        }
    });
    Some(sum)
}

fn count_occurrences(numbers: &[u32], target: u32) -> (bool, usize) {
    let contains = numbers.contains(&target);
    let count = numbers.iter().filter(|&&x| x == target).count();
    (contains, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
