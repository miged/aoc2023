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
    let rows = grid.keys().map(|&(row, _)| row).max().unwrap() + 2;
    let cols = grid.keys().map(|&(_, col)| col).max().unwrap() + 2;
    let mut sum = 0;

    for row in 0..rows {
        let mut num: Vec<u32> = vec![];
        let mut part = false;
        for col in 0..cols {
            let cell = grid.get(&(row, col)).unwrap_or(&'.');
            if cell.is_numeric() {
                if check_neighbor(&grid, row, col) {
                    part = true;
                }
                num.push(cell.to_digit(10).unwrap());
            } else if !num.is_empty() {
                if part {
                    // convert vec to number
                    let num: u32 = num
                        .iter()
                        .map(|&num| num.to_string())
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    sum += num;
                    part = false;
                }
                num.clear();
            }
        }
    }
    sum
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
    assert_eq!(part1(parse()), 560670);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(parse()), 0);
// }
