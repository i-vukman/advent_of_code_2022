use camino::Utf8PathBuf;
use id_tree::{Tree, Node, InsertBehavior};

use crate::{terminal_output::{command::Command, entry::Entry, line::Line}};

#[derive(Debug)]
pub struct FileSystemNode {
    path: Utf8PathBuf,
    size: u64,
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

#[derive(Debug)]
pub struct FileSystemTree {
    root: Tree<FileSystemNode>,
}

impl FileSystemTree {
    pub fn build_from_lines(lines: Vec<Line>) -> color_eyre::Result<FileSystemTree> {
        let mut tree = Tree::<FileSystemNode>::new();
        let root_node = tree.insert(Node::new(FileSystemNode {
            path: "/".into(),
            size: 0,
        }), 
        InsertBehavior::AsRoot)?;

        let mut current_node = root_node.clone();

        for line in lines {
            match line {
                Line::Command(cmd) => match cmd {
                    Command::Ls => {
                        //Ignored on purpose
                    }
                    Command::Cd(path) => match path.as_str() {
                        "/" => {
                            current_node = root_node.clone();
                        }
                        ".." => {
                            current_node = tree.get(&current_node)?.parent().unwrap().clone();
                        }
                        _ => {
                            let node = Node::new(FileSystemNode { 
                                path: path, 
                                size: 0, 
                            });
                            current_node = tree.insert(node, InsertBehavior::UnderNode(&current_node))?;
                        }
                    },
                },
                Line::Entry(entry) => match entry {
                    Entry::Dir(_dir) => {
                        // Ignored on purpose
                    }
                    Entry::File(size, path) => {
                        let node = Node::new(FileSystemNode { size, path });
                        tree.insert(node, InsertBehavior::UnderNode(&current_node))?;
                    }
                },
            }
        }
        Ok(FileSystemTree { root: tree })
    }

    pub fn total_size(&self, node: &Node<FileSystemNode>) -> color_eyre::Result<u64> {
        let mut total = node.data().size;
        for child in node.children() {
            total += self.total_size(self.root.get(child)?)?
        }
        Ok(total)
    }

    pub fn traverse(&self) -> color_eyre::Result<impl Iterator<Item = &Node<FileSystemNode>>> {
        Ok(self.root.traverse_pre_order(self.root.root_node_id().unwrap())?)
    }
}
