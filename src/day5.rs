use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Ship {
    stacks: HashMap<usize, Vec<char>>,
}
impl Ship {
    fn execute_instruction_set(&mut self, inset: InstructionSet) {
        for ins in inset.instructions {
            self.execute_ins(ins)
        }
    }
    fn execute_instruction_set_on_9001(&mut self, inset: InstructionSet) {
        for ins in inset.instructions {
            self.execute_ins_on_9001(ins)
        }
    }
    fn execute_ins(&mut self, ins: Instruction) {
        let from = self.stacks.get_mut(&ins.from).expect("Should exist");
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..ins.number {
            tmp.push_back(from.pop().expect("Should not remove from an empty stack"))
        }
        let to = self.stacks.get_mut(&ins.to).expect("Should exist");
        for _ in 0..ins.number {
            to.push(
                tmp.pop_front()
                    .expect("Should not remove from an empty stack"),
            )
        }
    }
    fn execute_ins_on_9001(&mut self, ins: Instruction) {
        let from = self.stacks.get_mut(&ins.from).expect("Should exist");
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..ins.number {
            tmp.push(from.pop().expect("Should not remove from an empty stack"))
        }
        let to = self.stacks.get_mut(&ins.to).expect("Should exist");
        for _ in 0..ins.number {
            to.push(tmp.pop().expect("Should not remove from an empty stack"))
        }
    }
    fn get_tops(&self) -> String {
        self.stacks
            .keys()
            .sorted()
            .filter_map(|id| self.stacks[id].last())
            .collect()
    }

    fn new(input: &str) -> Ship {
        let mut ship = Ship {
            stacks: HashMap::new(),
        };

        for line in input.rsplit('\n').skip(1) {
            for container in line.chars().chunks(4).into_iter().enumerate() {
                let c = container.1.collect_vec()[1];
                if c != ' ' {
                    ship.stacks.entry(container.0 + 1).or_default().push(c)
                }
            }
        }

        ship
    }
}

#[derive(Clone, Debug, PartialEq)]
struct InstructionSet {
    instructions: Vec<Instruction>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    number: u32,
    from: usize,
    to: usize,
}

fn input_generator_part_1(input: &str) -> (Ship, InstructionSet) {
    let mut input = input.split("\n\n");
    let ship_layout = input.next().unwrap();

    let instructions = input.next().unwrap().split('\n');
    let instruction_set = InstructionSet {
        instructions: instructions
            .filter(|x| !x.is_empty())
            .map(|line| {
                let mut parts = line.split(' ');
                let number = parts
                    .nth(1)
                    .expect("There should be enough items in the line")
                    .parse()
                    .expect("Should be parsable");
                let from = parts
                    .nth(1)
                    .expect("There should be enough items in the line")
                    .parse()
                    .expect("Should be parsable");
                let to = parts
                    .nth(1)
                    .expect("There should be enough items in the line")
                    .parse()
                    .expect("Should be parsable");
                Instruction { number, from, to }
            })
            .collect(),
    };
    (Ship::new(ship_layout), instruction_set)
}

fn input_generator_part_2(input: &str) -> (Ship, InstructionSet) {
    input_generator_part_1(input)
}

fn solve_part1(input: (Ship, InstructionSet)) -> String {
    let mut ship = input.0;
    let instructions = input.1;
    ship.execute_instruction_set(instructions);
    ship.get_tops()
}

fn solve_part2(input: (Ship, InstructionSet)) -> String {
    let mut ship = input.0;
    let instructions = input.1;
    ship.execute_instruction_set_on_9001(instructions);
    ship.get_tops()
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day5.txt").expect("Should have been able to read the file");
    let input_part_1 = input_generator_part_1(contents.as_str());
    let input_part_2 = input_generator_part_2(contents.as_str());
    let part1_answer = solve_part1(input_part_1);
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    let part2_answer = solve_part2(input_part_2);
    println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = input_generator_part_1("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");
        assert_eq!(
            input,
            (
                Ship {
                    stacks: HashMap::from([
                        (1, vec!['Z', 'N']),
                        (2, vec!['M', 'C', 'D']),
                        (3, vec!['P'])
                    ])
                },
                InstructionSet {
                    instructions: vec![
                        Instruction {
                            number: 1,
                            from: 2,
                            to: 1
                        },
                        Instruction {
                            number: 3,
                            from: 1,
                            to: 3
                        },
                        Instruction {
                            number: 2,
                            from: 2,
                            to: 1
                        },
                        Instruction {
                            number: 1,
                            from: 1,
                            to: 2
                        }
                    ]
                }
            )
        );
    }
    #[test]
    fn test_part_1_solver() {
        let input = input_generator_part_1("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");
        let output = solve_part1(input);
        assert_eq!(output, "CMZ");
    }
    //#[test]
    //fn test_part_2_generator() {
    //let input = input_generator_part_2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
    //assert_eq!(input, []);
    //}

    #[test]
    fn test_part_2_solver() {
        let input = input_generator_part_2("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");
        let output = solve_part2(input);
        assert_eq!(output, "MCD");
    }
}
