# Miracle Sudoku

https://www.youtube.com/watch?v=cvEq_XkQg8U

## Approach

1. Preparation

```rust
struct Pos { i: isize, j: isize };
struct Cell { pos: Pos, digit: usize, on: bool };
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
```

```plain
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

![](https://user-images.githubusercontent.com/997855/83323585-d5920000-a29a-11ea-9635-d5ac4bd152fa.png)
