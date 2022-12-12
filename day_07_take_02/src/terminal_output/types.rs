use camino::Utf8PathBuf;

#[derive(Debug)]
pub struct Ls;

#[derive(Debug)]
pub struct Cd(pub Utf8PathBuf);

#[derive(Debug)]
pub enum Command {
    Ls(Ls),
    Cd(Cd),
}

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}
