use anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    // Part 1
    let (a,b) = include_str!("input.txt")
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a,b)| a + b == 2020)
        .expect("No pair in the input sums to 2020");
    dbg!(a + b);
    dbg!(a * b);

    // Part 2
    let (a,b, c) = include_str!("input.txt")
    .lines()
    .map(str::parse::<i64>)
    .collect::<Result<Vec<_>,_>>()?
    .into_iter()
    .tuple_combinations()
    .find(|(a,b, c)| a + b + c == 2020)
    .expect("No triplet in the input sums to 2020");
    dbg!(a + b + c);
    dbg!(a * b * c);
    Ok(())
}
