
pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.lines(){
        let line = line.trim();
        if line == "A X" {
            score += 4;
        } else if line == "A Y" {
            score += 8;
        } else if line == "A Z" {
            score += 3;
        } else if line == "B X" {
            score += 1;
        } else if line == "B Y" {
            score += 5;
        } else if line == "B Z" {
            score += 9;
        } else if line == "C X" {
            score += 7;
        } else if line == "C Y" {
            score += 2;
        } else if line == "C Z" {
            score += 6;
        }
    }

    Some(score)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.lines(){
        let line = line.trim();
        if line == "A X" {
            score += 3;
        } else if line == "A Y" {
            score += 4;
        } else if line == "A Z" {
            score += 8;
        } else if line == "B X" {
            score += 1;
        } else if line == "B Y" {
            score += 5;
        } else if line == "B Z" {
            score += 9;
        } else if line == "C X" {
            score += 2;
        } else if line == "C Y" {
            score += 6;
        } else if line == "C Z" {
            score += 7;
        }
    }

    Some(score)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);  // 14264
    advent_of_code::solve!(2, part_two, input);  // 12382
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
