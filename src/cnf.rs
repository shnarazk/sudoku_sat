use crate::Rules;

pub fn dump_as_cnf(rules: &Rules) {
    let num_cls = rules.len();
    let num_var = rules
        .iter()
        .map(|v| v.iter().map(|v| v.abs() as usize).max().unwrap_or(0))
        .max()
        .unwrap_or(0);
    println!("p cnf {} {}", num_var, num_cls);
    for cls in rules.iter() {
        for l in cls.iter() {
            print!("{} ", *l);
        }
        println!("0");
    }
}
