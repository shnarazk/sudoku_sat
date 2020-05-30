use {
    miracle_sudoku::pos::*,
    splr::{Certificate::SAT, *},
    std::convert::TryFrom,
};

pub fn main() {
    println!("Hello, world!");
    let rules = sudoku_rules();
    println!("{}", rules.len());
    // println!("{:?}", &rules[1..20]);
    let mut solver = Solver::try_from((Config::default(), rules.as_ref())).expect("panic");
    let setting: Vec<i32> = vec![lit(1, 2, 3, true).unwrap()];
    for a in setting.iter() {
        solver.add_assignment(*a).expect("panic");
    }
    if let Ok(SAT(ref ans)) = solver.solve() {
        let mut picked = ans
            .iter()
            .filter(|l| 0 < **l)
            .map(|l| *l)
            .collect::<Vec<i32>>();
        println!("{:?}", picked);
        assert_eq!((RANGE * RANGE) as usize, picked.len());
        for _i in 1..=RANGE {
            for _j in 1..=RANGE {
                let (_i, _j, d, _b) = decode(picked.remove(0));
                print!("{:?} ", d);
            }
            println!();
        }
    }
}

fn sudoku_rules() -> Vec<Vec<i32>> {
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            // at least single assignments
            rules.push(vec![
                p.to_lit(1, true),
                p.to_lit(2, true),
                p.to_lit(3, true),
                p.to_lit(4, true),
                p.to_lit(5, true),
                p.to_lit(6, true),
                p.to_lit(7, true),
                p.to_lit(8, true),
                p.to_lit(9, true),
            ]);
            // at most single assignments
            for d in 1..=9 {
                for target_d in d + 1..=9 {
                    rules.push(requires(p.to_lit(d, true), p.to_lit(target_d, false)));
                }
            }
            // restriction
            // over the row
            let target_i = i;
            for target_j in j + 1..=RANGE {
                let t = Pos::at(target_i, target_j);
                for d in 1..=9 {
                    rules.push(requires(p.to_lit(d, true), t.to_lit(d, false)));
                }
            }
            // over the column
            let target_j = j;
            for target_i in i + 1..=RANGE {
                let t = Pos::at(target_i, target_j);
                for d in 1..=9 {
                    rules.push(requires(p.to_lit(d, true), t.to_lit(d, false)));
                }
            }
            // over the compartment
            for target_i in i + 1..=(i / 3) * 3 {
                for target_j in j + 1..=(j / 3) * 3 {
                    let t = Pos::at(target_i, target_j);
                    for d in 1..=9 {
                        rules.push(requires(p.to_lit(d, true), t.to_lit(d, false)));
                    }
                }
            }
        }
    }
    rules
}

/// convert `self => other` to `!self || other`
fn requires(pre: i32, then: i32) -> Vec<i32> {
    vec![-pre, then]
}

/// decode an assignment to (i, j, digit)
fn decode(a: i32) -> (isize, isize, usize, bool) {
    (
        (a as isize - 1) / (RANGE * RANGE) + 1,
        (a as isize - 1) / RANGE % RANGE + 1,
        (a as usize - 1) % (RANGE as usize) + 1,
        0 < a,
    )
}
