advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut guard_pos = find_guard(&grid).unwrap();
    let mut exited = false;
    let mut curr_dir: u32 = 0;
    while !exited {
        grid[guard_pos.0][guard_pos.1] = 'X';
        if curr_dir == 0 {
            if guard_pos.0 == 0 {
                exited = true;
            } else if guard_pos.0 > 0 && grid[guard_pos.0 - 1][guard_pos.1] != '#' {
                guard_pos = (guard_pos.0 - 1, guard_pos.1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else if curr_dir == 1 {
            if guard_pos.1 == grid[0].len() - 1 {
                exited = true;
            } else if guard_pos.1 < grid[0].len() - 1 && grid[guard_pos.0][guard_pos.1 + 1] != '#' {
                guard_pos = (guard_pos.0, guard_pos.1 + 1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else if curr_dir == 2 {
            if guard_pos.0 == grid.len() - 1 {
                exited = true;
            } else if guard_pos.0 < grid.len() - 1 && grid[guard_pos.0 + 1][guard_pos.1] != '#' {
                guard_pos = (guard_pos.0 + 1, guard_pos.1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else {
            if guard_pos.1 == 0 {
                exited = true;
            } else if guard_pos.1 > 0 && grid[guard_pos.0][guard_pos.1 - 1] != '#' {
                guard_pos = (guard_pos.0, guard_pos.1 - 1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        }
    }

    let total: usize = grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == 'X').collect::<Vec<_>>().len())
        .sum();
    Some(total as u32)
}

fn find_guard(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return Some((y, x));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let guard_pos = find_guard(&grid).unwrap();
    grid[guard_pos.0][guard_pos.1] = '.';

    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if (guard_pos.0 == y && guard_pos.1 == x) || grid[y][x] == '#' {
                continue;
            }
            grid[y][x] = '#';
            if stuck_in_loop(&grid, &guard_pos) {
                total += 1;
            }
            grid[y][x] = '.';
        }
    }

    Some(total)
}

fn stuck_in_loop(grid: &Vec<Vec<char>>, starting_pos: &(usize, usize)) -> bool {
    let mut guard_pos = (starting_pos.0, starting_pos.1);
    let mut exited = false;
    let mut step_cnt = 0;
    let mut curr_dir: u32 = 0;
    let cutoff = grid.len() * grid.len();
    loop {
        if curr_dir == 0 {
            if guard_pos.0 == 0 {
                exited = true;
            } else if guard_pos.0 > 0 && grid[guard_pos.0 - 1][guard_pos.1] != '#' {
                guard_pos = (guard_pos.0 - 1, guard_pos.1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else if curr_dir == 1 {
            if guard_pos.1 == grid[0].len() - 1 {
                exited = true;
            } else if guard_pos.1 < grid[0].len() - 1 && grid[guard_pos.0][guard_pos.1 + 1] != '#' {
                guard_pos = (guard_pos.0, guard_pos.1 + 1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else if curr_dir == 2 {
            if guard_pos.0 == grid.len() - 1 {
                exited = true;
            } else if guard_pos.0 < grid.len() - 1 && grid[guard_pos.0 + 1][guard_pos.1] != '#' {
                guard_pos = (guard_pos.0 + 1, guard_pos.1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        } else {
            if guard_pos.1 == 0 {
                exited = true;
            } else if guard_pos.1 > 0 && grid[guard_pos.0][guard_pos.1 - 1] != '#' {
                guard_pos = (guard_pos.0, guard_pos.1 - 1);
            } else {
                curr_dir = (curr_dir + 1) % 4;
            }
        }
        step_cnt += 1;
        if exited || step_cnt >= cutoff {
            break;
        }
    }

    if exited {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
