use std::fs;

#[derive(Debug)]
struct CubeGrab {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    grabs: Vec<CubeGrab>,
}


fn build_games(input: &String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for game_line in input.lines() {
        let mut game = Game {
            id: 0,
            grabs: Vec::new(),
        };

        let halves: Vec<&str> = game_line.split(": ").collect();
        // get game id from first half
        let id_str = &halves[0][5..];
        game.id = id_str.parse().unwrap();

        // parse out grabs
        let raw_grabs: Vec<&str> = halves[1].split("; ").collect();
        for raw_grab in raw_grabs.iter() {
            let mut grab = CubeGrab {
                red: 0,
                green: 0,
                blue: 0,
            };
            let cubes: Vec<&str> = raw_grab.split(", ").collect();
            for cube in cubes.iter() {
                let halves: Vec<&str> = cube.split(" ").collect();
                match halves[1] {
                    "red" => grab.red = halves[0].parse().unwrap(),
                    "green" => grab.green = halves[0].parse().unwrap(),
                    "blue" => grab.blue = halves[0].parse().unwrap(),
                    _ => panic!(),
                };
            }

            game.grabs.push(grab);
        }


        games.push(game);
    }

    games
}

fn part1(input: &String) {
    let games = build_games(input);

    let max_cubes = CubeGrab {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut id_sum = 0;

    for game in games {
        let mut game_possible = true;

        'grab: for grab in game.grabs {
            
            if grab.red > max_cubes.red ||
                grab.green > max_cubes.green ||
                grab.blue > max_cubes.blue {
                game_possible = false;
                break 'grab;
            }
        }

        if game_possible {
            id_sum += game.id;
        }
    }

    println!("possible games sum {}", id_sum);
}

// ---

fn min_playable_cubes(game: &Game) -> CubeGrab {
    let min_red = game.grabs.iter().map(|grab| grab.red).max().unwrap_or(0);
    let min_green = game.grabs.iter().map(|grab| grab.green).max().unwrap_or(0);
    let min_blue = game.grabs.iter().map(|grab| grab.blue).max().unwrap_or(0);

    let ret = CubeGrab {
        red: min_red,
        green: min_green,
        blue: min_blue,
    };
//    println!("{:?}", ret);
    ret
}

fn part2(input: &String) {
    let games = build_games(input);

    let mut power_sum = 0;

    for game in games {
        let min_cubes = min_playable_cubes(&game);
        power_sum += min_cubes.red * min_cubes.green * min_cubes.blue;
    }

    println!("power sum: {}", power_sum);
}

fn main() {

    let contents = fs::read_to_string("./input.txt").expect("file should exist");
//    let contents = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

    part2(&contents);
}
