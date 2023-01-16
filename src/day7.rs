use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use std::fs;
struct State<'a>(Vec<&'a Directory>);

#[derive(Clone, Debug, PartialEq)]
enum DirectoryItem {
    File(File),
    Directory(Directory),
}

impl DirectoryItem {
    fn get_size(&self) -> u32 {
        match self {
            DirectoryItem::File(file) => file.size,
            DirectoryItem::Directory(directory) => directory.get_size(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Directory(Vec<DirectoryItem>);

impl Directory {
    fn get_size(&self) -> u32 {
        (&self).0.iter().map(|item| item.get_size()).sum()
    }

    fn push(&mut self, item: DirectoryItem) {
        self.0.push(item)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct File {
    size: u32,
}

enum Line<'a> {
    Command(Command<'a>),
    DirectoryItem(DirectoryItem),
}

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

fn command(input: &str) -> IResult<&str, Command> {
    todo!()
}

fn directory(input: &str) -> IResult<&str, Directory> {
    todo!()
}

fn file(input: &str) -> IResult<&str, File> {
    todo!()
}
fn parseLine(input: &str) -> IResult<&str, Line> {
    todo!()
}

fn input_generator_part_1(input: &str) -> Directory {
    let root = Directory(vec![]);
    input
        .lines()
        .skip(1)
        .fold(State(vec![&root]), |mut state, line| {
            dbg!(line);
            let pwd = state.0.last_mut();

            state
        })
        .0
        .first()
        .expect("Will always include at least the root directory")
        .to_owned()
        .to_owned()
}

//fn input_generator_part_2(input: &str) -> Vec<Placeholder> {}

fn solve_part1(input: &Directory) -> u32 {
    todo!()
}
//fn solve_part2(input: &[Placeholder]) -> u32 {
//todo!()
//}

pub fn main() {
    let contents =
        fs::read_to_string("input/2022/dayn.txt").expect("Should have been able to read the file");
    let input_part_1 = input_generator_part_1(contents.as_str());
    //let input_part_2 = input_generator_part_2(contents.as_str());
    let part1_answer = solve_part1(&input_part_1);
    println!("Part 1:\n\n{}\n\n\n", part1_answer);
    //let part2_answer = solve_part2(input_part_2);
    //println!("Part 2:\n\n{}\n\n\n", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = input_generator_part_1("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k");
        assert_eq!(
            input,
            Directory(vec![
                DirectoryItem::Directory(Directory(vec![
                    DirectoryItem::Directory(Directory(vec![DirectoryItem::File(File {
                        size: 584
                    })])),
                    DirectoryItem::File(File { size: 29116 }),
                    DirectoryItem::File(File { size: 2557 }),
                    DirectoryItem::File(File { size: 62596 })
                ])),
                DirectoryItem::File(File { size: 14848514 }),
                DirectoryItem::File(File { size: 8504156 }),
                DirectoryItem::Directory(Directory(vec![DirectoryItem::File(File {
                    size: 4060174
                })]))
            ])
        );
    }
    //#[test]
    //fn test_part_1_solver() {
    //let input = input_generator_part_1("DATA");
    //let output = solve_part1(&input);
    //assert_eq!(output, "ANSWER");
    //}
    //#[test]
    //fn test_input_generator_part_2() {
    //let input = input_generator_part_2("DATA");
    //assert_eq!(input, []);
    //}
    //#[test]
    //fn test_part_2_solver() {
    //let input = input_generator_part_2("DATA");
    //let output = solve_part2(&input);
    //assert_eq!(output, "ANSWER");
    //}
}
