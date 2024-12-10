use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<u32>> = vec![];
    let mut th_positions: Vec<(usize, usize)> = vec![];

    let mut y_pos = 0;
    for line in input.lines() {
        let values = line
            .chars()
            .map(|n| n.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        for x in 0..values.len() {
            if values[x] == 0 {
                th_positions.push((y_pos, x))
            }
        }
        y_pos += 1;
        map.push(values);
    }

    Some(
        th_positions
            .iter()
            .map(|p| find_th_score(&map, p))
            .sum::<u32>(),
    )
}

fn find_th_score(map: &Vec<Vec<u32>>, th_pos: &(usize, usize)) -> u32 {
    find_th_unique_peaks(map, th_pos).len() as u32
}

fn find_th_unique_peaks(
    map: &Vec<Vec<u32>>,
    current_pos: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let mut peaks_reached: HashSet<(usize, usize)> = HashSet::new();
    let current_value = map[current_pos.0][current_pos.1];

    if current_value == 9 {
        peaks_reached.insert((current_pos.0, current_pos.1));
        return peaks_reached;
    }

    if current_pos.1 > 0 && map[current_pos.0][current_pos.1 - 1] == current_value + 1 {
        peaks_reached.extend(find_th_unique_peaks(
            map,
            &(current_pos.0, current_pos.1 - 1),
        ));
    }
    if current_pos.1 < map[0].len() - 1
        && map[current_pos.0][current_pos.1 + 1] == current_value + 1
    {
        peaks_reached.extend(find_th_unique_peaks(
            map,
            &(current_pos.0, current_pos.1 + 1),
        ));
    }
    if current_pos.0 > 0 && map[current_pos.0 - 1][current_pos.1] == current_value + 1 {
        peaks_reached.extend(find_th_unique_peaks(
            map,
            &(current_pos.0 - 1, current_pos.1),
        ));
    }
    if current_pos.0 < map.len() - 1 && map[current_pos.0 + 1][current_pos.1] == current_value + 1 {
        peaks_reached.extend(find_th_unique_peaks(
            map,
            &(current_pos.0 + 1, current_pos.1),
        ));
    }

    peaks_reached
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<u32>> = vec![];
    let mut th_positions: Vec<(usize, usize)> = vec![];

    let mut y_pos = 0;
    for line in input.lines() {
        let values = line
            .chars()
            .map(|n| n.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        for x in 0..values.len() {
            if values[x] == 0 {
                th_positions.push((y_pos, x))
            }
        }
        y_pos += 1;
        map.push(values);
    }

    Some(
        th_positions
            .iter()
            .map(|p| find_th_all_scores(&map, p))
            .sum::<u32>(),
    )
}

fn find_th_all_scores(map: &Vec<Vec<u32>>, current_pos: &(usize, usize)) -> u32 {
    let mut score = 0;
    let current_value = map[current_pos.0][current_pos.1];

    if current_value == 9 {
        return 1;
    }

    if current_pos.1 > 0 && map[current_pos.0][current_pos.1 - 1] == current_value + 1 {
        score += find_th_all_scores(map, &(current_pos.0, current_pos.1 - 1));
    }
    if current_pos.1 < map[0].len() - 1
        && map[current_pos.0][current_pos.1 + 1] == current_value + 1
    {
        score += find_th_all_scores(map, &(current_pos.0, current_pos.1 + 1));
    }
    if current_pos.0 > 0 && map[current_pos.0 - 1][current_pos.1] == current_value + 1 {
        score += find_th_all_scores(map, &(current_pos.0 - 1, current_pos.1));
    }
    if current_pos.0 < map.len() - 1 && map[current_pos.0 + 1][current_pos.1] == current_value + 1 {
        score += find_th_all_scores(map, &(current_pos.0 + 1, current_pos.1));
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
