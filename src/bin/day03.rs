use std::collections::HashMap;
type Grid = HashMap<(isize, isize), char>;

fn parse() -> Grid {
    let grid: Vec<String> = include_str!("../inputs/03.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    let mut grid_map: Grid = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid_map.insert((row.try_into().unwrap(), col.try_into().unwrap()), ch);
        }
    }
    grid_map
}

fn part1(grid: Grid) -> u32 {
    let max_rows = grid.keys().map(|&(row, _)| row).max().unwrap() + 1;
    let max_cols = grid.keys().map(|&(_, col)| col).max().unwrap() + 1;
    let mut parts: Vec<u32> = vec![];

    for row in 0..max_rows + 1 {
        let mut num: Vec<u32> = vec![];
        let mut part = false;
        for col in 0..max_cols + 1 {
            let cell = grid.get(&(row, col)).unwrap_or(&'.');
            if cell.is_numeric() {
                if check_neighbor(&grid, row, col) {
                    part = true;
                }
                num.push(cell.to_digit(10).unwrap());
            } else if !num.is_empty() {
                if part {
                    let num2: u32 = num.iter().fold(0, |acc, &x| acc * 10 + x);
                    parts.push(num2);
                    part = false;
                }
                num.clear();
            }
        }
    }
    parts.iter().sum()
}

fn _part2(_: Vec<String>) -> i32 {
    todo!();
}

fn check_neighbor(grid: &Grid, row: isize, col: isize) -> bool {
    let offsets = [-1, 0, 1];
    for or in offsets {
        for oc in offsets {
            if or == 0 && oc == 0 {
                continue;
            }
            let cell = grid.get(&(row + or, col + oc)).unwrap_or(&'.');
            if cell != &'.' && !cell.is_numeric() {
                return true;
            }
        }
    }
    false
}

pub fn main() {
    println!("D3P1 result: {}", part1(parse()));
    //println!("D3P2 result: {}", part2(parse()));
}

#[test]
fn test_p1() {
    assert_eq!(part1(parse()), 341535);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(parse()), 0);
// }
