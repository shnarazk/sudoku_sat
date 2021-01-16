use lazy_static::lazy_static;
use std::sync::atomic::{AtomicIsize, Ordering};
pub mod cell;
pub mod cnf;
pub mod miracle;
pub mod pos;
pub mod sudoku;

lazy_static! {
    static ref RANGE: AtomicIsize = AtomicIsize::new(9);
}

pub fn get_range() -> isize {
    RANGE.load(Ordering::Relaxed)
}

pub fn get_block_len() -> isize {
    (RANGE.load(Ordering::Relaxed) as f64).sqrt() as isize
}

pub fn set_range(val: isize) {
    RANGE.store(val, Ordering::Relaxed);
}

pub type Rules = Vec<Vec<i32>>;
