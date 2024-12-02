use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_nums = vec![];
    let mut right_nums = vec![];

    for line in input.lines() {
        let (left, right) = line.split_at(line.find(' ')?);
        left_nums.push(left.parse::<i32>().unwrap());
        right_nums.push(right.trim().parse::<i32>().unwrap());
    }

    let mut total = 0;
    left_nums.sort_unstable();
    right_nums.sort_unstable();

    for i in 0..left_nums.len() {
        total += (left_nums[i] - right_nums[i]).abs() as u32;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_nums = vec![];
    let mut right_nums: HashMap<u32,u32> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_at(line.find(' ')?);
        left_nums.push(left.parse::<u32>().unwrap());
        *right_nums.entry(right.trim().parse::<u32>().unwrap()).or_default() += 1;
    }

    let mut total = 0;

    for i in 0..left_nums.len() {
        if right_nums.contains_key(&left_nums[i]) {
            total += left_nums[i] * right_nums.get(&left_nums[i]).unwrap();
        }
    }

    Some(total)
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
