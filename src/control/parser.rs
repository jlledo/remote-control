use std::str::FromStr;

use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::*;
use nom::IResult;

use crate::control::models::Command;
use crate::control::models::Vector2;

pub fn parse_command(input: &str) -> anyhow::Result<Command> {
    match alt((move_mouse, left_click, right_click, scroll))(input) {
        Ok((_, command)) => Ok(command),
        Err(_) => Err(anyhow!("Unable to parse command")),
    }
}

type Result<'a, T> = IResult<&'a str, T>;

fn move_mouse(input: &str) -> Result<Command> {
    let (rest, (x, y)) = preceded(
        pair(tag("move"), space1),
        pair(i32, preceded(char(','), i32)),
    )(input)?;
    Ok((rest, Command::Move(Vector2 { x, y })))
}

fn scroll(input: &str) -> Result<Command> {
    let (rest, (x, y)) = preceded(
        pair(tag("scroll"), space1),
        pair(i32, preceded(char(','), i32)),
    )(input)?;
    Ok((rest, Command::Scroll(Vector2 { x, y })))
}

fn left_click(input: &str) -> Result<Command> {
    let (rest, _) = tag("lclick")(input)?;
    Ok((rest, Command::LeftClick))
}

fn right_click(input: &str) -> Result<Command> {
    let (rest, _) = tag("rclick")(input)?;
    Ok((rest, Command::RightClick))
}

fn i32(input: &str) -> Result<i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), FromStr::from_str)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_positive() {
        assert_eq!(
            move_mouse("move 2,2"),
            Ok(("", Command::Move(Vector2 { x: 2, y: 2 })))
        );
    }

    #[test]
    fn test_move_negative() {
        assert_eq!(
            move_mouse("move -2,-2"),
            Ok(("", Command::Move(Vector2 { x: -2, y: -2 })))
        );
    }
}
