use {
    crate::{cell::Cell, RANGE},
    std::{
        iter::Iterator,
        ops::{Add, Neg},
    },
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct World {
    pub range: usize,
}

impl World {
    pub fn at(i: usize, j: usize) -> Pos {
        Pos::at(i as isize, j as isize)
    }
    pub fn valid(&self, pos: Pos) -> Option<Pos> {
        pos.valid(self.range as isize)
    }
}

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

pub struct Neighbors<'a> {
    base: Pos,
    around: &'a [Pos],
    index: usize,
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..self.around.len() {
            if let Some(res) = (self.base + self.around[i]).valid(RANGE) {
                self.index = i + 1;
                return Some(res);
            }
        }
        None
    }
}

impl Pos {
    /// constructor without range check
    pub fn at(i: isize, j: isize) -> Pos {
        Pos { i, j }
    }
    /// into iterator
    pub fn neighbors<'a>(self, around: &'a [Pos]) -> Neighbors<'a> {
        Neighbors {
            base: self,
            around,
            index: 0,
        }
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
    pub fn valid(self, range: isize) -> Option<Pos> {
        if 1 <= self.i && self.i <= range && 1 <= self.j && self.j <= range {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_around() {
        assert_eq!(RANGE, 25);
        let kings_moves = vec![
            Pos::at(-1, 0),
            Pos::at(-1, 1),
            Pos::at(0, 1),
            Pos::at(1, 1),
            Pos::at(1, 0),
            Pos::at(1, -1),
            Pos::at(0, -1),
            Pos::at(-1, -1),
        ];
        assert_eq!(Pos::at(1, 1).neighbors(&kings_moves).count(), 3);
        assert_eq!(Pos::at(1, 2).neighbors(&kings_moves).count(), 5);
        assert_eq!(Pos::at(2, 2).neighbors(&kings_moves).count(), 8);
        assert_eq!(Pos::at(25, 25).neighbors(&kings_moves).count(), 3);
    }
}
