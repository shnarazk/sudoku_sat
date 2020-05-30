use crate::{pos::*, RANGE, Rules};


pub fn miracle_knights() -> Rules {
    let knights_moves = [
        Pos::at(-2,  1),
        Pos::at(-1,  2),
        Pos::at( 1,  2),
        Pos::at( 2,  1),
        Pos::at( 2, -1),
        Pos::at( 1, -2),
        Pos::at(-1, -2),
        Pos::at(-2, -1),
    ];
    forbid(&knights_moves)
}

pub fn miracle_kings() -> Rules {
    let kings_moves = vec![
        Pos::at(-1,  0),
        Pos::at(-1,  1),
        Pos::at( 0,  1),
        Pos::at( 1,  1),
        Pos::at( 1,  0),
        Pos::at( 1, -1),
        Pos::at( 0, -1),
        Pos::at(-1, -1),
    ];
    forbid(&kings_moves)
}

fn forbid(moves: &[Pos]) -> Rules {
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            for m in moves.iter() {
                if let Some(t) = (p + *m).valid() {
                    for d in 1..=RANGE as usize {
                        rules.push(p.state(d, true).requires(t.state(d, false)));
                    }
                }
            }
        }
    }
    rules
}

pub fn miracle_orthogonal() -> Rules {
    let dirs = [
        Pos::at(-1,  0),         // North
        Pos::at( 0,  1),         // East
        Pos::at( 1,  0),         // South
        Pos::at( 0, -1),         // West
    ];
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            for m in dirs.iter() {
                if let Some(t) = (p + *m).valid() {
                    for d in 1..RANGE as usize {
                        rules.push(p.state(d, true).requires(t.state(d + 1, false)));
                    }
                }
            }
        }
    }
    rules
}
