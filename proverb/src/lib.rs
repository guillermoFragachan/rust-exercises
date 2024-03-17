use std::fmt::format;

pub fn build_proverb(list: &[&str]) -> String {
let mut proverb: String = String::new();


if list.len() > 0{
  // println!("len {}", list[0].to_string());
  proverb = builder(list) ;
}
return proverb
}


fn builder(list: &[&str]) -> String {
  let mut reversed_list = reverse_vector(list);
  let mut proverb_list: Vec<String> = Vec::new();
  let first_line = format!("And all for the want of a {}.", list[0]);
  
  let mut prev_line = list[0].to_string();

  reversed_list.reverse();

  for line in reversed_list {
      if line == "nail" {
          continue;
      }
      let full_line = format!("For want of a {} the {} was lost.", prev_line, line);
      prev_line = line.to_string();
      proverb_list.push(full_line);
  }


  proverb_list.push(first_line);

  let new_line = proverb_list.join("\n");

  return new_line;
}


fn reverse_vector (list: &[&str]) -> Vec<String> {
  let mut vector = Vec::new();

  for string in list {
    vector.push(string.to_string())
  }
  vector.reverse();
  vector.pop();


  return vector
}


// #[test]
// #[ignore]
// fn two_pieces() {
//     let input = &["nail", "shoe"];
//     let output = proverb::build_proverb(input);
//     let expected: String = [
//         "For want of a nail the shoe was lost.",
//         "And all for the want of a nail.",
//     ]
//     .join("\n");
//     assert_eq!(output, expected);
// }
