fn parse() -> Vec<String> {
    include_str!("../inputs/01.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(_: Vec<String>) -> i32 {
    todo!();
}

fn _part2(_: Vec<String>) -> i32 {
    todo!();
}

pub fn main() {
    println!("DxP1 result: {}", part1(parse()));
    //println!("DxP2 result: {}", part2(parse()));
}

// #[test]
// fn test_p1() {
//     assert_eq!(part1(parse()), 0);
// }

// #[test]
// fn test_p2() {
//     assert_eq!(part2(parse()), 0);
// }
