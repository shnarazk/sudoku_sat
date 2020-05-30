use std::ops::Add;

pub const RANGE: isize = 9;

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Pos {
    i: isize,
    j: isize,
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
    /// constructor
    pub fn at(i: isize, j: isize) -> Pos {
        Pos { i, j }
    }
    /// return None if out of range.
    pub fn valid(self) -> Option<Pos> {
        if 1 <= self.i && self.i <= RANGE && 1 <= self.j && self.j <= RANGE {
            Some(self)
        } else {
            None
        }
    }
    /// return literal for a digit at (i, j).
    pub fn to_lit(&self, digit: usize, on: bool) -> i32 {
        let var = (self.j - 1) * RANGE * RANGE + (self.i - 1) * RANGE + ((digit - 1) as isize) + 1;
        if on {
            var as i32
        } else {
            -1 * (var as i32)
        }
    }
}

pub fn lit(i: isize, j: isize, d: usize, b: bool) -> Option<i32> {
    if 1 <= i && i <= RANGE && 1 <= j && j <= RANGE && 1 <= d && d <= RANGE as usize {
        let var = (j - 1) * RANGE * RANGE + (i - 1) * RANGE + ((d - 1) as isize) + 1;
        assert!(var.abs() < RANGE.pow(3));
        if b {
            Some(var as i32)
        } else {
            Some(-1 * (var as i32))
        }
    } else {
        None
    }
}
