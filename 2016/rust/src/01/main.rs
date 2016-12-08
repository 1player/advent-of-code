use std::io::{self, Read};
use std::fmt;
use std::str::FromStr;

enum Turn {
    Left,
    Right,
}

struct Direction(Turn, isize);

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ts, ss) = s.split_at(1);
        let s: isize = ss.parse().map_err(|_| ())?;

        match ts {
            "L" => Ok(Direction(Turn::Left, s)),
            "R" => Ok(Direction(Turn::Right, s)),
            _ => Err(()),
        }
    }
}


enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn(self, turn: Turn) -> Heading {
        match (self, turn) {
            (Heading::North, Turn::Right) => Heading::East,
            (Heading::North, Turn::Left) => Heading::West,
            (Heading::East, Turn::Right) => Heading::South,
            (Heading::East, Turn::Left) => Heading::North,
            (Heading::South, Turn::Right) => Heading::West,
            (Heading::South, Turn::Left) => Heading::East,
            (Heading::West, Turn::Right) => Heading::North,
            (Heading::West, Turn::Left) => Heading::South,
        }
    }
}

struct Position {
    heading: Heading,
    x: isize,
    y: isize,
}

impl Position {
    fn go(self, dir: Direction) -> Position {
        let new_heading = self.heading.turn(dir.0);
        Position { heading: new_heading, ..self }.advance(dir.1)
    }

    fn advance(self, steps: isize) -> Position {
        match self.heading {
            Heading::North => Position { y: self.y + steps, ..self},
            Heading::South => Position { y: self.y - steps, ..self},
            Heading::East => Position { x: self.x + steps, ..self},
            Heading::West => Position { x: self.x - steps, ..self},
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.x >= 0 {
            write!(f, "{} blocks East, ", self.x)?;
        } else {
            write!(f, "{} blocks West, ", -self.x)?;
        }

        if self.y >= 0 {
            write!(f, "{} blocks North, ", self.y)?;
        } else {
            write!(f, "{} blocks South, ", -self.y)?;
        }

        write!(f, "{} blocks away.", self.x.abs() + self.y.abs())?;

        Ok(())
    }
}

fn main() {
    let mut pos = Position { heading: Heading::North, x: 0, y: 0 };

    let mut directions_s = String::new();
    io::stdin().read_to_string(&mut directions_s).unwrap();

    for direction_s in directions_s.split(", ").map(|s| s.trim()).filter(|&s| s != "") {
        let direction = Direction::from_str(direction_s).unwrap();
        pos = pos.go(direction);
    }
    println!("{}", pos);
}
