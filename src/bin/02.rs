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
            println!("yes {:?}", data);
            secure.push(data);
        } else {
            unsecure.push(data);
        }
    }
    println!("no secure");
    for data in unsecure {
        println!("data: {:?}", data);
        for element in data.clone() {
            let mut temp = data.clone();
            temp.remove(temp.iter().position(|x| *x == element).unwrap());
            if temp.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
                || temp.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3)
            {
                println!("yes {:?}", data);
                secure.push(temp);
                break;
            } else {
                println!("no {:?}", temp);
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
}
