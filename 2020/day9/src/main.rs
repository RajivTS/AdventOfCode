use itertools::Itertools;

fn main() {
    // Part 1
    let numbers = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let batch = 25;
    let answer = numbers.windows(batch + 1).find_map(|s| {
        if (&s[..batch])
            .iter()
            .tuple_combinations()
            .any(|(x, y)| x + y == s[batch])
        {
            None
        } else {
            Some(s[batch])
        }
    });
    println!("The first weakness in XMAS data: {:?}", &answer);
}
