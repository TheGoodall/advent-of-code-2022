use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Match {
    their_move: Move,
    our_move: Move,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct MatchWithOutcome {
    their_move: Move,
    outcome: Outcome,
}

fn input_generator_part_1(input: &str) -> Vec<Match> {
    input
        .lines()
        .map(|m| {
            let line: Vec<Move> = m
                .split_whitespace()
                .map(|mov| {
                    if mov == "A" || mov == "X" {
                        Move::Rock
                    } else if mov == "B" || mov == "Y" {
                        Move::Paper
                    } else if mov == "C" || mov == "Z" {
                        Move::Scissors
                    } else {
                        unreachable!()
                    }
                })
                .collect();
            Match {
                their_move: line[0],
                our_move: line[1],
            }
        })
        .collect()
}

fn input_generator_part_2(input: &str) -> Vec<MatchWithOutcome> {
    input
        .lines()
        .map(|m| {
            let line: Vec<&str> = m.split_whitespace().take(2).collect();

            MatchWithOutcome {
                their_move: {
                    let mov = line[0];
                    if mov == "A" {
                        Move::Rock
                    } else if mov == "B" {
                        Move::Paper
                    } else if mov == "C" {
                        Move::Scissors
                    } else {
                        panic!("Invalid Char")
                    }
                },
                outcome: {
                    let mov = line[1];
                    if mov == "X" {
                        Outcome::Lose
                    } else if mov == "Y" {
                        Outcome::Draw
                    } else if mov == "Z" {
                        Outcome::Win
                    } else {
                        panic!("Invalid Char")
                    }
                },
            }
        })
        .collect()
}

fn solve_part1(input: &[Match]) -> u32 {
    let mut score = 0;
    for m in input {
        score += match m.our_move {
            Move::Rock => {
                1 + match m.their_move {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            }
            Move::Paper => {
                2 + match m.their_move {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            }
            Move::Scissors => {
                3 + match m.their_move {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
    }
    score
}

fn solve_part2(input: &[MatchWithOutcome]) -> u32 {
    let mut score = 0;
    for m in input {
        let our_move = match m.their_move {
            Move::Rock => match m.outcome {
                Outcome::Win => Move::Paper,
                Outcome::Draw => Move::Rock,
                Outcome::Lose => Move::Scissors,
            },
            Move::Paper => match m.outcome {
                Outcome::Win => Move::Scissors,
                Outcome::Draw => Move::Paper,
                Outcome::Lose => Move::Rock,
            },
            Move::Scissors => match m.outcome {
                Outcome::Win => Move::Rock,
                Outcome::Draw => Move::Scissors,
                Outcome::Lose => Move::Paper,
            },
        };
        score += match our_move {
            Move::Rock => {
                1 + match m.their_move {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            }
            Move::Paper => {
                2 + match m.their_move {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            }
            Move::Scissors => {
                3 + match m.their_move {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
    }
    score
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day2.txt").expect("Should have been able to read the file");
    let input_part_1 = input_generator_part_1(contents.as_str());
    let input_part_2 = input_generator_part_2(contents.as_str());
    let part1_answer = solve_part1(&input_part_1);
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    let part2_answer = solve_part2(&input_part_2);
    println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = input_generator_part_1("A Y\nB X\nC Z");
        assert_eq!(
            input,
            [
                Match {
                    their_move: Move::Rock,
                    our_move: Move::Paper
                },
                Match {
                    their_move: Move::Paper,
                    our_move: Move::Rock
                },
                Match {
                    their_move: Move::Scissors,
                    our_move: Move::Scissors
                }
            ]
        );
    }
    #[test]
    fn example2() {
        let input = input_generator_part_1("A Y\nB X\nC Z");
        let output = solve_part1(&input);
        assert_eq!(output, 15);
    }
    #[test]
    fn example3() {
        let input = input_generator_part_2("A Y\nB X\nC Z");
        assert_eq!(
            input,
            [
                MatchWithOutcome {
                    their_move: Move::Rock,
                    outcome: Outcome::Draw
                },
                MatchWithOutcome {
                    their_move: Move::Paper,
                    outcome: Outcome::Lose
                },
                MatchWithOutcome {
                    their_move: Move::Scissors,
                    outcome: Outcome::Win
                }
            ]
        );
    }
    #[test]
    fn example4() {
        let input = input_generator_part_2("A Y\nB X\nC Z");
        let output = solve_part2(&input);
        assert_eq!(output, 12);
    }
}
