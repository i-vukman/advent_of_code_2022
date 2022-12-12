use nom::{combinator::all_consuming, Finish};
use terminal_output::{parsers::parse_line, types::{Line, Command, Entry}};
use types::file_system_node::FileSystemNode;

pub mod terminal_output {
    pub mod parsers;
    pub mod types;
}

pub mod types {
    pub mod file_system_node;
}

//TODO: move this to Line associated function
pub fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
        .collect::<Vec<_>>()
}

//TODO: move this to node impl block
pub fn build_file_tree(lines: Vec<Line>) -> FileSystemNode {
    let mut node = FileSystemNode::default();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    //Ignored on purpose
                },
                Command::Cd(_) => todo!(),
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    node.children.entry(dir).or_default();
                },
                Entry::File(size, file) => {
                    node.children.entry(file).or_default().size = size as usize;
                }
            }
        }
    }

    node
}

//TODO: create new functions to calculate what is needed for task