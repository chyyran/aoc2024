use aoc_runner_derive::aoc;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map,
    multi::many_till, IResult, Parser,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Instr {
    Enable,
    Disable,
    Mul(i32, i32),
}

impl Instr {
    pub fn product(&self) -> i32 {
        match self {
            Instr::Mul(a, b) => a * b,
            _ => 0,
        }
    }
}

fn parse_mul(input: &str) -> IResult<&str, Instr> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = nom::character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = nom::character::complete::i32(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Instr::Mul(a, b)))
}

fn parse_dont(input: &str) -> IResult<&str, Instr> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instr::Disable))
}

fn parse_do(input: &str) -> IResult<&str, Instr> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instr::Enable))
}

pub fn parse_instr(input: &str) -> IResult<&str, Instr> {
    let (input, instr) = alt((parse_mul, parse_do, parse_dont)).parse(input)?;

    Ok((input, instr))
}

pub fn parse_until_next_instr(input: &str) -> IResult<&str, Instr> {
    let (input, (_, mul)) = many_till(map(anychar, drop), parse_instr).parse(input)?;
    Ok((input, mul))
}

#[aoc(day3, part1)]
pub fn part1(mut input: &str) -> i32 {
    let mut totals = 0;
    while input.len() != 0 {
        let Ok((new_input, mul)) = parse_until_next_instr(input) else {
            break;
        };

        totals += mul.product();
        input = new_input;
    }
    totals
}

#[aoc(day3, part2)]
pub fn part2(mut input: &str) -> i32 {
    let mut totals = 0;
    let mut enabled_flag = true;
    while input.len() != 0 {
        let Ok((new_input, instr)) = parse_until_next_instr(input) else {
            break;
        };

        match instr {
            Instr::Enable => enabled_flag = true,
            Instr::Disable => enabled_flag = false,
            Instr::Mul(a, b) => {
                if enabled_flag {
                    totals += a * b;
                }
            }
        }

        input = new_input;
    }

    totals
}

#[cfg(test)]
mod test {
    use crate::day3::{parse_mul, parse_until_next_instr, part1, part2, Instr};

    #[test]
    pub fn parses_mul() {
        let (_, value) = parse_mul("mul(2,4)").unwrap();
        assert_eq!(value, Instr::Mul(2, 4));
    }

    #[test]
    pub fn parses_until_mul() {
        let (_, value) = parse_until_next_instr("xmul(2,4)").unwrap();
        assert_eq!(value, Instr::Mul(2, 4));
    }

    #[test]
    pub fn part1_test() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    pub fn part2_test() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(INPUT), 48);
    }
}
