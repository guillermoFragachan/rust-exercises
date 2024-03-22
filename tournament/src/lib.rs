pub fn tally(match_results: &str) -> String {
  
  
  let teams = match_results.split('\n').collect::<Vec<&str>>();

  let mut vec: Vec<String> = Vec::new();



  


  for team in teams {



    if !team.contains(";") {
      break;
    }


    let outcome = find_game_outcome(team);


    let mut results: Vec<u32> = [0, 0, 0, 0, 0].to_vec();

    let winner = find_winner(team, outcome);
    let loser = find_winner(team, !outcome);
    let winner_results = get_results(winner, true,results);
    vec.push(value);
    let loser_results = get_results(loser, true, results);

    
    

    println!("{} won", winner);

    println!("{} lost", loser);

  }

  
  let headers = "Team                           | MP |  W |  D |  L |  P".to_string();
  

  return vec.join("\n");

}

// A win earns a team 3 points. A draw earns 1. A loss earns 0.




fn get_results (player: String, won: bool, results:Vec<u32> ) -> String {

  let winner_points: u32 = 3;
  let _draw_points: u32 = 1;

  let new_match : u32= 1;
  let _draw_matchh: u32 = 1;


  let _ = results[0] + new_match;

  if won {
    let _ = results[1] + new_match;
    // results[2] + new_match;
    let _ = results[4] + winner_points;

    let result_row = format!("{:<30} | {:^2} | {:^2} | {:^2} | {:^2} | {:^2}",
    player, results[0], results[1], results[2], results[3], results[4]);

    return result_row
  }

  let _ = results[3] + new_match;

  let result_row = format!("{:<30} | {:^2} | {:^2} | {:^2} | {:^2} | {:^2}",
  player, results[0], results[1], results[2], results[3], results[4]);

  return result_row
}


fn find_winner(row: &str, outcome:bool)  ->  String{ 
  let split_string = row.split(";").collect::<Vec<&str>>();

  println!("{:?}", split_string);

  if outcome == true {
  println!("{:?}", split_string);

   return split_string[0].to_string();
  }

return split_string[1].to_string();

}

fn find_game_outcome (row: &str)  -> bool {

    let split_string = row.split(";").collect::<Vec<&str>>();

  println!("{:?}", split_string);

  let outcome = split_string[2];



  if outcome == "win" {
    return true
  }

  return false
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
