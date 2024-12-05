use std::cmp;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);
    let mut total = 0;

    for update in updates {
        let mut valid_update = true;
        for i in 0..update.len() {
            if !rules.contains_key(&update[i]) {
                continue;
            }

            let num_rules = rules.get(&update[i]).unwrap();
            for rule in num_rules {
                if update.contains(&rule) && update[..i].contains(rule) {
                    valid_update = false;
                    break;
                }
            }
        }
        if valid_update {
            let middle_page = update.len() / 2;
            total += update[middle_page].parse::<u32>().unwrap();
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);
    let mut total = 0;

    for update in updates {
        let mut valid_update = true;
        for i in 0..update.len() {
            if !rules.contains_key(&update[i]) {
                continue;
            }

            let num_rules = rules.get(&update[i]).unwrap();
            for rule in num_rules {
                if update.contains(&rule) && update[..i].contains(rule) {
                    valid_update = false;
                    break;
                }
            }
        }
        if !valid_update {
            let mut fixed_update = update.clone();
            fixed_update.sort_by(|a, b| {
                if rules.contains_key(a) && rules[a].contains(b) {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Greater
                }
            });
            let middle_page = fixed_update.len() / 2;
            total += fixed_update[middle_page].parse::<u32>().unwrap();
        }
    }

    Some(total)
}

fn get_rules_and_updates(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut updates: Vec<Vec<&str>> = vec![];

    let mut read_updates = false;
    for line in input.lines() {
        if line.is_empty() {
            read_updates = true;
            continue;
        }

        if read_updates {
            updates.push(line.split(',').collect());
        } else {
            let (rule_num, rule_before) = line.split_once('|').unwrap();
            rules
                .entry(rule_num)
                .or_insert(Vec::new())
                .push(rule_before);
        }
    }

    (rules, updates)
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
        assert_eq!(result, Some(123));
    }
}
