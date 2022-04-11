use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
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
    size: Point,
    tiles: Vec<Tile>,
}

impl RoadMap {
    fn new(size: Point) -> Self {
        todo!()
    }

    fn set(&self, pos: Point, tile: Tile) {
        todo!()
    }

    fn get(&self, pos: Point) -> Tile {
        todo!()
    }

    fn parse(input: &[u8]) -> Self {
        todo!()
    }
}