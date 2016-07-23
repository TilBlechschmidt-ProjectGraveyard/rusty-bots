use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

/// Default type to save coordinates.
pub type Coordinate = i32;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

/// Location in a `Map`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    /// x component
    pub x: Coordinate,
    /// y component
    pub y: Coordinate
}

impl Location {
    /// Creates a new instance `Location`.
    pub fn new(x: Coordinate, y: Coordinate) -> Location {
        Location {
            x: x,
            y: y
        }
    }
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(mut self, _rhs: Direction) -> Location {
        match _rhs {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::UpLeft => { self.x -= 1; self.y -= 1; }
            Direction::UpRight => { self.x += 1; self.y -= 1; }
            Direction::DownLeft => { self.x -= 1; self.y += 1; }
            Direction::DownRight => { self.x += 1; self.y += 1; }
        }
        self
    }
}

impl Add<(Coordinate, Coordinate)> for Location {
    type Output = Location;
    fn add(mut self, _rhs: (Coordinate, Coordinate)) -> Location {
        self.x += _rhs.0;
        self.y += _rhs.1;
        self
    }
}

impl Div<(Coordinate, Coordinate)> for Location {
    type Output = Location;
    fn div(mut self, _rhs: (Coordinate, Coordinate)) -> Location {
        self.x /= _rhs.0;
        self.y /= _rhs.1;
        self
    }
}

impl Div<Coordinate> for Location {
    type Output = Location;
    fn div(self, _rhs: Coordinate) -> Location {
        self/(_rhs, _rhs)
    }
}

impl Mul<(Coordinate, Coordinate)> for Location {
    type Output = Location;
    fn mul(mut self, _rhs: (Coordinate, Coordinate)) -> Location {
        self.x *= _rhs.0;
        self.y *= _rhs.1;
        self
    }
}

impl Mul<Coordinate> for Location {
    type Output = Location;
    fn mul(self, _rhs: Coordinate) -> Location {
        self*(_rhs, _rhs)
    }
}
