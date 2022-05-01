use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sub)]
struct Pair {
    x: isize,
    y: isize,
}

impl Pair {
    fn manhattan_distance(self) -> usize {
        (self.x.abs() + self.y.abs()) as _
    }

    fn rotate(self, d: AngleDelta) -> Self {
        let Self {x, y} = self;
        match d.0.rem_euclid(4) {
            0 => Self { x, y },
            1 => Self { x: y, y: -x },
            2 => Self { x: -x, y: -y },
            3 => Self { x: -y, y: x },
            _ => unreachable!()
        }
    }
}

impl std::ops::Mul<isize> for Pair {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Pair {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Direction {
    fn pair(&self) -> Pair {
        match &self {
            Direction::East => Pair { x: 1, y: 0},
            Direction::South => Pair { x: 0, y: -1},
            Direction::West => Pair { x: -1, y: 0},
            Direction::North => Pair { x: 0, y: 1},
        }
    }
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        self as _
    }
}

impl std::convert::TryFrom<isize> for Direction {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if (0..=3).contains(&value) {
            Ok(unsafe {
                std::mem::transmute(value as u8)
            })
        } else {
            Err("Direction out of bounds")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AngleDelta(isize);

impl std::ops::Add<AngleDelta> for Direction {
    type Output = Self;

    fn add(self, rhs: AngleDelta) -> Self::Output {
        let angle: isize = self.into();
        (angle + rhs.0).rem_euclid(4).try_into().unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ShipState {
    pos: Pair,
    dir: Direction,
    waypoint: Pair,
}

impl std::ops::Add<Instruction> for ShipState {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Move(dir, units) => Self {
                waypoint: self.waypoint + dir.pair() * units,
                ..self
            },
            Instruction::Rotate(delta) => Self {
                waypoint: self.waypoint.rotate(delta),
                ..self
            },
            Instruction::Advance(units) => Self {
                pos: self.pos + self.waypoint * units,
                ..self
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Move(Direction, isize),
    Rotate(AngleDelta),
    Advance(isize),
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines()
    .map(|line| {
        let command = line.as_bytes()[0];
        let number = (&line[1..]).parse().unwrap();

        match command {
            b'N' => Instruction::Move(Direction::North, number),
            b'S' => Instruction::Move(Direction::South, number),
            b'E' => Instruction::Move(Direction::East, number),
            b'W' => Instruction::Move(Direction::West, number),
            b'L' => Instruction::Rotate(AngleDelta(-number / 90)),
            b'R' => Instruction::Rotate(AngleDelta(number / 90)),
            b'F' => Instruction::Advance(number),
            c => panic!("Unknown instruction {}", c as char)
        }
    })
}

fn main() {
    let start = ShipState {
        dir: Direction::East,
        pos: Pair { x: 0, y: 0 },
        waypoint: Pair { x: 10, y: 1 },
    };
    let end = parse_instructions(include_str!("input.txt")).fold(start, |state, ins| state + ins);

    dbg!(start, end, (end.pos - start.pos).manhattan_distance());
}
