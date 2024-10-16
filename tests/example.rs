

#[test]
fn no_lakes() {
    let grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    assert_eq!(Solution::how_many_lakes(grid), 0);
}

#[test]
fn all_one_lakes() {
    let grid = vec![
        vec![0, 0, 0 ,0],
        vec![0, 0, 0 ,0],
        vec![0, 0, 0 ,0],
        vec![0, 0, 0 ,0],
    ];
    assert_eq!(Solution::how_many_lakes(grid), 1);
}

#[test]
fn two_lakes() {
    let grid = vec![
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 0],
        vec![1, 1, 1, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(Solution::how_many_lakes(grid), 2);
}

#[test]
fn three_lakes() {
    let grid = vec![
        vec![1, 0, 1, 0],
        vec![1, 0, 1, 0],
        vec![1, 0, 0, 0],
        vec![0, 1, 1, 1]
    ];
    assert_eq!(Solution::how_many_lakes(grid), 2);
}

use std::collections::HashSet;
struct Solution;

impl Solution {
    fn how_many_lakes(grid: Vec<Vec<u8>>) -> u32 {
        let mut output = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        // fill map with all coordinates
        let width = grid.len();
        let height = grid[0].len();
        for x in 0..width {
            for y in 0..height {
                if visited.contains(&(x, y)) {
                    continue;
                }

                let value = grid[x][y];
                if value == 0 {
                    output += 1;
                    visited.insert((x,y));

                    // flood fill
                    check(&grid, x + 1, y, &mut visited);
                    if x > 0 {
                        check(&grid, x - 1, y, &mut visited);
                    }
                    check(&grid, x, y + 1, &mut visited);
                    if y > 0 {
                        check(&grid, x, y - 1, &mut visited);
                    }
                }
            }
        }

        return output;
    }
}

fn check(grid: &Vec<Vec<u8>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) {
    if visited.contains(&(x, y)) {
        return;
    }
    let Some(column) = grid.get(x) else {
        return;
    };
    let Some(value) = column.get(y) else {
        return;
    };
    if *value == 0 {
        visited.insert((x, y));

        // flood fill
        check(&grid, x + 1, y, visited);

        if x > 0 {
            check(&grid, x - 1, y, visited);
        }

        check(&grid, x, y + 1, visited);

        if y > 0 {
            check(&grid, x, y - 1, visited);
        }
    }
}