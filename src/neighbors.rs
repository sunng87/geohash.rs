#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors {
    pub sw: String,
    pub s: String,
    pub se: String,
    pub w: String,
    pub e: String,
    pub nw: String,
    pub n: String,
    pub ne: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// North
    N,
    /// North-east
    NE,
    /// Eeast
    E,
    /// South-east
    SE,
    /// South
    S,
    /// South-west
    SW,
    /// West
    W,
    /// North-west
    NW,
}

impl Direction {
    pub fn to_tuple(&self) -> (i8, i8) {
        match self {
            Direction::SW => (-1, -1),
            Direction::S => (-1, 0),
            Direction::SE => (-1, 1),
            Direction::W => (0, -1),
            Direction::E => (0, 1),
            Direction::NW => (1, -1),
            Direction::N => (1, 0),
            Direction::NE => (1, 1),
        }
    }
}