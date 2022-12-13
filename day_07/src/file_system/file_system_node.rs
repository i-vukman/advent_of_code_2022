use std::collections::{HashSet};
use std::hash::Hash;

use camino::Utf8PathBuf;

use crate::{terminal_output::{command::Command, entry::Entry, line::Line}};

#[derive(Debug, Eq)]
pub struct FileSystemNode {
    path: Utf8PathBuf,
    size: u64,
    children: HashSet<FileSystemNode>,
}

impl PartialEq for FileSystemNode {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.size == other.size
    }
}


impl Hash for FileSystemNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.size.hash(state);
    }
}

impl FileSystemNode {
    pub fn get_path(&self) -> String {
        self.path.to_string()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn is_dir(&self) -> bool {
        self.size == 0
    }
}

impl FileSystemNode {
    pub fn build_from_lines(lines: Vec<Line>) -> FileSystemNode {
        let mut dir_navigation_stack = vec![FileSystemNode {
            path: "/".into(),
            size: 0,
            children: HashSet::new(),
        }];

        for line in lines {
            match line {
                Line::Command(cmd) => match cmd {
                    Command::Ls => {
                        //Ignored on purpose
                    }
                    Command::Cd(path) => match path.as_str() {
                        "/" => {
                            dir_navigation_stack = Self::fold_stack(dir_navigation_stack);
                        }
                        ".." => {
                            let child = dir_navigation_stack.pop().unwrap();
                            dir_navigation_stack.last_mut().unwrap().children.insert(child);
                        }
                        _ => {
                            let node = FileSystemNode { 
                                path: path, 
                                size: 0, 
                                children: HashSet::new(),
                            };
                            dir_navigation_stack.push(node);
                        }
                    },
                },
                Line::Entry(entry) => match entry {
                    Entry::Dir(_dir) => {
                        // Ignored on purpose
                    }
                    Entry::File(size, path) => {
                        let node = FileSystemNode { size, path, children: HashSet::new() };
                        dir_navigation_stack.last_mut().unwrap().children.insert(node);
                    }
                },
            }
        }
        Self::fold_stack(dir_navigation_stack).pop().unwrap()
    }

    fn fold_stack(mut stack: Vec<FileSystemNode>) -> Vec<FileSystemNode> {
        let mut root = stack.pop().unwrap();
        while let Some(mut next) = stack.pop() {
            next.children.insert(root);
            root = next;
        }
        vec![root]
    }

    pub fn traverse(&self) -> Box<dyn Iterator<Item = &FileSystemNode> + '_> {
        Box::new(
            std::iter::once(self)
                .chain(self.children.iter()
                    .map(|c| c.traverse())
                    .flatten()))
    }
}
