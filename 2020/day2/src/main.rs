use anyhow::Result;
use peg::*;
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    char: u8,
    range: RangeInclusive<usize>,
    positions: (usize, usize),
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .copied()
                .filter(|&letter| letter == self.char)
                .count(),
        )
    }

    fn is_valid_in_position(&self, password: &str) -> bool {
        let is_at_first_positions = password
            .as_bytes()
            .get(self.positions.0 - 1)
            .copied()
            .unwrap()
            == self.char;
        let is_at_second_positions = password
            .as_bytes()
            .get(self.positions.1 - 1)
            .copied()
            .unwrap()
            == self.char;
        is_at_first_positions ^ is_at_second_positions
    }
}

fn parse_line(line: &str, with_position: bool) -> Result<(PasswordPolicy, &str)> {
    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule range() -> RangeInclusive<usize>
            = min:number() "-" max:number() { min..=max }

            rule positions() -> (usize, usize)
            = min:number() "-" max:number() { (min, max) }

            rule byte() -> u8
            = letter:$(['a'..='z']) { letter.as_bytes()[0] }

            rule password() -> &'input str
            = letters:$([_]*) { letters }

            pub(crate) rule line() -> (PasswordPolicy, &'input str)
            = range:range() " " byte:byte() ": " password:password() {
                (PasswordPolicy {range, char: byte, positions: (0,0)}, password)
            }

            pub(crate) rule line_with_positions() -> (PasswordPolicy, &'input str)
            = positions:positions() " " byte:byte() ": " password:password() {
                (PasswordPolicy {range: 0..=1, char: byte, positions}, password)
            }
        }
    }
    if !with_position {
        Ok(parser::line(line)?)
    } else {
        Ok(parser::line_with_positions(line)?)
    }
}

fn main() {
    // Part 1
    let count = include_str!("input.txt")
        .lines()
        .map(|l| parse_line(l, false))
        .map(std::result::Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!("{} passwords are valid", count);

    // Part 2
    let count = include_str!("input.txt")
    .lines()
    .map(|l| parse_line(l, true))
    .map(std::result::Result::unwrap)
    .filter(|(policy,password)| policy.is_valid_in_position(password))
    .count();
    println!("{} passwords are valid", count);
}

#[cfg(test)]
mod test;
