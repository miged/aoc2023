fn parse() -> Vec<String> {
    include_str!("../inputs/01.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(_: &[String]) -> isize {
    todo!();
}

fn _part2(_: &[String]) -> isize {
    todo!();
}

pub fn main() {
    let input = parse();
    println!("DxP1 result: {}", part1(&input));
    //println!("DxP2 result: {}", part2(&input));
}

// #[test]
// fn test_p1() {
//     assert_eq!(part1(&parse()), 0);
// }

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse()), 0);
// }
