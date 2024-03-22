#[derive(PartialEq)]
enum Outcome {
    Lost = 0,
    DRAW = 1,
    Win = 3,
}

enum PlayerIndex {
    FIRST,
    SECOND,
}
// let tab_string = "\t".repeat(7);
// let my_string = format!("Hello{}World", tab_string);

pub fn tally(match_results: &str) -> String {
    let games_played = match_results.split('\n').collect::<Vec<&str>>();

    let mut vec: Vec<String> = Vec::new();
    for game in games_played {
        if !game.contains(";") {
            break;
        }

        let scores = find_game_outcome(game);

        let mut results: Vec<u32> = vec![0, 0, 0, 0, 0];


        let winner_results = get_results(scores.first_player, results.clone());
        let loser_results =  get_results(scores.second_player, results.clone());

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

// A win earns a team 3 points. A draw earns 1. A loss earns 0.
struct Player {
    name: String,
    points: i32,
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
        Outcome::DRAW => {
            results[2] = results[2] + new_match;
            results[4] = results[4] + 1;
        }
    }

    let result_row = format!(
        "{:<30} |  {:^2}|  {:^2}|  {:^2}|  {:^2}|  {:^1}",
        player.name, results[0], results[1], results[2], results[3], results[4]
    );
    println!("{:?} won", result_row);
    return result_row;
}

fn get_team_by_index(game: &str, i: PlayerIndex) -> String {
    let split_string = game.split(";").collect::<Vec<&str>>();
    let index = match i {
        PlayerIndex::FIRST => 0,
        PlayerIndex::SECOND => 1,
    };

    return split_string[index].to_string();
}
fn find_winner(row: &str, outcome: bool) -> String {
    let split_string = row.split(";").collect::<Vec<&str>>();

    println!("{:?}", split_string);

    if outcome == true {
        println!("{:?}", split_string);

        return split_string[0].to_string();
    }

    return split_string[1].to_string();
}

fn find_game_outcome(game: &str) -> PlayerScores {
    let split_string = game.split(";").collect::<Vec<&str>>();
    let first_player = split_string[0].to_string();
    let second_player = split_string[1].to_string();
    // let first_team = get_team_by_index(game, PlayerIndex::FIRST);
    // let second_r= get_team_by_index(game, PlayerIndex::SECOND);
    let outcome = match split_string[2] {
        "win" => Outcome::Win,
        "draw" => Outcome::DRAW,
        _ => Outcome::Lost,
    };

    let points: PlayerScores = match outcome {
        Outcome::Win => PlayerScores {
            first_player: Player {
                name: first_player,
                points: 3,
                outcome: Outcome::Win
            },
            second_player: Player {
                name: second_player,
                points: 0,
                outcome: Outcome::Lost
            }
        },
        Outcome::Lost =>PlayerScores {
            first_player: Player {
                name: first_player,
                points: 0,
                outcome: Outcome::Lost
            },
            second_player: Player {
                name: second_player,
                points: 3,
                outcome: Outcome::Win
            }
        },
        Outcome::DRAW => PlayerScores {
            first_player: Player {
                name: first_player,
                points: 1,
                outcome: Outcome::DRAW
            },
            second_player: Player {
                name: second_player,
                points: 0,
                outcome: Outcome::DRAW
            }
        },
    };

    return points
}

// fn main() {
//   let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;win"];

//   // Split the input string into a vector of team names
//   let teams: Vec<&str> = input[0].split(';').collect();

//   // Create a vector of vectors to store the match results for each team
//   let mut results: Vec<Vec<usize>> = vec![vec![0; 5]; teams.len()];

//   // Populate the match results based on the given input
//   results[0][0] = 1; // MP for Allegoric Alaskans
//   results[0][1] = 1; // W for Allegoric Alaskans
//   results[0][4] = 3; // P for Allegoric Alaskans

//   results[1][0] = 1; // MP for Blithering Badgers
//   results[1][3] = 1; // L for Blithering Badgers

//   // Build the output string
//   let header = format!("{:<30} | {:^2} | {:^2} | {:^2} | {:^2} | {:^2}",
//                        "Team", "MP", "W", "D", "L", "P");
//   let mut output = vec![header];

//   for (i, team) in teams.iter().enumerate() {
//       let result_row = format!("{:<30} | {:^2} | {:^2} | {:^2} | {:^2} | {:^2}",
//                               team, results[i][0], results[i][1], results[i][2], results[i][3], results[i][4]);
//       output.push(result_row);
//   }

//   // Print the output string
//   for row in output {
//       println!("{}", row);
//   }
// }
