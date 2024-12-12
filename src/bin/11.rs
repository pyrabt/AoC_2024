use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let init: Vec<usize> = input
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut stones: HashMap<usize, usize> = HashMap::new();
    for stone in init {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..25 {
        let mut stones_after_blink: HashMap<usize, usize> = HashMap::new();
        for stone in stones.clone() {
            if stone.0 == 0 {
                *stones_after_blink.entry(1).or_default() += stone.1;
            } else if stone.0.to_string().chars().count() % 2 == 0 {
                let s = stone.0.to_string();
                let (left, right) = s.split_at(s.chars().count() / 2);
                *stones_after_blink.entry(left.parse().unwrap()).or_default() += stone.1;
                *stones_after_blink
                    .entry(right.parse().unwrap())
                    .or_default() += stone.1;
            } else {
                *stones_after_blink.entry(stone.0 * 2024).or_default() += stone.1;
            }
        }
        stones = stones_after_blink;
    }

    Some(stones.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let init: Vec<usize> = input
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut stones: HashMap<usize, usize> = HashMap::new();
    for stone in init {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..75 {
        let mut stones_after_blink: HashMap<usize, usize> = HashMap::new();
        for stone in stones.clone() {
            if stone.0 == 0 {
                *stones_after_blink.entry(1).or_default() += stone.1;
            } else if stone.0.to_string().chars().count() % 2 == 0 {
                let s = stone.0.to_string();
                let (left, right) = s.split_at(s.chars().count() / 2);
                *stones_after_blink.entry(left.parse().unwrap()).or_default() += stone.1;
                *stones_after_blink
                    .entry(right.parse().unwrap())
                    .or_default() += stone.1;
            } else {
                *stones_after_blink.entry(stone.0 * 2024).or_default() += stone.1;
            }
        }
        stones = stones_after_blink;
    }

    Some(stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
