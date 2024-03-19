use std::{collections::HashMap};

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut mapper: HashMap<String, u32> = HashMap::new();

    let space_bin = words.to_lowercase().split(' ').collect::<Vec<&str>>().join(" ");
    let n_bind = space_bin.split("\n").collect::<Vec<&str>>().join(" ");

    let comma_bin = n_bind.split(",");

    let mut words_without_specs = comma_bin.collect::<Vec<&str>>().join(" ").to_string();


    words_without_specs.retain(|c| c != ',' && c != '.' && c != ':' );

    let new_word = remove_single_quotes(words_without_specs).join(" ");
    let new_words = remove_single_quotes(new_word).join(" ");

    println!("{:?}", new_words);

    for word in new_words.split(" ") {

        let mut parsed_word = word;
        if word.to_string().starts_with("'"){
            parsed_word =   word.trim_start()
        }
        if word.to_string().ends_with("&"){
            parsed_word =   word.split("!").collect::<Vec<&str>>()[0];
        }
       if word.len() > 0 {
        if mapper.contains_key(parsed_word){
            let entry = mapper.get_key_value(parsed_word).unwrap();
 
            let new_entry = entry.1 + 1;
 
            mapper.insert(parsed_word.to_owned(), new_entry);
         } else{
             mapper.insert(parsed_word.to_string(), 1);
         }
       }
    }


    println!("{:?}", mapper);


    mapper

}


fn remove_single_quotes(words: String) -> Vec<String> {
    let mut result = Vec::new();

    for word in words.split(" ") {
        // Check if the word starts or ends with the character '
        let start = word.starts_with(&"'");
        let end = word.ends_with(&"'");

        // Remove the character ' from the beginning or the end of the word, but only if it is the first or last character
        let word = if start && word.len() > 1 {
            &word[1..]
        } else if end && word.len() > 1 {
            &word[..word.len()-1]
        } else {
            word
        };

        result.push(word.to_string());
    }

    result
}