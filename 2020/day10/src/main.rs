use std::collections::HashMap;

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

    // Part II
    let n = numbers.len();
    let mut num_paths = HashMap::new();
    num_paths.insert(numbers.last().copied().unwrap(), 1);
    for i in (0..(n - 1)).into_iter().rev() {
        let i_val = numbers[i];
        let range = (i + 1)..=(std::cmp::min(i + 3, n - 1));
        let neighbor_paths: usize = range.filter_map(|j| {
            let j_val = numbers[j];
            let gap = j_val - i_val;
            if (1..=3).contains(&gap) {
                Some(num_paths.get(&j_val).unwrap())
            } else {
                None
            }
        }).sum();
        num_paths.insert(i_val,neighbor_paths);
    }
    println!("Number of joltage adapter configurations is {}", num_paths.get(&0).unwrap())
}
