fn main() {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let input: &str = include_str!("input.txt");

    let mut games = Vec::new();
    for line in input.lines() {
        let g = parse_game(line);
        games.push(g);
    }

    for game in &games {
        println!("Game {}", game.number);
        for round in &game.rounds {
            println!("Round: blue {}, green {}, red {}", round.blue, round.green, round.red);
        }
    }

    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut sum = 0;
    'outer: for game in &games {
        for round in &game.rounds {
            if round.red > red {
                continue 'outer;
            }
            if round.blue > blue {
                continue 'outer;
            }
            if round.green > green {
                continue 'outer;
            }
        }
        println!("found valid game {}", game.number);
        sum += game.number;
    }

    println!("sum {}", sum);

    let mut sum_of_powers = 0;
    for game in &games {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for round in &game.rounds {
            if round.red > max_red {
                max_red = round.red;
            }
            if round.blue > max_blue {
                max_blue = round.blue;
            }
            if round.green > max_green {
                max_green = round.green;
            }
        }

        sum_of_powers += max_red * max_blue * max_green;
    }

    println!("sum of powers is {}", sum_of_powers);
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(":");
    let game_string = parts.nth(0).unwrap();
    let game_number: u32 = game_string.split(" ").nth(1).unwrap().parse().unwrap();

    let mut rounds = Vec::new();
    for round_raw in parts.nth(0).unwrap().split(";") {
        rounds.push(parse_round(round_raw));
    }

    Game { number: game_number, rounds }
}

fn parse_round(input: &str) -> Round {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;
    for color_line in input.split(",").map(str::trim) {
        let amount: u32 = color_line.split(" ").nth(0).unwrap().trim().parse().unwrap();
        let color = color_line.split(" ").nth(1);
        match color {
            Some("blue") => blue = amount,
            Some("green") => green = amount,
            Some("red") => red = amount,
            _ => panic!("Unknown color"),
        }
    }
    Round { blue, green, red }
}

struct Game {
    number: u32,
    rounds: Vec<Round>,
}

struct Round {
    blue: u32,
    green: u32,
    red: u32,
}
