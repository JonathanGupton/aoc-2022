use lazy_static::lazy_static;
use regex::Regex;

fn full_inclusion_comparison(left: (i32, i32), right: (i32, i32)) -> bool {
    (left.0 <= right.0 && left.1 >= right.1) || (right.0 <= left.0 && right.1 >= left.1)
}

fn partial_inclusion_comparison(left: (i32, i32), right: (i32, i32)) -> bool {
    (left.0 <= right.1) && (right.0 <= left.1)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut found = RE.find_iter(&line);
    let left: (i32, i32) = (
        found.next().unwrap().as_str().parse().unwrap(),
        found.next().unwrap().as_str().parse().unwrap(),
    );
    let right: (i32, i32) = (
        found.next().unwrap().as_str().parse().unwrap(),
        found.next().unwrap().as_str().parse().unwrap(),
    );
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut overlaps: u32 = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        if full_inclusion_comparison(left, right) {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlaps: u32 = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        if partial_inclusion_comparison(left, right) {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    parse_line("1-2,3-4");
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
