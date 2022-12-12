use nom::{combinator::all_consuming, Finish};
use terminal_output::{line::{Line, parse_line}, command::Command, entry::Entry};
use types::file_system_node::FileSystemNode;

pub mod terminal_output {
    pub mod path;
    pub mod cd;
    pub mod command;
    pub mod entry;
    pub mod line;
    pub mod ls;
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
    let mut root = FileSystemNode::default();
    let mut node = &mut root;

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    //Ignored on purpose
                },
                Command::Cd(path) => match path.as_str() {
                    "/" => {

                    }
                    ".." => {

                    }
                    _ => {
                        node = node.children.entry(path).or_default();
                    }
                },
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

    root
}

//TODO: create new functions to calculate what is needed for task