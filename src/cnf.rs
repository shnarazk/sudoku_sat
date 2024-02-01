use {crate::Rules, std::io::Write};

#[allow(clippy::ptr_arg)]
pub fn dump_as_cnf(rules: &Rules, assigns: &[i32]) {
    let num_cls = rules.len();
    let num_var = rules
        .iter()
        .map(|v| v.iter().map(|v| v.unsigned_abs()).max().unwrap_or(0))
        .max()
        .unwrap_or(0);
    println!("p cnf {} {}", num_var, num_cls + assigns.len());
    for cls in rules.iter() {
        for l in cls.iter() {
            print!("{} ", *l);
        }
        println!("0");
    }
    for asg in assigns.iter() {
        println!("{} 0", *asg);
    }
}

#[allow(clippy::ptr_arg)]
pub fn as_cnf_u8(rules: &Rules, assigns: &[i32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let num_cls = rules.len();
    let num_var = rules
        .iter()
        .map(|v| v.iter().map(|v| v.unsigned_abs()).max().unwrap_or(0))
        .max()
        .unwrap_or(0);
    result
        .write_all(format!("p cnf {} {}\n", num_var, num_cls + assigns.len()).as_bytes())
        .unwrap();
    for cls in rules.iter() {
        for l in cls.iter() {
            result.write_all(format!("{} ", *l).as_bytes()).unwrap();
        }
        result.write_all(b"0\n").unwrap();
    }
    for asg in assigns.iter() {
        result
            .write_all(format!("{} 0\n", *asg).as_bytes())
            .unwrap();
    }
    result
}
