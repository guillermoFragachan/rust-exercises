use std::collections::HashMap;

#[derive(PartialEq)]
enum Outcome {
    Lost,
    Draw,
    Win
}
#[derive(Default, Eq, PartialEq)]
struct PlayerResults {
    name: String,
    results: Vec<u32>,
}

impl PlayerResults {
    fn new(name: String) -> Self {
        Self {
            name,
            results: HashMap::new(),
        }
    }

    fn add_result(&mut self, key: String, value: i32) {
        self.results.insert(key, value);
    }

    fn get_result(&self, key: &str) -> Option<&i32> {
        self.results.get(key)
    }
}


pub fn tally(match_results: &str) -> String {
    let games_played = match_results.split('\n').collect::<Vec<&str>>();

    let mut vec: Vec<String> = Vec::new();
    let mut scores: HashMap<String, PlayerResults> = HashMap::new();
   
    for game in games_played {
        if !game.contains(";") {
            break;
        }

        let game = find_game_outcome(game);
        // let player_names = 
        let mut results: Vec<u32> = vec![0, 0, 0, 0, 0];


        scores
        .entry(game.first_player.name.into())
        .or_insert(PlayerResults::new(game.first_player.name.into()))
        .add_result(result);

        let winner_results = get_results(game.first_player, results.clone());
        let loser_results =  get_results(game.second_player, results.clone());

        vec.push(winner_results);
        vec.push(loser_results);
        println!("{:?} 2", vec.clone());
    }

    vec.sort_by(|a, b| {
        let a_last = a.split_whitespace().last().unwrap().parse::<i32>().unwrap();
        let b_last = b.split_whitespace().last().unwrap().parse::<i32>().unwrap();
        b_last.cmp(&a_last)
    });
    vec.insert(0 ,"Team                           | MP |  W |  D |  L |  P".to_string());

    return vec.join("\n");

}

struct Player {
    name: String,
    outcome: Outcome
}
struct PlayerScores {
    first_player: Player,
    second_player: Player,
}
fn get_results(player: Player, mut results: Vec<u32>) -> String {
    let new_match: u32 = 1;
    results[0] = results[0] + new_match;


    match player.outcome {
        Outcome::Win => {
            results[1] = results[1] + new_match;
            results[4] = results[4] + 3;
        },
        Outcome::Lost => {
            results[3] = results[3] + new_match;

            results[4] = results[4];
        },
        Outcome::Draw => {
            results[2] = results[2] + new_match;
            results[4] = results[4] + 1;
        }
    }

    let result_row = format!(
        "{:<30} |  {:^2}|  {:^2}|  {:^2}|  {:^2}|  {:^1}",
        player.name, results[0], results[1], results[2], results[3], results[4]
    );
    println!("{:?} won", result_row);
    return results;
}


fn build_row(player: Player, mut results: Vec<u32>) -> String {
    let result_row = format!(
        "{:<30} |  {:^2}|  {:^2}|  {:^2}|  {:^2}|  {:^1}",
        player.name, results[0], results[1], results[2], results[3], results[4]
    );
    println!("{:?} won", result_row);
    return result_row;
}

fn find_game_outcome(game: &str) -> PlayerScores {
    let split_string = game.split(";").collect::<Vec<&str>>();
    let first_player = split_string[0].to_string();
    let second_player = split_string[1].to_string();
    let outcome = match split_string[2] {
        "win" => Outcome::Win,
        "draw" => Outcome::Draw,
        _ => Outcome::Lost,
    };

    let points: PlayerScores = match outcome {
        Outcome::Win => PlayerScores {
            first_player: Player {
                name: first_player,
                outcome: Outcome::Win
            },
            second_player: Player {
                name: second_player,
                outcome: Outcome::Lost
            }
        },
        Outcome::Lost =>PlayerScores {
            first_player: Player {
                name: first_player,
                outcome: Outcome::Lost
            },
            second_player: Player {
                name: second_player,
                outcome: Outcome::Win
            }
        },
        Outcome::Draw => PlayerScores {
            first_player: Player {
                name: first_player,
                outcome: Outcome::Draw
            },
            second_player: Player {
                name: second_player,
                outcome: Outcome::Draw
            }
        },
    };

    return points
}
