use camino::Utf8PathBuf;

#[derive(Debug)]
pub struct Ls;

#[derive(Debug)]
pub struct Cd(pub Utf8PathBuf);

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
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
