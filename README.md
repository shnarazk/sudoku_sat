# Miracle Sudoku

![](https://2.bp.blogspot.com/-KNXWZSn8qlA/WMfBy-yWbCI/AAAAAAABCiI/5F9NWxzgYsI_JW725iMYyWn_jYcatAx3wCLcB/s400/book_nanpure.png)

## Approach

1. Preparation

```rust
struct Pos { i: isize, j: isize };
sturct Cell { pos: Pos, digit: usize, on: bool };
```

2. Generate Sudoku rules and extra rules

```rust
for i in 1..=RANGE {
    for j in 1..=RANGE {
        let p = Pos::from(i, j); 
        for target_i in i..=RANGE {
            for target_j in j..=RANGE {
                 let t = Pos::from(target_i, target_j);
                 for d in 1..=RANGE {
                     rules.push(p.state(d, true).requires(t.state(d, false));
		         }
		    }
		}
     }
}
```

# Results

```plain
#rules: 20061
9 4 8 3 7 2 6 1 5 
7 2 6 1 5 9 4 8 3 
1 5 9 4 8 3 7 2 6 
8 3 7 2 6 1 5 9 4 
2 6 1 5 9 4 8 3 7 
5 9 4 8 3 7 2 6 1 
3 7 2 6 1 5 9 4 8 
6 1 5 9 4 8 3 7 2 
4 8 3 7 2 6 1 5 9 

4 8 3 7 2 6 1 5 9 
7 2 6 1 5 9 4 8 3 
1 5 9 4 8 3 7 2 6 
8 3 7 2 6 1 5 9 4 
2 6 1 5 9 4 8 3 7 
5 9 4 8 3 7 2 6 1 
3 7 2 6 1 5 9 4 8 
6 1 5 9 4 8 3 7 2 
9 4 8 3 7 2 6 1 5 
```
