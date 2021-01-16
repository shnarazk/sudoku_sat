# Solve Sudoku variants with SAT solvers

## Sudoku 400x400

Gave up! 
- https://github.com/hkociemba/sudokuNxM/tree/master/sudokus
- http://forum.enjoysudoku.com/giant-sudoku-s-16x16-25x25-36x36-100x100-t6578-150.html#p269691

## Sudoku 144x144

- https://github.com/shnarazk/sudoku_sat/discussions/4#discussioncomment-283483

## Sudoku 100x100

- https://github.com/shnarazk/sudoku_sat/discussions/4

## Sudoku 64

https://shnarazk.github.io/2020/2020-12-18-sudoku64/

## Sudoku 25

https://shnarazk.github.io/2020/2020-08-19-sudoku25/


## Miracle Sudoku

https://www.youtube.com/watch?v=cvEq_XkQg8U

### Approach

1. Preparation

```rust
struct Pos { i: isize, j: isize };
struct Cell { pos: Pos, digit: usize, on: bool };
```

2. Generate Sudoku rules and extra rules

```rust
for i in 1..=RANGE {
    for j in 1..=RANGE {
        let p = Pos::at(i, j);
        for target_i in i..=RANGE {
            for target_j in j..=RANGE {
                 let t = Pos::at(target_i, target_j);
                 for d in 1..=RANGE {
                     rules.push(p.state(d, true).requires(t.state(d, false));
                 }
            }
        }
     }
}
```

3. Convert the rules to a CNF

4. Run SAT solver

### Result, the only one result

```plain
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/miracle_sudoku`
#rules: 22248
4 8 3 7 2 6 1 5 9 
7 2 6 1 5 9 4 8 3 
1 5 9 4 8 3 7 2 6 
8 3 7 2 6 1 5 9 4 
2 6 1 5 9 4 8 3 7 
5 9 4 8 3 7 2 6 1 
3 7 2 6 1 5 9 4 8 
6 1 5 9 4 8 3 7 2 
9 4 8 3 7 2 6 1 5 

$ 
```

![](https://user-images.githubusercontent.com/997855/83323585-d5920000-a29a-11ea-9635-d5ac4bd152fa.png)

### Stats

```plain
$ tokei
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Markdown                1           56           56            0            0
 Rust                    5          281          239           30           12
 TOML                    1           11            7            1            3
-------------------------------------------------------------------------------
 Total                   7          348          302           31           15
-------------------------------------------------------------------------------
```
