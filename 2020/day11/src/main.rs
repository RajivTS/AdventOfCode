use std::fmt::Debug;

use im::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pair {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::EmptySeat => write!(f, "L"),
            Self::OccupiedSeat => write!(f, "#"),
        }
    }
}

impl Tile {
    fn next<I>(&self, neighbors: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        match self {
            Self::Floor => Self::Floor,
            Self::EmptySeat => match neighbors
                .filter(|t| matches!(t, Self::OccupiedSeat))
                .count()
            {
                0 => Self::OccupiedSeat,
                _ => Self::EmptySeat,
            },
            Self::OccupiedSeat => match neighbors
                .filter(|t| matches!(t, Self::OccupiedSeat))
                .count()
            {
                0..=4 => Self::OccupiedSeat,
                _ => Self::EmptySeat,
            },
        }
    }
}

#[derive(PartialEq, Clone)]
struct SeatingMap<T>
where
    T: Clone,
{
    size: Pair,
    tiles: Vector<T>,
}

impl<T> SeatingMap<T>
where
    T: Default + Clone,
{
    fn new(size: Pair) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .map(|_| Default::default())
                .collect(),
        }
    }
}

impl<T> SeatingMap<T>
where
    T: Clone,
{
    fn index(&self, pos: Pair) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + self.size.x * pos.y) as _)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Pair, tile: T) {
        if let Some(idx) = self.index(pos) {
            self.tiles[idx] = tile
        }
    }

    fn neighbor_positions(&self, pos: Pair) -> impl Iterator<Item = Pair> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter_map(move |(dx, dy)| {
                if !(dx == 0 && dy == 0) {
                    Some(Pair {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    })
                } else {
                    None
                }
            })
    }

    fn rows_and_columns(input: &[u8]) -> (i64, i64) {
        let mut columns = 0;
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }
        (rows, columns)
    }
}

impl SeatingMap<Tile> {
    fn parse(input: &[u8]) -> Self {
        let (rows, columns) = Self::rows_and_columns(input);
        let mut iter = input.iter().copied();
        let mut map = Self::new(Pair {
            x: columns,
            y: rows,
        });
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Floor,
                    Some(b'#') => Tile::OccupiedSeat,
                    Some(b'L') => Tile::EmptySeat,
                    c => panic!("Expected '.', '#' or 'L' but found {:?}", c),
                };
                map.set(Pair { x: col, y: row }, tile)
            }
            iter.next();
        }
        map
    }

    fn next(&self) -> Self {
        let mut res = Self::new(self.size);
        res.extend(
            self.iter()
                .map(|Positioned(pos, tile)| Positioned(pos, tile.next(self.neighbor_tiles(pos)))),
        );
        res
    }

    fn next_extended(&self) -> Self {
        let mut res = Self::new(self.size);
        res.extend(
            self.iter()
                .map(|Positioned(pos, tile)| Positioned(pos, tile.next(self.visible_seats(pos)))),
        );
        res        
    }

    fn last(self) -> Self {
        use itertools::Itertools;
        itertools::iterate(self, SeatingMap::next)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }

    fn last_extended(self) -> Self {
        use itertools::Itertools;
        itertools::iterate(self, SeatingMap::next_extended)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }

    fn visible_seats(&self, pos: Pair) -> impl Iterator<Item = Tile> + '_ {
        use itertools::Itertools;
        (-1..=1)
        .map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .flatten()
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .map(move |(dx, dy)| {
            itertools::iterate(pos, move |v| Pair { x: v.x + dx, y: v.y + dy })
            .skip(1)
            .map(|pos| self.get(pos))
            .while_some()
            .filter_map(|tile| if matches!(tile, Tile::Floor) { None } else { Some(tile) })
            .take(1)
        })
        .flatten() 
    }
}

impl<T> SeatingMap<T>
where
    T: Copy,
{
    fn get(&self, pos: Pair) -> Option<T> {
        self.index(pos).map(|idx| self.tiles[idx])
    }

    fn neighbor_tiles(&self, pos: Pair) -> impl Iterator<Item = T> + '_ {
        self.neighbor_positions(pos).filter_map(|pos| self.get(pos))
    }

    fn iter(&self) -> impl Iterator<Item = Positioned<T>> + '_ {
        (0..self.size.y)
            .map(move |y| {
                (0..self.size.x).map(move |x| {
                    let pos = Pair { x, y };
                    Positioned(pos, self.get(pos).unwrap())
                })
            })
            .flatten()
    }
}

impl<T> Debug for SeatingMap<T>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.get(Pair { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<A> Extend<Positioned<A>> for SeatingMap<A>
where
    A: Clone,
{
    fn extend<T: IntoIterator<Item = Positioned<A>>>(&mut self, iter: T) {
        for Positioned(pos, tile) in iter {
            self.set(pos, tile)
        }
    }
}

#[derive(Debug)]
struct Positioned<T>(Pair, T);

fn main() {
    // Part I
    // let last = SeatingMap::<Tile>::parse(include_bytes!("input.txt")).last();
    // println!("{:?}", last);
    // println!(
    //     "there are {} occupied seats",
    //     last.iter()
    //         .filter(|p| matches!(p.1, Tile::OccupiedSeat))
    //         .count()
    // );
    // Part II
    let last = SeatingMap::<Tile>::parse(include_bytes!("input.txt")).last_extended();
    println!("{:?}", last);
    println!(
        "there are {} occupied seats",
        last.iter()
            .filter(|p| matches!(p.1, Tile::OccupiedSeat))
            .count()
    );
}
