use {
    crate::{pos::Pos, RANGE},
    std::ops::Neg,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cell {
    pub pos: Pos,
    pub digit: usize,
    pub on: bool,
}

impl Cell {
    /// convert a state to Splr's literal
    pub fn as_lit(&self) -> i32 {
        let Cell {
            pos: Pos { i, j },
            digit: d,
            on: b,
        } = *self;
        assert!(1 <= i);
        assert!(i <= RANGE);
        assert!(1 <= j);
        assert!(j <= RANGE);
        assert!(1 <= d);
        assert!(d <= RANGE as usize);
        let var = (i - 1) * RANGE * RANGE + (j - 1) * RANGE + ((d - 1) as isize) + 1;
        // dbg!(var);
        assert!(var.abs() <= RANGE.pow(3));
        if b {
            var as i32
        } else {
            (var as i32).neg()
        }
    }
    /// convert `a => b` to a clause `{!a, b}`
    pub fn requires(self, other: Cell) -> Vec<i32> {
        vec![self.as_lit().neg(), other.as_lit()]
    }
    /// decode an assignment returned by Splr to `(i, j, digit, flag)`
    pub fn decode(a: i32) -> (isize, isize, usize, bool) {
        (
            (a as isize - 1) / (RANGE * RANGE) + 1,
            (a as isize - 1) / RANGE % RANGE + 1,
            (a as usize - 1) % (RANGE as usize) + 1,
            0 < a,
        )
    }
}
