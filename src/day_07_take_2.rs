pub mod day_07_take_2 {
    use camino::Utf8PathBuf;
    use nom::{IResult, combinator::map, bytes::complete::{take_while1, tag}, sequence::{preceded, separated_pair}, branch::alt};

    fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
        map(
            take_while1(|c| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
            Into::into,
        )(i)
    }

    #[derive(Debug)]
    struct Ls;

    fn parse_ls(i: &str) -> IResult<&str, Ls> {
        map(tag("ls"), |_| Ls)(i)
    }

    #[derive(Debug)]
    struct Cd(Utf8PathBuf);

    fn parse_cd(i: &str) -> IResult<&str, Cd> {
        map(preceded(tag("cd "), parse_path), Cd)(i)
    }

    #[derive(Debug)]
    enum Command {
        Ls(Ls),
        Cd(Cd),
    }

    fn parse_command(i: &str) -> IResult<&str, Command> {
        let (i, _) = tag("$ ")(i)?;
        alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(i)
    }

    #[derive(Debug)]
    enum Entry {
        Dir(Utf8PathBuf),
        File(u64, Utf8PathBuf)
    }

    fn parse_entry(i: &str) -> IResult<&str, Entry> {
        let parse_file = map(
            separated_pair(nom::character::complete::u64, tag(" "), parse_path),
            |(size, path)| Entry::File(size, path),
        );
        let parse_dir = map(preceded(tag("dir"), parse_path), Entry::Dir);

        alt((parse_file, parse_dir))(i)
    }

    #[derive(Debug)]
    enum Line {
        Command(Command),
        Entry(Entry),
    }

    fn parse_line(i: &str) -> IResult<&str, Line> {
        alt((
            map(parse_command, Line::Command),
            map(parse_entry, Line::Entry)
        ))(i)
    }
}
