// pub fn part_one(input: &str) -> Option<u32> {
//     let mut top: [u32; 1] = [0];
//     let mut current: u32 = 0;
//     for line in input.lines() {
//         if line.is_empty() {
//             let mut c = current;
//             for top_item in top.iter_mut() {
//                 if c > *top_item {
//                     (*top_item, c) = (c, *top_item);
//                 }
//             }
//             current = 0;
//         } else {
//             current += line.trim().parse::<u32>().unwrap();
//         }
//     }
//     Some(top.iter().sum::<u32>())
// }
//
// pub fn part_two(input: &str) -> Option<u32> {
//     let mut top: [u32; 3] = [0; 3];
//     let mut current: u32 = 0;
//     for line in input.lines() {
//         if line.is_empty() {
//             let mut c = current;
//             for top_item in top.iter_mut() {
//                 if c > *top_item {
//                     (*top_item, c) = (c, *top_item);
//                 }
//             }
//             current = 0;
//         } else {
//             current += line.trim().parse::<u32>().unwrap();
//         }
//     }
//     Some(top.iter().sum::<u32>())
// }

pub fn part_one(input: &str) -> Option<u32> {
    Some(*input
        .split("\r\n\r\n")
        .map(|g| {
            g.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut e: Vec<u32> = input
        .split("\r\n\r\n")
        .map(|g| {
            g.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect::<Vec<u32>>();
    e.sort();
    Some(e[e.len() - 3..e.len()].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input); // 68802
    advent_of_code::solve!(2, part_two, input); // 205370
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
