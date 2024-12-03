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
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut secure = Vec::new();
    let mut unsecure = Vec::new();
    for line in splitted_input.clone() {
        let data: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if data.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
            || data.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
        {
            secure.push(data);
        } else {
            unsecure.push(data);
        }
    }
    for data in unsecure {
        for (i, element) in data.clone().into_iter().enumerate() {
            let mut temp = data.clone();
            temp.remove(i);
            if temp.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                || temp.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
            {
                secure.push(temp);
                break;
            } else {
            }
        }
    }
    Some(secure.len() as u32)
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one2() {
        let input = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7";
        let result = part_one(input);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two2() {
        let input = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7";
        let result = part_two(input);
        assert_eq!(result, Some(8));
    }
}
