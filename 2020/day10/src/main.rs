#[derive(Clone, Copy, Debug, Default)]
struct Results {
    ones: usize,
    threes: usize,
}

fn main() {
    // Part I
    let mut numbers = std::iter::once(0)
        .chain(
            include_str!("input.txt")
                .lines()
                .map(|x| x.parse().unwrap()),
        )
        .collect::<Vec<usize>>();
    numbers.sort_unstable();
    if let Some(&max) = numbers.iter().max() {
        numbers.push(max + 3);
    }
    let answer =
        numbers
            .windows(2)
            .map(|w| (w[0], w[1]))
            .fold(Results::default(), |acc, (x, y)| match y - x {
                1 => Results {
                    ones: acc.ones + 1,
                    ..acc
                },
                3 => Results {
                    threes: acc.threes + 1,
                    ..acc
                },
                gap => panic!("Found invalid gap {} between joltages", gap),
            });
    println!("There are {} differences of 1 jolt and {} differences of 3 jolt", answer.ones, answer.threes);
    println!("Effective jolt value is {}", answer.ones * answer.threes);
}
