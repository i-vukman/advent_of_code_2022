use day_07_take_02::{self, terminal_output::line::Line, file_system::file_system_node::{PrettyNode, FileSystemNode}};

fn main() {
    let input = include_str!("../input/day_07_sample.txt");
    let lines = Line::parse_lines(input);
    let root = FileSystemNode::build_from_lines(lines);

    println!("{:#?}", PrettyNode(&root));
}
