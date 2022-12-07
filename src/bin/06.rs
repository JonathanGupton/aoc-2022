//Refactored solution without imports that completes in 4-5µs
fn find_start_of_next_packet(packet: &str, distinct: usize) -> usize {
    let packet = packet.as_bytes();
    let mut left: usize = 0;
    let mut right: usize = 0;

    // count of each letter found in window
    let mut letters: [u32; 26] = [0; 26];
    let mut distinct_count: usize = 0;

    let mut c: &u8;
    while right < distinct {
        c = &packet[right];
        if letters[*c as usize - 97usize] == 0 {
            distinct_count += 1;
        }
        letters[*c as usize - 97usize] += 1;
        right += 1;
    }
    // Move the window until the break point is found
    while right < packet.len() {
        if distinct_count == distinct {
            break;
        }
        letters[(packet[left] as usize) - 97] -= 1;
        if letters[(packet[left] as usize) - 97] == 0 {
            distinct_count -= 1;
        }

        if letters[(packet[right] as usize) - 97] == 0 {
            distinct_count += 1;
        }
        letters[(packet[right] as usize) - 97] += 1;

        left += 1;
        right += 1;
    }
    right
}
//
//
// use std::collections::VecDeque;
//
// fn find_start_of_next_packet(packet: &str, distinct: usize) -> usize {
//     let mut position = 0;
//     // count of each letter found in distinct range
//     let mut letters: [u32; 26] = [0; 26];
//     // queue containing the current window of characters
//     let mut queue: VecDeque<char> = VecDeque::with_capacity(distinct);
//
//     let mut distinct_count: usize = 0;
//
//     let mut packet_iter = packet.chars();
//
//     // Load the holding deque and char counts
//     for _ in 0..distinct {
//         if let Some(c) = packet_iter.next() {
//             queue.push_back(c);
//             if letters[(c as usize) - 97usize] == 0 {
//                 distinct_count += 1;
//             }
//             letters[(c as usize) - 97usize] += 1;
//
//             position += 1;
//         }
//     }
//
//     // loop through chars to find break point
//     while let Some(c) = packet_iter.next() {
//         if distinct_count == distinct {
//             break;
//         } else {
//             let remove = queue.pop_front().unwrap();
//             letters[(remove as usize) - 97usize] -= 1;
//             if letters[(remove as usize) - 97usize] == 0 {
//                 distinct_count -= 1;
//             }
//             queue.push_back(c);
//             if letters[(c as usize) - 97usize] == 0 {
//                 distinct_count += 1;
//             }
//             letters[(c as usize) - 97usize] += 1;
//             position += 1;
//         }
//     }
//
//     position
// }

// Original solution, twice the time of the solution above
// fn find_start_of_next_packet(packet: &str, distinct: usize) -> usize {
//     let mut position = 0;
//     // count of each letter found in distinct range
//     let mut letters: [u32; 26] = [0; 26];
//     // queue containing the current window of characters
//     let mut queue: VecDeque<char> = VecDeque::with_capacity(distinct);
//
//     let mut packet_iter = packet.chars();
//
//     // Load the holding deque and char counts
//     for _ in 0..distinct {
//         if let Some(c) = packet_iter.next() {
//             queue.push_back(c);
//             letters[(c as usize) - 97usize] += 1;
//             position += 1;
//         }
//     }
//
//     // loop through chars to find break point
//     while let Some(c) = packet_iter.next() {
//         if letters.iter().filter(|v| **v > 0u32).count() == distinct {
//             break;
//         } else {
//             let remove = queue.pop_front().unwrap();
//             letters[(remove as usize) - 97usize] -= 1;
//             queue.push_back(c);
//             letters[(c as usize) - 97usize] += 1;
//             position += 1;
//         }
//     }
//
//     position
// }

// The solution below is 10 times slower than my solution
// use std::collections::HashSet;
// fn find_start_of_next_packet(packet: &str, distinct: usize) -> usize {
//     let mut set: HashSet<char> = HashSet::new();
//     let mut output: usize = 0;
//     for i in distinct..packet.len() {
//         let inner_packet: &str = &packet[(i - distinct)..i];
//         set.extend(inner_packet.chars());
//         if set.len() == distinct {
//             output = i;
//             break;
//         }
//         set.clear();
//     }
//     output
// }

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_start_of_next_packet(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_start_of_next_packet(input, 14) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
