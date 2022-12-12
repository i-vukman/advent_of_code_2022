use camino::Utf8PathBuf;
use indexmap::IndexMap;
use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{terminal_output::{command::Command, entry::Entry, line::Line}};

type FileSystemNodeHandle = Rc<RefCell<FileSystemNode>>;

#[derive(Default)]
pub struct FileSystemNode {
    size: usize,
    children: IndexMap<Utf8PathBuf, FileSystemNodeHandle>,
    parent: Option<FileSystemNodeHandle>,
}

impl fmt::Debug for FileSystemNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileSystemNode")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

pub struct PrettyNode<'a>(pub &'a FileSystemNodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }

        Ok(())
    }
}

impl FileSystemNode {
    pub fn build_from_lines(lines: Vec<Line>) -> FileSystemNodeHandle {
        let root_handle = Rc::new(RefCell::new(FileSystemNode::default()));
        let mut node = root_handle.clone();

        for line in lines {
            match line {
                Line::Command(cmd) => match cmd {
                    Command::Ls => {
                        //Ignored on purpose
                    }
                    Command::Cd(path) => match path.as_str() {
                        "/" => {
                            node = root_handle.clone();
                        }
                        ".." => {
                            let parent = node.borrow_mut().parent.clone().unwrap();
                            node = parent;
                        }
                        _ => {
                            let child = node.borrow_mut().children.entry(path).or_default().clone();
                            node = child;
                        }
                    },
                },
                Line::Entry(entry) => match entry {
                    Entry::Dir(dir) => {
                        let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                        entry.borrow_mut().parent = Some(node.clone());
                    }
                    Entry::File(size, file) => {
                        let entry = node.borrow_mut().children.entry(file).or_default().clone();
                        entry.borrow_mut().size = size as usize;
                        entry.borrow_mut().parent = Some(node.clone());
                    }
                },
            }
        }
        root_handle
    }

    pub fn is_dir(&self) -> bool {
        self.size == 0
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}
