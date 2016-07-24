use std::ops::{Add, Div, Mul, Sub};

/// Default type to save coordinates.
pub type Coordinate = i32;

/// Distance between two `Location`s
pub type Distance = f64;

/// Default enum to save directions
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
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

    /// Calculate the square of the distance between two arbitrary locations
    #[allow(non_snake_case)]
    pub fn linear_distance_squared_to(&self, other: &Location) -> Distance {
        let delta_x = (self.x - other.x).abs() as Distance;
        let delta_y = (self.y - other.y).abs() as Distance;

        delta_x * delta_x + delta_y * delta_y
    }

    /// Calculate the distance between two arbitrary locations
    #[allow(non_snake_case)]
    pub fn linear_distance_to(&self, other: &Location) -> Distance {
        self.linear_distance_squared_to(other).sqrt()
    }

    /// Calculate the walking distance (via pathfinding) between two arbitrary location
    pub fn walking_distance_to(&self, _other: &Location) -> Distance {
        unimplemented!()
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

impl Add for Location {
    type Output = Location;
    fn add(mut self, _rhs: Location) -> Location {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self
    }
}

impl Sub for Location {
    type Output = Location;
    fn sub(mut self, _rhs: Location) -> Location {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
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

impl Mul<(f64, f64)> for Location {
    type Output = Location;
    fn mul(self, _rhs: (f64, f64)) -> Location {
        self * (_rhs.0 as Coordinate, _rhs.1 as Coordinate)
    }
}

impl Mul<f64> for Location {
    type Output = Location;
    fn mul(self, _rhs: f64) -> Location {
        self * (_rhs, _rhs)
    }
}
