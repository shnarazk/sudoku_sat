use {
    miracle_sudoku::{cell::*, pos::*, sudoku25::*, Rules, RANGE},
    splr::*,
    std::{convert::TryFrom, io::Write, fs::File},
};

pub fn main() {
    let mut rules: Rules = Vec::new();
    rules.append(&mut sudoku_ident());
    rules.append(&mut sudoku_ident2());
    rules.append(&mut sudoku_row());
    rules.append(&mut sudoku_column());
    rules.append(&mut sudoku_block());
    let setting: Vec<i32> = parse_s25()
        .iter()
        .map(|(p, d)| p.state(*d, true).as_lit())
        .collect::<Vec<_>>();
    let mut file = File::create("sudoku25.cnf").expect("fail to create 'sudoku25.cnf'");
    file.write_all(&miracle_sudoku::cnf::as_cnf_u8(&rules, &setting)).expect("fail to write 'sudoku25.cnf'");
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
        assert_eq!((RANGE * RANGE) as usize, picked.len());
        for _i in 1..=RANGE {
            for _j in 1..=RANGE {
                let (_i, _j, d, _b) = Cell::decode(*picked.remove(0));
                print!("{:>2} ", d);
            }
            println!();
        }
        println!();
    }
}

const S25: &str = "
+--------------+--------------+--------------+--------------+--------------+
| . 12  .  .  .| .  .  .  .  .| .  .  .  9  .| .  . 15  .  .|22  .  .  .  .|
| .  .  .  .  .| .  9  . 19  .| .  . 10 11  .| .  .  .  .  .| .  .  .  .  .|
| .  4  . 22  .| .  .  .  .  .| .  .  .  .  .| .  . 12  .  .|20 15  1  .  .|
|16  1 20 15  .| .  .  .  .  .| .  .  .  .  .|14  .  4  . 22|12 25  .  .  .|
| .  .  .  .  .| .  7  2 11  .|23  . 19  8  .| .  .  . 13  .| .  .  .  .  .|
+--------------+--------------+--------------+--------------+--------------+
|13  .  8  .  2| .  .  .  .  .| .  .  7 23  6| .  9  . 19 11| .  .  .  .  .|
| .  .  .  . 23| .  .  .  . 16| .  .  .  .  .| .  .  .  .  .| 1  .  .  .  .|
| 7  .  .  . 10| 3  .  .  .  .| .  .  9 19  .| . 13  . 23  .| .  .  .  5  .|
| .  .  .  .  .|15  .  .  . 22| .  .  .  .  .| .  .  .  .  .|25 20  .  .  .|
| .  .  .  .  .|12  . 14  1 25| .  .  .  .  .| .  .  3  .  .|16  4 15  .  .|
+--------------+--------------+--------------+--------------+--------------+
| .  .  .  .  .| . 19  9  .  .| .  . 13  7  .| .  .  .  5  .| .  .  . 23 10|
| . 22  . 25 17| .  .  .  .  .| .  .  .  .  .|12  . 20  .  .| .  .  .  .  .|
| . 20 12 16  .| .  .  .  .  .| .  .  .  . 14|15 22  1  . 25| .  .  .  .  .|
| . 15  .  .  .| . 11  .  .  .| .  .  .  .  .| .  . 16  .  .| .  .  .  9  .|
| .  .  .  1  .| . 10  . 23  .| .  .  .  . 18| .  .  .  .  .| .  .  .  .  8|
+--------------+--------------+--------------+--------------+--------------+
|10  .  .  .  8| . 13  .  5  .| .  .  .  .  .| . 19  . 11 23| .  .  .  6  .|
| .  .  . 17  7| .  .  .  .  .| .  .  .  .  1| .  .  .  .  .| 4 22  .  .  .|
| .  .  .  . 11| . 23  .  .  .| .  .  .  . 20| .  .  .  2  .|14  .  .  .  .|
|19  . 23  .  5| .  8  .  9  .| . 21  .  .  .| . 10  .  7  .| .  .  .  .  .|
| .  3  .  .  .| .  .  .  .  .|25  4  .  . 12| .  .  .  .  .|15  1 16  .  .|
+--------------+--------------+--------------+--------------+--------------+
| .  .  .  .  .| .  .  .  . 15| . 12  .  . 25| 1  . 22  .  .| 3  .  .  .  .|
|23  .  .  . 19| .  2  .  .  .| .  .  .  .  .| .  .  . 10  .| .  .  .  7 11|
| .  .  . 18  .| .  .  .  .  .| . 20  .  .  .| .  .  .  .  .| .  .  .  .  .|
| .  .  .  .  .| .  .  .  .  4|14 15  .  . 22| .  .  .  .  .| .  .  . 10  .|
|11  .  .  .  9| .  .  .  .  .| .  .  .  .  .| .  .  .  .  .| .  .  . 19  .|
+--------------+--------------+--------------+--------------+--------------+
";

fn parse_s25() -> Vec<(Pos, usize)> {
    let mut vec: Vec<(Pos, usize)> = Vec::new();
    let mut i = 0;
    for (ii, l) in S25.lines().skip(1).enumerate() {
        if l.is_empty() || ii % 6 == 0 {
            continue;
        }
        i += 1;
        let mut k = 1;
        for j in 1..=25 {
            match &l[k..k + 2] {
                " ." => (),
                s => {
                    let p = Pos::at(i as isize, j);
                    if let Ok(d) = s.trim().parse::<usize>() {
                        vec.push((p, d));
                    }
                }
            }
            k += 3;
        }
    }
    vec
}
