use std::fs;

type Elf = Vec<u32>;

pub fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|one_elf| {
            one_elf
                .lines()
                .map(|foodstuff| foodstuff.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve_part1(input: &Vec<Elf>) -> u32 {
    input
        .into_iter()
        .map(|one_elf| one_elf.into_iter().sum())
        .max()
        .unwrap()
}

pub fn solve_part2(input: &Vec<Elf>) -> u32 {
    input
        .into_iter()
        .map(|one_elf| one_elf.into_iter().sum())
        .fold([0, 0, 0], |mut acc, t| {
            let current_min: u32 = acc.into_iter().min().unwrap();
            if t > current_min {
                acc[acc.into_iter().position(|x| x == current_min).unwrap()] = t;
                acc
            } else {
                acc
            }
        })
        .into_iter()
        .sum()
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day1.txt").expect("Should have been able to read the file");
    let input = input_generator(contents.as_str());
    let part1_answer = solve_part1(&input);
    let part2_answer = solve_part2(&input);
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input =
            input_generator("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(solve_part1(&input), 24000);
    }
    #[test]
    fn part2_example1() {
        let input =
            input_generator("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(solve_part2(&input), 45000);
    }
}
