use nom::{IResult, combinator::{map, all_consuming}, sequence::{preceded}, bytes::complete::tag, branch::alt, Finish};

#[derive(Debug)]
pub enum HeadMove {
    Up(i64),
    Right(i64),
    Down(i64),
    Left(i64),
}

impl HeadMove {
    pub fn parse_moves(input: &str) -> Vec<HeadMove> {
        input
            .lines()
            .map(|l| all_consuming(parse_move)(l).finish().unwrap().1)
            .collect()
    }
}

impl From<Up> for HeadMove {
    fn from(up: Up) -> Self {
        HeadMove::Up(up.0)
    }
}

impl From<Right> for HeadMove {
    fn from(right: Right) -> Self {
        HeadMove::Right(right.0)
    }
}

impl From<Down> for HeadMove {
    fn from(down: Down) -> Self {
        HeadMove::Down(down.0)
    }
}

impl From<Left> for HeadMove {
    fn from(left: Left) -> Self {
        HeadMove::Left(left.0)
    }
}

pub fn parse_move(i: &str) -> IResult<&str, HeadMove> {
    alt((map(parse_up, Into::into), 
         map(parse_right, Into::into),
         map(parse_down, Into::into),
         map(parse_left, Into::into)))(i)
}

struct Up(i64);

fn parse_up(i: &str) -> IResult<&str, Up> {
    map(preceded(tag("U "), nom::character::complete::i64), Up)(i)
}

struct Right(i64);

fn parse_right(i: &str) -> IResult<&str, Right> {
    map(preceded(tag("R "), nom::character::complete::i64), Right)(i)
}

struct Down(i64);

fn parse_down(i: &str) -> IResult<&str, Down> {
    map(preceded(tag("D "), nom::character::complete::i64), Down)(i)
}

struct Left(i64);

fn parse_left(i: &str) -> IResult<&str, Left> {
    map(preceded(tag("L "), nom::character::complete::i64), Left)(i)
}
