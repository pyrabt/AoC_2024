use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let regex = Regex::new(r"mul\((\d*,\d*)\)").unwrap();
    let results: Vec<&str> = regex
        .captures_iter(input)
        .map(|caps| {
            let (_, [c]) = caps.extract();
            c
        })
        .collect();

    for calculation in results {
        let numbers: Vec<u32> = calculation.split(',').map(|c| c.parse().unwrap()).collect();
        total += numbers[0] * numbers[1];
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let regex = Regex::new(r"mul\((\d*,\d*)?\)|((do\(\))|(don\'t\(\)))").unwrap();
    let results: Vec<(&str, &str, &str)> = regex
        .captures_iter(input)
        .map(|caps| {
            let calc = caps.get(1).map_or("", |m| m.as_str());
            let dont = caps.get(2).map_or("", |m| m.as_str());
            let do_c = caps.get(3).map_or("", |m| m.as_str());
            (calc, dont, do_c)
        })
        .collect();

    let mut do_calc: bool = true;
    for mtch in results {
        if mtch.2 == "do()" {
            do_calc = true;
            continue;
        }
        if mtch.1 == "don't()" {
            do_calc = false;
            continue;
        }
        if !do_calc {
            continue;
        }
        let numbers: Vec<u32> = mtch.0.split(',').map(|c| c.parse().unwrap()).collect();
        total += numbers[0] * numbers[1];
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
