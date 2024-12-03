advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut is_safe = true;
        let is_descending = levels[0] - levels[1] > 0;

        for i in 0..levels.len() - 1 {
            let diff = levels[i] - levels[i + 1];
            if is_safe && diff.abs() <= 3 && diff.abs() > 0 {
                if diff > 0 && !is_descending {
                    is_safe = false;
                    break;
                }
                if diff < 0 && is_descending {
                    is_safe = false;
                }
            } else {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if !levels_are_safe(&levels) {
            for i in 0..levels.len() {
                let mut mut_levels = levels.clone();
                mut_levels.remove(i);
                if levels_are_safe(&mut_levels) {
                    total += 1;
                    break;
                }
            }
        } else {
            total += 1;
        }
    }

    Some(total)
}

fn levels_are_safe(levels: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let is_descending = levels[0] - levels[1] > 0;
    for i in 0..levels.len() - 1 {
        let diff = levels[i] - levels[i + 1];
        if is_safe && diff.abs() <= 3 && diff.abs() > 0 {
            if diff > 0 && !is_descending {
                is_safe = false;
                break;
            }
            if diff < 0 && is_descending {
                is_safe = false;
            }
        } else {
            is_safe = false;
            break;
        }
    }

    is_safe
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
