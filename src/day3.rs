use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
type Compartment = Vec<char>;

#[derive(Clone, Debug, PartialEq)]
struct Rucksack {
    compartment1: Compartment,
    compartment2: Compartment,
}

impl Rucksack {
    fn get_overlapping_char(&self) -> char {
        let c1: HashSet<char> = self.compartment1.iter().cloned().collect();
        let c2: HashSet<char> = self.compartment2.iter().cloned().collect();
        c1.intersection(&c2)
            .into_iter()
            .nth(0)
            .expect("Problem definition states that there will always be exactly one.")
            .clone()
    }

    fn new(line: &str) -> Rucksack {
        let l = line.len();
        let compartment_length = l / 2;
        Rucksack {
            compartment1: line[0..compartment_length].chars().collect(),
            compartment2: line[compartment_length..l].chars().collect(),
        }
    }
}

lazy_static! {
    static ref scores: HashMap<char, u32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
}

#[derive(Clone, Debug, PartialEq)]
struct Group {
    elf1: Rucksack,
    elf2: Rucksack,
    elf3: Rucksack,
}

impl Group {
    fn get_common_char(&self) -> char {
        [&self.elf1, &self.elf2, &self.elf3]
            .into_iter()
            .map(|rs| {
                let c1 = &rs.compartment1;
                let c2 = &rs.compartment2;
                let all_chars = c1.iter().chain(c2.iter());
                HashSet::from_iter(all_chars)
            })
            .reduce(|acc: HashSet<&char>, set: HashSet<&char>| {
                acc.intersection(&set).cloned().collect()
            })
            .expect("Empty list")
            .into_iter()
            .nth(0)
            .expect("Problem definition states that there will always be exactly one.")
            .clone()
    }

    fn get_badge_priority(&self) -> u32 {
        let c = self.get_common_char();
        scores[&c]
    }
}

fn input_generator_part_1(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::new).collect()
}

fn input_generator_part_2(input: &str) -> Vec<Group> {
    input_generator_part_1(input)
        .chunks_exact(3)
        .map(|chunk| Group {
            elf1: chunk[0].to_owned(),
            elf2: chunk[1].to_owned(),
            elf3: chunk[2].to_owned(),
        })
        .collect()
}

fn solve_part1(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|rucksack| rucksack.get_overlapping_char())
        .map(|c| scores[&c])
        .sum()
}

fn solve_part2(input: &[Group]) -> u32 {
    input
        .into_iter()
        .map(|x| x.get_badge_priority())
        .sum::<u32>()
}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/day3.txt").expect("Should have been able to read the file");
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
    fn test_input_generator() {
        let input = input_generator_part_1(
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
            );
        assert_eq!(
            input,
            [
                Rucksack {
                    compartment1: "vJrwpWtwJgWr".chars().collect(),
                    compartment2: "hcsFMMfFFhFp".chars().collect()
                },
                Rucksack {
                    compartment1: "jqHRNqRjqzjGDLGL".chars().collect(),
                    compartment2: "rsFMfFZSrLrFZsSL".chars().collect()
                },
                Rucksack {
                    compartment1: "PmmdzqPrV".chars().collect(),
                    compartment2: "vPwwTWBwg".chars().collect()
                },
                Rucksack {
                    compartment1: "wMqvLMZHhHMvwLH".chars().collect(),
                    compartment2: "jbvcjnnSBnvTQFn".chars().collect()
                },
                Rucksack {
                    compartment1: "ttgJtRGJ".chars().collect(),
                    compartment2: "QctTZtZT".chars().collect()
                },
                Rucksack {
                    compartment1: "CrZsJsPPZsGz".chars().collect(),
                    compartment2: "wwsLwLmpwMDw".chars().collect()
                },
            ]
        );
    }
    #[test]
    fn test_part_1_solver() {
        let input = input_generator_part_1("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let output = solve_part1(&input);
        assert_eq!(output, 157);
    }
    #[test]
    fn test_part_2_generator() {
        let input = input_generator_part_2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(
            input,
            [
                Group {
                    elf1: Rucksack {
                        compartment1: "vJrwpWtwJgWr".chars().collect(),
                        compartment2: "hcsFMMfFFhFp".chars().collect()
                    },
                    elf2: Rucksack {
                        compartment1: "jqHRNqRjqzjGDLGL".chars().collect(),
                        compartment2: "rsFMfFZSrLrFZsSL".chars().collect()
                    },
                    elf3: Rucksack {
                        compartment1: "PmmdzqPrV".chars().collect(),
                        compartment2: "vPwwTWBwg".chars().collect()
                    }
                },
                Group {
                    elf1: Rucksack {
                        compartment1: "wMqvLMZHhHMvwLH".chars().collect(),
                        compartment2: "jbvcjnnSBnvTQFn".chars().collect()
                    },
                    elf2: Rucksack {
                        compartment1: "ttgJtRGJ".chars().collect(),
                        compartment2: "QctTZtZT".chars().collect()
                    },
                    elf3: Rucksack {
                        compartment1: "CrZsJsPPZsGz".chars().collect(),
                        compartment2: "wwsLwLmpwMDw".chars().collect()
                    }
                }
            ]
        );
    }

    #[test]
    fn test_part_2_group_get_common_char() {
        let g = Group {
            elf1: Rucksack {
                compartment1: "vJrwpWtwJgWr".chars().collect(),
                compartment2: "hcsFMMfFFhFp".chars().collect(),
            },
            elf2: Rucksack {
                compartment1: "jqHRNqRjqzjGDLGL".chars().collect(),
                compartment2: "rsFMfFZSrLrFZsSL".chars().collect(),
            },
            elf3: Rucksack {
                compartment1: "PmmdzqPrV".chars().collect(),
                compartment2: "vPwwTWBwg".chars().collect(),
            },
        };
        assert_eq!(g.get_common_char(), 'r')
    }

    #[test]
    fn test_part_2_group_get_badge_priority_a() {
        let g = Group {
            elf1: Rucksack {
                compartment1: "vJrwpWtwJgWr".chars().collect(),
                compartment2: "hcsFMMfFFhFp".chars().collect(),
            },
            elf2: Rucksack {
                compartment1: "jqHRNqRjqzjGDLGL".chars().collect(),
                compartment2: "rsFMfFZSrLrFZsSL".chars().collect(),
            },
            elf3: Rucksack {
                compartment1: "PmmdzqPrV".chars().collect(),
                compartment2: "vPwwTWBwg".chars().collect(),
            },
        };
        assert_eq!(g.get_badge_priority(), 18)
    }

    #[test]
    fn test_part_2_group_get_badge_priority_b() {
        let g = Group {
            elf1: Rucksack {
                compartment1: "wMqvLMZHhHMvwLH".chars().collect(),
                compartment2: "jbvcjnnSBnvTQFn".chars().collect(),
            },
            elf2: Rucksack {
                compartment1: "ttgJtRGJ".chars().collect(),
                compartment2: "QctTZtZT".chars().collect(),
            },
            elf3: Rucksack {
                compartment1: "CrZsJsPPZsGz".chars().collect(),
                compartment2: "wwsLwLmpwMDw".chars().collect(),
            },
        };
        assert_eq!(g.get_badge_priority(), 52)
    }

    #[test]
    fn test_part_2_solver() {
        let input = input_generator_part_2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        let output = solve_part2(&input);
        assert_eq!(output, 70);
    }
}
