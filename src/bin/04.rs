advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut search_grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        search_grid.push(line.chars().collect());
    }

    let max_x = search_grid[0].len() - 1 - 3;
    let max_y = search_grid.len() - 1 - 3;

    let mut total = 0;
    for y in 0..search_grid.len() {
        for x in 0..search_grid[0].len() {
            if search_grid[y][x] != 'X' {
                continue;
            }

            //left
            if x >= 3 {
                if search_grid[y][x - 1] == 'M'
                    && search_grid[y][x - 2] == 'A'
                    && search_grid[y][x - 3] == 'S'
                {
                    total += 1;
                }
            }
            //left up
            if x >= 3 && y >= 3 {
                if search_grid[y - 1][x - 1] == 'M'
                    && search_grid[y - 2][x - 2] == 'A'
                    && search_grid[y - 3][x - 3] == 'S'
                {
                    total += 1;
                }
            }
            //up
            if y >= 3 {
                if search_grid[y - 1][x] == 'M'
                    && search_grid[y - 2][x] == 'A'
                    && search_grid[y - 3][x] == 'S'
                {
                    total += 1;
                }
            }
            //right up
            if x <= max_x && y >= 3 {
                if search_grid[y - 1][x + 1] == 'M'
                    && search_grid[y - 2][x + 2] == 'A'
                    && search_grid[y - 3][x + 3] == 'S'
                {
                    total += 1;
                }
            }
            //right
            if x <= max_x {
                if search_grid[y][x + 1] == 'M'
                    && search_grid[y][x + 2] == 'A'
                    && search_grid[y][x + 3] == 'S'
                {
                    total += 1;
                }
            }
            //right down
            if x <= max_x && y <= max_y {
                if search_grid[y + 1][x + 1] == 'M'
                    && search_grid[y + 2][x + 2] == 'A'
                    && search_grid[y + 3][x + 3] == 'S'
                {
                    total += 1;
                }
            }
            //down
            if y <= max_y {
                if search_grid[y + 1][x] == 'M'
                    && search_grid[y + 2][x] == 'A'
                    && search_grid[y + 3][x] == 'S'
                {
                    total += 1;
                }
            }
            //left down
            if x >= 3 && y <= max_y {
                if search_grid[y + 1][x - 1] == 'M'
                    && search_grid[y + 2][x - 2] == 'A'
                    && search_grid[y + 3][x - 3] == 'S'
                {
                    total += 1;
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut search_grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        search_grid.push(line.chars().collect());
    }

    let max_x = search_grid[0].len() - 1 - 1;
    let max_y = search_grid.len() - 1 - 1;

    let mut total = 0;
    for y in 0..search_grid.len() {
        for x in 0..search_grid[0].len() {
            if search_grid[y][x] != 'A' {
                continue;
            }
            if x >= 1 && x <= max_x && y >= 1 && y <= max_y {
                let diag_left_found = search_grid[y - 1][x - 1] == 'M'
                    && search_grid[y + 1][x + 1] == 'S'
                    || search_grid[y - 1][x - 1] == 'S' && search_grid[y + 1][x + 1] == 'M';
                let diag_right_fround = search_grid[y + 1][x - 1] == 'M'
                    && search_grid[y - 1][x + 1] == 'S'
                    || search_grid[y + 1][x - 1] == 'S' && search_grid[y - 1][x + 1] == 'M';
                if diag_left_found && diag_right_fround {
                    total += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
