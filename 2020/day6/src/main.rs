use im::HashSet;
use std::fmt;

pub struct Answers(HashSet<u8>);

impl fmt::Debug for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &answer in &self.0 {
            write!(f, "{}", answer as char)?;
        }
        Ok(())
    }
}

fn main() {
    // Part 1
    let answer: usize = include_str!("input.txt")
        .split("\n\n")
        .map(|group| {
            HashSet::<u8>::unions(
                group
                    .lines()
                    .map(|line| line.as_bytes().iter().copied().collect()),
            )
            .len()
        })
        .sum();

    println!(
        "Questions to which atleast one person per group answered yes: {}",
        answer
    );

    // Part 2
    let answer: usize = include_str!("input.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.as_bytes().iter().copied().collect::<HashSet<u8>>())
                .reduce(|acc, x| acc.intersection(x))
                .unwrap_or_default()
                .len()
        })
        .sum();

    println!(
        "Questions to which everyone in group answered yes: {}",
        answer
    );
}
