advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut numbers: Vec<u32> = Vec::new();
    let mut sum = 0;
    splitted_input.iter().for_each(|line| {
        if line.contains('|') {
            let temp = line.replace(['(', ')'], "");
            let temp2: Vec<u32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            rules.push((temp2[0], temp2[1]));
            numbers.push(temp2[0]);
            numbers.push(temp2[1]);
        }
        if line.contains(",") {
            let temp: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            updates.push(temp);
        }
    });
    updates.iter().for_each(|update| {
        println!("{:?}", update);
        let mut correct = true;
        let pairs = update.windows(2);
        for pair in pairs {
            if numbers.contains(&pair[0])
                && numbers.contains(&pair[1])
                && !rules.contains(&(pair[0], pair[1]))
            {
                correct = false;
            }
        }
        //update.windows(2).for_each(|pair| {
        //    println!("{:?}", pair);
        //    if !rules.contains(&(pair[0], pair[1])) {
        //        correct = true;
        //    }
        //    println!("{}", correct);
        //});
        if correct {
            let middle_index = update.len() / 2;
            println!("{} {}", middle_index, update[middle_index]);
            sum += update[middle_index];
        }
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
