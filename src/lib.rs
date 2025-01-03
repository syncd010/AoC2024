use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug)]
pub enum AoCResult {
    None,
    Str(String),
    Int(i64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dir {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn can_move_by(&self, by: Dir, dim: Pos) -> bool {
        ((by.x < 0 && self.x >= by.x.abs() as usize)
            || (by.x >= 0 && self.x + (by.x as usize) < dim.x))
            && ((by.y < 0 && self.y >= by.y.abs() as usize)
                || (by.y >= 0 && self.y + (by.y as usize) < dim.y))
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, dir: Dir) -> Pos {
        Pos {
            x: (self.x as isize + dir.x) as usize,
            y: (self.y as isize + dir.y) as usize,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign<Dir> for Pos {
    fn add_assign(&mut self, dir: Dir) {
        *self = Self {
            x: (self.x as isize + dir.x) as usize,
            y: (self.y as isize + dir.y) as usize,
        };
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Dir> for Pos {
    type Output = Pos;

    fn sub(self, dir: Dir) -> Pos {
        Pos {
            x: (self.x as isize - dir.x) as usize,
            y: (self.y as isize - dir.y) as usize,
        }
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl SubAssign<Dir> for Pos {
    fn sub_assign(&mut self, dir: Dir) {
        *self = Self {
            x: (self.x as isize - dir.x) as usize,
            y: (self.y as isize - dir.y) as usize,
        };
    }
}

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Dir {
    pub fn rotate(&self, towards: Rotation) -> Dir {
        match towards {
            Rotation::Clockwise => Dir {
                y: self.x,
                x: self.y * -1,
            },
            Rotation::CounterClockwise => Dir {
                y: self.x * -1,
                x: self.y,
            },
        }
    }
}

impl Sub for Dir {
    type Output = Dir;

    fn sub(self, other: Dir) -> Dir {
        Dir {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
