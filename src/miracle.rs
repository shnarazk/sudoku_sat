use crate::{get_range, pos::*, Rules};

pub fn miracle_knights() -> Rules {
    let knights_moves = [
        Pos::at(-2, 1),
        Pos::at(-1, 2),
        Pos::at(1, 2),
        Pos::at(2, 1),
        Pos::at(2, -1),
        Pos::at(1, -2),
        Pos::at(-1, -2),
        Pos::at(-2, -1),
    ];
    forbid(&knights_moves)
}

pub fn miracle_kings() -> Rules {
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
    forbid(&kings_moves)
}

fn forbid(moves: &[Pos]) -> Rules {
    let range = get_range();
    let mut rules = Vec::new();
    for i in 1..=range {
        for j in 1..=range {
            let p = Pos::at(i, j);
            for q in p.neighbors(moves) {
                for d in 1..=range as usize {
                    rules.push(p.state(d, true).requires(q.state(d, false)));
                }
            }
        }
    }
    rules
}

pub fn miracle_orthogonal() -> Rules {
    let range = get_range();
    let dirs = [
        Pos::at(-1, 0), // North
        Pos::at(0, 1),  // East
        Pos::at(1, 0),  // South
        Pos::at(0, -1), // West
    ];
    let mut rules = Vec::new();
    for i in 1..=range {
        for j in 1..=range {
            let p = Pos::at(i, j);
            for q in p.neighbors(&dirs) {
                for d in 1..range as usize {
                    rules.push(p.state(d, true).requires(q.state(d + 1, false)));
                }
            }
        }
    }
    rules
}
