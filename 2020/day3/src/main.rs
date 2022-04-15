use std::fmt::Debug;

#[cfg(test)]
mod test;

fn main() {
    let map = RoadMap::parse(include_bytes!("input.txt"));

    let deltas: &[Pair] = &[
        (1, 1).into(),
        (3, 1).into(),
        (5, 1).into(),
        (7, 1).into(),
        (1, 2).into(),
    ];
    let answer = deltas
        .iter()
        .copied()
        // generate all itineraries
        .map(|delta| generate_itinerary(&map, delta))
        // count trees
        .map(|itin| {
            itin.into_iter()
                .filter(|&pos| map.get(pos) == Tile::Tree)
                .count()
        })
        // multiply everything together
        .product::<usize>();

    println!("The answer is {}", answer);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Pair {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Open
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Tree => write!(f, "#"),
        }
    }
}

struct RoadMap {
    size: Pair,
    tiles: Vec<Tile>,
}

impl RoadMap {
    fn new(size: Pair) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
            .into_iter()
            .map(|_| Default::default())
            .collect()
        }
    }

    fn set(&mut self, pos: Pair, tile: Tile) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile;
        }
    }

    fn get(&self, pos: Pair) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }

    fn size(input: &[u8]) -> Pair {
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
        (columns, rows).into()        
    }

    fn parse(input: &[u8]) -> Self {
        let mut iter = input.iter().copied();
        let mut map = Self::new(Self::size(input));
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Open,
                    Some(b'#') => Tile::Tree,
                    c => panic!("Expected '.' or '#', but got: {:?}", c),
                };
                map.set((col, row).into(), tile);
            }
            iter.next();
        }
        map
    }

    fn index(&self, pos: Pair) -> Option<usize> {
        self.normalize_pos(pos).map(|val| (val.x + val.y * self.size.x) as _)
    }

    fn normalize_pos(&self, pos: Pair) -> Option<Pair> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let x = pos.x % self.size.x;
            let x = if x < 0 { self.size.x + x } else { x };
            Some((x, pos.y).into())
        }
    }
}

impl Debug for RoadMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((row, col).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn generate_itinerary(map: &RoadMap, delta: Pair) -> Vec<Pair> {
    let mut pos = Pair::from((0, 0));
    let mut res: Vec<_> = Default::default();

    while map.normalize_pos(pos).is_some() {
        res.push(pos);
        pos.x += delta.x;
        pos.y += delta.y;
    }
    res
}