use {
    crate::{cell::Cell, RANGE},
    std::ops::{Add, Neg},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub i: isize,
    pub j: isize,
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Self::Output {
        Pos {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Pos {
    /// constructor without range check
    pub fn at(i: isize, j: isize) -> Pos {
        Pos { i, j }
    }
    /// set a digit
    pub fn state(self, digit: usize, on: bool) -> Cell {
        Cell {
            pos: self,
            digit,
            on,
        }
    }
    /// return literal for a digit at (i, j).
    pub fn to_lit(&self, digit: usize, on: bool) -> i32 {
        let var = (self.j - 1) * RANGE * RANGE + (self.i - 1) * RANGE + ((digit - 1) as isize) + 1;
        if on {
            var as i32
        } else {
            (var as i32).neg()
        }
    }
    /// return None if out of range.
    pub fn valid(self) -> Option<Pos> {
        if 1 <= self.i && self.i <= RANGE && 1 <= self.j && self.j <= RANGE {
            Some(self)
        } else {
            None
        }
    }
}
