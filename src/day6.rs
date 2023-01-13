use std::{collections::VecDeque, fs};

use itertools::{FoldWhile, Itertools};

#[derive(Clone, Debug, PartialEq)]
struct State {
    queue: VecDeque<char>,
    count: u32,
}

impl State {
    fn enqueue(&mut self, c: char) -> bool {
        if self.queue.len() >= 4 {
            self.queue.pop_front();
            self.queue.push_back(c);
        } else {
            self.queue.push_back(c);
        }
        self.count += 1;

        self.queue.len() >= 4 && self.queue.iter().duplicates().next().is_none()
    }
    fn enqueue_part_2(&mut self, c: char) -> bool {
        if self.queue.len() >= 14 {
            self.queue.pop_front();
            self.queue.push_back(c);
        } else {
            self.queue.push_back(c);
        }
        self.count += 1;

        self.queue.len() >= 14 && self.queue.iter().duplicates().next().is_none()
    }
}

fn solve_part1(input: &str) -> u32 {
    input
        .chars()
        .fold_while(
            State {
                queue: VecDeque::new(),
                count: 0,
            },
            |mut state, element| {
                let finished = state.enqueue(element);
                if !finished {
                    FoldWhile::Continue(state)
                } else {
                    FoldWhile::Done(state)
                }
            },
        )
        .into_inner()
        .count
}
fn solve_part2(input: &str) -> u32 {
    input
        .chars()
        .fold_while(
            State {
                queue: VecDeque::new(),
                count: 0,
            },
            |mut state, element| {
                let finished = state.enqueue_part_2(element);
                if !finished {
                    FoldWhile::Continue(state)
                } else {
                    FoldWhile::Done(state)
                }
            },
        )
        .into_inner()
        .count
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day6.txt").expect("Should have been able to read the file");
    let part1_answer = solve_part1(contents.as_str());
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    let part2_answer = solve_part2(contents.as_str());
    println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    #[test]
    fn test_part_1_solver_a() {
        let output = solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(output, 7);
    }
    #[test]
    fn test_part_1_solver_b() {
        let output = solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(output, 5);
    }
    #[test]
    fn test_part_1_solver_c() {
        let output = solve_part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(output, 6);
    }
    #[test]
    fn test_part_1_solver_d() {
        let output = solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(output, 10);
    }
    #[test]
    fn test_part_1_solver_e() {
        let output = solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(output, 11);
    }

    // Part 2
    #[test]
    fn test_part_2_solver_a() {
        let output = solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(output, 19);
    }
    #[test]
    fn test_part_2_solver_b() {
        let output = solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(output, 23);
    }
    #[test]
    fn test_part_2_solver_c() {
        let output = solve_part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(output, 23);
    }
    #[test]
    fn test_part_2_solver_d() {
        let output = solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(output, 29);
    }
    #[test]
    fn test_part_2_solver_e() {
        let output = solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(output, 26);
    }
}
