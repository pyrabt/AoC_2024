advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<String> = vec![];
    let chars: Vec<String> = input.chars().map(|c| c.to_string()).collect();
    for c in 0..chars.len() {
        let size: usize = chars[c].parse::<usize>().unwrap();
        if c % 2 == 0 {
            let char_to_set: String = if c == 0 {
                "0".to_owned()
            } else {
                (c / 2).to_string()
            };
            blocks.extend(vec![char_to_set; size]);
        } else {
            blocks.extend(vec![".".to_owned(); size]);
        }
    }

    let mut first_empty = blocks.iter().position(|c| c == ".").unwrap();
    let mut last_num = (blocks.len() - 1) - blocks.iter().rev().position(|c| c != ".").unwrap();
    while first_empty < last_num {
        blocks.swap(first_empty, last_num);
        first_empty = blocks.iter().position(|c| c == ".").unwrap();
        last_num = (blocks.len() - 1) - blocks.iter().rev().position(|c| c != ".").unwrap();
    }

    let mut total = 0;
    for i in 0..blocks.len() {
        if blocks[i] == "." {
            break;
        }
        total += i as u64 * blocks[i].parse::<u64>().unwrap();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<(usize, Vec<String>)> = vec![];
    let chars: Vec<String> = input.chars().map(|c| c.to_string()).collect();
    for c in 0..chars.len() {
        if chars[c] == "0" {
            continue;
        }
        let size: usize = chars[c].parse::<usize>().unwrap();
        if c % 2 == 0 {
            let char_to_set: String = if c == 0 {
                "0".to_owned()
            } else {
                (c / 2).to_string()
            };
            blocks.push((0, vec![char_to_set; size]));
        } else {
            blocks.push((size, vec![".".to_owned(); size]));
        }
    }

    loop {
        let mut swap_made = false;
        for id in (0..blocks.len()).rev() {
            if blocks[id].0 == 0 {
                for s in 0..blocks.len() {
                    if s >= id {
                        break;
                    }
                    if blocks[s].0 >= blocks[id].1.len() {
                        let diff = blocks[s].0 - blocks[id].1.len();
                        if diff > 0 {
                            for _ in 0..diff {
                                blocks[s].1.pop();
                            }
                            blocks[s].0 = blocks[id].1.len();
                        }
                        blocks.swap(id, s);
                        if diff > 0 {
                            blocks.insert(s + 1, (diff, vec![".".to_owned(); diff]));
                        }
                        swap_made = true;
                        break;
                    }
                }
            }
            if swap_made {
                break;
            }
        }
        if !swap_made {
            break;
        }
    }

    let mut total = 0;
    let joined_blocks: Vec<String> = blocks
        .iter()
        .map(|b| b.1.clone())
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect();
    for i in 0..joined_blocks.len() {
        if joined_blocks[i] == "." {
            continue;
        }
        total += i as u64 * joined_blocks[i].parse::<u64>().unwrap();
    }

    Some(total)
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
        assert_eq!(result, Some(2858));
    }
}
