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
                cubes.push((str[0].parse::<i32>().unwrap(), str[1].to_string()));
            }
            sets.push(cubes);
        }
        games.push(sets);
    }
    games
}

fn part1(games: &[Game]) -> i32 {
    let reds = 12;
    let greens = 13;
    let blues = 14;
    let mut sum = 0;
    let mut game_num = 1;

    for game in games {
        let mut impossible = false;
        for sets in game {
            for (num, color) in sets {
                if (color == "red" && num > &reds)
                    || (color == "green" && num > &greens)
                    || (color == "blue" && num > &blues)
                {
                    impossible = true;
                }
            }
        }
        if !impossible {
            sum += game_num;
        }
        game_num += 1;
    }
    sum
}

fn part2(games: &[Game]) -> i32 {
    let mut sum = 0;
    for game in games {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;
        for sets in game {
            for (num, color) in sets {
                if color == "red" && num > &reds {
                    reds = *num;
                }
                if color == "green" && num > &greens {
                    greens = *num;
                }
                if color == "blue" && num > &blues {
                    blues = *num;
                }
            }
        }
        sum += reds * greens * blues;
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
