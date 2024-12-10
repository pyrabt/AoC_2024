use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines() {
        let (value, numbers) = line.split_once(":").unwrap();
        let target: u64 = value.parse().unwrap();
        let n: Vec<&str> = numbers.split_whitespace().collect();
        let f: Vec<&str> = n
            .iter()
            .interleave(vec![&""; n.len() - 1])
            .copied()
            .collect();

        let permutations = get_permutations(f, 1, false);
        for p in permutations {
            let mut t: u64 = 0;
            for i in (1..p.len()).step_by(2) {
                if p[i] == "+" {
                    if i == 1 {
                        t += p[i - 1].parse::<u64>().unwrap() + p[i + 1].parse::<u64>().unwrap();
                    } else {
                        t += p[i + 1].parse::<u64>().unwrap();
                    }
                } else if i == 1 {
                    t += p[i - 1].parse::<u64>().unwrap() * p[i + 1].parse::<u64>().unwrap();
                } else {
                    t *= p[i + 1].parse::<u64>().unwrap();
                }
                if t > target {
                    break;
                }
            }
            if t == target {
                total += target;
                break;
            }
        }
    }

    Some(total)
}

fn get_permutations(numbers: Vec<&str>, op_index: usize, is_part_two: bool) -> Vec<Vec<&str>> {
    let mut permutations: Vec<Vec<&str>> = vec![];
    if op_index >= numbers.len() - 1 {
        permutations.push(numbers);
        return permutations;
    }

    let mut mult = numbers.clone();
    mult[op_index] = "*";
    permutations.extend(get_permutations(mult, op_index + 2, is_part_two));

    let mut plus = numbers.clone();
    plus[op_index] = "+";
    permutations.extend(get_permutations(plus, op_index + 2, is_part_two));

    if is_part_two {
        let mut con_cat = numbers.clone();
        con_cat[op_index] = "||";
        permutations.extend(get_permutations(con_cat, op_index + 2, is_part_two));
    }

    permutations
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines() {
        let (value, numbers) = line.split_once(":").unwrap();
        let target: u64 = value.parse().unwrap();
        let n: Vec<&str> = numbers.split_whitespace().collect();
        let f: Vec<&str> = n
            .iter()
            .interleave(vec![&""; n.len() - 1])
            .copied()
            .collect();

        let permutations = get_permutations(f, 1, true);
        for p in permutations {
            let mut t: u64 = 0;
            for i in (1..p.len()).step_by(2) {
                if p[i] == "+" {
                    if i == 1 {
                        t += p[i - 1].parse::<u64>().unwrap() + p[i + 1].parse::<u64>().unwrap();
                    } else {
                        t += p[i + 1].parse::<u64>().unwrap();
                    }
                } else if p[i] == "*" {
                    if i == 1 {
                        t += p[i - 1].parse::<u64>().unwrap() * p[i + 1].parse::<u64>().unwrap();
                    } else {
                        t *= p[i + 1].parse::<u64>().unwrap();
                    }
                } else if i == 1 {
                    let mut concatenated = p[i - 1].to_owned();
                    concatenated.push_str(p[i + 1]);
                    t += concatenated.parse::<u64>().unwrap();
                } else {
                    let mut concatenated = t.to_string();
                    concatenated.push_str(p[i + 1]);
                    t = concatenated.parse::<u64>().unwrap();
                }
                if t > target {
                    break;
                }
            }
            if t == target {
                total += target;
                break;
            }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
