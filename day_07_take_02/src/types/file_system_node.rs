use std::collections::HashMap;
use camino::Utf8PathBuf;

#[derive(Debug, Default)]
pub struct FileSystemNode {
    pub(crate) size: usize,
    //TODO: change this to be private and move parsing from lines in impl block here
    pub(crate) children: HashMap<Utf8PathBuf, FileSystemNode>,
}
