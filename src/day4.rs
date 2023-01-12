use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct PLACEMARKER {}

struct Elf {
    start_id: u32,
    end_id: u32,
}

struct ElfPair {
    elf1: Elf,
    elf2: Elf,
}

fn input_generator_part_1(input: &str) -> Vec<ElfPair> {
    input
        .lines()
        .map(|input| {
            let elves: Vec<Elf> = input
                .split(",")
                .map(|input| {
                    let range: Vec<u32> = input.split("-").map(|id| id.parse::<u32>()).collect();
                    Elf {
                        start_id: range[0],
                        end_id: range[1],
                    }
                })
                .collect();
            ElfPair {
                elf1: elves[0],
                elf2: elves[1],
            }
        })
        .collect()
}

//fn input_generator_part_2(input: &str) -> Vec<Group> {
//todo!()
//}

fn solve_part1(input: &[ElfPair]) -> u32 {
    todo!()
}

//fn solve_part2(input: &[Group]) -> u32 {
//todo!()
//}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day4.txt").expect("Should have been able to read the file");
    let input_part_1 = input_generator_part_1(contents.as_str());
    //let input_part_2 = input_generator_part_2(contents.as_str());
    let part1_answer = solve_part1(&input_part_1);
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    //let part2_answer = solve_part2(&input_part_2);
    //println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = input_generator_part_1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        assert_eq!(input, []);
    }
    #[test]
    fn test_part_1_solver() {
        let input = input_generator_part_1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        let output = solve_part1(&input);
        assert_eq!(output, 2);
    }
    //#[test]
    //fn test_part_2_generator() {
    //let input = input_generator_part_2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
    //assert_eq!(input, []);
    //}

    //#[test]
    //fn test_part_2_solver() {
    //let input = input_generator_part_2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
    //let output = solve_part2(&input);
    //assert_eq!(output, 70);
    //}
}
