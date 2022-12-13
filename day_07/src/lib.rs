use file_system::file_system_node::{FileSystemNode};
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

pub fn calculate_total_size_of_dirs_smaller_than(input: &str, max_size: u64) -> u64 {
    let lines = Line::parse_lines(input);
    let file_tree = FileSystemNode::build_from_lines(lines);

    file_tree.traverse()
            .filter(|f| f.is_dir())
            .map(|f| f.traverse().map(|f| f.get_size()).sum::<u64>())
            .filter(|&sum| sum <= max_size)
            .sum()
}

pub fn get_min_file_size_to_free_up_storage(input: &str, storage: u64, required_free_storage: u64) -> Option<u64> {
    let lines = Line::parse_lines(input);
    let file_tree = FileSystemNode::build_from_lines(lines);
    let total_size_taken = file_tree.traverse()
            .map(|f| f.get_size())
            .sum::<u64>();
    
    let min_dir_size = required_free_storage - (storage - total_size_taken);

    file_tree.traverse()
            .filter(|f| f.is_dir())
            .map(|f| f.traverse().map(|f| f.get_size()).sum())
            .filter(|&sum| sum > min_dir_size)
            .min()
}
