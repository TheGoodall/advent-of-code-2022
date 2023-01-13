#[derive(Clone, Debug, PartialEq)]
struct Placeholder {}

fn input_generator_part_1(input: &str) -> Vec<Placeholder> {
    todo!()
}

fn input_generator_part_2(input: &str) -> Vec<Placeholder> {}

fn solve_part1(input: &[Placeholder]) -> u32 {
    todo!()
}
fn solve_part2(input: &[Placeholder]) -> u32 {
    todo!()
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/dayn.txt").expect("Should have been able to read the file");
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
        let input = input_generator_part_1("DATA");
        assert_eq!(input, []);
    }
    #[test]
    fn test_part_1_solver() {
        let input = input_generator_part_1("DATA");
        let output = solve_part1(&input);
        assert_eq!(output, "ANSWER");
    }
    #[test]
    fn test_input_generator_part_2() {
        let input = input_generator_part_2("DATA");
        assert_eq!(input, []);
    }
    #[test]
    fn test_part_2_solver() {
        let input = input_generator_part_2("DATA");
        let output = solve_part2(&input);
        assert_eq!(output, "ANSWER");
    }
}
