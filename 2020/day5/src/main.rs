use bitvec::prelude::*;

#[cfg(test)]
mod test;

fn main() {
    // Part 1
    let max_id = itertools::max(
        include_str!("input.txt")
            .lines()
            .map(Seat::parse)
            .map(|seat| seat.id()),
    );
    println!("The maximum seat ID is {:?}", max_id);

    // Part 2
    let mut ids: Vec<_> = include_str!("input.txt").lines().map(Seat::parse).collect();
    ids.sort();

    let mut last_id: Option<Seat> = None;
    for id in ids {
        if let Some(last_id) = last_id {
            let gap = id.0 - last_id.0;
            if gap > 1 {
                println!("Our seat ID is {}", last_id.0 + 1);
                return;
            }
        }
        last_id = Some(id);
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Seat(u16);

impl Seat {
    fn parse(input: &str) -> Self {
        let mut res: Seat = Default::default();

        let bits = BitSlice::<_, Lsb0>::from_element_mut(&mut res.0);
        for (i, &b) in input.as_bytes().iter().rev().enumerate() {
            bits.set(
                i,
                match b {
                    b'F' | b'L' => false,
                    b'B' | b'R' => true,
                    _ => panic!("unexpected letter: {}", b as char),
                },
            )
        }

        res
    }

    fn id(&self) -> u64 {
        self.0 as u64
    }
}

