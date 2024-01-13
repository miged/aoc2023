type Cube = (i32, String);
type Set = Vec<Cube>;
type Game = Vec<Set>;

fn parse() -> Vec<Game> {
    let lines: Vec<String> = include_str!("../inputs/02.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut games: Vec<Game> = vec![];
    for mut line in lines {
        line = line[line.find(':').unwrap() + 2..].to_string();
        let split: Vec<&str> = line.split(';').collect();
        let mut sets: Game = vec![];
        for set in split {
            let split2: Vec<&str> = set.trim().split(',').collect();
            let mut cubes: Set = vec![];
            for str in split2 {
                let str: Vec<&str> = str.trim().split(' ').collect();
                let cube: Cube = (str[0].parse::<i32>().unwrap(), str[1].to_owned());
                cubes.push(cube);
            }
            sets.push(cubes);
        }
        games.push(sets);
    }

    games
}

fn part1(games: &[Game]) -> isize {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let mut sum = 0;
    let mut game_number = 1;

    for game in games {
        let mut impossible = false;
        for sets in game {
            for (number, color) in sets {
                if (color == "red" && number > &red_cubes)
                    || (color == "green" && number > &green_cubes)
                    || (color == "blue" && number > &blue_cubes)
                {
                    impossible = true;
                }
            }
        }
        if !impossible {
            sum += game_number;
        }
        game_number += 1;
    }
    sum
}

fn _part2(_: &[String]) -> isize {
    todo!();
}

pub fn main() {
    let input = parse();
    println!("D2P1 result: {}", part1(&input));
    //println!("D2P2 result: {}", part2(&input));
}

// #[test]
// fn test_p1() {
//     assert_eq!(part1(&parse()), 2685);
// }

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse()), 0);
// }
