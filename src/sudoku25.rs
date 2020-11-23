use crate::{pos::*, Rules, RANGE};

pub fn sudoku_ident() -> Rules {
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            // at-least constraints
            let v = (1..=(RANGE as usize))
                .map(|d| p.state(d, true).as_lit())
                .collect::<Vec<_>>();
            rules.push(v);
            // at-most constraints
            for d in 1..=(RANGE as usize) {
                for dd in 1..(RANGE as usize) {
                    if d != dd {
                        rules.push(p.state(d, true).requires(p.state(dd, false)));
                    }
                }
            }
        }
    }
    rules
}

pub fn sudoku_ident2() -> Rules {
    let mut rules = Vec::new();

    // squares
    let bsize = (RANGE as f64).sqrt() as isize;
    let mut block_walk = Vec::new();
    for i in 0..bsize {
        for j in 0..bsize {
            block_walk.push(Pos::at(i, j));
        }
    }

    // for values
    for n in 1..=RANGE {
        // rows
        for i in 1..=RANGE {
            rules.push(
                (1..=RANGE).map(|j| Pos::at(i, j).state(n as usize, true).as_lit()).collect::<Vec<_>>()
            );
        }
        // columns
        for j in 1..=RANGE {
            rules.push(
                (1..=RANGE).map(|i| Pos::at(i, j).state(n as usize, true).as_lit()).collect::<Vec<_>>()
            );
        }
        // squares
        for i in (0..bsize).map(|k| k * bsize + 1) {
            for j in (0..bsize).map(|k| k * bsize + 1) {
                let base = Pos::at(i, j);
                let mut temp = Vec::new();
                for offset in &block_walk {
                    if let Some(p) = (base + *offset).valid(25) {
                        temp.push(p.state(n as usize, true).as_lit());
                    }
                }
                rules.push(temp);
            }
        }
    }
    rules
}

pub fn sudoku_row() -> Rules {
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            for jj in j + 1..=RANGE {
                let q = Pos::at(i, jj);
                for d in 1..=(RANGE as usize) {
                    rules.push(p.state(d, true).requires(q.state(d, false)));
                }
            }
        }
    }
    rules
}

pub fn sudoku_column() -> Rules {
    let mut rules = Vec::new();
    for j in 1..=RANGE {
        for i in 1..=RANGE {
            let p = Pos::at(i, j);
            for ii in i + 1..=RANGE {
                let q = Pos::at(ii, j);
                for d in 1..=(RANGE as usize) {
                    rules.push(p.state(d, true).requires(q.state(d, false)));
                }
            }
        }
    }
    rules
}

pub fn sudoku_block() -> Rules {
    let bsize = (RANGE as f64).sqrt() as isize;
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
                if let Some(p) = (base + block_walk[tail]).valid(25) {
                    for offset in &block_walk[tail + 1..] {
                        if let Some(q) = (base + *offset).valid(25) {
                            for d in 1..=(RANGE as usize) {
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
