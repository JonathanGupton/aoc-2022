fn parse_crates(s: &str) -> Vec<Vec<char>> {
    let n_crates: usize = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut crates: Vec<Vec<char>> = Vec::with_capacity(n_crates);
    for _ in 0..n_crates {
        crates.push(Vec::new())
    }
    for row in s.lines().rev() {
        for (i, c) in row.chars().enumerate() {
            if c.is_alphabetic() {
                let idx = (i - 1) / 4;
                crates.get_mut(idx).unwrap().push(c);
            }
        }
    }
    crates
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let mut instruction = instruction.split_whitespace();
    instruction.next();
    let n: usize = instruction.next().unwrap().parse().unwrap();
    instruction.next();
    let source: usize = instruction.next().unwrap().parse().unwrap();
    instruction.next();
    let destination: usize = instruction.next().unwrap().parse().unwrap();
    (n, source, destination)
}

fn get_top_crates(crates: Vec<Vec<char>>) -> String {
    String::from_iter(
        crates
            .iter()
            .map(|stack| *stack.iter().last().unwrap())
            .collect::<Vec<char>>(),
    )
}

pub fn part_one(input: &str) -> Option<String> {
    let mut split_input = input.split("\r\n\r\n");
    let mut crates = parse_crates(split_input.next().unwrap());
    for instruction in split_input.next().unwrap().lines() {
        let (n, source, destination) = parse_instruction(instruction);
        for _ in 0..n {
            let v = crates.get_mut(source - 1).unwrap().pop().unwrap();
            crates.get_mut(destination - 1).unwrap().push(v);
        }
    }
    let output = get_top_crates(crates);
    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut split_input = input.split("\r\n\r\n");
    let mut crates = parse_crates(split_input.next().unwrap());
    let mut hold: Vec<char> = Vec::new();
    for instruction in split_input.next().unwrap().lines() {
        let (n, source, destination) = parse_instruction(instruction);
        for _ in 0..n {
            let v = crates.get_mut(source - 1).unwrap().pop().unwrap();
            hold.push(v);
        }
        while let Some(v) = hold.pop() {
            crates.get_mut(destination - 1).unwrap().push(v);
        }
    }
    let output = get_top_crates(crates);
    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
