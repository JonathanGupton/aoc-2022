use std::collections::HashSet;

fn score_char(letter: &char) -> u32 {
    let i = *letter as u32;
    if i >= 97 {
        i - 96
    } else {
        i - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut compartment_a: HashSet<char> = HashSet::with_capacity(25);
    let mut priorities: u32 = 0;
    for line in input.lines() {
        let pivot = line.len() / 2;
        for c in line[0..pivot].chars() {
            compartment_a.insert(c);
        }
        for c in line[pivot..].chars() {
            if compartment_a.contains(&c) {
                priorities += score_char(&c);
                break;
            }
        }
        compartment_a.clear();
    }
    Some(priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut priorities: u32 = 0;
    let mut set1: HashSet<char> = HashSet::with_capacity(50);
    let mut set2: HashSet<char> = HashSet::with_capacity(50);
    let mut set3: HashSet<char> = HashSet::with_capacity(50);
    let mut set4: HashSet<char> = HashSet::with_capacity(50);

    for lines in input.lines().collect::<Vec<&str>>().chunks_exact_mut(3) {
        let mut l = lines.iter_mut();
        for c in l.next().unwrap().chars() {
            set1.insert(c);
        }
        for c in l.next().unwrap().chars() {
            set2.insert(c);
        }
        for c in l.next().unwrap().chars() {
            set3.insert(c);
        }
        for c in set1.intersection(&set2) {
            set4.insert(*c);
        }
        priorities += score_char(set3.intersection(&set4).next().unwrap());

        set1.clear();
        set2.clear();
        set3.clear();
        set4.clear();
    }
    Some(priorities)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
