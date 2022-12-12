use file_system::file_system_node::FileSystemNode;
use terminal_output::line::Line;

pub mod terminal_output {
    pub mod path;
    pub mod cd;
    pub mod command;
    pub mod entry;
    pub mod line;
    pub mod ls;
}

pub mod file_system {
    pub mod file_system_node;
}

pub fn calculate_total_size_of_dirs_smaller_than(input: &str, max_size: usize) -> usize {
    let lines = Line::parse_lines(input);
    let file_tree = FileSystemNode::build_from_lines(lines);
    FileSystemNode::into_iter(file_tree)
            .filter(|f| f.borrow().is_dir())
            .map(|f| FileSystemNode::into_iter(f).map(|f| f.borrow().get_size()).sum::<usize>())
            .filter(|&dir_size| dir_size <= max_size)
            .sum::<usize>()
}