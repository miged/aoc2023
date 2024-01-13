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
                let (num, color) = str.trim().split_once(' ').unwrap();
                cubes.push((num.parse::<i32>().unwrap(), color.to_string()));
            }
            sets.push(cubes);
        }
        games.push(sets);
    }
    games
}

fn part1(games: &[Game]) -> i32 {
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;
    let mut game_num = 0;

    'outer: for game in games {
        game_num += 1;
        for sets in game {
            for (num, color) in sets {
                if (color == "red" && num > &red)
                    || (color == "green" && num > &green)
                    || (color == "blue" && num > &blue)
                {
                    continue 'outer;
                }
            }
        }
        sum += game_num;
    }
    sum
}

fn part2(games: &[Game]) -> i32 {
    let mut sum = 0;
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for sets in game {
            for (num, color) in sets {
                match color.as_str() {
                    "red" => red = red.max(*num),
                    "green" => green = green.max(*num),
                    "blue" => blue = blue.max(*num),
                    _ => (),
                };
            }
        }
        sum += red * green * blue;
    }
    sum
}

pub fn main() {
    let input = parse();
    println!("D2P1 result: {}", part1(&input));
    println!("D2P2 result: {}", part2(&input));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse()), 2685);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse()), 83707);
}
