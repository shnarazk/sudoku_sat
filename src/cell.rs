use {
    crate::{get_range, pos::Pos},
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
        let range = get_range();
        let Cell {
            pos: Pos { i, j },
            digit: d,
            on: b,
        } = *self;
        assert!(1 <= i);
        assert!(i <= range);
        assert!(1 <= j);
        if range < j {
            panic!("range: {}, j: {}", range, j);
        }
        assert!(j <= range);
        assert!(1 <= d);
        if (range as usize) < d {
            panic!("range: {}, d: {}", range, d);
        }
        assert!(d <= (range as usize));
        let var = (i - 1) * range * range + (j - 1) * range + ((d - 1) as isize) + 1;
        // dbg!(var);
        assert!(var.abs() <= range.pow(3));
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
        let range = get_range();
        (
            (a as isize - 1) / (range * range) + 1,
            (a as isize - 1) / range % range + 1,
            (a as usize - 1) % (range as usize) + 1,
            0 < a,
        )
    }
}
