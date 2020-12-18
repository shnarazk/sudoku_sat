use crate::{pos::*, Rules, get_range};

/// 1. At least one number sholud be assigned on each cell.
/// 2. So a positive assginment should be a trigger to assgin the rest vars negatively.
pub fn sudoku_ident() -> Rules {
    let range = get_range();
    let mut rules = Vec::new();
    for i in 1..=range {
        for j in 1..=range {
            let p = Pos::at(i, j);
            // at-least constraints
            let v = (1..=(range as usize))
                .map(|d| p.state(d, true).as_lit())
                .collect::<Vec<_>>();
            rules.push(v);
            // at-most constraints
            for d in 1..=(range as usize) {
                for dd in 1..(range as usize) {
                    if d != dd {
                        rules.push(p.state(d, true).requires(p.state(dd, false)));
                    }
                }
            }
        }
    }
    rules
}

/// 1. At least each number should be assigned on each group once.
pub fn sudoku_ident2() -> Rules {
    let range = get_range();
    let mut rules = Vec::new();

    // squares
    let bsize = (range as f64).sqrt() as isize;
    let mut block_walk = Vec::new();
    for i in 0..bsize {
        for j in 0..bsize {
            block_walk.push(Pos::at(i, j));
        }
    }

    // for values
    for n in 1..=range {
        // rows
        for i in 1..=range {
            rules.push(
                (1..=range).map(|j| Pos::at(i, j).state(n as usize, true).as_lit()).collect::<Vec<_>>()
            );
        }
        // columns
        for j in 1..=range {
            rules.push(
                (1..=range).map(|i| Pos::at(i, j).state(n as usize, true).as_lit()).collect::<Vec<_>>()
            );
        }
        // squares
        for i in (0..bsize).map(|k| k * bsize + 1) {
            for j in (0..bsize).map(|k| k * bsize + 1) {
                let base = Pos::at(i, j);
                let mut temp = Vec::new();
                for offset in &block_walk {
                    if let Some(p) = (base + *offset).valid(range) {
                        temp.push(p.state(n as usize, true).as_lit());
                    }
                }
                rules.push(temp);
            }
        }
    }
    rules
}

/// 1. In Each row, each number should be assgined at most once.
pub fn sudoku_row() -> Rules {
    let range = get_range();
    let mut rules = Vec::new();
    for i in 1..=range {
        for j in 1..=range {
            let p = Pos::at(i, j);
            for jj in j + 1..=range {
                let q = Pos::at(i, jj);
                for d in 1..=(range as usize) {
                    rules.push(p.state(d, true).requires(q.state(d, false)));
                }
            }
        }
    }
    rules
}

/// 1. In Each column, each number should be assgined at most once.
pub fn sudoku_column() -> Rules {
    let range = get_range();
    let mut rules = Vec::new();
    for j in 1..=range {
        for i in 1..=range {
            let p = Pos::at(i, j);
            for ii in i + 1..=range {
                let q = Pos::at(ii, j);
                for d in 1..=(range as usize) {
                    rules.push(p.state(d, true).requires(q.state(d, false)));
                }
            }
        }
    }
    rules
}

/// 1. In Each square block, each number should be assgined at most once.
pub fn sudoku_block() -> Rules {
    let range = get_range();
    let bsize = (range as f64).sqrt() as isize;
    let mut rules = Vec::new();
    let mut block_walk = Vec::new();
    for i in 0..bsize {
        for j in 0..bsize {
            block_walk.push(Pos::at(i, j));
        }
    }
    for i in (0..bsize).map(|k| k * bsize + 1) {
        for j in (0..bsize).map(|k| k * bsize + 1) {
            let base = Pos::at(i, j);
            for tail in 0..block_walk.len() {
                if let Some(p) = (base + block_walk[tail]).valid(range) {
                    for offset in &block_walk[tail + 1..] {
                        if let Some(q) = (base + *offset).valid(range) {
                            for d in 1..=(range as usize) {
                                rules.push(p.state(d, true).requires(q.state(d, false)));
                            }
                        }
                    }
                }
            }
        }
    }
    rules
}
