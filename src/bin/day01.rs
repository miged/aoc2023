fn parse() -> Vec<String> {
    include_str!("../inputs/01.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(input: &[String]) -> isize {
    let mut values: Vec<isize> = vec![];
    for string in input {
        // get first char in string
        let v1 = find_first_number(string);
        let v2: String = string.chars().rev().collect();
        let v2 = find_first_number(&v2);
        values.push(format!("{}{}", v1, v2).parse().unwrap());
    }
    values.iter().sum()
}

fn part2(input: &[String]) -> isize {
    todo!();
}

fn find_first_number(input: &str) -> char {
    for c in input.chars() {
        if c.is_numeric() {
            return c;
        }
    }
    '0'
}

pub fn main() {
    let input = parse();
    println!("D1P1 result: {}", part1(&input));
    println!("D1P2 result: {}", part2(&input));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse()), 56042);
}

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse()), 0);
// }
