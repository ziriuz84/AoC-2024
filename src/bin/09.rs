use std::u32;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<char> = input.chars().collect();
    let numbers: Vec<u32> = splitted_input
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut filesystem: Vec<char> = Vec::new();
    let mut counter: usize = 0;
    let mut number_list: Vec<(usize, char)> = Vec::new();
    let mut dot_list: Vec<(usize, char)> = Vec::new();
    for (i, n) in numbers.iter().enumerate() {
        println!("{} {}", i, n);
        let number = n.clone() as usize;
        for j in i..number + i {
            if i % 2 == 1 {
                filesystem.push('.');
                dot_list.push((j + i, '.'));
            } else {
                filesystem.push(counter.to_string().as_bytes()[0] as char);
                number_list.push((j + i, counter.to_string().as_bytes()[0] as char));
            }
        }
        if i % 2 == 0 {
            counter += 1;
        }
    }
    println!("{:?}", filesystem);
    println!(
        "{} {} {}",
        filesystem.len(),
        number_list.len(),
        dot_list.len()
    );
    println!("{:?}", number_list);
    println!("{:?}", dot_list);
    for i in 0..filesystem.len() {
        //println!("sto parsando: {} {}", i, filesystem[i]);
        if filesystem[i] == '.' {
            let (position, number) = number_list.pop().unwrap();
            //println!("Ho preso: {} {}", position, number);
            filesystem[i] = number;
            filesystem.remove(position);
            filesystem.push('.');
        }
        //println!("{:?}", filesystem);
    }
    println!("{:?}", filesystem);
    let mut sum = 0;
    for i in 0..filesystem.len() - 1 {
        if filesystem[i] != '.' {
            sum = filesystem[i].to_string().parse::<u32>().unwrap() * i as u32 + sum;
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
