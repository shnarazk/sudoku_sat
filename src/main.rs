use {
    miracle_sudoku::{cell::*, pos::*, RANGE, miracle::*, Rules},
    splr::*,
    std::convert::TryFrom,
};

pub fn main() {
    let mut rules = sudoku_rules();
    rules.append(&mut miracle_knights());
    rules.append(&mut miracle_kings());
    rules.append(&mut miracle_orthogonal());
    println!("#rules: {}", rules.len()); // println!("{:?}", &rules[1..20]);
    let mut solver = Solver::try_from((Config::default(), rules.as_ref())).expect("panic");
    let setting: Vec<i32> = vec![
        // the given problem
        Pos::at(5, 3).state(1, true).as_lit(),
        Pos::at(6, 7).state(2, true).as_lit(),
        // his answer
        // Pos::at(1, 1).state(4, true).as_lit(),
        // Pos::at(1, 2).state(8, true).as_lit(),
        // Pos::at(1, 3).state(3, true).as_lit(),
        // Pos::at(1, 4).state(7, true).as_lit(),
        // Pos::at(1, 5).state(2, true).as_lit(),
    ];
    for a in setting.iter() {
        solver.add_assignment(*a).expect("panic");
    }
    for ans in solver.iter().take(8) {
        let mut picked = ans.iter().filter(|l| 0 < **l).collect::<Vec<&i32>>();
        assert_eq!((RANGE * RANGE) as usize, picked.len());
        for _i in 1..=RANGE {
            for _j in 1..=RANGE {
                let (_i, _j, d, _b) = Cell::decode(*picked.remove(0));
                print!("{:?} ", d);
            }
            println!();
        }
        println!();
    }
}

fn sudoku_rules() -> Rules {
    let mut rules = Vec::new();
    for i in 1..=RANGE {
        for j in 1..=RANGE {
            let p = Pos::at(i, j);
            //
            // at-least single assignments
            //
            rules.push(vec![
                p.state(1, true).as_lit(),
                p.state(2, true).as_lit(),
                p.state(3, true).as_lit(),
                p.state(4, true).as_lit(),
                p.state(5, true).as_lit(),
                p.state(6, true).as_lit(),
                p.state(7, true).as_lit(),
                p.state(8, true).as_lit(),
                p.state(9, true).as_lit(),
            ]);
            //
            // at-most single assignments
            //
            for d in 1..=RANGE as usize {
                for target_d in d + 1..=RANGE as usize {
                    rules.push(p.state(d, true).requires(p.state(target_d, false)));
                }
            }
            //
            // constraints over the row
            //
            let target_i = i;
            for target_j in j + 1..=RANGE {
                let t = Pos::at(target_i, target_j);
                for d in 1..=RANGE as usize {
                    rules.push(p.state(d, true).requires(t.state(d, false)));
                }
            }
            //
            // constraints over the column
            //
            let target_j = j;
            for target_i in i + 1..=RANGE {
                let t = Pos::at(target_i, target_j);
                for d in 1..=RANGE as usize {
                    rules.push(p.state(d, true).requires(t.state(d, false)));
                }
            }
            //
            // constraints over the compartment
            //
            for target_i in i + 1..=(i / 3) * 3 {
                for target_j in j + 1..=(j / 3) * 3 {
                    let t = Pos::at(target_i, target_j);
                    for d in 1..=RANGE as usize {
                        rules.push(p.state(d, true).requires(t.state(d, false)));
                    }
                }
            }
        }
    }
    rules
}
