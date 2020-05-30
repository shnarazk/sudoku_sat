# Miracle Sudoku

![](https://2.bp.blogspot.com/-KNXWZSn8qlA/WMfBy-yWbCI/AAAAAAABCiI/5F9NWxzgYsI_JW725iMYyWn_jYcatAx3wCLcB/s400/book_nanpure.png)

## Approach

0. Preparation

```rust
const RANGE: isize = 9;

#[derive(Clone, Eq, Debug, PartialEq)]
struct Pos {
    i: isize,
	j: isize,
}

impl Add for Pos {..}

impl Pos {
  /// return None if out of range.
  fn valid(&self) -> Option<Pos>;
  /// return literal for a digit at (i, j). 
  fn to_lit(&self, digit: usize, on: bool) -> i32;
```

1. Generate Sudoku rules

```rust
let rules = Vec::new();

for j in 0..RANGE {
  for i in 0..RANGE {
    let p = Pos::from(i, j);
    // for column
	let target_j = j;
    for target_i in i+1..RANGE {
	   let t = Pos::from(target_i, target_j);	
       for d in 1..=9 {
		   rules.push(vec![p.to_lit(d, true), t.to_lit(d, false)]);
	   }
    }
	// for column
	let target_i = i;
    for target_j in j+1..RANGE {
	   ...
    }
  }
}
for block_j in 0..RANGE/3 {
  for block_i in 0..RANGE/3 {
    ..
  }
}
```

2 Add extra rules
