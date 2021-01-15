use {
    miracle_sudoku::{cell::*, pos::*, set_range, sudoku::*, Rules},
    splr::*,
    std::{convert::TryFrom, fs::File, io::prelude::*, io::Write},
};

pub fn main() {
    let range = 400;
    set_range(range);
    let setting: Vec<i32> = parse(20)
        .iter()
        .map(|(p, d)| p.state(*d, true).as_lit())
        .collect::<Vec<_>>();
    let mut rules: Rules = Vec::new();
    rules.append(&mut sudoku_ident());
    rules.append(&mut sudoku_ident2());
    rules.append(&mut sudoku_row());
    rules.append(&mut sudoku_column());
    rules.append(&mut sudoku_block());
    let mut file = File::create("sudoku400.cnf").expect("fail to create 'sudoku400.cnf'");
    file.write_all(&miracle_sudoku::cnf::as_cnf_u8(&rules, &setting))
        .expect("fail to write 'sudoku400.cnf'");
    println!("#rules: {}", rules.len()); // println!("{:?}", &rules[1..20]);
    let mut config = Config::default();
    config.splr_interface = true;
    config.quiet_mode = false;
    let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");
    for a in setting.iter() {
        solver.add_assignment(*a).expect("panic");
    }
    println!("running...");
    for ans in solver.iter().take(1) {
        println!("found!");
        let mut picked = ans.iter().filter(|l| 0 < **l).collect::<Vec<&i32>>();
        // println!("{}: {:?}", ans.len(), picked);
        assert_eq!((range * range) as usize, picked.len());
        for _i in 1..=range {
            for _j in 1..=range {
                let (_i, _j, d, _b) = Cell::decode(*picked.remove(0));
                print!("{:>2} ", d);
            }
            println!();
        }
        println!();
    }
}

fn parse(tick: usize) -> Vec<(Pos, usize)> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).expect("fail to read");
    let mut vec: Vec<(Pos, usize)> = Vec::new();
    let mut i = 0;
    for (ii, l) in buf.lines().enumerate() {
        if l.is_empty() || ii % (tick + 1) == 0 {
            continue;
        }
        i += 1;
        let mut j = 0;
        for w in l.split_whitespace() {
            match w {
                "." => (),
                "+" => (),
                "|" => (),
                _ => {
                    j += 1;
                    let p = Pos::at(i as isize, j as isize);
                    if let Ok(d) = w.trim().parse::<usize>() {
                        vec.push((p, d));
                    } else {
                        dbg!(w);
                    }

                }
            }
        }
        /*
        let mut k = 1;
        for j in 1..=tick * tick {
            match &l[k..k + 2] {
                " ." => (),
                s => {
                    let p = Pos::at(i as isize, j as isize);
                    if let Ok(d) = s.trim().parse::<usize>() {
                        vec.push((p, d));
                    }
                }
            }
            k += 3;
            println(".");
        } */
    }
    vec
}
