use nom::{IResult, branch::alt, combinator::{map, all_consuming}, bytes::complete::tag, Finish, sequence::{preceded}};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_add))(i)
}

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(i)
}

fn parse_add(i: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), nom::character::complete::i64), Instruction::AddX)(i)
}

impl Instruction {
    pub fn parse_instructions(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|l| all_consuming(parse_instruction)(l).finish().unwrap().1)
            .collect()
    }
}
