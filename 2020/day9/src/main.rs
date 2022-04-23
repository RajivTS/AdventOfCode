use itertools::Itertools;

fn main() {
    // Part I
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

    // Part II
    let answer = answer.unwrap();
    let weakness = (2..numbers.len())
        .into_iter()
        .map(|n| {
            numbers
                .windows(n)
                .enumerate()
                .map(move |(i, window)| (i, n, window.iter().sum::<usize>()))
        })
        .flatten()
        .find(|&(_, _, sum)| sum == answer);
    let (start, size, _) = weakness.unwrap();
    let data_set = &numbers[start..(start + size)];
    println!(
        "The final weakness in the XMAS data: {:?}",
        (data_set.iter().min().unwrap() + data_set.iter().max().unwrap())
    );
}
