use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub enum AoCResult {
    None,
    Str(String),
    Int(i64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dir {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn can_move_by(&self, by: Dir, dim: Pos) -> bool {
        ((by.x < 0 && self.x >= by.x.abs() as usize)
            || (by.x >= 0 && self.x + (by.x as usize) < dim.x))
            && ((by.y < 0 && self.y >= by.y.abs() as usize)
                || (by.y >= 0 && self.y + (by.y as usize) < dim.x))
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
